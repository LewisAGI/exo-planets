//! Feature row: transit geometry + TTV/TDV + HEK flags + FORECASTER class.

use crate::forecaster::{assign_prior, ForecasterClass, ForecasterPrior};
use crate::geometry::{compute_geometry, TransitGeometry};
use crate::hek::HekFlags;
use crate::ingest::CatalogPlanet;
use crate::inject::{hek_for_draw, TimingDraw};
use crate::labels::{ExampleKind, Split, TrainTarget};
use crate::tdv::long_cadence_smear;
use serde::{Deserialize, Serialize};

pub const FEATURE_NAMES: &[&str] = &[
    "log10_period_days",
    "log10_rp_earth",
    "depth_ppm_log10",
    "impact_b",
    "duration_hr",
    "ttv_rms_min",
    "tdv_rms_min",
    "eta",
    "log10_hek_maxdev_min",
    "d_hill",
    "dyn_stable_prograde",
    "ttv_alias_risk",
    "long_cadence_smear",
    "forecaster_terran",
    "forecaster_neptunian",
    "forecaster_jovian",
    "photometry_only",
    "mass_is_extrapolated",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureRow {
    pub id: String,
    pub name: String,
    pub split: Split,
    pub kind: ExampleKind,
    pub target: Option<TrainTarget>,
    pub geometry: GeometryOut,
    pub timing: TimingOut,
    pub hek: HekFlags,
    pub forecaster: ForecasterPrior,
    pub vector: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometryOut {
    pub depth_ppm: f64,
    pub rp_over_rstar: f64,
    pub impact_b: Option<f64>,
    pub t14_hr: Option<f64>,
    pub ingress_hr: Option<f64>,
    pub grazing: bool,
    pub a_from_kepler3: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingOut {
    pub ttv_rms_min: f64,
    pub tdv_rms_min: f64,
    pub eta: f64,
    pub hek_i_maxdev_min: f64,
    pub moon_period_days: Option<f64>,
    pub sigma_timing_min: f64,
    pub n_transits: usize,
    pub noise_model: String,
}

pub fn geometry_for(planet: &CatalogPlanet) -> TransitGeometry {
    compute_geometry(
        planet.period_days,
        planet.rp_earth,
        planet.rstar_rsun,
        planet.a_au,
        planet.mstar_msun,
        planet.incl_deg,
        planet.impact_b,
        planet.duration_hr,
        planet.depth_ppm,
    )
}

pub fn prior_for(planet: &CatalogPlanet) -> ForecasterPrior {
    assign_prior(planet.mp_earth, planet.mp_is_upper_limit, planet.rp_earth)
}

pub fn resolved_a_au(planet: &CatalogPlanet, geom: &TransitGeometry) -> Option<f64> {
    if let Some(a) = planet.a_au {
        if a > 0.0 {
            return Some(a);
        }
    }
    match (geom.a_over_rstar, planet.rstar_rsun) {
        (Some(ar), Some(rs)) if ar > 0.0 && rs > 0.0 => {
            Some(ar * crate::geometry::rsun_to_m(rs) / crate::constants::AU_M)
        }
        _ => None,
    }
}

fn one_hot_class(class: ForecasterClass) -> (f64, f64, f64) {
    match class {
        ForecasterClass::Terran => (1.0, 0.0, 0.0),
        ForecasterClass::Neptunian => (0.0, 1.0, 0.0),
        ForecasterClass::Jovian | ForecasterClass::Stellar => (0.0, 0.0, 1.0),
    }
}

fn log10p(x: f64) -> f64 {
    if x > 0.0 {
        x.log10()
    } else {
        -3.0
    }
}

pub fn build_row(
    planet: &CatalogPlanet,
    geom: &TransitGeometry,
    prior: &ForecasterPrior,
    draw: &TimingDraw,
    split: Split,
    photometry_only: bool,
) -> FeatureRow {
    let hek = hek_for_draw(planet, draw, photometry_only);
    let (ft, fnp, fj) = one_hot_class(prior.class);
    let d_hill = draw.hypothesis.as_ref().map(|h| h.d_hill).unwrap_or(0.0);
    let alias = draw
        .moon_period_days
        .map(|ps| ps > crate::constants::TTV_UNIQUE_PS_OVER_PP * planet.period_days)
        .unwrap_or(false);
    let smear = long_cadence_smear(geom.ingress_hr);
    let rp = planet.rp_earth.unwrap_or(1.0);
    let depth = if geom.depth_ppm > 0.0 {
        geom.depth_ppm
    } else {
        planet.depth_ppm.unwrap_or(100.0)
    };
    let dur = planet.duration_hr.or(geom.t14_hr).unwrap_or(3.0);
    let vector = vec![
        log10p(planet.period_days),
        log10p(rp),
        log10p(depth),
        geom.impact_b.or(planet.impact_b).unwrap_or(0.3),
        dur,
        draw.ttv_rms_min,
        draw.tdv_rms_min,
        draw.eta,
        log10p(draw.hek_i_maxdev_min + 1e-3),
        d_hill,
        if hek.d_within_prograde_dmax && d_hill > 0.0 {
            1.0
        } else {
            0.0
        },
        if alias { 1.0 } else { 0.0 },
        if smear { 1.0 } else { 0.0 },
        ft,
        fnp,
        fj,
        if photometry_only { 1.0 } else { 0.0 },
        if prior.from_radius_extrapolation {
            1.0
        } else {
            0.0
        },
    ];
    FeatureRow {
        id: match &draw.hypothesis {
            Some(h) => format!(
                "{}|injected|ms{:.2}|d{:.2}|{:?}",
                planet.id, h.ms_earth, h.d_hill, h.sense
            ),
            None => format!("{}|{:?}", planet.id, draw.kind),
        },
        name: planet.name.clone(),
        split,
        kind: draw.kind,
        target: match split {
            Split::Holdout => None,
            Split::Train => TrainTarget::from_kind(draw.kind),
        },
        geometry: GeometryOut {
            depth_ppm: geom.depth_ppm,
            rp_over_rstar: geom.rp_over_rstar,
            impact_b: geom.impact_b,
            t14_hr: geom.t14_hr,
            ingress_hr: geom.ingress_hr,
            grazing: geom.grazing,
            a_from_kepler3: geom.a_from_kepler3,
        },
        timing: TimingOut {
            ttv_rms_min: draw.ttv_rms_min,
            tdv_rms_min: draw.tdv_rms_min,
            eta: draw.eta,
            hek_i_maxdev_min: draw.hek_i_maxdev_min,
            moon_period_days: draw.moon_period_days,
            sigma_timing_min: draw.sigma_timing_min,
            n_transits: draw.n_transits,
            noise_model: draw.noise_model.to_string(),
        },
        hek,
        forecaster: prior.clone(),
        vector,
    }
}
