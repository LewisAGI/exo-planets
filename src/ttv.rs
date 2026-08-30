//! TTV: Kipping (2009a) circular coplanar RMS + HEK I max-dev + sampling aliases.
//!
//! RMS: δTTV = a_W / (√2 v_{B⊥})
//! δTTV ∝ M_S a_S  (when M_S ≪ M_P)
//!
//! Periodic sampling at P_P undersamples P_S. P_S ≲ 0.6 P_P or TTV alone
//! recovers harmonics, not a unique moon period.
//!
//! Keplerian Hill relation used here (see `moon_period_days`):
//! P_S = P_B √(D³/3)   ⇒   P_S(D=1) = P_B / √3
//!
//! The lock text writes P_SB = P_B / √(D³/3). That expression is the
//! *reciprocal* of the Kepler+Hill period and is **not** what we evaluate
//! for P_S. We implement the Keplerian relation and the cut
//! P_S ≤ P_B / √3 (equivalently D ≤ 1 before Domingos D_max).

use crate::constants::{
    DAY_S, HEK_I_MAXDEV_PREFACTOR_MIN, MJUP_OVER_MEARTH, TTV_UNIQUE_PS_OVER_PP, YEAR_DAYS,
};
use crate::geometry::{au_to_m, mplanet_to_kg, mstar_to_kg, period_days_to_s};

#[derive(Debug, Clone, PartialEq)]
pub struct TtvPrediction {
    /// Circular coplanar RMS TTV (minutes).
    pub dttv_rms_min: f64,
    /// HEK I max-dev scale (minutes).
    pub hek_i_maxdev_min: f64,
    /// Moon barycentric wobble amplitude of the planet a_W (m).
    pub a_w_m: f64,
    /// Sky-projected barycenter speed v_{B⊥} (m/s).
    pub v_b_perp_m_s: f64,
    pub moon_period_days: f64,
    /// Hill-sphere moon period P_B / √3 (days).
    pub hill_period_days: f64,
    pub ttv_unique_ps_unlikely: bool,
    pub moon_inside_hill_period_cut: bool,
}

/// Planet reflex radius about the planet–moon barycenter: a_W = a_S M_S / (M_P + M_S).
pub fn planet_wobble_m(a_s_m: f64, ms_kg: f64, mp_kg: f64) -> f64 {
    a_s_m * ms_kg / (mp_kg + ms_kg)
}

/// Circular barycenter sky speed ≈ 2π a / P_B.
pub fn barycenter_speed_m_s(a_m: f64, period_s: f64) -> f64 {
    2.0 * std::f64::consts::PI * a_m / period_s
}

/// δTTV = a_W / (√2 v_{B⊥}) in minutes.
pub fn dttv_rms_minutes(a_w_m: f64, v_b_perp_m_s: f64) -> f64 {
    let seconds = a_w_m / (std::f64::consts::SQRT_2 * v_b_perp_m_s);
    seconds / 60.0
}

/// Hill radius R_H = a (M_P / (3 M_*))^{1/3}.
pub fn hill_radius_m(a_m: f64, mp_kg: f64, mstar_kg: f64) -> f64 {
    a_m * (mp_kg / (3.0 * mstar_kg)).cbrt()
}

/// P_S = P_B √(D³/3). At D = 1 this is P_B / √3.
pub fn moon_period_days(planet_period_days: f64, d_hill: f64) -> f64 {
    planet_period_days * (d_hill.powi(3) / 3.0).sqrt()
}

pub fn hill_sphere_period_days(planet_period_days: f64) -> f64 {
    planet_period_days / 3.0_f64.sqrt()
}

/// HEK I max-dev (minutes):
/// Δt ~ 36.0 D (M_S/M⊕) (P_B/yr) (M_J/M_P)^{2/3} (M_☉/M_*)^{1/3}
pub fn hek_i_maxdev_minutes(
    d_hill: f64,
    ms_earth: f64,
    period_days: f64,
    mp_earth: f64,
    mstar_msun: f64,
) -> f64 {
    let pb_yr = period_days / YEAR_DAYS;
    let mp_jup = mp_earth / MJUP_OVER_MEARTH;
    HEK_I_MAXDEV_PREFACTOR_MIN
        * d_hill
        * ms_earth
        * pb_yr
        * mp_jup.powf(-2.0 / 3.0)
        * mstar_msun.powf(-1.0 / 3.0)
}

pub fn predict_ttv(
    period_days: f64,
    a_au: f64,
    mp_earth: f64,
    mstar_msun: f64,
    ms_earth: f64,
    d_hill: f64,
) -> TtvPrediction {
    let period_s = period_days_to_s(period_days);
    let a_m = au_to_m(a_au);
    let mp = mplanet_to_kg(mp_earth);
    let ms = mplanet_to_kg(ms_earth);
    let mstar = mstar_to_kg(mstar_msun);
    let r_h = hill_radius_m(a_m, mp, mstar);
    let a_s = d_hill * r_h;
    let a_w = planet_wobble_m(a_s, ms, mp);
    let v_b = barycenter_speed_m_s(a_m, period_s);
    let dttv = dttv_rms_minutes(a_w, v_b);
    let moon_p = moon_period_days(period_days, d_hill);
    let hill_p = hill_sphere_period_days(period_days);
    TtvPrediction {
        dttv_rms_min: dttv,
        hek_i_maxdev_min: hek_i_maxdev_minutes(d_hill, ms_earth, period_days, mp_earth, mstar_msun),
        a_w_m: a_w,
        v_b_perp_m_s: v_b,
        moon_period_days: moon_p,
        hill_period_days: hill_p,
        ttv_unique_ps_unlikely: moon_p > TTV_UNIQUE_PS_OVER_PP * period_days,
        moon_inside_hill_period_cut: moon_p <= hill_p + 1e-12,
    }
}

/// Number of transits over a fixed Kepler-like span. Synthetic, not a duty cycle.
pub fn synthetic_n_transits(period_days: f64, mission_days: f64) -> usize {
    ((mission_days / period_days).floor() as usize).max(2)
}

pub fn period_s_from_days(p: f64) -> f64 {
    p * DAY_S
}
