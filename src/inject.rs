//! Synthetic TTV/TDV injections for **training only**.
//!
//! Injected moons are labelled `injected`. They are not observations.
//! Planet-only rows get synthetic null timing at a crude Kepler-like noise
//! scale — also not published non-detections.

use crate::constants::{HEK_LARGE_MOON_MEARTH, KEPLER_MISSION_DAYS_SYNTHETIC};
use crate::forecaster::ForecasterPrior;
use crate::geometry::TransitGeometry;
use crate::hek::evaluate_hek;
use crate::ingest::CatalogPlanet;
use crate::labels::ExampleKind;
use crate::tdv::{predict_tdv, MoonSense};
use crate::ttv::{predict_ttv, synthetic_n_transits};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoonHypothesis {
    pub ms_earth: f64,
    pub d_hill: f64,
    pub sense: MoonSense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingDraw {
    pub ttv_rms_min: f64,
    pub tdv_rms_min: f64,
    pub eta: f64,
    pub hek_i_maxdev_min: f64,
    pub moon_period_days: Option<f64>,
    pub sigma_timing_min: f64,
    pub n_transits: usize,
    pub kind: ExampleKind,
    pub hypothesis: Option<MoonHypothesis>,
    pub noise_model: String,
}

/// Crude per-transit timing noise (minutes). Training device, not a catalog σ.
pub fn synthetic_sigma_timing_min(geom: &TransitGeometry, period_days: f64) -> f64 {
    let depth = geom.depth.max(1e-8);
    let dur = geom.t14_hr.or(None).unwrap_or(2.0).max(0.3);
    // White-noise-ish: brighter / deeper / longer transits time better.
    let sigma =
        0.8 * (1.0e-3 / depth).sqrt() * (2.0 / dur).sqrt() * (period_days / 10.0).powf(0.15);
    sigma.clamp(0.15, 8.0)
}

fn rms_of_zero_mean<R: Rng>(rng: &mut R, sigma: f64, n: usize) -> f64 {
    let n = n.max(2);
    let mut sumsq = 0.0;
    for _ in 0..n {
        let u1: f64 = rng.gen::<f64>().clamp(1e-12, 1.0);
        let u2: f64 = rng.gen::<f64>();
        let z = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        sumsq += (z * sigma).powi(2);
    }
    (sumsq / n as f64).sqrt()
}

pub fn draw_planet_only<R: Rng>(
    rng: &mut R,
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
) -> TimingDraw {
    let n = synthetic_n_transits(planet.period_days, KEPLER_MISSION_DAYS_SYNTHETIC);
    let sigma = synthetic_sigma_timing_min(geom, planet.period_days);
    let ttv = rms_of_zero_mean(rng, sigma, n);
    // Duration noise is typically a few× worse than mid-time.
    let tdv = rms_of_zero_mean(rng, sigma * 2.5, n);
    let eta = if ttv > 1e-8 { tdv / ttv } else { 0.0 };
    TimingDraw {
        ttv_rms_min: ttv,
        tdv_rms_min: tdv,
        eta,
        hek_i_maxdev_min: 0.0,
        moon_period_days: None,
        sigma_timing_min: sigma,
        n_transits: n,
        kind: ExampleKind::PlanetOnly,
        hypothesis: None,
        noise_model: "synthetic white timing; not a published TTV catalog".into(),
    }
}

pub fn draw_injected<R: Rng>(
    rng: &mut R,
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
    prior: &ForecasterPrior,
    hypo: MoonHypothesis,
    a_au: f64,
) -> Option<TimingDraw> {
    if hypo.ms_earth < HEK_LARGE_MOON_MEARTH {
        // Keep injections on the HEK "large moon" side of 0.1 M⊕.
        return None;
    }
    let ttv = predict_ttv(
        planet.period_days,
        a_au,
        prior.mass_used_earth,
        planet.mstar_msun.unwrap_or(1.0),
        hypo.ms_earth,
        hypo.d_hill,
    );
    let t14 = geom.t14_hr.or(planet.duration_hr)?;
    let a_m = crate::geometry::au_to_m(a_au);
    let rstar_m = crate::geometry::rsun_to_m(planet.rstar_rsun.unwrap_or(1.0));
    let b = geom.impact_b.unwrap_or(0.3);
    let tdv = predict_tdv(
        &ttv,
        t14,
        planet.period_days,
        a_m,
        rstar_m,
        geom.rp_over_rstar,
        b,
        geom.ingress_hr,
        hypo.sense,
    );
    let n = synthetic_n_transits(planet.period_days, KEPLER_MISSION_DAYS_SYNTHETIC);
    let sigma = synthetic_sigma_timing_min(geom, planet.period_days);
    let ttv_obs = (ttv.dttv_rms_min.powi(2) + rms_of_zero_mean(rng, sigma, n).powi(2)).sqrt();
    let tdv_obs =
        (tdv.tdv_combined_rms_min.powi(2) + rms_of_zero_mean(rng, sigma * 2.5, n).powi(2)).sqrt();
    let eta = if ttv_obs > 1e-8 {
        tdv_obs / ttv_obs
    } else {
        0.0
    };
    Some(TimingDraw {
        ttv_rms_min: ttv_obs,
        tdv_rms_min: tdv_obs,
        eta,
        hek_i_maxdev_min: ttv.hek_i_maxdev_min,
        moon_period_days: Some(ttv.moon_period_days),
        sigma_timing_min: sigma,
        n_transits: n,
        kind: ExampleKind::Injected,
        hypothesis: Some(hypo),
        noise_model: "Kipping circular TTV/TDV + synthetic white noise; labelled injected".into(),
    })
}

pub fn default_injections() -> Vec<MoonHypothesis> {
    vec![
        MoonHypothesis {
            ms_earth: 0.3,
            d_hill: 0.25,
            sense: MoonSense::Prograde,
        },
        MoonHypothesis {
            ms_earth: 3.0,
            d_hill: 0.40,
            sense: MoonSense::Prograde,
        },
    ]
}

pub fn hek_for_draw(
    planet: &CatalogPlanet,
    draw: &TimingDraw,
    photometry_only: bool,
) -> crate::hek::HekFlags {
    let (d, ms, sense) = match &draw.hypothesis {
        Some(h) => (h.d_hill, h.ms_earth, h.sense),
        None => (0.0, 0.0, MoonSense::Prograde),
    };
    evaluate_hek(
        d,
        planet.period_days,
        ms,
        sense,
        draw.ttv_rms_min,
        draw.sigma_timing_min,
        draw.n_transits,
        draw.eta,
        photometry_only,
    )
}
