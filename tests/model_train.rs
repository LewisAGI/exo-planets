use exo_planets::ingest::load_cache;
use exo_planets::model::{evaluate, train};
use exo_planets::pipeline::build_dataset;
use std::path::Path;

#[test]
fn linfa_recovers_injected_timing_better_than_null() {
    let planets = load_cache(Path::new("data/cache")).unwrap();
    let (train_rows, hold_rows) = build_dataset(&planets).unwrap();
    let model = train(&train_rows).expect("linfa fit");
    assert_eq!(
        model.weights.len(),
        exo_planets::features::FEATURE_NAMES.len()
    );
    assert!(model.backend.contains("linfa"));
    let ev = evaluate(&model, &train_rows);
    assert!(
        ev.mean_p_injected_on_injected > ev.mean_p_injected_on_planet_only + 0.15,
        "model should rank injected above null (got {} vs {})",
        ev.mean_p_injected_on_injected,
        ev.mean_p_injected_on_planet_only
    );
    assert!(ev.accuracy > 0.7, "synthetic recovery acc={}", ev.accuracy);

    // Holdout rows must not have been fit as moon-positive.
    assert!(hold_rows.iter().all(|r| r.target.is_none()));
    // Scoring them must not panic; we do not assert a detection.
    for r in &hold_rows {
        let p = model.predict_proba(&r.vector);
        assert!(p.is_finite());
        assert!((0.0..=1.0).contains(&p));
    }
}
