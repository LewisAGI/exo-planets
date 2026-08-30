//! Locked holdout score cards. Candidates stay candidates.
//!
//! - Kepler-1625b-i: CANDIDATE (Hubble dip model-dependent; unconfirmed)
//! - Kepler-1708 b-i: CANDIDATE (planet validated; moon not; 2 transits;
//!   predicted TTV 1.2–77 min 95%)
//! - Kepler-90g moon: FALSE POSITIVE (SPSD / pixel-centroid)
//! - JWST Kepler-167e (GO 6491): SEARCH (residual 7–17 min after linear
//!   ephemeris). Do not promote a moon.

use crate::features::FeatureRow;
use crate::ingest::CatalogPlanet;
use crate::labels::{locked_holdout_status, HoldoutStatus};
use crate::model::TrainedModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldoutCard {
    pub object_id: String,
    pub host_planet: String,
    pub status: HoldoutStatus,
    pub status_text: String,
    pub locked_note: String,
    pub published_timing: Option<String>,
    pub catalog_found: bool,
    pub model_p_injected_like: Option<f64>,
    pub model_input: Option<String>,
    pub geometry_depth_ppm: Option<f64>,
    pub period_days: Option<f64>,
    pub mp_earth: Option<f64>,
    pub mp_is_upper_limit: bool,
    pub forecaster_class: Option<String>,
    pub forecaster_extrapolated: Option<bool>,
    pub photometry_only_caution: bool,
    pub lc_available: bool,
    pub extra_dip_snr: Option<f64>,
    pub photometry_only_would_flag: bool,
    pub overlapping_disc_flag: bool,
    pub jwst_doi: Option<String>,
    pub jwst_photometry_cached: bool,
    pub columbia_product_cached: bool,
    pub published_product_note: Option<String>,
    pub do_not: String,
}

pub fn locked_cards() -> Vec<HoldoutCard> {
    vec![
        HoldoutCard {
            object_id: "Kepler-1625b-i".into(),
            host_planet: "Kepler-1625 b".into(),
            status: HoldoutStatus::Candidate,
            status_text: HoldoutStatus::Candidate.as_str().into(),
            locked_note: "Hubble dip is model-dependent. Authors call the moon unconfirmed.".into(),
            published_timing: None,
            catalog_found: false,
            model_p_injected_like: None,
            model_input: None,
            geometry_depth_ppm: None,
            period_days: None,
            mp_earth: None,
            mp_is_upper_limit: false,
            forecaster_class: None,
            forecaster_extrapolated: None,
            photometry_only_caution: true,
            lc_available: false,
            extra_dip_snr: None,
            photometry_only_would_flag: false,
            overlapping_disc_flag: false,
            jwst_doi: None,
            jwst_photometry_cached: false,
            columbia_product_cached: false,
            published_product_note: Some(
                "Columbia Academic Commons DOI 10.7916/D8795NHS: Anubis bot-wall; /download 404. No product cached. Hubble-dependent, unconfirmed. Stays CANDIDATE."
                    .into(),
            ),
            do_not: "Do not train as a confirmed moon. Do not invent a TTV. Do not confirm.".into(),
        },
        HoldoutCard {
            object_id: "Kepler-1708 b-i".into(),
            host_planet: "Kepler-1708 b".into(),
            status: HoldoutStatus::Candidate,
            status_text: HoldoutStatus::Candidate.as_str().into(),
            locked_note: "Planet validated; moon not. Only two Kepler transits.".into(),
            published_timing: Some("Predicted TTV 1.2–77 min (95%). Not a detection.".into()),
            catalog_found: false,
            model_p_injected_like: None,
            model_input: None,
            geometry_depth_ppm: None,
            period_days: None,
            mp_earth: None,
            mp_is_upper_limit: false,
            forecaster_class: None,
            forecaster_extrapolated: None,
            photometry_only_caution: false,
            lc_available: false,
            extra_dip_snr: None,
            photometry_only_would_flag: false,
            overlapping_disc_flag: false,
            jwst_doi: None,
            jwst_photometry_cached: false,
            columbia_product_cached: false,
            published_product_note: None,
            do_not: "Do not train as a confirmed moon. Predicted TTV range is not a measurement."
                .into(),
        },
        HoldoutCard {
            object_id: "Kepler-90g moon".into(),
            host_planet: "Kepler-90 g".into(),
            status: HoldoutStatus::FalsePositive,
            status_text: HoldoutStatus::FalsePositive.as_str().into(),
            locked_note:
                "SPSD / pixel-centroid false positive. HEK-family lesson: photometry can lie."
                    .into(),
            published_timing: None,
            catalog_found: false,
            model_p_injected_like: None,
            model_input: None,
            geometry_depth_ppm: None,
            period_days: None,
            mp_earth: None,
            mp_is_upper_limit: false,
            forecaster_class: None,
            forecaster_extrapolated: None,
            photometry_only_caution: true,
            lc_available: false,
            extra_dip_snr: None,
            photometry_only_would_flag: false,
            overlapping_disc_flag: false,
            jwst_doi: None,
            jwst_photometry_cached: false,
            columbia_product_cached: false,
            published_product_note: None,
            do_not: "Status is FALSE POSITIVE regardless of any model score.".into(),
        },
        HoldoutCard {
            object_id: "JWST Kepler-167e (GO 6491)".into(),
            host_planet: "Kepler-167 e".into(),
            status: HoldoutStatus::Search,
            status_text: HoldoutStatus::Search.as_str().into(),
            locked_note: "JWST GO 6491 is a search, not a detection.".into(),
            published_timing: Some(
                "Residual timing 7–17 min after linear ephemeris. Do not promote a moon.".into(),
            ),
            catalog_found: false,
            model_p_injected_like: None,
            model_input: None,
            geometry_depth_ppm: None,
            period_days: None,
            mp_earth: None,
            mp_is_upper_limit: false,
            forecaster_class: None,
            forecaster_extrapolated: None,
            photometry_only_caution: false,
            lc_available: false,
            extra_dip_snr: None,
            photometry_only_would_flag: false,
            overlapping_disc_flag: false,
            jwst_doi: Some("10.17909/e50n-4y96".into()),
            jwst_photometry_cached: false,
            columbia_product_cached: false,
            published_product_note: Some(
                "MAST/DOI metadata only (GO 6491 NIRSpec CAOM). Time series not cached. Residual 7–17 min is lock text, not a moon."
                    .into(),
            ),
            do_not: "Do not promote a moon from a 7–17 min residual.".into(),
        },
    ]
}

pub fn attach_catalog_and_model(
    mut cards: Vec<HoldoutCard>,
    planets: &[CatalogPlanet],
    holdout_rows: &[FeatureRow],
    model: &TrainedModel,
) -> Vec<HoldoutCard> {
    for card in &mut cards {
        debug_assert_eq!(
            locked_holdout_status(&card.object_id),
            Some(card.status),
            "holdout status drifted from the science lock"
        );
        if let Some(p) = planets
            .iter()
            .find(|p| p.name.eq_ignore_ascii_case(&card.host_planet))
        {
            card.catalog_found = true;
            card.period_days = Some(p.period_days);
            card.mp_earth = p.mp_earth;
            card.mp_is_upper_limit = p.mp_is_upper_limit;
        }
        // Score using the planet-only (null timing) row if present — this is a
        // *feature-card* probability, not a published-timing claim. For 1708
        // and 167e we also score a one-off vector that substitutes the locked
        // residual scale when we have a matching row.
        if let Some(row) = holdout_rows.iter().find(|r| r.name == card.host_planet) {
            card.geometry_depth_ppm = Some(row.geometry.depth_ppm);
            card.forecaster_class = Some(row.forecaster.class.as_str().into());
            card.forecaster_extrapolated = Some(row.forecaster.from_radius_extrapolation);
            card.lc_available = row.photometry.lc_available;
            card.extra_dip_snr = Some(row.photometry.extra_dip_snr);
            card.photometry_only_would_flag = row.photometry.photometry_only_would_flag;
            card.overlapping_disc_flag = row.luna.overlapping_disc_possible;
            if row.photometry.photometry_only_would_flag {
                card.photometry_only_caution = true;
            }
            let p_null = model.predict_proba(&row.vector);
            match card.object_id.as_str() {
                "Kepler-1708 b-i" => {
                    // Mid of the locked predicted TTV range (1.2–77 min). Not a measurement.
                    let mut v = row.vector.clone();
                    if let Some(i) = feature_index("ttv_rms_min") {
                        v[i] = (1.2_f64 * 77.0).sqrt(); // geometric mid ≈ 9.6 min
                    }
                    card.model_p_injected_like = Some(model.predict_proba(&v));
                    card.model_input = Some(
                        if row.photometry.lc_available {
                            "catalog features + geometric-mid of locked 1.2–77 min TTV *prediction* (not data); cached Q1 Kepler LC flags (no catalog transit in that window). Moon stays CANDIDATE."
                        } else {
                            "catalog features + geometric-mid of locked 1.2–77 min TTV *prediction* (not data)"
                        }
                        .into(),
                    );
                    let _ = p_null;
                }
                "JWST Kepler-167e (GO 6491)" => {
                    let mut v = row.vector.clone();
                    if let Some(i) = feature_index("ttv_rms_min") {
                        v[i] = (7.0_f64 + 17.0) / 2.0;
                    }
                    card.model_p_injected_like = Some(model.predict_proba(&v));
                    card.model_input = Some(
                        if row.photometry.lc_available {
                            "catalog features + midpoint of locked 7–17 min residual (search, not moon); cached Q1 Kepler LC flags (no catalog transit in that window). Status stays SEARCH."
                        } else {
                            "catalog features + midpoint of locked 7–17 min residual (search, not moon)"
                        }
                        .into(),
                    );
                }
                "Kepler-1625b-i" => {
                    card.model_p_injected_like = Some(p_null);
                    card.model_input = Some(
                        "catalog geometry + synthetic-null timing + cached Q8 Kepler LC flags (no catalog transit in that window). No TTV invented. Moon stays CANDIDATE. Photometry-only caution applies."
                            .into(),
                    );
                }
                "Kepler-90g moon" => {
                    card.model_p_injected_like = Some(p_null);
                    card.model_input = Some(
                        "catalog geometry + synthetic-null timing. Status remains FALSE POSITIVE even if the model is confused."
                            .into(),
                    );
                }
                _ => {
                    card.model_p_injected_like = Some(p_null);
                    card.model_input = Some("catalog features + synthetic-null timing".into());
                }
            }
        }
    }
    cards
}

fn feature_index(name: &str) -> Option<usize> {
    crate::features::FEATURE_NAMES
        .iter()
        .position(|n| *n == name)
}
