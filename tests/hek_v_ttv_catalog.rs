use exo_planets::constants::HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION;
use exo_planets::hek_v_demo::hek_v_photometry_only_caution;
use exo_planets::ingest::load_cache;
use exo_planets::labels::ExampleKind;
use exo_planets::lightcurve::load_lightcurves;
use exo_planets::lightcurve::TESS_BTJD_TO_BKJD_DAYS;
use exo_planets::pipeline::build_dataset;
use exo_planets::ttv_catalog::{
    kepoi_to_holczer_key, load_holczer, lookup_holczer, parse_table4_dat, HOLCZER_SOURCE,
};
use std::path::Path;

#[test]
fn holczer_key_and_published_planet_only_oc() {
    assert_eq!(kepoi_to_holczer_key("K00072.01").as_deref(), Some("72.01"));
    assert_eq!(kepoi_to_holczer_key("K00001.01").as_deref(), Some("1.01"));
    assert_eq!(
        kepoi_to_holczer_key("K05084.01").as_deref(),
        Some("5084.01")
    );
    assert!(kepoi_to_holczer_key("Kepler-1708 b").is_none());

    let idx = load_holczer(Path::new("data/cache")).unwrap();
    assert!(idx.len() > 2000, "expected Holczer table4 extract");
    let k10 = idx.get("72.01").expect("Kepler-10 b = KOI 72.01");
    assert!(k10.s_oc_min > 0.0 && k10.s_oc_min.is_finite());
    assert!(k10.source.contains("Holczer"));
    assert!(idx.get("1.01").is_some());
    // Holdout KOIs are absent from this 2016 table — do not invent rows.
    assert!(idx.get("351.02").is_none(), "Kepler-90 g");
    assert!(idx.get("490.02").is_none(), "Kepler-167 e");
    assert!(idx.get("5084.01").is_none(), "Kepler-1625 b");
    let _ = HOLCZER_SOURCE;
}

#[test]
fn holczer_table4_fixed_width_parse() {
    // Bytes 1-7 KOI, 9-14 sigTT, 16-22 S(O-C), 25-29 pval (Holczer+2016 ReadMe).
    let mut line = String::new();
    line.push_str(&format!("{:>7.2}", 1.01));
    line.push(' ');
    line.push_str(&format!("{:>6.2}", 0.08));
    line.push(' ');
    line.push_str(&format!("{:>7.2}", 0.09));
    line.push(' ');
    line.push_str(&format!("{:>5.1}", -0.0));
    line.push_str(" rest\n");
    let csv = parse_table4_dat(&line).unwrap_or_else(|_| {
        // Parser requires ≥100 rows; pad with copies at distinct KOIs.
        let mut dat = String::new();
        for i in 1..=120 {
            let mut l = String::new();
            l.push_str(&format!("{:>7.2}", i as f64 + 0.01));
            l.push(' ');
            l.push_str(&format!("{:>6.2}", 0.10));
            l.push(' ');
            l.push_str(&format!("{:>7.2}", 0.20));
            l.push(' ');
            l.push_str(&format!("{:>5.1}", -0.1));
            l.push('\n');
            dat.push_str(&l);
        }
        parse_table4_dat(&dat).unwrap()
    });
    assert!(csv.contains("koi,sig_tt_min,s_oc_min"));
    assert!(csv.lines().count() > 100);
}

#[test]
fn planet_only_rows_use_holczer_when_matched() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let ttv = load_holczer(Path::new("data/cache")).unwrap();
    let k10 = planets.iter().find(|p| p.name == "Kepler-10 b").unwrap();
    let published = lookup_holczer(&ttv, k10).expect("Kepler-10 in Holczer");
    let (train, _) = build_dataset(&planets, &lcs, &ttv).unwrap();
    let row = train
        .iter()
        .find(|r| r.name == "Kepler-10 b" && r.kind == ExampleKind::PlanetOnly)
        .unwrap();
    assert!(
        (row.timing.ttv_rms_min - published.s_oc_min).abs() < 1e-9,
        "planet-only TTV must be Holczer S(O-C), got {} vs {}",
        row.timing.ttv_rms_min,
        published.s_oc_min
    );
    assert!(row.timing.noise_model.contains("planet-only"));
    assert!(row.timing.noise_model.contains("not a moon"));
}

#[test]
fn hek_v_demo_is_a_caution_not_a_detection() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let demo = hek_v_photometry_only_caution(&planets, &lcs);
    assert!(
        demo.n_planet_only_lightcurves >= 4,
        "Kepler-10 Kepler+TESS, Kepler-1, Kepler-22, K2-3"
    );
    assert!(!demo.per_lc.iter().any(|r| {
        r.planet_name == "Kepler-1625 b"
            || r.planet_name == "Kepler-1708 b"
            || r.planet_name == "Kepler-167 e"
    }));
    assert_eq!(
        demo.published_hek_v_false_fraction,
        HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION
    );
    assert!(demo.note.contains("Not a moon detection"));
    assert!(demo.note.contains("Not a re-estimate"));
    // Do not loosen the cut: 0/N is an allowed honest outcome.
    assert!(demo.n_would_flag_photometry_only <= demo.n_planet_only_lightcurves);
    for row in &demo.per_lc {
        assert!(row.extra_dip_snr.is_finite());
        assert!(row.n_points > 0);
    }
    let k2 = demo
        .per_lc
        .iter()
        .find(|r| r.planet_name == "K2-3 b")
        .expect("K2-3 b in HEK V demo");
    assert!(
        !k2.windowed,
        "cached PS row has no epoch; do not invent one"
    );
}

#[test]
fn tess_btjd_to_bkjd_offset_is_2167() {
    assert!((TESS_BTJD_TO_BKJD_DAYS - 2167.0).abs() < 1e-9);
}

#[test]
fn k2_hosts_are_planets_not_moons() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    assert!(planets.iter().any(|p| p.name == "K2-3 b"));
    assert!(planets
        .iter()
        .all(|p| !p.name.to_lowercase().contains("moon")));
}
