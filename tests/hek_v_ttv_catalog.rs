use exo_planets::constants::{HEK_I_SIGMA_THRESHOLD, HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION};
use exo_planets::features::FEATURE_NAMES;
use exo_planets::hek_v_demo::hek_v_photometry_only_caution;
use exo_planets::holdout::locked_cards;
use exo_planets::ingest::load_cache;
use exo_planets::labels::{ExampleKind, HoldoutStatus};
use exo_planets::lightcurve::{load_lightcurves, n_cached_lightcurves, TESS_BTJD_TO_BKJD_DAYS};
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
        demo.n_planet_only_lightcurves >= 12,
        "Kepler-10 Kepler+TESS plus Kepler-1/2/4–9/11/22 and K2 hosts"
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
        if row.photometry_only_would_flag {
            assert!(
                row.extra_dip_snr >= HEK_I_SIGMA_THRESHOLD,
                "{} extra-dip SNR {} below HEK I 4σ scale",
                row.planet_name,
                row.extra_dip_snr
            );
        }
    }
    assert!(
        demo.per_lc.iter().any(|r| r.planet_name == "Kepler-11 b"),
        "Kepler-11 b Q1 is a confirmed-planet LC in the HEK V demo"
    );
    let k11 = demo
        .per_lc
        .iter()
        .find(|r| r.planet_name == "Kepler-11 b")
        .unwrap();
    assert!(
        k11.windowed,
        "KOI lc-host epoch t0≈138.50 is catalog, not invented"
    );
    assert!(k11.n_in_transit > 0);
    assert!(!demo.per_lc.iter().any(|r| r.planet_name == "Kepler-90 g"));
    assert!(
        !demo
            .per_lc
            .iter()
            .any(|r| r.planet_name == "Kepler-11 f" || r.planet_name == "Kepler-11 g"),
        "11 f/g catalog epochs miss Q1; do not invent a window"
    );
    assert!(
        !demo.per_lc.iter().any(|r| r.planet_name == "Kepler-12 b"),
        "Kepler-12 b prior catalog epoch ≈166.57 misses Q1; no LC invented"
    );
    assert!(
        !demo.per_lc.iter().any(|r| r.planet_name == "Kepler-25 b"),
        "Kepler-25 b prior catalog epoch ≈165.47 misses Q1; no LC invented"
    );
    assert!(
        !demo.per_lc.iter().any(|r| r.planet_name == "Kepler-30 c"
            || r.planet_name == "Kepler-62 b"
            || r.planet_name == "Kepler-80 b"
            || r.planet_name == "Kepler-51 c"
            || r.planet_name == "Kepler-51 d"
            || r.planet_name == "Kepler-68 b"
            || r.planet_name == "Kepler-62 f"),
        "30 c / 51 c/d / 62 b / 62 f / 68 b / 80 b catalog epochs miss Q1; do not invent a window"
    );
    for name in ["K2-3 b", "K2-3 c"] {
        let k2 = demo
            .per_lc
            .iter()
            .find(|r| r.planet_name == name)
            .unwrap_or_else(|| panic!("{name} in HEK V demo"));
        assert!(
            !k2.windowed,
            "{name} cached PS row has no epoch; do not invent one"
        );
        assert_eq!(k2.n_in_transit, 0);
    }
}

#[test]
fn tess_btjd_to_bkjd_offset_is_2167() {
    assert!((TESS_BTJD_TO_BKJD_DAYS - 2167.0).abs() < 1e-9);
}

#[test]
fn k2_hosts_are_planets_not_moons() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    assert!(planets.iter().any(|p| p.name == "K2-3 b"));
    assert!(planets.iter().any(|p| p.name == "K2-18 b"));
    assert!(planets
        .iter()
        .all(|p| !p.name.to_lowercase().contains("moon")));
}

#[test]
fn cached_lc_count_and_tess_extract_size() {
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    assert_eq!(n_cached_lightcurves(&lcs), 85);
    let tess = lcs
        .get("Kepler-10 b")
        .unwrap()
        .iter()
        .find(|lc| lc.mission == "TESS")
        .unwrap();
    assert_eq!(tess.len(), 2500);
    assert!(tess.time_bkjd[0] > 3800.0);
}

#[test]
fn holczer_kepler1_is_planet_only_scatter() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let ttv = load_holczer(Path::new("data/cache")).unwrap();
    let k1 = planets.iter().find(|p| p.name == "Kepler-1 b").unwrap();
    let pubd = lookup_holczer(&ttv, k1).expect("Kepler-1 b = KOI 1.01");
    assert!((pubd.s_oc_min - 0.09).abs() < 1e-6);
    assert!(pubd.source.contains("Holczer"));
}

#[test]
fn injected_rows_are_not_labelled_holczer_planet_only() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let ttv = load_holczer(Path::new("data/cache")).unwrap();
    let (train, _) = build_dataset(&planets, &lcs, &ttv).unwrap();
    for r in train.iter().filter(|r| r.kind == ExampleKind::Injected) {
        assert!(r.id.contains("injected"));
        assert!(r.timing.noise_model.contains("injected"));
        assert!(!r.timing.noise_model.contains("not a moon"));
        assert_eq!(r.vector.len(), FEATURE_NAMES.len());
    }
}

#[test]
fn hek_v_kepler22_is_windowed_catalog_transit() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let demo = hek_v_photometry_only_caution(&planets, &lcs);
    let k22 = demo
        .per_lc
        .iter()
        .find(|r| r.planet_name == "Kepler-22 b")
        .expect("Kepler-22 b in HEK V demo");
    assert!(k22.windowed);
    assert!(k22.n_in_transit > 0);
    for name in [
        "Kepler-2 b",
        "Kepler-4 b",
        "Kepler-5 b",
        "Kepler-6 b",
        "Kepler-7 b",
        "Kepler-8 b",
        "Kepler-9 b",
        "Kepler-9 c",
        "Kepler-11 c",
        "Kepler-11 d",
        "Kepler-11 e",
        "Kepler-3 b",
        "Kepler-18 b",
        "Kepler-18 c",
        "Kepler-18 d",
        "Kepler-19 b",
        "Kepler-20 b",
        "Kepler-20 c",
        "Kepler-20 d",
        "Kepler-20 e",
        "Kepler-20 f",
        "Kepler-21 b",
        "Kepler-30 b",
        "Kepler-30 d",
        "Kepler-36 b",
        "Kepler-36 c",
        "Kepler-48 b",
        "Kepler-48 c",
        "Kepler-48 d",
        "Kepler-51 b",
        "Kepler-79 b",
        "Kepler-79 c",
        "Kepler-79 d",
        "Kepler-79 e",
        "Kepler-68 c",
        "Kepler-89 b",
        "Kepler-89 c",
        "Kepler-89 d",
        "Kepler-89 e",
        "Kepler-102 b",
        "Kepler-102 c",
        "Kepler-102 d",
        "Kepler-102 e",
        "Kepler-102 f",
        "Kepler-62 c",
        "Kepler-62 d",
        "Kepler-62 e",
        "Kepler-37 b",
        "Kepler-37 c",
        "Kepler-37 d",
        "Kepler-444 b",
        "Kepler-444 c",
        "Kepler-444 d",
        "Kepler-444 e",
        "Kepler-444 f",
        "Kepler-42 b",
        "Kepler-42 c",
        "Kepler-42 d",
        "Kepler-138 b",
        "Kepler-138 c",
        "Kepler-138 d",
        "Kepler-65 b",
        "Kepler-65 c",
        "Kepler-65 d",
        "Kepler-32 b",
        "Kepler-32 c",
        "Kepler-32 d",
        "Kepler-32 e",
        "Kepler-32 f",
        "Kepler-33 b",
        "Kepler-33 c",
        "Kepler-33 d",
        "Kepler-33 e",
        "Kepler-33 f",
    ] {
        let row = demo
            .per_lc
            .iter()
            .find(|r| r.planet_name == name)
            .unwrap_or_else(|| panic!("{name} in HEK V demo"));
        assert!(row.windowed, "{name} uses catalog KOI epoch");
        assert!(row.n_in_transit > 0, "{name} Q1 covers a catalog transit");
    }
    // Cache fraction is not the published 1/4.
    assert!((demo.fraction_on_this_cache - HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION).abs() > 1e-6);
}

#[test]
fn locked_cards_still_four_named_statuses() {
    let cards = locked_cards();
    assert_eq!(cards.len(), 4);
    assert_eq!(
        cards
            .iter()
            .find(|c| c.object_id == "Kepler-1625b-i")
            .unwrap()
            .status,
        HoldoutStatus::Candidate
    );
    assert_eq!(
        cards
            .iter()
            .find(|c| c.object_id == "Kepler-1708 b-i")
            .unwrap()
            .status,
        HoldoutStatus::Candidate
    );
    assert_eq!(
        cards
            .iter()
            .find(|c| c.object_id == "Kepler-90g moon")
            .unwrap()
            .status,
        HoldoutStatus::FalsePositive
    );
    assert_eq!(
        cards
            .iter()
            .find(|c| c.object_id.contains("167"))
            .unwrap()
            .status,
        HoldoutStatus::Search
    );
}
