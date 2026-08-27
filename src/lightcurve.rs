//! Cached MAST / Kepler long-cadence light curves (public HTTP, no secrets).
//!
//! Full MAST downloads are huge. The in-repo cache is a small PDCSAP extract
//! from real Kepler LLC FITS. See `data/cache/lightcurves/SOURCE.md`.

use crate::error::{ExoError, Result};
use crate::fits_llc::{parse_llc_fits, LlcRow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightCurveSpec {
    pub name: &'static str,
    pub kic: u64,
    pub mission: &'static str,
    pub cadence: &'static str,
    pub cache_csv: &'static str,
    pub source_url: &'static str,
    pub note: &'static str,
}

/// Kepler-10 b and Kepler-1 b are confirmed planets (training hosts, not moons).
/// Kepler-1625 b is a **holdout** host: the LC is for photometry flags only.
pub fn cached_specs() -> Vec<LightCurveSpec> {
    vec![
        LightCurveSpec {
            name: "Kepler-10 b",
            kic: 11_904_151,
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler10b_kic11904151_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0119/011904151/kplr011904151-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet. Not a moon host in this crate.",
        },
        LightCurveSpec {
            name: "Kepler-1 b",
            kic: 11_446_443,
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler1b_kic11446443_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0114/011446443/kplr011446443-2009166043257_llc.fits",
            note: "Q1 LLC (TrES-2b). Confirmed planet. Not a moon host in this crate.",
        },
        LightCurveSpec {
            name: "Kepler-1625 b",
            kic: 4_760_478,
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler1625b_kic4760478_q8_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits",
            note: "Q8 LLC. Holdout host. This quarter does not cover a catalog transit of Kepler-1625 b (P≈287 d, t0≈348.83 BKJD). Photometry flags only. Moon stays CANDIDATE.",
        },
    ]
}

#[derive(Debug, Clone)]
pub struct LightCurve {
    pub name: String,
    pub kic: u64,
    pub mission: String,
    pub time_bkjd: Vec<f64>,
    pub flux: Vec<f64>,
    pub flux_err: Vec<f64>,
    pub quality: Vec<i32>,
    pub source_url: String,
    pub cache_file: String,
    pub note: String,
}

impl LightCurve {
    pub fn len(&self) -> usize {
        self.time_bkjd.len()
    }

    pub fn is_empty(&self) -> bool {
        self.time_bkjd.is_empty()
    }
}

pub type LightCurveIndex = HashMap<String, LightCurve>;

pub fn load_lightcurves(cache_dir: &Path) -> Result<LightCurveIndex> {
    let mut out = HashMap::new();
    for spec in cached_specs() {
        let path = cache_dir.join(spec.cache_csv);
        if !path.exists() {
            continue;
        }
        let lc = read_llc_csv(&path, spec)?;
        out.insert(spec.name.to_string(), lc);
    }
    Ok(out)
}

fn read_llc_csv(path: &Path, spec: LightCurveSpec) -> Result<LightCurve> {
    let f = File::open(path)?;
    let mut time = Vec::new();
    let mut flux = Vec::new();
    let mut err = Vec::new();
    let mut qual = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(BufReader::new(f));
    for rec in rdr.records() {
        let rec = rec?;
        let t: f64 = rec.get(0).unwrap_or("").parse().unwrap_or(f64::NAN);
        let fl: f64 = rec.get(1).unwrap_or("").parse().unwrap_or(f64::NAN);
        let e: f64 = rec.get(2).unwrap_or("").parse().unwrap_or(0.0);
        let q: i32 = rec.get(3).unwrap_or("0").parse().unwrap_or(0);
        if t.is_finite() && fl.is_finite() && fl > 0.0 {
            time.push(t);
            flux.push(fl);
            err.push(e);
            qual.push(q);
        }
    }
    if time.is_empty() {
        return Err(ExoError::Parse(format!(
            "empty light curve {}",
            path.display()
        )));
    }
    Ok(LightCurve {
        name: spec.name.into(),
        kic: spec.kic,
        mission: spec.mission.into(),
        time_bkjd: time,
        flux,
        flux_err: err,
        quality: qual,
        source_url: spec.source_url.into(),
        cache_file: spec.cache_csv.into(),
        note: spec.note.into(),
    })
}

fn download_bytes(url: &str) -> Result<Vec<u8>> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(120))
        .call()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    if resp.status() != 200 {
        return Err(ExoError::Http(format!("HTTP {} for {url}", resp.status())));
    }
    let mut r = resp.into_reader();
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;
    Ok(buf)
}

fn write_llc_csv(path: &Path, rows: &[LlcRow]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut f = File::create(path)?;
    writeln!(f, "time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality")?;
    for row in rows {
        writeln!(
            f,
            "{:.8},{:.8},{:.8},{}",
            row.time_bkjd, row.flux, row.flux_err, row.quality
        )?;
    }
    Ok(())
}

/// Re-download the cached Kepler LLC FITS files and refresh the CSV extracts.
pub fn fetch_lightcurves(cache_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut written = Vec::new();
    for spec in cached_specs() {
        let bytes = download_bytes(spec.source_url)?;
        let rows = parse_llc_fits(&bytes)?;
        let path = cache_dir.join(spec.cache_csv);
        write_llc_csv(&path, &rows)?;
        written.push(path);
    }
    Ok(written)
}

/// Used by tests to confirm the CSV header is the documented extract.
pub fn peek_header(path: &Path) -> Result<String> {
    let f = File::open(path)?;
    let mut line = String::new();
    BufReader::new(f).read_line(&mut line)?;
    Ok(line.trim().to_string())
}
