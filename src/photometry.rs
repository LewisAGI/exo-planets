//! Extra-dip / residual photometry flags from a real light curve.
//!
//! This is **not** LUNA and **not** a moon detection. HEK V: photometry-only
//! (ignoring timing) would have falsely claimed moons in 1/4 of KOIs because
//! of correlated noise. A high extra-dip SNR is a caution flag.

use crate::constants::HEK_I_SIGMA_THRESHOLD;
use crate::geometry::TransitGeometry;
use crate::ingest::CatalogPlanet;
use crate::lightcurve::LightCurve;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhotometryFlags {
    pub lc_available: bool,
    pub n_points: usize,
    pub n_in_transit: usize,
    pub extra_dip_snr: f64,
    /// True if a photometry-only extra-dip cut would fire (HEK I 4σ scale).
    /// This is **not** a moon. HEK V says this class of cut false-claims.
    pub photometry_only_would_flag: bool,
    pub hek_v_caution: bool,
    pub mission: String,
    pub notes: String,
}

impl Default for PhotometryFlags {
    fn default() -> Self {
        Self {
            lc_available: false,
            n_points: 0,
            n_in_transit: 0,
            extra_dip_snr: 0.0,
            photometry_only_would_flag: false,
            hek_v_caution: false,
            mission: String::new(),
            notes: "no cached light curve for this host".into(),
        }
    }
}

fn median(xs: &mut [f64]) -> f64 {
    if xs.is_empty() {
        return 0.0;
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = xs.len();
    if n % 2 == 1 {
        xs[n / 2]
    } else {
        0.5 * (xs[n / 2 - 1] + xs[n / 2])
    }
}

fn mad_sigma(xs: &[f64], med: f64) -> f64 {
    if xs.len() < 8 {
        return 0.0;
    }
    let mut dev: Vec<f64> = xs.iter().map(|x| (x - med).abs()).collect();
    let mad = median(&mut dev);
    (1.4826 * mad).max(1e-12)
}

fn wrapped_phase(t: f64, t0: f64, period: f64) -> f64 {
    let mut p = (t - t0) / period;
    p -= p.floor();
    if p > 0.5 {
        p - 1.0
    } else {
        p
    }
}

/// Merge extra-dip flags across every cached extract for one planet.
pub fn photometry_flags(
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
    lcs: Option<&[LightCurve]>,
) -> PhotometryFlags {
    let Some(curves) = lcs.filter(|c| !c.is_empty()) else {
        return PhotometryFlags::default();
    };
    let mut merged = PhotometryFlags::default();
    merged.lc_available = true;
    let mut notes = Vec::new();
    let mut missions = Vec::new();
    for lc in curves {
        let one = photometry_flags_one(planet, geom, lc);
        merged.n_points += one.n_points;
        merged.n_in_transit += one.n_in_transit;
        if one.extra_dip_snr > merged.extra_dip_snr {
            merged.extra_dip_snr = one.extra_dip_snr;
        }
        if one.photometry_only_would_flag {
            merged.photometry_only_would_flag = true;
            merged.hek_v_caution = true;
        }
        missions.push(format!("{} {}", lc.mission, lc.cadence));
        notes.push(one.notes);
    }
    merged.mission = missions.join("+");
    merged.notes = notes.join(" ");
    merged
}

/// Box-subtracted extra-dip SNR on one cached LC. Flags, not a detection.
pub fn photometry_flags_one(
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
    lc: &LightCurve,
) -> PhotometryFlags {
    if lc.flux.is_empty() {
        return PhotometryFlags::default();
    }
    let mut raw = lc.flux.clone();
    let med = median(&mut raw);
    if med <= 0.0 {
        return PhotometryFlags::default();
    }
    let norm: Vec<f64> = lc.flux.iter().map(|f| f / med).collect();
    let duration_days = planet.duration_hr.or(geom.t14_hr).unwrap_or(3.0) / 24.0;
    let half_w = (duration_days / (2.0 * planet.period_days)).clamp(0.002, 0.25);

    let mut in_mask = vec![false; norm.len()];
    let mut n_in = 0usize;
    if let Some(t0) = planet.epoch_bkjd {
        for (i, &t) in lc.time_bkjd.iter().enumerate() {
            let ph = wrapped_phase(t, t0, planet.period_days);
            if ph.abs() <= half_w {
                in_mask[i] = true;
                n_in += 1;
            }
        }
    }

    let oot: Vec<f64> = norm
        .iter()
        .zip(in_mask.iter())
        .filter_map(|(&f, &inn)| if !inn { Some(f) } else { None })
        .collect();
    let oot_med = median(&mut oot.clone());
    let sigma = mad_sigma(&oot, oot_med);
    let in_vals: Vec<f64> = norm
        .iter()
        .zip(in_mask.iter())
        .filter_map(|(&f, &inn)| if inn { Some(f) } else { None })
        .collect();
    let depth = if n_in >= 3 {
        (oot_med - {
            let mut v = in_vals;
            median(&mut v)
        })
        .max(0.0)
    } else {
        0.0
    };

    // Residual after a box planet model. Extra dips are OOT (or leftover).
    let mut extra: f64 = 0.0;
    let mut run = 0i32;
    let mut best_run: f64 = 0.0;
    for (i, &f) in norm.iter().enumerate() {
        let model = if in_mask[i] { oot_med - depth } else { oot_med };
        let res = f - model;
        let snr = if sigma > 0.0 { -res / sigma } else { 0.0 };
        if !in_mask[i] && snr > 2.5 {
            run += 1;
            extra = extra.max(snr);
            if run >= 2 {
                best_run = best_run.max(snr);
            }
        } else {
            run = 0;
        }
        extra = extra.max(if in_mask[i] { 0.0 } else { snr.max(0.0) });
    }
    let extra_dip_snr = extra.max(best_run);
    let would = extra_dip_snr >= HEK_I_SIGMA_THRESHOLD && best_run > 0.0;
    let mut notes = format!(
        "{} {} cadence, {} finite PDCSAP points from {}. Extra-dip SNR is a residual flag, not a moon.",
        lc.mission, lc.cadence, lc.len(), lc.cache_file
    );
    if planet.name.eq_ignore_ascii_case("Kepler-1625 b") {
        notes.push_str(" This Q8 window does not cover a catalog transit; do not invent one. Moon stays CANDIDATE.");
    }
    if planet.name.eq_ignore_ascii_case("Kepler-1708 b") {
        notes.push_str(" This Q1 window does not cover a catalog transit (P≈737 d); do not invent one. Moon stays CANDIDATE.");
    }
    if planet.name.eq_ignore_ascii_case("Kepler-167 e") {
        notes.push_str(" This Q1 window does not cover a catalog transit (P≈1071 d); do not invent one. Status stays SEARCH.");
    }
    if planet.epoch_bkjd.is_none() {
        notes.push_str(
            " No catalog epoch in this cache; extra-dip is unwindowed (not a transit invention).",
        );
    }
    if would {
        notes.push_str(" HEK V: photometry-only (ignoring timing) would have falsely claimed moons in 1/4 of KOIs.");
    }
    PhotometryFlags {
        lc_available: true,
        n_points: lc.len(),
        n_in_transit: n_in,
        extra_dip_snr,
        photometry_only_would_flag: would,
        hek_v_caution: would,
        mission: lc.mission.clone(),
        notes,
    }
}
