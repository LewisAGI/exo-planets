use exo_planets::fits_llc::parse_llc_fits;
use std::path::Path;

fn card(s: &str) -> String {
    let mut c = s.to_string();
    while c.len() < 80 {
        c.push(' ');
    }
    c.truncate(80);
    c
}

fn header_block(cards: &[&str]) -> Vec<u8> {
    let mut v = Vec::new();
    for c in cards {
        v.extend(card(c).into_bytes());
    }
    v.extend(card("END").into_bytes());
    while v.len() % 2880 != 0 {
        v.push(b' ');
    }
    v
}

#[test]
fn parse_synthetic_llc_bintable() {
    let mut fits = header_block(&[
        "SIMPLE  =                    T",
        "BITPIX  =                    8",
        "NAXIS   =                    0",
    ]);
    let ext = header_block(&[
        "XTENSION= 'BINTABLE'",
        "BITPIX  =                    8",
        "NAXIS   =                    2",
        "NAXIS1  =                   24",
        "NAXIS2  =                    2",
        "PCOUNT  =                    0",
        "GCOUNT  =                    1",
        "TFIELDS =                    4",
        "TTYPE1  = 'TIME    '",
        "TFORM1  = 'D       '",
        "TTYPE2  = 'PDCSAP_FLUX'",
        "TFORM2  = 'E       '",
        "TTYPE3  = 'PDCSAP_FLUX_ERR'",
        "TFORM3  = 'E       '",
        "TTYPE4  = 'SAP_QUALITY'",
        "TFORM4  = 'J       '",
    ]);
    fits.extend(ext);
    // row: D + E + E + J = 8+4+4+4 = 20 — pad to NAXIS1=24
    let mut data = Vec::new();
    for (t, f) in [(131.5_f64, 1000.0_f32), (131.52, 999.5)] {
        data.extend(t.to_be_bytes());
        data.extend(f.to_be_bytes());
        data.extend(1.0_f32.to_be_bytes());
        data.extend(0_i32.to_be_bytes());
        data.extend([0u8; 4]);
    }
    while data.len() % 2880 != 0 {
        data.push(0);
    }
    fits.extend(data);
    let rows = parse_llc_fits(&fits).unwrap();
    assert_eq!(rows.len(), 2);
    assert!((rows[0].time_bkjd - 131.5).abs() < 1e-9);
    assert!((rows[0].flux - 1000.0).abs() < 1e-3);
}

#[test]
fn parse_downloaded_kepler_q1_when_present() {
    let p = Path::new("/tmp/mast_lc/kplr011904151-2009166043257_llc.fits");
    if !p.exists() {
        return;
    }
    let bytes = std::fs::read(p).unwrap();
    let rows = parse_llc_fits(&bytes).unwrap();
    assert!(rows.len() > 1000);
    assert!(rows.iter().all(|r| r.flux > 0.0 && r.time_bkjd.is_finite()));
}
