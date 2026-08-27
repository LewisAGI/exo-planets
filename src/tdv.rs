//! TDV-V and first-order TDV-TIP. Circular + coplanar unless noted.
//!
//! TDV-V ∝ M_S a_S^{-1/2}, π/2 out of phase with TTV.
//! η = δTDV / δTTV solves M_S, a_S if circular and coplanar.
//!
//! For the circular coplanar pair implemented here:
//!   δTTV = a_W / (√2 v_B)
//!   δTDV_V = T · (a_W / a) · (P_B / P_S) / √2
//!   η_V = 2π T / P_S
//!
//! TDV-TIP (impact-parameter / chord) is **additive** for prograde and
//! **subtractive** for retrograde (Kipping). This file uses a first-order
//! |dT/db| · (a_W / R_*) / √2 scale — **not** LUNA photodynamics.
//!
//! Kepler long-cadence (30 min) smears ingress; prefer short cadence for TDV.

use crate::constants::{DAY_S, G, KEPLER_LC_MIN, MEARTH_KG};
use crate::geometry::mplanet_to_kg;
use crate::ttv::TtvPrediction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoonSense {
    Prograde,
    Retrograde,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TdvPrediction {
    /// Velocity-induced TDV RMS (minutes).
    pub tdv_v_rms_min: f64,
    /// First-order TIP TDV RMS (minutes).
    pub tdv_tip_rms_min: f64,
    /// Combined RMS: V+TIP prograde, |V−TIP| retrograde.
    pub tdv_combined_rms_min: f64,
    /// η = δTDV_V / δTTV  (= 2π T / P_S for this circular model).
    pub eta_v: f64,
    /// η using the combined TDV.
    pub eta_combined: f64,
    pub long_cadence_smear: bool,
    pub sense: MoonSense,
}

/// Circular TDV-V RMS in minutes.
///
/// δTDV_V = T · (a_W / a) · (P_B / P_S) / √2
pub fn tdv_v_rms_minutes(
    t14_hr: f64,
    a_w_over_a: f64,
    period_days: f64,
    moon_period_days: f64,
) -> f64 {
    if moon_period_days <= 0.0 || t14_hr <= 0.0 {
        return 0.0;
    }
    let t_days = t14_hr / 24.0;
    let days = t_days * a_w_over_a * (period_days / moon_period_days) / std::f64::consts::SQRT_2;
    days * 24.0 * 60.0
}

/// η_V = 2π T / P_S  (T and P_S in the same unit).
pub fn eta_velocity(t14_hr: f64, moon_period_days: f64) -> f64 {
    if moon_period_days <= 0.0 {
        return 0.0;
    }
    let t_days = t14_hr / 24.0;
    2.0 * std::f64::consts::PI * t_days / moon_period_days
}

/// Invert circular+coplanar η: P_S = 2π T / η.
pub fn moon_period_from_eta(t14_hr: f64, eta: f64) -> Option<f64> {
    if eta <= 0.0 || !eta.is_finite() {
        return None;
    }
    Some(2.0 * std::f64::consts::PI * (t14_hr / 24.0) / eta)
}

/// First-order chord sensitivity: dT/db = − T b / ((1+k)^2 − b^2)
/// then δTDV_TIP ≈ |dT/db| · (a_W / R_*) / √2.
pub fn tdv_tip_rms_minutes(t14_hr: f64, k: f64, b: f64, a_w_over_rstar: f64) -> f64 {
    let denom = (1.0 + k) * (1.0 + k) - b * b;
    if denom <= 0.0 || t14_hr <= 0.0 {
        return 0.0;
    }
    let dtd_b_hr = (t14_hr * b.abs() / denom).abs();
    let hours = dtd_b_hr * a_w_over_rstar / std::f64::consts::SQRT_2;
    hours * 60.0
}

pub fn long_cadence_smear(ingress_hr: Option<f64>) -> bool {
    match ingress_hr {
        Some(ing) => ing * 60.0 <= 2.0 * KEPLER_LC_MIN,
        None => false,
    }
}

pub fn predict_tdv(
    ttv: &TtvPrediction,
    t14_hr: f64,
    period_days: f64,
    a_m: f64,
    rstar_m: f64,
    k: f64,
    b: f64,
    ingress_hr: Option<f64>,
    sense: MoonSense,
) -> TdvPrediction {
    let a_w_over_a = if a_m > 0.0 { ttv.a_w_m / a_m } else { 0.0 };
    let tdv_v = tdv_v_rms_minutes(t14_hr, a_w_over_a, period_days, ttv.moon_period_days);
    let a_w_over_rstar = if rstar_m > 0.0 {
        ttv.a_w_m / rstar_m
    } else {
        0.0
    };
    let tdv_tip = tdv_tip_rms_minutes(t14_hr, k, b, a_w_over_rstar);
    let combined = match sense {
        MoonSense::Prograde => tdv_v + tdv_tip,
        MoonSense::Retrograde => (tdv_v - tdv_tip).abs(),
    };
    let eta_v = if ttv.dttv_rms_min > 0.0 {
        tdv_v / ttv.dttv_rms_min
    } else {
        0.0
    };
    let eta_c = if ttv.dttv_rms_min > 0.0 {
        combined / ttv.dttv_rms_min
    } else {
        0.0
    };
    TdvPrediction {
        tdv_v_rms_min: tdv_v,
        tdv_tip_rms_min: tdv_tip,
        tdv_combined_rms_min: combined,
        eta_v,
        eta_combined: eta_c,
        long_cadence_smear: long_cadence_smear(ingress_hr),
        sense,
    }
}

/// Closed-form circular solver: from η and δTTV recover P_S, then a_S, M_S
/// given M_P, a, P_B.
#[derive(Debug, Clone, PartialEq)]
pub struct CircularSolve {
    pub moon_period_days: f64,
    pub a_s_m: f64,
    pub ms_earth: f64,
    pub assumption: &'static str,
}

pub fn solve_ms_as_circular(
    eta: f64,
    dttv_rms_min: f64,
    t14_hr: f64,
    period_days: f64,
    a_m: f64,
    mp_earth: f64,
) -> Option<CircularSolve> {
    let ps = moon_period_from_eta(t14_hr, eta)?;
    if ps <= 0.0 || dttv_rms_min <= 0.0 || a_m <= 0.0 {
        return None;
    }
    let dttv_s = dttv_rms_min * 60.0;
    let period_s = period_days * DAY_S;
    let v_b = 2.0 * std::f64::consts::PI * a_m / period_s;
    let a_w = dttv_s * std::f64::consts::SQRT_2 * v_b;
    let mp = mplanet_to_kg(mp_earth);
    let ps_s = ps * DAY_S;
    let a_s = ((ps_s * ps_s * G * mp) / (4.0 * std::f64::consts::PI * std::f64::consts::PI)).cbrt();
    if a_s <= a_w {
        return None;
    }
    let ms = mp * a_w / (a_s - a_w);
    Some(CircularSolve {
        moon_period_days: ps,
        a_s_m: a_s,
        ms_earth: ms / MEARTH_KG,
        assumption: "circular+coplanar; M_S << M_P Kepler inversion; not LUNA",
    })
}
