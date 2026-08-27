//! NASA Exoplanet Archive TAP ingest (public HTTP, no secrets).
//!
//! Primary tables: `cumulative` (KOI) and `ps` (Planetary Systems).
//! A small cached slice lives in `data/cache/`. `fetch` re-pulls the same
//! queries. Empty archive fields stay empty — we do not invent fills.

use crate::error::{ExoError, Result};
use crate::labels::is_holdout_host;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};

pub const TAP_SYNC: &str = "https://exoplanetarchive.ipac.caltech.edu/TAP/sync";

pub const Q_KOI_CONFIRMED_GEOMETRY: &str = "SELECT TOP 60 kepoi_name,kepid,kepler_name,koi_disposition,koi_pdisposition,koi_score,koi_period,koi_time0bk,koi_impact,koi_duration,koi_depth,koi_prad,koi_sma,koi_incl,koi_steff,koi_slogg,koi_srad,koi_smass FROM cumulative WHERE koi_disposition='CONFIRMED' AND koi_period IS NOT NULL AND koi_prad IS NOT NULL AND koi_impact IS NOT NULL AND koi_duration IS NOT NULL AND koi_depth IS NOT NULL AND koi_srad IS NOT NULL AND koi_smass IS NOT NULL AND koi_sma IS NOT NULL ORDER BY koi_period";

pub const Q_KOI_LONG_PERIOD: &str = "SELECT TOP 25 kepoi_name,kepid,kepler_name,koi_disposition,koi_pdisposition,koi_score,koi_period,koi_time0bk,koi_impact,koi_duration,koi_depth,koi_prad,koi_sma,koi_incl,koi_steff,koi_slogg,koi_srad,koi_smass FROM cumulative WHERE koi_disposition='CONFIRMED' AND koi_period>100 AND koi_period IS NOT NULL AND koi_prad IS NOT NULL AND koi_impact IS NOT NULL AND koi_duration IS NOT NULL AND koi_depth IS NOT NULL AND koi_srad IS NOT NULL AND koi_smass IS NOT NULL AND koi_sma IS NOT NULL ORDER BY koi_period";

pub const Q_KOI_NAMED: &str = "SELECT kepoi_name,kepid,kepler_name,koi_disposition,koi_pdisposition,koi_score,koi_period,koi_time0bk,koi_impact,koi_duration,koi_depth,koi_prad,koi_sma,koi_incl,koi_steff,koi_slogg,koi_srad,koi_smass FROM cumulative WHERE kepler_name IN ('Kepler-1625 b','Kepler-1708 b','Kepler-90 g','Kepler-167 e','Kepler-22 b','Kepler-10 b')";

/// Confirmed LC-backed planets (epochs from KOI, not invented).
pub const Q_KOI_LC_HOSTS: &str = "SELECT kepoi_name,kepid,kepler_name,koi_disposition,koi_pdisposition,koi_score,koi_period,koi_time0bk,koi_impact,koi_duration,koi_depth,koi_prad,koi_sma,koi_incl,koi_steff,koi_slogg,koi_srad,koi_smass FROM cumulative WHERE kepler_name IN ('Kepler-11 b','Kepler-11 c','Kepler-11 d','Kepler-11 e','Kepler-9 b','Kepler-9 c','Kepler-2 b','Kepler-8 b','Kepler-4 b','Kepler-5 b','Kepler-6 b','Kepler-7 b','Kepler-3 b','Kepler-18 b','Kepler-18 c','Kepler-18 d','Kepler-19 b','Kepler-20 b','Kepler-20 c','Kepler-20 d','Kepler-20 e','Kepler-20 f','Kepler-21 b','Kepler-30 b','Kepler-30 d','Kepler-36 b','Kepler-36 c','Kepler-48 b','Kepler-48 c','Kepler-48 d','Kepler-51 b','Kepler-79 b','Kepler-79 c','Kepler-79 d','Kepler-79 e','Kepler-68 c','Kepler-89 b','Kepler-89 c','Kepler-89 d','Kepler-89 e','Kepler-102 b','Kepler-102 c','Kepler-102 d','Kepler-102 e','Kepler-102 f','Kepler-62 c','Kepler-62 d','Kepler-62 e','Kepler-37 b','Kepler-37 c','Kepler-37 d','Kepler-444 b','Kepler-444 c','Kepler-444 d','Kepler-444 e','Kepler-444 f','Kepler-42 b','Kepler-42 c','Kepler-42 d','Kepler-138 b','Kepler-138 c','Kepler-138 d','Kepler-65 b','Kepler-65 c','Kepler-65 d','Kepler-32 b','Kepler-32 c','Kepler-32 d','Kepler-32 e','Kepler-32 f','Kepler-33 b','Kepler-33 c','Kepler-33 d','Kepler-33 e','Kepler-33 f','Kepler-186 b','Kepler-186 c','Kepler-186 d','Kepler-186 e','Kepler-26 b','Kepler-26 c','Kepler-26 d','Kepler-80 c','Kepler-80 d','Kepler-80 e','Kepler-80 f','Kepler-29 b','Kepler-29 c','Kepler-93 b','Kepler-100 b','Kepler-100 c','Kepler-100 d','Kepler-88 b','Kepler-23 b','Kepler-23 c','Kepler-24 b','Kepler-24 c','Kepler-27 b','Kepler-27 c','Kepler-28 b','Kepler-28 c','Kepler-41 b','Kepler-56 b','Kepler-56 c','Kepler-57 b','Kepler-57 c','Kepler-69 b','Kepler-76 b','Kepler-58 b','Kepler-58 c','Kepler-58 d','Kepler-59 b','Kepler-59 c','Kepler-60 b','Kepler-60 c','Kepler-60 d','Kepler-84 b','Kepler-84 c','Kepler-84 d','Kepler-84 e','Kepler-85 b','Kepler-85 c','Kepler-85 d','Kepler-85 e','Kepler-54 b','Kepler-54 c','Kepler-54 d','Kepler-55 b','Kepler-55 c','Kepler-55 d','Kepler-55 e','Kepler-52 b','Kepler-52 c','Kepler-52 d','Kepler-53 b','Kepler-53 c','Kepler-31 b','Kepler-31 c','Kepler-50 b','Kepler-50 c','Kepler-81 b','Kepler-81 c','Kepler-81 d','Kepler-94 b','Kepler-95 b','Kepler-61 b','Kepler-66 b','Kepler-74 b','Kepler-43 b','Kepler-44 b','Kepler-92 b','Kepler-92 c','Kepler-92 d','Kepler-49 b','Kepler-49 c','Kepler-75 b','Kepler-83 b','Kepler-83 c','Kepler-83 d','Kepler-39 b','Kepler-40 b','Kepler-45 b','Kepler-46 b','Kepler-63 b','Kepler-82 b','Kepler-82 d','Kepler-82 e','Kepler-91 b','Kepler-96 b','Kepler-97 b','Kepler-98 b','Kepler-99 b','Kepler-101 b','Kepler-101 c','Kepler-103 b','Kepler-104 b','Kepler-104 c','Kepler-105 b','Kepler-105 c','Kepler-106 b','Kepler-106 c','Kepler-106 d','Kepler-106 e','Kepler-107 b','Kepler-107 c','Kepler-107 d','Kepler-107 e')";

pub const Q_PS_KEPLER_SAMPLE: &str = "SELECT TOP 80 pl_name,hostname,pl_letter,sy_pnum,discoverymethod,disc_year,disc_facility,pl_orbper,pl_orbsmax,pl_rade,pl_radj,pl_bmasse,pl_bmassj,pl_orbeccen,pl_orbincl,pl_imppar,pl_trandep,pl_trandur,st_teff,st_rad,st_mass,default_flag,tran_flag FROM ps WHERE default_flag=1 AND tran_flag=1 AND disc_facility LIKE '%Kepler%' AND pl_orbper IS NOT NULL AND pl_rade IS NOT NULL AND st_rad IS NOT NULL AND st_mass IS NOT NULL ORDER BY pl_orbper";

pub const Q_PS_NAMED: &str = "SELECT pl_name,hostname,pl_letter,sy_pnum,discoverymethod,disc_year,disc_facility,pl_orbper,pl_orbsmax,pl_rade,pl_radj,pl_bmasse,pl_bmassj,pl_bmasselim,pl_orbeccen,pl_orbincl,pl_imppar,pl_trandep,pl_trandur,st_teff,st_rad,st_mass,default_flag,tran_flag FROM ps WHERE default_flag=1 AND pl_name IN ('Kepler-1625 b','Kepler-1708 b','Kepler-90 g','Kepler-167 e','Kepler-22 b','Kepler-10 b','Kepler-11 b','Kepler-16 b','Kepler-51 d','Kepler-79 d','Kepler-9 b','Kepler-9 c')";

/// Confirmed K2 hosts used only as LC-backed **planets** (not moons).
pub const Q_PS_K2_HOSTS: &str = "SELECT pl_name,hostname,pl_letter,sy_pnum,discoverymethod,disc_year,disc_facility,pl_orbper,pl_orbsmax,pl_rade,pl_radj,pl_bmasse,pl_bmassj,pl_bmasselim,pl_orbeccen,pl_orbincl,pl_imppar,pl_trandep,pl_trandur,st_teff,st_rad,st_mass,default_flag,tran_flag FROM ps WHERE default_flag=1 AND pl_name IN ('K2-3 b','K2-3 c','K2-18 b','K2-18 c')";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatalogSource {
    KoiCumulative,
    PlanetarySystems,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogPlanet {
    pub id: String,
    pub name: String,
    pub source: CatalogSource,
    pub period_days: f64,
    pub rp_earth: Option<f64>,
    pub mp_earth: Option<f64>,
    pub mp_is_upper_limit: bool,
    pub a_au: Option<f64>,
    pub impact_b: Option<f64>,
    pub duration_hr: Option<f64>,
    pub depth_ppm: Option<f64>,
    pub incl_deg: Option<f64>,
    pub rstar_rsun: Option<f64>,
    pub mstar_msun: Option<f64>,
    pub teff_k: Option<f64>,
    pub disposition: Option<String>,
    pub kepid: Option<u64>,
    /// KOI `koi_time0bk` (BKJD). Missing on PS-only rows.
    pub epoch_bkjd: Option<f64>,
}

impl CatalogPlanet {
    pub fn is_holdout_host(&self) -> bool {
        is_holdout_host(&self.name)
    }
}

fn parse_f64(s: &str) -> Option<f64> {
    let t = s.trim().trim_matches('"');
    if t.is_empty() {
        None
    } else {
        t.parse().ok()
    }
}

fn parse_string(s: &str) -> String {
    s.trim().trim_matches('"').to_string()
}

fn get<'a>(row: &'a HashMap<String, String>, key: &str) -> &'a str {
    row.get(key).map(String::as_str).unwrap_or("")
}

fn read_csv_rows(path: &Path) -> Result<Vec<HashMap<String, String>>> {
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(BufReader::new(file));
    let headers = rdr.headers()?.clone();
    let mut out = Vec::new();
    for rec in rdr.records() {
        let rec = rec?;
        let mut map = HashMap::new();
        for (h, v) in headers.iter().zip(rec.iter()) {
            map.insert(h.to_string(), v.to_string());
        }
        out.push(map);
    }
    Ok(out)
}

fn planet_from_koi(row: &HashMap<String, String>) -> Option<CatalogPlanet> {
    let name = parse_string(get(row, "kepler_name"));
    let id = parse_string(get(row, "kepoi_name"));
    if id.is_empty() {
        return None;
    }
    let period = parse_f64(get(row, "koi_period"))?;
    let display = if name.is_empty() { id.clone() } else { name };
    Some(CatalogPlanet {
        id,
        name: display,
        source: CatalogSource::KoiCumulative,
        period_days: period,
        rp_earth: parse_f64(get(row, "koi_prad")),
        mp_earth: None,
        mp_is_upper_limit: false,
        a_au: parse_f64(get(row, "koi_sma")),
        impact_b: parse_f64(get(row, "koi_impact")),
        duration_hr: parse_f64(get(row, "koi_duration")),
        depth_ppm: parse_f64(get(row, "koi_depth")),
        incl_deg: parse_f64(get(row, "koi_incl")),
        rstar_rsun: parse_f64(get(row, "koi_srad")),
        mstar_msun: parse_f64(get(row, "koi_smass")),
        teff_k: parse_f64(get(row, "koi_steff")),
        disposition: {
            let d = parse_string(get(row, "koi_disposition"));
            if d.is_empty() {
                None
            } else {
                Some(d)
            }
        },
        kepid: parse_f64(get(row, "kepid")).map(|v| v as u64),
        epoch_bkjd: parse_f64(get(row, "koi_time0bk")),
    })
}

fn planet_from_ps(row: &HashMap<String, String>) -> Option<CatalogPlanet> {
    let name = parse_string(get(row, "pl_name"));
    if name.is_empty() {
        return None;
    }
    let period = parse_f64(get(row, "pl_orbper"))?;
    // PS transit depth is in percent (NASA TAP unit). Convert to ppm when present.
    let depth_ppm = parse_f64(get(row, "pl_trandep")).map(|pct| pct * 1.0e4);
    let lim = parse_f64(get(row, "pl_bmasselim")).unwrap_or(0.0);
    Some(CatalogPlanet {
        id: name.clone(),
        name,
        source: CatalogSource::PlanetarySystems,
        period_days: period,
        rp_earth: parse_f64(get(row, "pl_rade")),
        mp_earth: parse_f64(get(row, "pl_bmasse")),
        mp_is_upper_limit: lim > 0.0,
        a_au: parse_f64(get(row, "pl_orbsmax")),
        impact_b: parse_f64(get(row, "pl_imppar")),
        duration_hr: parse_f64(get(row, "pl_trandur")),
        depth_ppm,
        incl_deg: parse_f64(get(row, "pl_orbincl")),
        rstar_rsun: parse_f64(get(row, "st_rad")),
        mstar_msun: parse_f64(get(row, "st_mass")),
        teff_k: parse_f64(get(row, "st_teff")),
        disposition: Some("PS_DEFAULT".into()),
        kepid: None,
        epoch_bkjd: None,
    })
}

fn merge_ps_onto_koi(koi: &mut CatalogPlanet, ps: &CatalogPlanet) {
    if koi.mp_earth.is_none() {
        koi.mp_earth = ps.mp_earth;
        koi.mp_is_upper_limit = ps.mp_is_upper_limit;
    }
    if koi.a_au.is_none() {
        koi.a_au = ps.a_au;
    }
    if koi.incl_deg.is_none() {
        koi.incl_deg = ps.incl_deg;
    }
    if koi.impact_b.is_none() {
        koi.impact_b = ps.impact_b;
    }
    if koi.kepid.is_none() {
        koi.kepid = ps.kepid;
    }
    if koi.epoch_bkjd.is_none() {
        koi.epoch_bkjd = ps.epoch_bkjd;
    }
    if koi.duration_hr.is_none() {
        koi.duration_hr = ps.duration_hr;
    }
}

/// Load the in-repo cache. KOI geometry is the training backbone; PS overlays
/// masses / limits for named systems. Holdout hosts stay in the catalog so
/// score cards can be built; they are excluded from training later.
pub fn load_cache(cache_dir: &Path) -> Result<Vec<CatalogPlanet>> {
    let mut by_name: HashMap<String, CatalogPlanet> = HashMap::new();

    for fname in [
        "nasa_koi_confirmed_geometry_sample.csv",
        "nasa_koi_long_period_sample.csv",
        "nasa_koi_named_systems.csv",
        "nasa_koi_lc_hosts.csv",
    ] {
        let path = cache_dir.join(fname);
        if !path.exists() {
            return Err(ExoError::Parse(format!("missing cache file {fname}")));
        }
        for row in read_csv_rows(&path)? {
            if let Some(p) = planet_from_koi(&row) {
                by_name.entry(p.name.clone()).or_insert(p);
            }
        }
    }

    for fname in [
        "nasa_ps_named_systems.csv",
        "nasa_ps_kepler_transiting_sample.csv",
        "nasa_ps_k2_hosts.csv",
    ] {
        let path = cache_dir.join(fname);
        if !path.exists() {
            continue;
        }
        for row in read_csv_rows(&path)? {
            if let Some(ps) = planet_from_ps(&row) {
                if let Some(existing) = by_name.get_mut(&ps.name) {
                    merge_ps_onto_koi(existing, &ps);
                } else if ps.is_holdout_host() || fname.contains("named") || fname.contains("k2") {
                    // Keep named / holdout / K2 LC-host PS rows even when KOI
                    // is missing (Kepler-1708 b is not in the cumulative KOI pull).
                    by_name.entry(ps.name.clone()).or_insert(ps);
                }
            }
        }
    }

    let mut planets: Vec<CatalogPlanet> = by_name.into_values().collect();
    planets.sort_by(|a, b| {
        a.period_days
            .partial_cmp(&b.period_days)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.name.cmp(&b.name))
    });
    Ok(planets)
}

pub fn tap_url(query: &str) -> String {
    let encoded: String = urlencoding_lite(query);
    format!("{TAP_SYNC}?query={encoded}&format=csv")
}

/// Minimal application/x-www-form-urlencoded encoder (no extra crate).
fn urlencoding_lite(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

fn download(url: &str) -> Result<String> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(90))
        .call()
        .map_err(|e| ExoError::Http(e.to_string()))?;
    if resp.status() != 200 {
        return Err(ExoError::Http(format!("HTTP {}", resp.status())));
    }
    resp.into_string()
        .map_err(|e| ExoError::Http(e.to_string()))
}

pub struct FetchSpec {
    pub filename: &'static str,
    pub query: &'static str,
}

pub fn fetch_specs() -> Vec<FetchSpec> {
    vec![
        FetchSpec {
            filename: "nasa_koi_confirmed_geometry_sample.csv",
            query: Q_KOI_CONFIRMED_GEOMETRY,
        },
        FetchSpec {
            filename: "nasa_koi_long_period_sample.csv",
            query: Q_KOI_LONG_PERIOD,
        },
        FetchSpec {
            filename: "nasa_koi_named_systems.csv",
            query: Q_KOI_NAMED,
        },
        FetchSpec {
            filename: "nasa_koi_lc_hosts.csv",
            query: Q_KOI_LC_HOSTS,
        },
        FetchSpec {
            filename: "nasa_ps_kepler_transiting_sample.csv",
            query: Q_PS_KEPLER_SAMPLE,
        },
        FetchSpec {
            filename: "nasa_ps_named_systems.csv",
            query: Q_PS_NAMED,
        },
        FetchSpec {
            filename: "nasa_ps_k2_hosts.csv",
            query: Q_PS_K2_HOSTS,
        },
    ]
}

pub fn fetch_cache(cache_dir: &Path) -> Result<Vec<PathBuf>> {
    fs::create_dir_all(cache_dir)?;
    let mut written = Vec::new();
    for spec in fetch_specs() {
        let url = tap_url(spec.query);
        let body = download(&url)?;
        if !body.contains(',') {
            return Err(ExoError::Http(format!(
                "TAP response for {} did not look like CSV",
                spec.filename
            )));
        }
        let path = cache_dir.join(spec.filename);
        let mut f = File::create(&path)?;
        f.write_all(body.as_bytes())?;
        written.push(path);
    }
    let lc_paths = crate::lightcurve::fetch_lightcurves(cache_dir)?;
    written.extend(lc_paths);
    match crate::ttv_catalog::fetch_holczer(cache_dir) {
        Ok(p) => written.push(p),
        Err(e) => {
            // CDS TAP is flaky (503). Keep the in-repo Holczer extract.
            eprintln!("Holczer table4 refresh skipped (cache kept): {e}");
        }
    }
    match crate::jwst_search::fetch_jwst_go6491(cache_dir) {
        Ok(paths) => written.extend(paths),
        Err(e) => {
            eprintln!("JWST GO 6491 metadata refresh skipped (cache kept): {e}");
        }
    }
    Ok(written)
}
