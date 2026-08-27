//! FORECASTER (Chen & Kipping 2017) as a **mass-prior class**, not a confirmation.
//!
//! Classes: Terran / Neptunian / Jovian / stellar.
//! Terran–Neptunian transition 2.0^{+0.7}_{-0.6} M⊕ (lock text).
//!
//! When only a radius is available the class is a **discretized extrapolation**
//! of FORECASTER, not a draw from the official probabilistic code.

use crate::constants::{
    DEFAULT_MASS_JOVIAN_MEARTH, DEFAULT_MASS_NEPTUNIAN_MEARTH, DEFAULT_MASS_STELLAR_MEARTH,
    DEFAULT_MASS_TERRAN_MEARTH, FORECASTER_JOVIAN_STELLAR_MSUN, FORECASTER_NEPTUNIAN_JOVIAN_MJUP,
    FORECASTER_RADIUS_JOVIAN_REARTH, FORECASTER_RADIUS_NEPTUNIAN_REARTH,
    FORECASTER_RADIUS_TERRAN_REARTH, FORECASTER_TERRAN_NEPTUNIAN_MEARTH,
    FORECASTER_TERRAN_NEPTUNIAN_MINUS, FORECASTER_TERRAN_NEPTUNIAN_PLUS, MJUP_OVER_MEARTH,
    MSUN_OVER_MEARTH,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ForecasterClass {
    Terran,
    Neptunian,
    Jovian,
    Stellar,
}

impl ForecasterClass {
    pub fn as_str(self) -> &'static str {
        match self {
            ForecasterClass::Terran => "terran",
            ForecasterClass::Neptunian => "neptunian",
            ForecasterClass::Jovian => "jovian",
            ForecasterClass::Stellar => "stellar",
        }
    }

    pub fn default_mass_earth(self) -> f64 {
        match self {
            ForecasterClass::Terran => DEFAULT_MASS_TERRAN_MEARTH,
            ForecasterClass::Neptunian => DEFAULT_MASS_NEPTUNIAN_MEARTH,
            ForecasterClass::Jovian => DEFAULT_MASS_JOVIAN_MEARTH,
            ForecasterClass::Stellar => DEFAULT_MASS_STELLAR_MEARTH,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForecasterPrior {
    pub class: ForecasterClass,
    /// True when the class came from a radius bin, not a catalog mass.
    pub from_radius_extrapolation: bool,
    /// True when catalog mass is an upper limit (archive `pl_bmasselim = 1`).
    pub mass_is_upper_limit: bool,
    /// True when the mass used for Hill/TTV scales is a class default, not measured.
    pub hill_mass_is_class_default: bool,
    pub transition_zone_terran_neptunian: bool,
    pub mass_used_earth: f64,
    pub notes: String,
}

pub fn neptunian_jovian_mearth() -> f64 {
    FORECASTER_NEPTUNIAN_JOVIAN_MJUP * MJUP_OVER_MEARTH
}

pub fn jovian_stellar_mearth() -> f64 {
    FORECASTER_JOVIAN_STELLAR_MSUN * MSUN_OVER_MEARTH
}

pub fn class_from_mass_earth(m: f64) -> ForecasterClass {
    if m < FORECASTER_TERRAN_NEPTUNIAN_MEARTH {
        ForecasterClass::Terran
    } else if m < neptunian_jovian_mearth() {
        ForecasterClass::Neptunian
    } else if m < jovian_stellar_mearth() {
        ForecasterClass::Jovian
    } else {
        ForecasterClass::Stellar
    }
}

/// Discretized radius bins. Labelled extrapolation — not official FORECASTER.
pub fn class_from_radius_earth_extrapolated(r: f64) -> ForecasterClass {
    if r < FORECASTER_RADIUS_TERRAN_REARTH {
        ForecasterClass::Terran
    } else if r < FORECASTER_RADIUS_NEPTUNIAN_REARTH {
        ForecasterClass::Neptunian
    } else if r < FORECASTER_RADIUS_JOVIAN_REARTH {
        ForecasterClass::Jovian
    } else {
        ForecasterClass::Stellar
    }
}

pub fn in_terran_neptunian_zone(m: f64) -> bool {
    let lo = FORECASTER_TERRAN_NEPTUNIAN_MEARTH - FORECASTER_TERRAN_NEPTUNIAN_MINUS;
    let hi = FORECASTER_TERRAN_NEPTUNIAN_MEARTH + FORECASTER_TERRAN_NEPTUNIAN_PLUS;
    m >= lo && m <= hi
}

pub fn assign_prior(
    mp_earth: Option<f64>,
    mp_is_upper_limit: bool,
    rp_earth: Option<f64>,
) -> ForecasterPrior {
    match (mp_earth.filter(|m| *m > 0.0), mp_is_upper_limit) {
        (Some(m), false) => ForecasterPrior {
            class: class_from_mass_earth(m),
            from_radius_extrapolation: false,
            mass_is_upper_limit: false,
            hill_mass_is_class_default: false,
            transition_zone_terran_neptunian: in_terran_neptunian_zone(m),
            mass_used_earth: m,
            notes: "FORECASTER class from catalog mass. Prior class only, not a confirmation."
                .into(),
        },
        (Some(_), true) | (None, _) => {
            let class = rp_earth
                .map(class_from_radius_earth_extrapolated)
                .unwrap_or(ForecasterClass::Neptunian);
            let why = if mp_is_upper_limit {
                "Catalog mass is an upper limit; class and Hill mass use radius-bin defaults (extrapolation)."
            } else {
                "No catalog mass; class and Hill mass use radius-bin defaults (extrapolation)."
            };
            ForecasterPrior {
                class,
                from_radius_extrapolation: true,
                mass_is_upper_limit: mp_is_upper_limit,
                hill_mass_is_class_default: true,
                transition_zone_terran_neptunian: false,
                mass_used_earth: class.default_mass_earth(),
                notes: format!(
                    "{why} Radius bins ({:.1}/{:.1}/{:.1} R⊕) are a discretization, not official FORECASTER draws.",
                    FORECASTER_RADIUS_TERRAN_REARTH,
                    FORECASTER_RADIUS_NEPTUNIAN_REARTH,
                    FORECASTER_RADIUS_JOVIAN_REARTH
                ),
            }
        }
    }
}
