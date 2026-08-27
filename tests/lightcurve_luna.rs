use exo_planets::constants::HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION;
use exo_planets::features::geometry_for;
use exo_planets::ingest::load_cache;
use exo_planets::inject::MoonHypothesis;
use exo_planets::labels::ExampleKind;
use exo_planets::lightcurve::{load_lightcurves, n_cached_lightcurves, peek_header};
use exo_planets::luna::{luna_style_flags, moon_radius_earth_extrap};
use exo_planets::photometry::photometry_flags;
use exo_planets::pipeline::build_dataset;
use exo_planets::tdv::MoonSense;
use exo_planets::ttv_catalog::load_holczer;
use std::path::Path;

fn slice<'a>(
    lcs: &'a exo_planets::lightcurve::LightCurveIndex,
    name: &str,
) -> Option<&'a [exo_planets::lightcurve::LightCurve]> {
    lcs.get(name).map(|v| v.as_slice())
}

#[test]
fn cached_lcs_are_real_pdcsap_kepler_k2_tess() {
    let dir = Path::new("data/cache");
    let lcs = load_lightcurves(dir).unwrap();
    assert!(
        n_cached_lightcurves(&lcs) >= 8,
        "expected Kepler-10 (2), Kepler-1, Kepler-22, 1625, 1708, 167, K2-3"
    );
    for name in [
        "Kepler-10 b",
        "Kepler-1 b",
        "Kepler-22 b",
        "Kepler-1625 b",
        "Kepler-1708 b",
        "Kepler-167 e",
        "K2-3 b",
    ] {
        let curves = lcs.get(name).unwrap_or_else(|| panic!("missing {name}"));
        assert!(curves.iter().any(|lc| lc.len() > 500), "{name} too short");
        for lc in curves {
            assert!(lc.flux.iter().all(|f| f.is_finite() && *f > 0.0));
            assert!(lc.source_url.contains("archive.stsci.edu"));
        }
    }
    let k10 = lcs.get("Kepler-10 b").unwrap();
    assert!(
        k10.iter().any(|lc| lc.mission == "Kepler"),
        "Kepler-10 Kepler Q1"
    );
    assert!(
        k10.iter().any(|lc| lc.mission == "TESS"),
        "Kepler-10 TESS S14"
    );
    let tess = k10.iter().find(|lc| lc.mission == "TESS").unwrap();
    assert!(
        tess.time_bkjd.iter().all(|t| *t > 3000.0),
        "TESS times must be BKJD (BTJD+2167), not raw BTJD"
    );
    let k2 = lcs.get("K2-3 b").unwrap()[0].time_bkjd[0];
    assert!(k2 > 1900.0 && k2 < 2200.0, "K2 C1 is BKJD ~1977–2057");
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
    let photo = photometry_flags(k10, &geom, slice(&lcs, "Kepler-10 b"));
    assert!(photo.lc_available);
    assert!(photo.n_points > 500);
    assert!(
        photo.n_in_transit > 0,
        "Q1 / TESS S14 should cover Kepler-10 b transits"
    );
    assert!(photo.notes.contains("not a moon"));
    if photo.photometry_only_would_flag {
        assert!(photo.hek_v_caution);
        assert!(photo.notes.contains("HEK V"));
    }

    let k22 = planets.iter().find(|p| p.name == "Kepler-22 b").unwrap();
    let p22 = photometry_flags(k22, &geometry_for(k22), slice(&lcs, "Kepler-22 b"));
    assert!(p22.lc_available);
    assert!(
        p22.n_in_transit > 0,
        "Kepler-22 b catalog epoch t0≈133.70 falls in Q1; transit is catalog, not invented"
    );

    let k1625 = planets.iter().find(|p| p.name == "Kepler-1625 b").unwrap();
    let g2 = geometry_for(k1625);
    let p2 = photometry_flags(k1625, &g2, slice(&lcs, "Kepler-1625 b"));
    assert!(p2.lc_available);
    assert!(p2.notes.contains("CANDIDATE"));
    assert_eq!(
        p2.n_in_transit, 0,
        "Q8 window has no catalog transit; do not invent one"
    );

    let k1708 = planets.iter().find(|p| p.name == "Kepler-1708 b").unwrap();
    let p1708 = photometry_flags(k1708, &geometry_for(k1708), slice(&lcs, "Kepler-1708 b"));
    assert!(p1708.lc_available);
    assert_eq!(
        p1708.n_in_transit, 0,
        "1708 Q1 has no catalog transit; do not invent one"
    );
    assert!(p1708.notes.contains("CANDIDATE"));

    let k167 = planets.iter().find(|p| p.name == "Kepler-167 e").unwrap();
    let p167 = photometry_flags(k167, &geometry_for(k167), slice(&lcs, "Kepler-167 e"));
    assert!(p167.lc_available);
    assert_eq!(
        p167.n_in_transit, 0,
        "167e Q1 has no catalog transit; do not invent one"
    );
    assert!(p167.notes.contains("SEARCH"));
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
    let ttv = load_holczer(Path::new("data/cache")).unwrap();
    let (train, hold) = build_dataset(&planets, &lcs, &ttv).unwrap();
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
    assert!(hold
        .iter()
        .any(|r| r.name == "Kepler-1708 b" && r.photometry.lc_available));
    assert!(hold
        .iter()
        .any(|r| r.name == "Kepler-167 e" && r.photometry.lc_available));
    assert!(hold.iter().all(|r| r.target.is_none()));
}
