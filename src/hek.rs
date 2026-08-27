//! HEK / LUNA-style **flags and proxies**. This is not LUNA.
//!
//! LUNA integrates 3-body sky motion + overlapping discs. Detection in HEK
//! is a Bayes factor (planet+moon vs planet-only). HEK I used a 4σ threshold
//! and 90% upper limits on nulls. HEK II–V are **all null**. HEK V caution:
//! photometry-only (ignoring timing) would have falsely claimed moons in 1/4
//! of KOIs because of correlated noise. HEK VI stacked 284 KOIs: η < 0.38
//! (95%), a dearth not a detection; Bayes factor ~2 is a hint.
//!
//! Dynamical cuts: P_S ≤ P_B / √3 and D ≤ D_max
//! (0.4895 prograde, 0.9309 retrograde; Domingos 2006).

use crate::constants::{
    DMAX_PROGRADE, DMAX_RETROGRADE, HEK_I_SIGMA_THRESHOLD, HEK_LARGE_MOON_MEARTH,
    HEK_VI_ETA_95_UPPER, HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION,
};
use crate::tdv::MoonSense;
use crate::ttv::{hill_sphere_period_days, moon_period_days};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HekFlags {
    pub d_hill: f64,
    pub d_within_prograde_dmax: bool,
    pub d_within_retrograde_dmax: bool,
    pub moon_period_inside_hill_cut: bool,
    pub large_moon_scale: bool,
    /// Proxy only. Not a LUNA / HEK Bayes factor.
    pub bayes_proxy: BayesProxy,
    pub photometry_only_caution: bool,
    pub hek_v_false_claim_fraction: f64,
    pub eta_above_hek_vi_stack: bool,
    pub hek_ii_to_v_are_null: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BayesProxy {
    pub sigma: f64,
    pub n_transits: usize,
    pub log10_bf_proxy: f64,
    pub above_hek_i_4sigma: bool,
    pub method: String,
}

/// χ²-style proxy: z = δTTV_rms / σ_timing.
/// log10 BF_proxy ≈ (z² − k ln n) / (2 ln 10) with k=2 (amp+phase).
/// This is **not** HEK's photometric Bayes factor.
pub fn bayes_proxy_from_ttv(
    dttv_rms_min: f64,
    sigma_timing_min: f64,
    n_transits: usize,
) -> BayesProxy {
    let sigma = if sigma_timing_min > 0.0 {
        dttv_rms_min / sigma_timing_min
    } else {
        0.0
    };
    let n = n_transits.max(2) as f64;
    let k = 2.0;
    let delta_bic = sigma * sigma - k * n.ln();
    let log10_bf = delta_bic / (2.0 * std::f64::consts::LN_10);
    BayesProxy {
        sigma,
        n_transits: n_transits.max(2),
        log10_bf_proxy: log10_bf,
        above_hek_i_4sigma: sigma >= HEK_I_SIGMA_THRESHOLD,
        method: "timing-RMS chi2 proxy (amp+phase BIC); not LUNA photodynamics".into(),
    }
}

pub fn dynamical_ok(d_hill: f64, period_days: f64, sense: MoonSense) -> (bool, bool, bool) {
    let ps = moon_period_days(period_days, d_hill);
    let hill_p = hill_sphere_period_days(period_days);
    let inside_hill = ps <= hill_p + 1e-12;
    let pro = d_hill <= DMAX_PROGRADE + 1e-12;
    let ret = d_hill <= DMAX_RETROGRADE + 1e-12;
    let d_ok = match sense {
        MoonSense::Prograde => pro,
        MoonSense::Retrograde => ret,
    };
    (inside_hill && d_ok, pro, ret)
}

pub fn evaluate_hek(
    d_hill: f64,
    period_days: f64,
    ms_earth: f64,
    _sense: MoonSense,
    dttv_rms_min: f64,
    sigma_timing_min: f64,
    n_transits: usize,
    eta: f64,
    photometry_only: bool,
) -> HekFlags {
    let ps = moon_period_days(period_days, d_hill);
    let hill_p = hill_sphere_period_days(period_days);
    let mut notes = vec![
        "HEK II–V are ALL NULL. Do not cite them as detections.".into(),
        "HEK VI η < 0.38 (95%) on 284 stacked KOIs is a dearth, not a detection.".into(),
        "Bayes factor here is a timing-RMS proxy, not LUNA.".into(),
    ];
    if photometry_only {
        notes.push(format!(
            "HEK V caution: photometry-only would have falsely claimed moons in {:.0}% of KOIs.",
            HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION * 100.0
        ));
    }
    HekFlags {
        d_hill,
        d_within_prograde_dmax: d_hill <= DMAX_PROGRADE + 1e-12,
        d_within_retrograde_dmax: d_hill <= DMAX_RETROGRADE + 1e-12,
        moon_period_inside_hill_cut: ps <= hill_p + 1e-12,
        large_moon_scale: ms_earth + 1e-12 >= HEK_LARGE_MOON_MEARTH,
        bayes_proxy: bayes_proxy_from_ttv(dttv_rms_min, sigma_timing_min, n_transits),
        photometry_only_caution: photometry_only,
        hek_v_false_claim_fraction: HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION,
        eta_above_hek_vi_stack: eta > HEK_VI_ETA_95_UPPER,
        hek_ii_to_v_are_null: true,
        notes,
    }
}
