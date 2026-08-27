//! Published TTV catalog: Holczer et al. 2016 (VizieR J/ApJS/225/9 table4).
//!
//! NASA TAP has `ttv_flag` on PS, not a TTV time series. This cache is the
//! Holczer O−C **scatter** (minutes): planet-only timing, often planet–planet
//! interactions. **Not moons.** Holdout KOIs 351.02 / 490.02 / 5084.01 are
//! absent from this 2016 table.

use crate::error::{ExoError, Result};
use crate::ingest::CatalogPlanet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

pub const HOLCZER_CACHE: &str = "holczer2016_table4_oc_scatter.csv";
pub const HOLCZER_SOURCE: &str = "Holczer+2016 ApJS 225, 9; VizieR J/ApJS/225/9 table4";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishedTtv {
    pub koi: f64,
    /// Median TTV *uncertainty* (minutes), not the O−C amplitude.
    pub sig_tt_min: f64,
    /// O−C scatter: 1.4826 × MAD (minutes). Planet-only timing.
    pub s_oc_min: f64,
    pub source: String,
}

pub type TtvIndex = HashMap<String, PublishedTtv>;

/// `K00072.01` / `K0072.01` → `72.01`.
pub fn kepoi_to_holczer_key(kepoi: &str) -> Option<String> {
    let s = kepoi.trim().trim_start_matches(['K', 'k']);
    let v: f64 = s.parse().ok()?;
    if v <= 0.0 {
        return None;
    }
    Some(format!("{v:.2}"))
}

pub fn load_holczer(cache_dir: &Path) -> Result<TtvIndex> {
    let path = cache_dir.join(HOLCZER_CACHE);
    if !path.exists() {
        return Ok(HashMap::new());
    }
    let f = File::open(&path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(BufReader::new(f));
    let mut out = HashMap::new();
    for rec in rdr.records() {
        let rec = rec?;
        let koi: f64 = rec
            .get(0)
            .unwrap_or("")
            .parse()
            .map_err(|e| ExoError::Parse(format!("holczer koi: {e}")))?;
        let sig: f64 = rec.get(1).unwrap_or("").parse().unwrap_or(f64::NAN);
        let soc: f64 = rec.get(2).unwrap_or("").parse().unwrap_or(f64::NAN);
        if !soc.is_finite() {
            continue;
        }
        let key = format!("{koi:.2}");
        out.insert(
            key,
            PublishedTtv {
                koi,
                sig_tt_min: sig,
                s_oc_min: soc,
                source: HOLCZER_SOURCE.into(),
            },
        );
    }
    Ok(out)
}

pub fn lookup_holczer<'a>(idx: &'a TtvIndex, planet: &CatalogPlanet) -> Option<&'a PublishedTtv> {
    let key = kepoi_to_holczer_key(&planet.id)?;
    idx.get(&key)
}

/// VizieR TAP (table4) — often 503; CDS HTTP `/ftp/` is the cache source.
pub const HOLCZER_VIZIER_TAP: &str = "https://tapvizier.cds.unistra.fr/TAPVizieR/tap/sync";
pub const HOLCZER_TABLE4_DAT: &str = "https://cdsarc.cds.unistra.fr/ftp/J/ApJS/225/9/table4.dat";
pub const HOLCZER_README: &str = "https://cdsarc.cds.unistra.fr/ftp/J/ApJS/225/9/ReadMe";

/// Parse VizieR `table4.dat` (fixed-width, Holczer+2016 ReadMe).
pub fn parse_table4_dat(dat: &str) -> Result<String> {
    let mut out = String::from("koi,sig_tt_min,s_oc_min,pval,source\n");
    let mut n = 0usize;
    for line in dat.lines() {
        if line.len() < 22 {
            continue;
        }
        let koi: f64 = line.get(0..7).unwrap_or("").trim().parse().unwrap_or(0.0);
        let sig: f64 = line
            .get(8..14)
            .unwrap_or("")
            .trim()
            .parse()
            .unwrap_or(f64::NAN);
        let soc: f64 = line
            .get(15..22)
            .unwrap_or("")
            .trim()
            .parse()
            .unwrap_or(f64::NAN);
        let pval = line.get(24..29).unwrap_or("").trim();
        if koi <= 0.0 || !soc.is_finite() {
            continue;
        }
        out.push_str(&format!(
            "{koi:.2},{sig:.2},{soc:.2},{pval},Holczer+2016_J/ApJS/225/9_table4\n"
        ));
        n += 1;
    }
    if n < 100 {
        return Err(ExoError::Parse(format!(
            "Holczer table4 parse produced only {n} rows"
        )));
    }
    Ok(out)
}

/// Re-download table4.dat. On HTTP failure, leave the in-repo cache in place.
pub fn fetch_holczer(cache_dir: &Path) -> Result<PathBuf> {
    fs::create_dir_all(cache_dir)?;
    let path = cache_dir.join(HOLCZER_CACHE);
    let resp = ureq::get(HOLCZER_TABLE4_DAT)
        .timeout(std::time::Duration::from_secs(90))
        .call()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    if resp.status() != 200 {
        return Err(ExoError::Http(format!(
            "HTTP {} for Holczer table4 (cache kept)",
            resp.status()
        )));
    }
    let body = resp
        .into_string()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    let csv = parse_table4_dat(&body)?;
    let mut f = File::create(&path)?;
    f.write_all(csv.as_bytes())?;
    Ok(path)
}
