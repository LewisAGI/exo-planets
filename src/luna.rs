//! LUNA-**style flags**. This is not a LUNA port.
//!
//! LUNA (Kipping) integrates 3-body sky motion + overlapping discs.
//! We only raise geometric flags: can a moon disc overlap the planet disc
//! (syzygy), can the moon's sky track hit the star (extra dip), and a
//! coplanar syzygy *timescale*. No 3-body integrator, no overlapping-disc
//! photometry, no HEK Bayes factor.

use crate::constants::{DMAX_PROGRADE, DMAX_RETROGRADE};
use crate::forecaster::ForecasterPrior;
use crate::geometry::{au_to_m, earth_radii_to_m, rsun_to_m, TransitGeometry};
use crate::ingest::CatalogPlanet;
use crate::inject::MoonHypothesis;
use crate::tdv::MoonSense;
use crate::ttv::{hill_radius_m, moon_period_days};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunaStyleFlags {
    pub overlapping_disc_possible: bool,
    pub syzygy_in_transit_possible: bool,
    pub extra_dip_on_star_possible: bool,
    /// True if |b| − a_S/R_* ≥ 1+k: the moon's coplanar track never hits the star.
    pub moon_can_miss_star: bool,
    /// D ≤ D_max for the hypothesis sense (Domingos 2006). Flag, not a fit.
    pub d_inside_dmax: bool,
    pub moon_k: f64,
    pub a_s_over_rstar: f64,
    /// (R_p + R_m) / a_S — brief-syzygy scale, not a LUNA light curve.
    pub overlap_over_as: f64,
    /// Order-of-magnitude coplanar syzygy duration (hours): (R_p+R_m)/a_S × P_S/(2π).
    /// Not a LUNA light curve and not a detection statistic.
    pub syzygy_timescale_hr: f64,
    pub method: String,
}

impl Default for LunaStyleFlags {
    fn default() -> Self {
        Self {
            overlapping_disc_possible: false,
            syzygy_in_transit_possible: false,
            extra_dip_on_star_possible: false,
            moon_can_miss_star: false,
            d_inside_dmax: false,
            moon_k: 0.0,
            a_s_over_rstar: 0.0,
            overlap_over_as: 0.0,
            syzygy_timescale_hr: 0.0,
            method: "no moon hypothesis; geometric LUNA-style flags off".into(),
        }
    }
}

/// R/R⊕ ≈ M^{0.28}. Discretized mass–radius, **not** official FORECASTER, not LUNA.
pub fn moon_radius_earth_extrap(ms_earth: f64) -> f64 {
    ms_earth.max(1e-6).powf(0.28)
}

/// Coplanar extra-dip: moon hits the star if |b| − a_S/R_* < 1+k.
pub fn extra_dip_possible(impact_b: f64, a_s_over_rstar: f64, moon_k: f64) -> bool {
    impact_b.abs() - a_s_over_rstar < 1.0 + moon_k
}

/// Coplanar syzygy duration scale in hours. Geometry only.
pub fn syzygy_timescale_hours(overlap_over_as: f64, moon_period_days: f64) -> f64 {
    if overlap_over_as <= 0.0 || moon_period_days <= 0.0 {
        return 0.0;
    }
    overlap_over_as * moon_period_days * 24.0 / (2.0 * std::f64::consts::PI)
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
    let b = geom.impact_b.or(planet.impact_b).unwrap_or(0.3);
    let d_inside = match h.sense {
        MoonSense::Prograde => h.d_hill <= DMAX_PROGRADE + 1e-12,
        MoonSense::Retrograde => h.d_hill <= DMAX_RETROGRADE + 1e-12,
    };
    // Coplanar: planet–moon sky sep can go to ~0, so finite discs can overlap.
    let overlapping = a_s > 0.0 && (rp + rm) > 0.0;
    let extra_dip = extra_dip_possible(b, a_s_over_rstar, moon_k);
    let t14 = geom.t14_hr.or(planet.duration_hr).unwrap_or(0.0);
    let ps = moon_period_days(planet.period_days, h.d_hill);
    // In-transit syzygy needs a planet chord *and* the moon able to sit on the star.
    let syzygy = overlapping && extra_dip && t14 > 0.0 && ps > 0.0;
    let t_syz = syzygy_timescale_hours(overlap_over_as, ps);
    LunaStyleFlags {
        overlapping_disc_possible: overlapping,
        syzygy_in_transit_possible: syzygy,
        extra_dip_on_star_possible: extra_dip,
        moon_can_miss_star: !extra_dip,
        d_inside_dmax: d_inside,
        moon_k,
        a_s_over_rstar,
        overlap_over_as,
        syzygy_timescale_hr: t_syz,
        method:
            "geometric overlapping-disc / syzygy / extra-dip FLAGS only. Not a LUNA integrator."
                .into(),
    }
}
