//! Transit geometry: Kipping 2009b / Seager & Mallén-Ornelas (2003).
//!
//! depth δ ≈ (R_p / R_*)^2
//! impact b = a cos i / R_*
//! chord duration T_14; ingress T_12.
//!
//! Limb darkening is ignored. Grazing transits (b + k ≥ 1) are flagged.

use crate::constants::{AU_M, DAY_S, G, MEARTH_KG, MJUP_KG, MSUN_KG, REARTH_M, RSUN_M};

#[derive(Debug, Clone, PartialEq)]
pub struct TransitGeometry {
    /// (R_p / R_*)^2, dimensionless.
    pub depth: f64,
    pub depth_ppm: f64,
    pub rp_over_rstar: f64,
    /// Impact parameter b. `None` if a, i, or R_* missing and not derivable.
    pub impact_b: Option<f64>,
    /// Predicted T_14 (first–fourth contact) in hours, circular orbit.
    pub t14_hr: Option<f64>,
    /// Predicted ingress T_12 in hours.
    pub ingress_hr: Option<f64>,
    /// Catalog duration minus predicted T_14 (hours), if both exist.
    pub duration_residual_hr: Option<f64>,
    pub grazing: bool,
    pub a_over_rstar: Option<f64>,
    /// True when a was computed from Kepler's third law, not taken from the catalog.
    pub a_from_kepler3: bool,
}

/// δ ≈ (R_p / R_*)^2.
pub fn depth_from_radii(rp_m: f64, rstar_m: f64) -> f64 {
    let k = rp_m / rstar_m;
    k * k
}

pub fn rp_over_rstar_from_depth(depth: f64) -> f64 {
    depth.max(0.0).sqrt()
}

/// b = a cos i / R_*. `incl_deg` is 90 = edge-on.
pub fn impact_parameter(a_m: f64, rstar_m: f64, incl_deg: f64) -> f64 {
    let incl = incl_deg.to_radians();
    (a_m * incl.cos() / rstar_m).abs()
}

/// Circular Keplerian a from P and M_* (no planet mass). Labelled derived.
pub fn a_from_kepler3(period_s: f64, mstar_kg: f64) -> f64 {
    ((G * mstar_kg * period_s * period_s) / (4.0 * std::f64::consts::PI * std::f64::consts::PI))
        .cbrt()
}

/// Seager & Mallén-Ornelas circular T_14 (hours).
///
/// T_14 = (P/π) arcsin( (R_*/a) √((1+k)^2 − b^2) / sin i )
pub fn t14_hours(period_s: f64, a_over_rstar: f64, k: f64, b: f64, incl_deg: f64) -> Option<f64> {
    chord_hours(period_s, a_over_rstar, 1.0 + k, b, incl_deg)
}

/// Circular T_23 (second–third) if (1−k)^2 > b^2; else `None` (no flat bottom).
pub fn t23_hours(period_s: f64, a_over_rstar: f64, k: f64, b: f64, incl_deg: f64) -> Option<f64> {
    chord_hours(period_s, a_over_rstar, 1.0 - k, b, incl_deg)
}

fn chord_hours(
    period_s: f64,
    a_over_rstar: f64,
    one_pm_k: f64,
    b: f64,
    incl_deg: f64,
) -> Option<f64> {
    if a_over_rstar <= 0.0 || !one_pm_k.is_finite() {
        return None;
    }
    let inside = one_pm_k * one_pm_k - b * b;
    if inside <= 0.0 {
        return None;
    }
    let sini = incl_deg.to_radians().sin().abs();
    if sini == 0.0 {
        return None;
    }
    // (R*/a) * sqrt((1±k)^2 - b^2) / sin i
    let arg = inside.sqrt() / (a_over_rstar * sini);
    if arg.abs() >= 1.0 {
        return None;
    }
    let t = (period_s / std::f64::consts::PI) * arg.asin();
    Some(t / 3600.0)
}

/// Ingress ≈ (T_14 − T_23) / 2 when a flat bottom exists; else T_14 / 2.
pub fn ingress_hours(t14_hr: f64, t23_hr: Option<f64>) -> f64 {
    match t23_hr {
        Some(t23) if t23 > 0.0 && t14_hr > t23 => 0.5 * (t14_hr - t23),
        _ => 0.5 * t14_hr,
    }
}

pub fn earth_radii_to_m(rp_earth: f64) -> f64 {
    rp_earth * REARTH_M
}

pub fn rsun_to_m(rstar_rsun: f64) -> f64 {
    rstar_rsun * RSUN_M
}

pub fn au_to_m(a_au: f64) -> f64 {
    a_au * AU_M
}

pub fn mstar_to_kg(mstar_msun: f64) -> f64 {
    mstar_msun * MSUN_KG
}

pub fn mplanet_to_kg(mp_earth: f64) -> f64 {
    mp_earth * MEARTH_KG
}

pub fn mjup_to_kg(mp_jup: f64) -> f64 {
    mp_jup * MJUP_KG
}

pub fn period_days_to_s(p_days: f64) -> f64 {
    p_days * DAY_S
}

/// Build geometry from catalog-like optional fields. Does not invent missing
/// masses or depths; derives a from Kepler III only when a is absent but
/// P and M_* exist (flagged).
pub fn compute_geometry(
    period_days: f64,
    rp_earth: Option<f64>,
    rstar_rsun: Option<f64>,
    a_au: Option<f64>,
    mstar_msun: Option<f64>,
    incl_deg: Option<f64>,
    catalog_b: Option<f64>,
    catalog_duration_hr: Option<f64>,
    catalog_depth_ppm: Option<f64>,
) -> TransitGeometry {
    let rstar_m = rstar_rsun.map(rsun_to_m);
    let rp_m = rp_earth.map(earth_radii_to_m);
    let k = match (rp_m, rstar_m) {
        (Some(rp), Some(rs)) if rs > 0.0 => rp / rs,
        _ => catalog_depth_ppm
            .map(|ppm| rp_over_rstar_from_depth(ppm / 1.0e6))
            .unwrap_or(0.0),
    };
    let depth = k * k;
    let depth_ppm = depth * 1.0e6;

    let period_s = period_days_to_s(period_days);
    let mut a_was_kepler3 = false;
    let a_m = match a_au {
        Some(a) if a > 0.0 => Some(au_to_m(a)),
        _ => match mstar_msun {
            Some(ms) if ms > 0.0 && period_days > 0.0 => {
                a_was_kepler3 = true;
                Some(a_from_kepler3(period_s, mstar_to_kg(ms)))
            }
            _ => None,
        },
    };

    let a_over_rstar = match (a_m, rstar_m) {
        (Some(a), Some(rs)) if rs > 0.0 => Some(a / rs),
        _ => None,
    };

    let incl = incl_deg.unwrap_or(90.0);
    let impact_b = catalog_b.or_else(|| match (a_m, rstar_m) {
        (Some(a), Some(rs)) if rs > 0.0 => Some(impact_parameter(a, rs, incl)),
        _ => None,
    });

    let grazing = impact_b.map(|b| b + k >= 1.0).unwrap_or(false);

    let (t14_hr, ingress_hr) = match (a_over_rstar, impact_b) {
        (Some(ar), Some(b)) => {
            let t14 = t14_hours(period_s, ar, k, b, incl);
            let t23 = t23_hours(period_s, ar, k, b, incl);
            let ingress = t14.map(|t| ingress_hours(t, t23));
            (t14, ingress)
        }
        _ => (None, None),
    };

    let duration_residual_hr = match (catalog_duration_hr, t14_hr) {
        (Some(obs), Some(pred)) => Some(obs - pred),
        _ => None,
    };

    TransitGeometry {
        depth,
        depth_ppm,
        rp_over_rstar: k,
        impact_b,
        t14_hr,
        ingress_hr,
        duration_residual_hr,
        grazing,
        a_over_rstar,
        a_from_kepler3: a_was_kepler3,
    }
}
