use exo_planets::constants::HEK_V_PHOTOMETRY_ONLY_FALSE_FRACTION;
use exo_planets::constants::{DMAX_PROGRADE, DMAX_RETROGRADE};
use exo_planets::features::{geometry_for, FEATURE_NAMES};
use exo_planets::ingest::load_cache;
use exo_planets::ingest::{CatalogPlanet, CatalogSource};
use exo_planets::inject::MoonHypothesis;
use exo_planets::labels::ExampleKind;
use exo_planets::lightcurve::{load_lightcurves, n_cached_lightcurves, peek_header};
use exo_planets::luna::{
    extra_dip_possible, luna_style_flags, moon_radius_earth_extrap, syzygy_timescale_hours,
};
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
        n_cached_lightcurves(&lcs) >= 141,
        "expected Kepler-10 (2), Kepler-1/2/4–9/11/22 + siblings, 1625, 1708, 167, K2-3, K2-18"
    );
    for name in [
        "Kepler-10 b",
        "Kepler-1 b",
        "Kepler-2 b",
        "Kepler-3 b",
        "Kepler-4 b",
        "Kepler-5 b",
        "Kepler-6 b",
        "Kepler-7 b",
        "Kepler-8 b",
        "Kepler-9 b",
        "Kepler-9 c",
        "Kepler-11 b",
        "Kepler-11 c",
        "Kepler-11 d",
        "Kepler-11 e",
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
        "Kepler-26 b",
        "Kepler-26 c",
        "Kepler-26 d",
        "Kepler-186 b",
        "Kepler-186 c",
        "Kepler-186 d",
        "Kepler-186 e",
        "Kepler-80 c",
        "Kepler-80 d",
        "Kepler-80 e",
        "Kepler-80 f",
        "Kepler-29 b",
        "Kepler-29 c",
        "Kepler-93 b",
        "Kepler-100 b",
        "Kepler-100 c",
        "Kepler-100 d",
        "Kepler-88 b",
        "Kepler-23 b",
        "Kepler-23 c",
        "Kepler-24 b",
        "Kepler-24 c",
        "Kepler-27 b",
        "Kepler-27 c",
        "Kepler-28 b",
        "Kepler-28 c",
        "Kepler-41 b",
        "Kepler-56 b",
        "Kepler-56 c",
        "Kepler-57 b",
        "Kepler-57 c",
        "Kepler-69 b",
        "Kepler-76 b",
        "Kepler-58 b",
        "Kepler-58 c",
        "Kepler-58 d",
        "Kepler-59 b",
        "Kepler-59 c",
        "Kepler-60 b",
        "Kepler-60 c",
        "Kepler-60 d",
        "Kepler-84 b",
        "Kepler-84 c",
        "Kepler-84 d",
        "Kepler-84 e",
        "Kepler-85 b",
        "Kepler-85 c",
        "Kepler-85 d",
        "Kepler-85 e",
        "Kepler-54 b",
        "Kepler-54 c",
        "Kepler-54 d",
        "Kepler-55 b",
        "Kepler-55 c",
        "Kepler-55 d",
        "Kepler-55 e",
        "Kepler-22 b",
        "Kepler-1625 b",
        "Kepler-1708 b",
        "Kepler-167 e",
        "K2-3 b",
        "K2-3 c",
        "K2-18 b",
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

    let k11 = planets.iter().find(|p| p.name == "Kepler-11 b").unwrap();
    assert!(
        k11.epoch_bkjd.is_some(),
        "KOI lc-host pull supplies Kepler-11 b t0≈138.50"
    );
    let p11 = photometry_flags(k11, &geometry_for(k11), slice(&lcs, "Kepler-11 b"));
    assert!(p11.lc_available);
    assert!(
        p11.n_in_transit > 0,
        "Kepler-11 b catalog epoch t0≈138.50 falls in Q1; transit is catalog, not invented"
    );

    let k2b = planets.iter().find(|p| p.name == "Kepler-2 b").unwrap();
    let p2b = photometry_flags(k2b, &geometry_for(k2b), slice(&lcs, "Kepler-2 b"));
    assert!(p2b.lc_available);
    assert!(
        p2b.n_in_transit > 0,
        "Kepler-2 b catalog ephemeris (P≈2.20 d) has transits in Q1; not invented"
    );

    let k8 = planets.iter().find(|p| p.name == "Kepler-8 b").unwrap();
    let p8 = photometry_flags(k8, &geometry_for(k8), slice(&lcs, "Kepler-8 b"));
    assert!(p8.lc_available);
    assert!(
        p8.n_in_transit > 0,
        "Kepler-8 b catalog ephemeris (P≈3.52 d) has transits in Q1; not invented"
    );

    let k9 = planets.iter().find(|p| p.name == "Kepler-9 b").unwrap();
    let t0 = k9.epoch_bkjd.expect("Kepler-9 b KOI epoch");
    assert!(
        (t0 - 182.5436).abs() < 1e-3,
        "catalog t0, not an invented Q1 epoch"
    );
    let p9 = photometry_flags(k9, &geometry_for(k9), slice(&lcs, "Kepler-9 b"));
    assert!(p9.lc_available);
    assert!(
        p9.n_in_transit > 0,
        "previous catalog epoch ≈163.27 falls in Q1; not invented"
    );

    for name in [
        "Kepler-4 b",
        "Kepler-5 b",
        "Kepler-6 b",
        "Kepler-7 b",
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
        "Kepler-26 b",
        "Kepler-26 c",
        "Kepler-26 d",
        "Kepler-186 b",
        "Kepler-186 c",
        "Kepler-186 d",
        "Kepler-186 e",
        "Kepler-80 c",
        "Kepler-80 d",
        "Kepler-80 e",
        "Kepler-80 f",
        "Kepler-29 b",
        "Kepler-29 c",
        "Kepler-93 b",
        "Kepler-100 b",
        "Kepler-100 c",
        "Kepler-100 d",
        "Kepler-88 b",
        "Kepler-23 b",
        "Kepler-23 c",
        "Kepler-24 b",
        "Kepler-24 c",
        "Kepler-27 b",
        "Kepler-27 c",
        "Kepler-28 b",
        "Kepler-28 c",
        "Kepler-41 b",
        "Kepler-56 b",
        "Kepler-56 c",
        "Kepler-57 b",
        "Kepler-57 c",
        "Kepler-69 b",
        "Kepler-76 b",
        "Kepler-58 b",
        "Kepler-58 c",
        "Kepler-58 d",
        "Kepler-59 b",
        "Kepler-59 c",
        "Kepler-60 b",
        "Kepler-60 c",
        "Kepler-60 d",
        "Kepler-84 b",
        "Kepler-84 c",
        "Kepler-84 d",
        "Kepler-84 e",
        "Kepler-85 b",
        "Kepler-85 c",
        "Kepler-85 d",
        "Kepler-85 e",
        "Kepler-54 b",
        "Kepler-54 c",
        "Kepler-54 d",
        "Kepler-55 b",
        "Kepler-55 c",
        "Kepler-55 d",
        "Kepler-55 e",
    ] {
        let p = planets.iter().find(|pl| pl.name == name).unwrap();
        assert!(p.epoch_bkjd.is_some(), "{name} catalog epoch");
        let flags = photometry_flags(p, &geometry_for(p), slice(&lcs, name));
        assert!(flags.lc_available, "{name} LC");
        assert!(
            flags.n_in_transit > 0,
            "{name} Q1 covers a catalog transit; not invented"
        );
    }
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
    assert!(!on.moon_can_miss_star);
    assert!(on.d_inside_dmax);
    assert!(on.moon_k > 0.0);
    assert!(on.syzygy_timescale_hr > 0.0);
    assert!(on.method.contains("Not a LUNA integrator"));
    assert!(moon_radius_earth_extrap(1.0) > 0.5);

    let no_a = luna_style_flags(k10, &geom, &prior, Some(&hypo), None);
    assert!(!no_a.overlapping_disc_possible);
    assert!(no_a.method.contains("no moon hypothesis") || no_a.moon_k == 0.0);

    let unstable = MoonHypothesis {
        ms_earth: 0.3,
        d_hill: DMAX_PROGRADE + 0.2,
        sense: MoonSense::Prograde,
    };
    let far = luna_style_flags(k10, &geom, &prior, Some(&unstable), k10.a_au);
    assert!(!far.d_inside_dmax);
    assert!(far.method.contains("Not a LUNA integrator"));

    assert!(extra_dip_possible(0.2, 0.5, 0.01));
    assert!(
        extra_dip_possible(0.2, 0.5, 0.02),
        "wide moon can still graze the star"
    );
    assert!(
        !extra_dip_possible(1.8, 0.2, 0.02),
        "high-b planet + tight moon misses the star"
    );
    assert!(!extra_dip_possible(2.5, 0.1, 0.01));
    assert!(
        !extra_dip_possible(1.0, 0.0, 0.0),
        "grazing-equal |b|-a_S/R* = 1+k is a miss (strict <)"
    );
    assert!(extra_dip_possible(0.999, 0.0, 0.0));
    let t_syz = syzygy_timescale_hours(0.1, 10.0);
    assert!(t_syz > 0.0);
    assert!((t_syz - 0.1 * 10.0 * 24.0 / (2.0 * std::f64::consts::PI)).abs() < 1e-12);
    assert_eq!(syzygy_timescale_hours(0.0, 10.0), 0.0);
    assert_eq!(syzygy_timescale_hours(0.1, 0.0), 0.0);

    let retro_ok = MoonHypothesis {
        ms_earth: 0.3,
        d_hill: DMAX_RETROGRADE,
        sense: MoonSense::Retrograde,
    };
    let retro = luna_style_flags(k10, &geom, &prior, Some(&retro_ok), k10.a_au);
    assert!(retro.d_inside_dmax);
    let retro_far = MoonHypothesis {
        ms_earth: 0.3,
        d_hill: DMAX_RETROGRADE + 0.05,
        sense: MoonSense::Retrograde,
    };
    let retro_out = luna_style_flags(k10, &geom, &prior, Some(&retro_far), k10.a_au);
    assert!(!retro_out.d_inside_dmax);

    let high_b = CatalogPlanet {
        id: "geom-high-b".into(),
        name: "geom-high-b".into(),
        source: CatalogSource::KoiCumulative,
        period_days: 10.0,
        rp_earth: Some(2.0),
        mp_earth: Some(10.0),
        mp_is_upper_limit: false,
        a_au: Some(0.09),
        impact_b: Some(1.8),
        duration_hr: Some(4.0),
        depth_ppm: Some(400.0),
        incl_deg: Some(86.0),
        rstar_rsun: Some(1.0),
        mstar_msun: Some(1.0),
        teff_k: Some(5700.0),
        disposition: Some("CONFIRMED".into()),
        kepid: None,
        epoch_bkjd: None,
    };
    let tight = MoonHypothesis {
        ms_earth: 0.1,
        d_hill: 0.05,
        sense: MoonSense::Prograde,
    };
    let miss = luna_style_flags(
        &high_b,
        &geometry_for(&high_b),
        &exo_planets::features::prior_for(&high_b),
        Some(&tight),
        high_b.a_au,
    );
    assert!(miss.overlapping_disc_possible);
    assert!(miss.moon_can_miss_star);
    assert!(!miss.extra_dip_on_star_possible);
    assert!(
        !miss.syzygy_in_transit_possible,
        "syzygy needs the moon able to sit on the star"
    );
    assert!(miss.method.contains("Not a LUNA integrator"));

    let mut no_rstar = k10.clone();
    no_rstar.rstar_rsun = None;
    let blank = luna_style_flags(&no_rstar, &geom, &prior, Some(&hypo), k10.a_au);
    assert!(!blank.overlapping_disc_possible);
    assert!(!blank.syzygy_in_transit_possible);
    assert_eq!(blank.syzygy_timescale_hr, 0.0);
    assert!(blank.method.contains("no moon hypothesis"));

    let mut no_mstar = k10.clone();
    no_mstar.mstar_msun = None;
    let blank_m = luna_style_flags(&no_mstar, &geom, &prior, Some(&hypo), k10.a_au);
    assert!(!blank_m.overlapping_disc_possible);
    assert!(blank_m.method.contains("no moon hypothesis"));
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
    assert_eq!(FEATURE_NAMES.len(), 25);
    assert!(
        !FEATURE_NAMES.contains(&"syzygy_timescale_hr")
            && !FEATURE_NAMES.contains(&"d_inside_dmax")
            && !FEATURE_NAMES.contains(&"moon_can_miss_star"),
        "new LUNA-style scalars stay off the model vector (geometry notes only)"
    );
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
