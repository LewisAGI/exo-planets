use exo_planets::holdout::locked_cards;
use exo_planets::ingest::load_cache;
use exo_planets::labels::{is_holdout_host, HoldoutStatus, TrainTarget};
use exo_planets::lightcurve::load_lightcurves;
use exo_planets::pipeline::build_dataset;
use exo_planets::ttv_catalog::load_holczer;
use std::path::Path;

#[test]
fn cache_parses_and_contains_locked_hosts() {
    let planets = load_cache(Path::new("data/cache")).expect("cache");
    assert!(
        planets.len() >= 50,
        "expected a real TAP slice, got {}",
        planets.len()
    );
    for host in [
        "Kepler-1625 b",
        "Kepler-1708 b",
        "Kepler-90 g",
        "Kepler-167 e",
    ] {
        assert!(
            planets.iter().any(|p| p.name == host),
            "missing holdout host {host}"
        );
    }
    // Kepler-1708 b mass is an archive upper limit (pl_bmasselim=1).
    let k1708 = planets.iter().find(|p| p.name == "Kepler-1708 b").unwrap();
    assert!(k1708.mp_is_upper_limit, "1708 mass must stay a limit");
}

#[test]
fn holdout_hosts_never_enter_training_as_moons() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let lcs = load_lightcurves(Path::new("data/cache")).unwrap();
    let ttv = load_holczer(Path::new("data/cache")).unwrap();
    let (train, hold) = build_dataset(&planets, &lcs, &ttv).unwrap();
    assert!(train.iter().all(|r| !is_holdout_host(&r.name)));
    assert!(train.iter().all(|r| r.target.is_some()));
    assert!(hold.iter().all(|r| is_holdout_host(&r.name)));
    assert!(hold
        .iter()
        .all(|r| r.target.is_none() || r.kind != exo_planets::labels::ExampleKind::Injected));
    // No confirmed-moon target exists.
    for r in train.iter().chain(hold.iter()) {
        if let Some(t) = r.target {
            assert!(matches!(
                t,
                TrainTarget::PlanetOnly | TrainTarget::MoonLikeTiming
            ));
        }
        if r.kind == exo_planets::labels::ExampleKind::Injected {
            assert!(r.id.contains("injected"));
        }
    }
    assert!(train
        .iter()
        .any(|r| r.kind == exo_planets::labels::ExampleKind::Injected));
    assert!(train
        .iter()
        .any(|r| r.kind == exo_planets::labels::ExampleKind::PlanetOnly));
}

#[test]
fn locked_scorecards_do_not_upgrade() {
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
