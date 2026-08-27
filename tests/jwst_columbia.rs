use exo_planets::columbia1625::columbia_1625_skip;
use exo_planets::holdout::locked_cards;
use exo_planets::jwst_search::{caom_json_has_kepler167, load_jwst_go6491};
use exo_planets::labels::HoldoutStatus;
use exo_planets::pipeline::run_train_score;
use std::path::Path;

#[test]
fn jwst_go6491_fixture_is_search_without_photometry() {
    let fix = load_jwst_go6491(Path::new("data/cache"))
        .unwrap()
        .expect("GO 6491 search fixture");
    assert!(fix.is_search());
    assert_eq!(fix.status, "SEARCH");
    assert!(!fix.photometry_cached);
    assert_eq!(fix.doi, "10.17909/e50n-4y96");
    assert_eq!(fix.proposal_id, "6491");
    assert_eq!(fix.target_name, "Kepler-167");
    assert!(fix.obs_id_cal3.contains("jw06491"));
    assert!(fix.t_max_mjd > fix.t_min_mjd);
    assert!(fix.locked_residual_note.contains("Do not promote a moon"));
    assert!(fix.note.contains("not a detection"));
    assert!(!fix.mast_product_uri.ends_with(".csv"));
}

#[test]
fn jwst_caom_csv_is_metadata_not_a_lightcurve() {
    let body = std::fs::read_to_string("data/cache/jwst_go6491_mast_caom.csv").unwrap();
    assert!(caom_json_has_kepler167(&body) || body.contains("Kepler-167"));
    assert!(body.contains("6491"));
    assert!(body.contains("t_min_mjd"));
    assert!(!body.contains("pdcsap_flux"));
}

#[test]
fn columbia_1625_skipped_candidate_unconfirmed() {
    let skip = columbia_1625_skip();
    assert!(!skip.cached);
    assert_eq!(skip.status, "CANDIDATE");
    assert_eq!(skip.object_id, "Kepler-1625b-i");
    assert!(skip.note.contains("unconfirmed"));
    assert!(skip.note.contains("CANDIDATE"));
    assert!(skip.http_notes.iter().any(|n| n.contains("404")));
    assert!(skip.http_notes.iter().any(|n| n.contains("Anubis")));
}

#[test]
fn pipeline_holdouts_keep_jwst_search_and_1625_candidate() {
    let tmp = tempfile_dir();
    let report = run_train_score(Path::new("data/cache"), &tmp).unwrap();
    let j = report.jwst_go6491.expect("fixture");
    assert_eq!(j.status, "SEARCH");
    assert!(!j.photometry_cached);
    assert!(!report.columbia_1625.cached);
    assert_eq!(report.columbia_1625.status, "CANDIDATE");
    let c167 = report
        .holdouts
        .iter()
        .find(|c| c.object_id.contains("167"))
        .unwrap();
    assert_eq!(c167.status, HoldoutStatus::Search);
    assert_eq!(c167.jwst_doi.as_deref(), Some("10.17909/e50n-4y96"));
    assert!(!c167.jwst_photometry_cached);
    let c1625 = report
        .holdouts
        .iter()
        .find(|c| c.object_id == "Kepler-1625b-i")
        .unwrap();
    assert_eq!(c1625.status, HoldoutStatus::Candidate);
    assert!(!c1625.columbia_product_cached);
    assert!(c1625
        .published_product_note
        .as_deref()
        .unwrap()
        .contains("CANDIDATE"));
}

#[test]
fn locked_cards_expose_product_honesty() {
    let cards = locked_cards();
    let jwst = cards.iter().find(|c| c.object_id.contains("6491")).unwrap();
    assert!(!jwst.jwst_photometry_cached);
    assert_eq!(jwst.jwst_doi.as_deref(), Some("10.17909/e50n-4y96"));
    let k1625 = cards
        .iter()
        .find(|c| c.object_id == "Kepler-1625b-i")
        .unwrap();
    assert!(!k1625.columbia_product_cached);
}

fn tempfile_dir() -> std::path::PathBuf {
    let p = std::env::temp_dir().join(format!("exo-planets-jwst-test-{}", std::process::id()));
    let _ = std::fs::create_dir_all(&p);
    p
}
