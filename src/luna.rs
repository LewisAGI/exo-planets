//! LUNA-**style flags**. This is not a LUNA port.
//!
//! LUNA (Kipping) integrates 3-body sky motion + overlapping discs.
//! We only raise geometric flags: can a moon disc overlap the planet disc
//! (syzygy), and can the moon add an extra dip on the star. No 3-body
//! integrator, no overlapping-disc photometry, no HEK Bayes factor.

use crate::forecaster::ForecasterPrior;
use crate::geometry::{au_to_m, earth_radii_to_m, rsun_to_m, TransitGeometry};
use crate::ingest::CatalogPlanet;
use crate::inject::MoonHypothesis;
use crate::ttv::{hill_radius_m, moon_period_days};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunaStyleFlags {
    pub overlapping_disc_possible: bool,
    pub syzygy_in_transit_possible: bool,
    pub extra_dip_on_star_possible: bool,
    pub moon_k: f64,
    pub a_s_over_rstar: f64,
    /// (R_p + R_m) / a_S — brief-syzygy scale, not a LUNA light curve.
    pub overlap_over_as: f64,
    pub method: String,
}

impl Default for LunaStyleFlags {
    fn default() -> Self {
        Self {
            overlapping_disc_possible: false,
            syzygy_in_transit_possible: false,
            extra_dip_on_star_possible: false,
            moon_k: 0.0,
            a_s_over_rstar: 0.0,
            overlap_over_as: 0.0,
            method: "no moon hypothesis; geometric LUNA-style flags off".into(),
        }
    }
}

/// R/R⊕ ≈ M^{0.28}. Discretized mass–radius, **not** official FORECASTER, not LUNA.
pub fn moon_radius_earth_extrap(ms_earth: f64) -> f64 {
    ms_earth.max(1e-6).powf(0.28)
}

pub fn luna_style_flags(
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
    prior: &ForecasterPrior,
    hypo: Option<&MoonHypothesis>,
    a_au: Option<f64>,
) -> LunaStyleFlags {
    let Some(h) = hypo else {
        return LunaStyleFlags::default();
    };
    let Some(a_au) = a_au else {
        return LunaStyleFlags::default();
    };
    let Some(rstar) = planet.rstar_rsun else {
        return LunaStyleFlags::default();
    };
    let Some(mstar) = planet.mstar_msun else {
        return LunaStyleFlags::default();
    };
    let a_m = au_to_m(a_au);
    let rstar_m = rsun_to_m(rstar);
    let mp = crate::geometry::mplanet_to_kg(prior.mass_used_earth);
    let mstar_kg = crate::geometry::mstar_to_kg(mstar);
    let r_h = hill_radius_m(a_m, mp, mstar_kg);
    let a_s = h.d_hill * r_h;
    let rm = earth_radii_to_m(moon_radius_earth_extrap(h.ms_earth));
    let rp = planet
        .rp_earth
        .map(earth_radii_to_m)
        .unwrap_or(geom.rp_over_rstar * rstar_m);
    let moon_k = if rstar_m > 0.0 { rm / rstar_m } else { 0.0 };
    let a_s_over_rstar = if rstar_m > 0.0 { a_s / rstar_m } else { 0.0 };
    let overlap_over_as = if a_s > 0.0 { (rp + rm) / a_s } else { 0.0 };
    let b = geom.impact_b.unwrap_or(0.3);
    // Coplanar moon: sky sep can go to ~0 (syzygy) so discs can overlap.
    let overlapping = a_s > 0.0 && (rp + rm) > 0.0;
    let t14 = geom.t14_hr.or(planet.duration_hr).unwrap_or(0.0);
    let ps = moon_period_days(planet.period_days, h.d_hill);
    let syzygy = overlapping && t14 > 0.0 && ps > 0.0;
    // Moon can hit the stellar disc if its impact range reaches |b| < 1+k.
    let extra_dip = (b - a_s_over_rstar).abs() < 1.0 + moon_k
        || (b + a_s_over_rstar).abs() < 1.0 + moon_k
        || b < 1.0 + moon_k;
    LunaStyleFlags {
        overlapping_disc_possible: overlapping,
        syzygy_in_transit_possible: syzygy,
        extra_dip_on_star_possible: extra_dip,
        moon_k,
        a_s_over_rstar,
        overlap_over_as,
        method:
            "geometric overlapping-disc / syzygy / extra-dip FLAGS only. Not a LUNA integrator."
                .into(),
    }
}
