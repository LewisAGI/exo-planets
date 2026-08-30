//! HEK V photometry-only **caution** on the cached confirmed-planet LCs.
//!
//! Kipping, Schmitt, Lunine & Teachey (HEK V): a photometry-only search
//! (ignoring timing) would have falsely claimed moons in ~1/4 of KOIs
//! because of correlated noise. This module runs the crate's extra-dip
//! cut on the small cached LC set. It is **not** a re-estimate of 1/4
//! and **not** a moon detection.

use crate::constants::HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION;
use crate::features::geometry_for;
use crate::ingest::CatalogPlanet;
use crate::lightcurve::LightCurveIndex;
use crate::photometry::photometry_flags_one;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HekVLcCaution {
    pub planet_name: String,
    pub mission: String,
    pub cache_file: String,
    pub n_points: usize,
    pub n_in_transit: usize,
    pub extra_dip_snr: f64,
    pub photometry_only_would_flag: bool,
    pub windowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HekVCautionDemo {
    pub n_planet_only_lightcurves: usize,
    pub n_windowed: usize,
    pub n_would_flag_photometry_only: usize,
    pub fraction_on_this_cache: f64,
    pub published_hek_v_false_fraction: f64,
    pub per_lc: Vec<HekVLcCaution>,
    pub note: String,
}

/// Run the extra-dip cut on **non-holdout** confirmed-planet LCs only.
pub fn hek_v_photometry_only_caution(
    planets: &[CatalogPlanet],
    lcs: &LightCurveIndex,
) -> HekVCautionDemo {
    let mut per_lc = Vec::new();
    for planet in planets {
        if planet.is_holdout_host() {
            continue;
        }
        let Some(curves) = lcs.get(&planet.name) else {
            continue;
        };
        let geom = geometry_for(planet);
        for lc in curves {
            let flags = photometry_flags_one(planet, &geom, lc);
            per_lc.push(HekVLcCaution {
                planet_name: planet.name.clone(),
                mission: lc.mission.clone(),
                cache_file: lc.cache_file.clone(),
                n_points: flags.n_points,
                n_in_transit: flags.n_in_transit,
                extra_dip_snr: flags.extra_dip_snr,
                photometry_only_would_flag: flags.photometry_only_would_flag,
                windowed: planet.epoch_bkjd.is_some(),
            });
        }
    }
    let n = per_lc.len();
    let n_windowed = per_lc.iter().filter(|r| r.windowed).count();
    let n_flag = per_lc
        .iter()
        .filter(|r| r.photometry_only_would_flag)
        .count();
    let fraction = if n == 0 {
        0.0
    } else {
        n_flag as f64 / n as f64
    };
    HekVCautionDemo {
        n_planet_only_lightcurves: n,
        n_windowed,
        n_would_flag_photometry_only: n_flag,
        fraction_on_this_cache: fraction,
        published_hek_v_false_fraction: HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION,
        per_lc,
        note: "HEK V photometry-only caution on this tiny cached LC set. \
Not a re-estimate of the published ~1/4 KOI false-claim rate. \
Not a moon detection. The 4σ + 2-cadence extra-dip cut is not loosened \
to manufacture a fire."
            .into(),
    }
}
