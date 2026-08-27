use exo_planets::constants::HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION;
use exo_planets::features::geometry_for;
use exo_planets::ingest::load_cache;
use exo_planets::inject::MoonHypothesis;
use exo_planets::labels::ExampleKind;
use exo_planets::lightcurve::{load_lightcurves, peek_header};
use exo_planets::luna::{luna_style_flags, moon_radius_earth_extrap};
use exo_planets::photometry::photometry_flags;
use exo_planets::pipeline::build_dataset;
use exo_planets::tdv::MoonSense;
use std::path::Path;

#[test]
fn cached_kepler_lcs_are_real_pdcsap() {
    let dir = Path::new("data/cache");
    let lcs = load_lightcurves(dir).unwrap();
    assert!(
        lcs.len() >= 3,
        "expected Kepler-10, Kepler-1, Kepler-1625 LCs"
    );
    for name in ["Kepler-10 b", "Kepler-1 b", "Kepler-1625 b"] {
        let lc = lcs.get(name).unwrap_or_else(|| panic!("missing {name}"));
        assert!(lc.len() > 500, "{name} too short: {}", lc.len());
        assert!(lc.flux.iter().all(|f| f.is_finite() && *f > 0.0));
        assert!(lc.source_url.contains("archive.stsci.edu"));
    }
    let hdr = peek_header(&dir.join("lightcurves/kepler10b_kic11904151_q1_llc.csv")).unwrap();
    assert_eq!(hdr, "time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality");
}

#[test]
fn extra_dip_flag_is_not_a_moon_and_hek_v_fraction_locked() {
    assert_eq!(HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION, 0.25);
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let k10 = planets.iter().find(|p| p.name == "Kepler-10 b").unwrap();
    let geom = geometry_for(k10);
    let photo = photometry_flags(k10, &geom, lcs.get("Kepler-10 b"));
    assert!(photo.lc_available);
    assert!(photo.n_points > 500);
    assert!(
        photo.n_in_transit > 0,
        "Q1 should cover Kepler-10 b transits"
    );
    assert!(photo.notes.contains("not a moon"));
    // A high extra-dip SNR, if any, is HEK V caution — not a confirmation.
    if photo.photometry_only_would_flag {
        assert!(photo.hek_v_caution);
        assert!(photo.notes.contains("HEK V"));
    }

    let k1625 = planets.iter().find(|p| p.name == "Kepler-1625 b").unwrap();
    let g2 = geometry_for(k1625);
    let p2 = photometry_flags(k1625, &g2, lcs.get("Kepler-1625 b"));
    assert!(p2.lc_available);
    assert!(p2.notes.contains("CANDIDATE"));
    assert_eq!(
        p2.n_in_transit, 0,
        "Q8 window has no catalog transit; do not invent one"
    );
}

#[test]
fn luna_style_flags_are_geometric_not_an_integrator() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let k10 = planets.iter().find(|p| p.name == "Kepler-10 b").unwrap();
    let geom = geometry_for(k10);
    let prior = exo_planets::features::prior_for(k10);
    let off = luna_style_flags(k10, &geom, &prior, None, k10.a_au);
    assert!(!off.overlapping_disc_possible);
    assert!(off.method.contains("no moon hypothesis"));

    let hypo = MoonHypothesis {
        ms_earth: 0.3,
        d_hill: 0.25,
        sense: MoonSense::Prograde,
    };
    let on = luna_style_flags(k10, &geom, &prior, Some(&hypo), k10.a_au);
    assert!(on.overlapping_disc_possible);
    assert!(on.syzygy_in_transit_possible);
    assert!(on.extra_dip_on_star_possible);
    assert!(on.moon_k > 0.0);
    assert!(on.method.contains("Not a LUNA integrator"));
    assert!(moon_radius_earth_extrap(1.0) > 0.5);
}

#[test]
fn injected_rows_carry_luna_flags_holdouts_do_not_train_moons() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let (train, hold) = build_dataset(&planets, &lcs).unwrap();
    let inj = train
        .iter()
        .find(|r| r.kind == ExampleKind::Injected)
        .unwrap();
    assert!(inj.luna.overlapping_disc_possible);
    assert!(inj.luna.method.contains("Not a LUNA integrator"));
    let po = train
        .iter()
        .find(|r| r.kind == ExampleKind::PlanetOnly && r.photometry.lc_available)
        .expect("Kepler-10/1 planet_only row with LC");
    assert!(!po.luna.overlapping_disc_possible);
    assert!(hold
        .iter()
        .any(|r| r.name == "Kepler-1625 b" && r.photometry.lc_available));
    assert!(hold.iter().all(|r| r.target.is_none()));
}
