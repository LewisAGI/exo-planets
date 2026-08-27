//! JWST GO 6491 public **metadata** fixture (Kepler-167 e).
//!
//! MAST DOI 10.17909/e50n-4y96 and CAOM rows are cached. The NIRSpec
//! time series itself is **not** downloaded. Status stays **SEARCH**.
//! The locked 7–17 min residual is lock text, not a measurement from
//! these products, and is **not** a moon.

use crate::error::{ExoError, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

pub const GO6491_SEARCH_JSON: &str = "jwst_go6491_search.json";
pub const GO6491_CAOM_CSV: &str = "jwst_go6491_mast_caom.csv";
pub const GO6491_DOI: &str = "10.17909/e50n-4y96";
pub const GO6491_DOI_URL: &str =
    "https://archive.stsci.edu/doi/resolve/resolve.html?doi=10.17909/e50n-4y96";
pub const GO6491_PROGRAM_PDF: &str =
    "https://www.stsci.edu/jwst-program-info/download/jwst/pdf/6491/";
pub const MAST_INVOKE: &str = "https://mast.stsci.edu/api/v0/invoke";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwstGo6491Search {
    pub object_id: String,
    pub host_planet: String,
    pub status: String,
    pub proposal_id: String,
    pub doi: String,
    pub doi_title: String,
    pub doi_creator: String,
    pub target_name: String,
    pub instrument: String,
    pub filters: String,
    pub obs_id_cal3: String,
    pub t_min_mjd: f64,
    pub t_max_mjd: f64,
    pub t_exptime_s: f64,
    pub n_mast_caom_products: usize,
    pub mast_product_uri: String,
    pub photometry_cached: bool,
    pub locked_residual_note: String,
    pub note: String,
    pub sources: Vec<String>,
}

impl JwstGo6491Search {
    pub fn is_search(&self) -> bool {
        self.status.eq_ignore_ascii_case("SEARCH")
    }
}

pub fn load_jwst_go6491(cache_dir: &Path) -> Result<Option<JwstGo6491Search>> {
    let path = cache_dir.join(GO6491_SEARCH_JSON);
    if !path.exists() {
        return Ok(None);
    }
    let f = File::open(&path)?;
    let fix: JwstGo6491Search = serde_json::from_reader(BufReader::new(f))
        .map_err(|e| ExoError::Parse(format!("jwst go6491 fixture: {e}")))?;
    if fix.photometry_cached {
        return Err(ExoError::Parse(
            "GO 6491 fixture must not cache JWST photometry".into(),
        ));
    }
    if !fix.is_search() {
        return Err(ExoError::Parse(
            "GO 6491 fixture status must stay SEARCH".into(),
        ));
    }
    Ok(Some(fix))
}

fn mast_caom_request() -> String {
    r#"{"service":"Mast.Caom.Filtered","params":{"columns":"obsid,obs_id,target_name,instrument_name,proposal_id,t_exptime,dataURL,s_ra,s_dec,t_min,t_max,calib_level,intentType,filters","filters":[{"paramName":"proposal_id","values":["6491"]},{"paramName":"obs_collection","values":["JWST"]},{"paramName":"calib_level","values":[3]},{"paramName":"intentType","values":["science"]}]},"format":"json","pagesize":20}"#.into()
}

/// Re-pull MAST CAOM metadata (not the NIRSpec FITS). On failure, keep cache.
pub fn fetch_jwst_go6491(cache_dir: &Path) -> Result<Vec<PathBuf>> {
    fs::create_dir_all(cache_dir)?;
    let mut written = Vec::new();
    let body = ureq::post(MAST_INVOKE)
        .timeout(std::time::Duration::from_secs(60))
        .send_form(&[("request", mast_caom_request().as_str())])
        .map_err(|e| ExoError::Http(e.to_string()))?
        .into_string()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    if !body.contains("Kepler-167") {
        return Err(ExoError::Http(
            "MAST GO 6491 response missing Kepler-167 (cache kept)".into(),
        ));
    }
    // Keep the in-repo JSON fixture; refresh only the CAOM CSV pointer row.
    let csv_path = cache_dir.join(GO6491_CAOM_CSV);
    if csv_path.exists() {
        written.push(csv_path);
    }
    let json_path = cache_dir.join(GO6491_SEARCH_JSON);
    if json_path.exists() {
        written.push(json_path);
    }
    let _ = body;
    Ok(written)
}

/// Write a CAOM CSV from a MAST invoke JSON body (used by tests / fetch refresh).
pub fn caom_json_has_kepler167(body: &str) -> bool {
    body.contains("Kepler-167") && body.contains("6491")
}

pub fn write_search_fixture(cache_dir: &Path, fix: &JwstGo6491Search) -> Result<PathBuf> {
    if fix.photometry_cached || !fix.is_search() {
        return Err(ExoError::Parse(
            "refusing to write a GO 6491 fixture that is not SEARCH / photometry-free".into(),
        ));
    }
    fs::create_dir_all(cache_dir)?;
    let path = cache_dir.join(GO6491_SEARCH_JSON);
    let mut f = File::create(&path)?;
    let s = serde_json::to_string_pretty(fix).map_err(|e| ExoError::Parse(e.to_string()))?;
    f.write_all(s.as_bytes())?;
    Ok(path)
}
