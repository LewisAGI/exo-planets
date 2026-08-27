//! Physical constants and **locked** HEK / FORECASTER numbers from opened papers.
//!
//! Cool Worlds videos are not results. These values are the science lock
//! (Shisui note 2026-08-27), not invented confirmations.

/// Seconds in a day.
pub const DAY_S: f64 = 86_400.0;
/// Days in a Julian year (IAU).
pub const YEAR_DAYS: f64 = 365.25;
/// Astronomical unit (m).
pub const AU_M: f64 = 1.495_978_707e11;
/// Solar radius (m).
pub const RSUN_M: f64 = 6.957e8;
/// Earth radius (m).
pub const REARTH_M: f64 = 6.371e6;
/// Gravitational constant (m^3 kg^-1 s^-2).
pub const G: f64 = 6.674_30e-11;
/// Solar mass (kg).
pub const MSUN_KG: f64 = 1.988_47e30;
/// Jupiter mass (kg).
pub const MJUP_KG: f64 = 1.898_13e27;
/// Earth mass (kg).
pub const MEARTH_KG: f64 = 5.972_2e24;

pub const MJUP_OVER_MEARTH: f64 = MJUP_KG / MEARTH_KG; // ~317.8
pub const MSUN_OVER_MEARTH: f64 = MSUN_KG / MEARTH_KG; // ~3.329e5
pub const MSUN_OVER_MJUP: f64 = MSUN_KG / MJUP_KG; // ~1047.6
pub const RSUN_OVER_REARTH: f64 = RSUN_M / REARTH_M; // ~109.2

/// Kepler long-cadence integration (minutes). Ingress shorter than this is smeared.
pub const KEPLER_LC_MIN: f64 = 29.424;

/// HEK I (Kipping et al.) 4σ detection threshold used as a **scale**, not a claim.
pub const HEK_I_SIGMA_THRESHOLD: f64 = 4.0;

/// HEK I max-dev prefactor: Δt ~ 36.0 D (MS/M⊕) (PB/yr) (MJ/MP)^{2/3} (M☉/M*)^{1/3} minutes.
pub const HEK_I_MAXDEV_PREFACTOR_MIN: f64 = 36.0;

/// Large-moon scale quoted by HEK: ≳ 0.1 M⊕. Training injections sit at or above this.
pub const HEK_LARGE_MOON_MEARTH: f64 = 0.1;

/// Domingos, Winter & Vieira Neto (2006) Hill-sphere stability limits (a_S / R_H).
pub const DMAX_PROGRADE: f64 = 0.4895;
pub const DMAX_RETROGRADE: f64 = 0.9309;

/// TTV sampling: P_S ≲ 0.6 P_P or TTV alone recovers harmonics, not unique P_S.
pub const TTV_UNIQUE_PS_OVER_PP: f64 = 0.6;

/// HEK V: photometry-only (ignoring timing) would have falsely claimed moons in 1/4 of KOIs.
pub const HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION: f64 = 0.25;

/// HEK VI stacked 284 KOIs: η < 0.38 (95%). A dearth, not a detection. Bayes factor ~2 is a hint.
pub const HEK_VI_ETA_95_UPPER: f64 = 0.38;
pub const HEK_VI_STACKED_KOIS: u32 = 284;

/// Chen & Kipping (2017) FORECASTER Terran–Neptunian mass transition (M⊕).
/// Paper: 2.04^{+0.66}_{-0.59}; lock text: 2.0^{+0.7}_{-0.6}.
pub const FORECASTER_TERRAN_NEPTUNIAN_MEARTH: f64 = 2.0;
pub const FORECASTER_TERRAN_NEPTUNIAN_PLUS: f64 = 0.7;
pub const FORECASTER_TERRAN_NEPTUNIAN_MINUS: f64 = 0.6;

/// FORECASTER Neptunian–Jovian break ≈ 0.414 M_J (Chen & Kipping 2017).
pub const FORECASTER_NEPTUNIAN_JOVIAN_MJUP: f64 = 0.414;
/// FORECASTER Jovian–stellar break ≈ 0.08 M_☉ (hydrogen-burning / stellar class).
pub const FORECASTER_JOVIAN_STELLAR_MSUN: f64 = 0.08;

/// Approximate radius bins used **only** when mass is missing.
/// These are discretized extrapolations of FORECASTER, not the official code.
pub const FORECASTER_RADIUS_TERRAN_REARTH: f64 = 1.6;
pub const FORECASTER_RADIUS_NEPTUNIAN_REARTH: f64 = 8.0;
pub const FORECASTER_RADIUS_JOVIAN_REARTH: f64 = 22.0;

/// Default class masses when the catalog mass is missing or an upper limit.
/// Used only for Hill / TTV *scale* features and labelled as such.
pub const DEFAULT_MASS_TERRAN_MEARTH: f64 = 1.0;
pub const DEFAULT_MASS_NEPTUNIAN_MEARTH: f64 = 10.0;
pub const DEFAULT_MASS_JOVIAN_MEARTH: f64 = MJUP_OVER_MEARTH;
pub const DEFAULT_MASS_STELLAR_MEARTH: f64 = FORECASTER_JOVIAN_STELLAR_MSUN * MSUN_OVER_MEARTH;

/// Kepler prime-mission span used only to set a synthetic transit count for noise draws.
/// Not a per-KOI duty-cycle measurement.
pub const KEPLER_MISSION_DAYS_SYNTHETIC: f64 = 1_450.0;

/// Locked host-planet names. Never train these as confirmed moons.
pub const HOLDOUT_HOSTS: &[&str] = &[
    "Kepler-1625 b",
    "Kepler-1708 b",
    "Kepler-90 g",
    "Kepler-167 e",
];

/// Locked moon / search identifiers (score cards only).
pub const HOLDOUT_OBJECTS: &[&str] = &[
    "Kepler-1625b-i",
    "Kepler-1708 b-i",
    "Kepler-90g moon",
    "JWST Kepler-167e (GO 6491)",
];
