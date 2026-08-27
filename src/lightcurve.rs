//! Cached MAST / Kepler / K2 / TESS light curves (public HTTP, no secrets).
//!
//! Full mission downloads are huge. The in-repo cache is a small PDCSAP
//! extract from real LLC / SPOC FITS. See `data/cache/lightcurves/SOURCE.md`.

use crate::error::{ExoError, Result};
use crate::fits_llc::{parse_llc_fits, LlcRow};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};

/// TESS TIME is BTJD (BJD−2457000). BKJD = BJD−2454833 ⇒ offset +2167 d.
pub const TESS_BTJD_TO_BKJD_DAYS: f64 = 2457000.0 - 2454833.0;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightCurveSpec {
    pub name: &'static str,
    /// KIC / EPIC / TIC numeric id (see `id_kind`).
    pub target_id: u64,
    pub id_kind: &'static str,
    pub mission: &'static str,
    pub cadence: &'static str,
    pub cache_csv: &'static str,
    pub source_url: &'static str,
    pub note: &'static str,
    /// Added to TIME after FITS parse (TESS BTJD → BKJD). Cached CSVs already include it.
    pub time_offset_days: f64,
    /// Keep this many points from the middle of a large FITS file (TESS SPOC).
    pub extract_mid_points: Option<usize>,
}

/// Confirmed-planet LCs (Kepler-10, Kepler-1/2/4–9/11/22, K2-3/18) are
/// training hosts, not moons. Kepler-1625 / 1708 / 167 are **holdout**
/// hosts: photometry flags only. Sibling planets (11 c/d, 9 c) reuse the
/// same host Q1 extract with their own catalog epochs.
pub fn cached_specs() -> Vec<LightCurveSpec> {
    vec![
        LightCurveSpec {
            name: "Kepler-10 b",
            target_id: 11_904_151,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler10b_kic11904151_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0119/011904151/kplr011904151-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-11 b",
            target_id: 6_541_920,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler11b_kic6541920_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0065/006541920/kplr006541920-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet. Catalog epoch t0≈138.50 BKJD falls in this quarter — not invented. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-11 c",
            target_id: 6_541_920,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler11b_kic6541920_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0065/006541920/kplr006541920-2009166043257_llc.fits",
            note: "Same Q1 LLC as Kepler-11 b (KIC 6541920). Catalog epoch t0≈138.18 BKJD falls in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-11 d",
            target_id: 6_541_920,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler11b_kic6541920_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0065/006541920/kplr006541920-2009166043257_llc.fits",
            note: "Same Q1 LLC as Kepler-11 b (KIC 6541920). Catalog epoch t0≈148.46 BKJD falls in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-2 b",
            target_id: 10_666_592,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler2b_kic10666592_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0106/010666592/kplr010666592-2009166043257_llc.fits",
            note: "Q1 LLC (HAT-P-7b). Confirmed planet (P≈2.20 d, t0≈121.36 BKJD). Catalog transits fall in this quarter — not invented. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-8 b",
            target_id: 6_922_244,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler8b_kic6922244_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0069/006922244/kplr006922244-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈3.52 d, t0≈121.12 BKJD). Catalog transits fall in this quarter — not invented. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-9 b",
            target_id: 3_323_887,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler9b_kic3323887_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0033/003323887/kplr003323887-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈19.27 d, catalog t0≈182.54 BKJD). The previous catalog epoch (≈163.27) falls in Q1 — not invented. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-9 c",
            target_id: 3_323_887,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler9b_kic3323887_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0033/003323887/kplr003323887-2009166043257_llc.fits",
            note: "Same Q1 LLC as Kepler-9 b (KIC 3323887). Catalog t0≈175.43; previous epoch ≈136.52 falls in Q1 — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-4 b",
            target_id: 11_853_905,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler4b_kic11853905_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0118/011853905/kplr011853905-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈3.21 d, t0≈123.61 BKJD). Catalog transits fall in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-5 b",
            target_id: 8_191_672,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler5b_kic8191672_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0081/008191672/kplr008191672-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈3.55 d, t0≈122.90 BKJD). Catalog transits fall in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-6 b",
            target_id: 10_874_614,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler6b_kic10874614_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0108/010874614/kplr010874614-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈3.23 d, t0≈121.49 BKJD). Catalog transits fall in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-7 b",
            target_id: 5_780_885,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler7b_kic5780885_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0057/005780885/kplr005780885-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈4.89 d, t0≈134.28 BKJD). Catalog transit falls in this quarter — not invented. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-22 b",
            target_id: 10_593_626,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler22b_kic10593626_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0105/010593626/kplr010593626-2009166043257_llc.fits",
            note: "Q1 LLC. Confirmed planet (P≈290 d, t0≈133.70 BKJD). Catalog transit falls in this quarter — not invented. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-1 b",
            target_id: 11_446_443,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler1b_kic11446443_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0114/011446443/kplr011446443-2009166043257_llc.fits",
            note: "Q1 LLC (TrES-2b). Confirmed planet. Not a moon host in this crate.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-1625 b",
            target_id: 4_760_478,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler1625b_kic4760478_q8_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits",
            note: "Q8 LLC. Holdout host. This quarter does not cover a catalog transit of Kepler-1625 b (P≈287 d, t0≈348.83 BKJD). Photometry flags only. Moon stays CANDIDATE.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-1708 b",
            target_id: 7_906_827,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler1708b_kic7906827_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0079/007906827/kplr007906827-2009166043257_llc.fits",
            note: "Q1 LLC. Holdout host. P≈737 d — this quarter does not cover a catalog transit. Do not invent one. Moon stays CANDIDATE.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-167 e",
            target_id: 3_239_945,
            id_kind: "KIC",
            mission: "Kepler",
            cadence: "long",
            cache_csv: "lightcurves/kepler167e_kic3239945_q1_llc.csv",
            source_url: "https://archive.stsci.edu/pub/kepler/lightcurves/0032/003239945/kplr003239945-2009166043257_llc.fits",
            note: "Q1 LLC. Holdout host. P≈1071 d, t0≈420.29 BKJD — this quarter does not cover a catalog transit. Do not invent one. Status stays SEARCH.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "Kepler-10 b",
            target_id: 377_780_790,
            id_kind: "TIC",
            mission: "TESS",
            cadence: "2-min",
            cache_csv: "lightcurves/kepler10b_tic377780790_tess_s14_pdcsap.csv",
            source_url: "https://archive.stsci.edu/missions/tess/tid/s0014/0000/0003/7778/0790/tess2019198215352-s0014-0000000377780790-0150-s_lc.fits",
            note: "TESS S14 SPOC 2-min PDCSAP (mid 2500 points). TIME converted BTJD→BKJD (+2167). Confirmed planet. Not a moon.",
            time_offset_days: TESS_BTJD_TO_BKJD_DAYS,
            extract_mid_points: Some(2_500),
        },
        LightCurveSpec {
            name: "K2-3 b",
            target_id: 201_367_065,
            id_kind: "EPIC",
            mission: "K2",
            cadence: "long",
            cache_csv: "lightcurves/k2_3_epic201367065_c01_llc.csv",
            source_url: "https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits",
            note: "K2 Campaign 1 LLC. Confirmed planet. Cached PS row has no transit epoch; extra-dip is unwindowed. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "K2-3 c",
            target_id: 201_367_065,
            id_kind: "EPIC",
            mission: "K2",
            cadence: "long",
            cache_csv: "lightcurves/k2_3_epic201367065_c01_llc.csv",
            source_url: "https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits",
            note: "Same K2 C1 LLC as K2-3 b. Cached PS row has no transit epoch; extra-dip is unwindowed. Do not invent one. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
        LightCurveSpec {
            name: "K2-18 b",
            target_id: 201_912_552,
            id_kind: "EPIC",
            mission: "K2",
            cadence: "long",
            cache_csv: "lightcurves/k2_18_epic201912552_c01_llc.csv",
            source_url: "https://archive.stsci.edu/pub/k2/lightcurves/c1/201900000/12000/ktwo201912552-c01_llc.fits",
            note: "K2 Campaign 1 LLC. Confirmed planet. Cached PS row has no transit epoch; extra-dip is unwindowed. Not a moon.",
            time_offset_days: 0.0,
            extract_mid_points: None,
        },
    ]
}

#[derive(Debug, Clone)]
pub struct LightCurve {
    pub name: String,
    pub target_id: u64,
    pub id_kind: String,
    pub mission: String,
    pub cadence: String,
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

    /// Backward-compatible alias: Kepler KIC, else the mission catalog id.
    pub fn kic(&self) -> u64 {
        self.target_id
    }
}

/// One planet may have several cached extracts (Kepler + TESS for Kepler-10).
pub type LightCurveIndex = HashMap<String, Vec<LightCurve>>;

pub fn n_cached_lightcurves(lcs: &LightCurveIndex) -> usize {
    lcs.values().map(|v| v.len()).sum()
}

pub fn load_lightcurves(cache_dir: &Path) -> Result<LightCurveIndex> {
    let mut out: LightCurveIndex = HashMap::new();
    for spec in cached_specs() {
        let path = cache_dir.join(spec.cache_csv);
        if !path.exists() {
            continue;
        }
        let lc = read_llc_csv(&path, spec)?;
        out.entry(spec.name.to_string()).or_default().push(lc);
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
        target_id: spec.target_id,
        id_kind: spec.id_kind.into(),
        mission: spec.mission.into(),
        cadence: spec.cadence.into(),
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
        .timeout(std::time::Duration::from_secs(180))
        .call()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    if resp.status() != 200 {
        return Err(ExoError::Http(format!("HTTP {} for {url}", resp.status())));
    }
    let mut r = resp.into_reader();
    let mut buf = Vec::new();
    r.read_to_end(&mut buf)?;
    if buf.is_empty() {
        return Err(ExoError::Http(format!("empty body for {url}")));
    }
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

/// Re-download the cached LLC / SPOC FITS files and refresh the CSV extracts.
pub fn fetch_lightcurves(cache_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut written = Vec::new();
    for spec in cached_specs() {
        let bytes = download_bytes(spec.source_url)?;
        let mut rows = parse_llc_fits(&bytes)?;
        if spec.time_offset_days != 0.0 {
            for r in &mut rows {
                r.time_bkjd += spec.time_offset_days;
            }
        }
        if let Some(n) = spec.extract_mid_points {
            if rows.len() > n {
                let start = (rows.len() - n) / 2;
                rows = rows[start..start + n].to_vec();
            }
        }
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
