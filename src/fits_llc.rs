//! Minimal Kepler/K2 long-cadence FITS BINTABLE reader (public MAST files).
//!
//! This is not a general FITS library. It reads TIME / PDCSAP_FLUX /
//! PDCSAP_FLUX_ERR / SAP_QUALITY from a LIGHTCURVE extension.

use crate::error::{ExoError, Result};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LlcRow {
    pub time_bkjd: f64,
    pub flux: f64,
    pub flux_err: f64,
    pub quality: i32,
}

fn read_header_cards(data: &[u8], mut off: usize) -> Result<(Vec<String>, usize)> {
    let mut cards = Vec::new();
    loop {
        if off + 2880 > data.len() {
            return Err(ExoError::Parse("truncated FITS header".into()));
        }
        for i in 0..36 {
            let s = off + i * 80;
            let card = String::from_utf8_lossy(&data[s..s + 80]).into_owned();
            cards.push(card.clone());
            if card.starts_with("END") {
                return Ok((cards, off + 2880));
            }
        }
        off += 2880;
    }
}

fn header_map(cards: &[String]) -> HashMap<String, String> {
    let mut kv = HashMap::new();
    for c in cards {
        if c.starts_with("END") || c.starts_with("COMMENT") || c.starts_with("HISTORY") {
            continue;
        }
        let key = c[..8.min(c.len())].trim().to_string();
        if key.is_empty() {
            continue;
        }
        if let Some(eq) = c.find('=') {
            let rest = c[eq + 1..].split('/').next().unwrap_or("").trim();
            let val = rest.trim_matches('\'').trim().to_string();
            kv.insert(key, val);
        }
    }
    kv
}

fn parse_i(s: &str) -> i64 {
    s.trim().parse().unwrap_or(0)
}

fn hdu_data_len(h: &HashMap<String, String>) -> usize {
    let naxis = parse_i(h.get("NAXIS").map(String::as_str).unwrap_or("0"));
    if naxis <= 0 {
        return 0;
    }
    let bitpix =
        parse_i(h.get("BITPIX").map(String::as_str).unwrap_or("8")).unsigned_abs() as usize;
    let gcount = parse_i(h.get("GCOUNT").map(String::as_str).unwrap_or("1")).max(1) as usize;
    let pcount = parse_i(h.get("PCOUNT").map(String::as_str).unwrap_or("0")).max(0) as usize;
    let mut n = 1usize;
    for i in 1..=naxis {
        n *= parse_i(
            h.get(&format!("NAXIS{i}"))
                .map(String::as_str)
                .unwrap_or("0"),
        )
        .max(0) as usize;
    }
    (bitpix / 8) * gcount * (pcount + n)
}

fn pad2880(n: usize) -> usize {
    (2880 - (n % 2880)) % 2880
}

fn form_size(form: &str) -> Option<(char, usize, usize)> {
    let form = form.trim();
    let fmt = form.chars().last()?;
    let nrep: usize = form[..form.len() - 1].parse().unwrap_or(1);
    let unit = match fmt {
        'D' => 8,
        'E' => 4,
        'J' => 4,
        'K' => 8,
        'I' => 2,
        'B' | 'L' | 'A' => 1,
        _ => return None,
    };
    Some((fmt, nrep, unit * nrep))
}

fn read_be_f64(row: &[u8], off: usize) -> f64 {
    let mut b = [0u8; 8];
    b.copy_from_slice(&row[off..off + 8]);
    f64::from_be_bytes(b)
}

fn read_be_f32(row: &[u8], off: usize) -> f32 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&row[off..off + 4]);
    f32::from_be_bytes(b)
}

fn read_be_i32(row: &[u8], off: usize) -> i32 {
    let mut b = [0u8; 4];
    b.copy_from_slice(&row[off..off + 4]);
    i32::from_be_bytes(b)
}

/// Parse a Kepler/K2 LLC FITS buffer into finite PDCSAP samples.
pub fn parse_llc_fits(data: &[u8]) -> Result<Vec<LlcRow>> {
    let (cards0, mut off) = read_header_cards(data, 0)?;
    let h0 = header_map(&cards0);
    let d0 = hdu_data_len(&h0);
    off += d0 + pad2880(d0);
    let (cards, off) = read_header_cards(data, off)?;
    let h = header_map(&cards);
    let xten = h.get("XTENSION").map(String::as_str).unwrap_or("");
    if !xten.contains("BINTABLE") {
        return Err(ExoError::Parse(format!("expected BINTABLE, got {xten}")));
    }
    let nfields = parse_i(h.get("TFIELDS").map(String::as_str).unwrap_or("0")) as usize;
    let rowlen = parse_i(h.get("NAXIS1").map(String::as_str).unwrap_or("0")) as usize;
    let nrows = parse_i(h.get("NAXIS2").map(String::as_str).unwrap_or("0")) as usize;
    if rowlen == 0 || nrows == 0 {
        return Err(ExoError::Parse("empty BINTABLE".into()));
    }
    let mut want: HashMap<String, (char, usize)> = HashMap::new();
    let mut cursor = 0usize;
    for i in 1..=nfields {
        let name = h
            .get(&format!("TTYPE{i}"))
            .map(|s| s.trim().to_string())
            .unwrap_or_default();
        let form = h
            .get(&format!("TFORM{i}"))
            .map(String::as_str)
            .unwrap_or("D");
        let (fmt, _n, sz) =
            form_size(form).ok_or_else(|| ExoError::Parse(format!("unsupported TFORM {form}")))?;
        if matches!(
            name.as_str(),
            "TIME" | "PDCSAP_FLUX" | "PDCSAP_FLUX_ERR" | "SAP_QUALITY"
        ) {
            want.insert(name, (fmt, cursor));
        }
        cursor += sz;
    }
    let time = *want
        .get("TIME")
        .ok_or_else(|| ExoError::Parse("TIME column missing".into()))?;
    let flux = *want
        .get("PDCSAP_FLUX")
        .ok_or_else(|| ExoError::Parse("PDCSAP_FLUX column missing".into()))?;
    let ferr = want.get("PDCSAP_FLUX_ERR").copied();
    let qual = want.get("SAP_QUALITY").copied();
    if off + rowlen * nrows > data.len() {
        return Err(ExoError::Parse("truncated BINTABLE data".into()));
    }
    let mut out = Vec::new();
    for r in 0..nrows {
        let row = &data[off + r * rowlen..off + (r + 1) * rowlen];
        let t = match time.0 {
            'D' => read_be_f64(row, time.1),
            'E' => read_be_f32(row, time.1) as f64,
            _ => continue,
        };
        let f = match flux.0 {
            'D' => read_be_f64(row, flux.1),
            'E' => read_be_f32(row, flux.1) as f64,
            _ => continue,
        };
        if !t.is_finite() || !f.is_finite() || f <= 0.0 {
            continue;
        }
        let e = ferr
            .map(|(fmt, o)| match fmt {
                'D' => read_be_f64(row, o),
                'E' => read_be_f32(row, o) as f64,
                _ => f64::NAN,
            })
            .unwrap_or(f64::NAN);
        let q = qual
            .map(|(fmt, o)| match fmt {
                'J' => read_be_i32(row, o),
                _ => 0,
            })
            .unwrap_or(0);
        out.push(LlcRow {
            time_bkjd: t,
            flux: f,
            flux_err: if e.is_finite() { e } else { 0.0 },
            quality: q,
        });
    }
    if out.is_empty() {
        return Err(ExoError::Parse("no finite PDCSAP samples".into()));
    }
    Ok(out)
}
