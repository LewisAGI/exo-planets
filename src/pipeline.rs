//! Fetch → features → train → holdout score cards.

use crate::error::Result;
use crate::features::{build_row, geometry_for, prior_for, resolved_a_au, FeatureRow};
use crate::hek_v_demo::{hek_v_photometry_only_caution, HekVCautionDemo};
use crate::holdout::{attach_catalog_and_model, locked_cards, HoldoutCard};
use crate::ingest::{fetch_cache, load_cache, CatalogPlanet};
use crate::inject::{default_injections, draw_injected, draw_planet_only};
use crate::labels::Split;
use crate::lightcurve::{load_lightcurves, n_cached_lightcurves, LightCurveIndex};
use crate::luna::luna_style_flags;
use crate::model::{evaluate, train, EvalReport, TrainedModel};
use crate::photometry::photometry_flags;
use crate::ttv_catalog::{load_holczer, lookup_holczer, TtvIndex};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

/// Reproducible seed (the number is the KOI of Kepler-1625, not a claim).
pub const RNG_SEED: u64 = 5_084;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineReport {
    pub n_catalog_planets: usize,
    pub n_train_planets: usize,
    pub n_holdout_planets: usize,
    pub n_train_rows: usize,
    pub n_injected_rows: usize,
    pub n_planet_only_rows: usize,
    pub n_cached_lightcurves: usize,
    pub n_published_ttv_matches: usize,
    pub hek_v_demo: HekVCautionDemo,
    pub eval_held_fraction: EvalReport,
    pub eval_train: EvalReport,
    pub holdouts: Vec<HoldoutCard>,
    pub model: TrainedModel,
}

pub fn build_dataset(
    planets: &[CatalogPlanet],
    lcs: &LightCurveIndex,
    ttv: &TtvIndex,
) -> Result<(Vec<FeatureRow>, Vec<FeatureRow>)> {
    let mut rng = ChaCha8Rng::seed_from_u64(RNG_SEED);
    let mut train_rows = Vec::new();
    let mut holdout_rows = Vec::new();
    for planet in planets {
        if planet.period_days <= 0.0 || planet.rstar_rsun.is_none() || planet.mstar_msun.is_none() {
            continue;
        }
        let geom = geometry_for(planet);
        let prior = prior_for(planet);
        let photo = photometry_flags(planet, &geom, lcs.get(&planet.name).map(|v| v.as_slice()));
        let split = if planet.is_holdout_host() {
            Split::Holdout
        } else {
            Split::Train
        };
        let null = draw_planet_only(&mut rng, planet, &geom, lookup_holczer(ttv, planet));
        let luna0 = luna_style_flags(planet, &geom, &prior, None, resolved_a_au(planet, &geom));
        let row = build_row(planet, &geom, &prior, &null, split, &photo, &luna0);
        if split == Split::Holdout {
            holdout_rows.push(row);
            continue;
        }
        train_rows.push(row);
        if let Some(a_au) = resolved_a_au(planet, &geom) {
            for hypo in default_injections() {
                if let Some(draw) = draw_injected(&mut rng, planet, &geom, &prior, hypo, a_au) {
                    let luna = luna_style_flags(
                        planet,
                        &geom,
                        &prior,
                        draw.hypothesis.as_ref(),
                        Some(a_au),
                    );
                    train_rows.push(build_row(
                        planet,
                        &geom,
                        &prior,
                        &draw,
                        Split::Train,
                        &photo,
                        &luna,
                    ));
                }
            }
        }
    }
    Ok((train_rows, holdout_rows))
}

fn stratified_holdout<'a>(
    rows: &'a [FeatureRow],
    frac: f64,
    seed: u64,
) -> (Vec<&'a FeatureRow>, Vec<&'a FeatureRow>) {
    let mut inj: Vec<&FeatureRow> = rows
        .iter()
        .filter(|r| r.kind == crate::labels::ExampleKind::Injected)
        .collect();
    let mut po: Vec<&FeatureRow> = rows
        .iter()
        .filter(|r| r.kind == crate::labels::ExampleKind::PlanetOnly)
        .collect();
    inj.sort_by(|a, b| a.id.cmp(&b.id));
    po.sort_by(|a, b| a.id.cmp(&b.id));
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    use rand::seq::SliceRandom;
    inj.shuffle(&mut rng);
    po.shuffle(&mut rng);
    let n_inj = ((inj.len() as f64) * frac).round() as usize;
    let n_po = ((po.len() as f64) * frac).round() as usize;
    let mut eval = Vec::new();
    let mut train = Vec::new();
    eval.extend(inj.iter().take(n_inj).copied());
    train.extend(inj.iter().skip(n_inj).copied());
    eval.extend(po.iter().take(n_po).copied());
    train.extend(po.iter().skip(n_po).copied());
    (train, eval)
}

pub fn run_train_score(cache_dir: &Path, out_dir: &Path) -> Result<PipelineReport> {
    std::fs::create_dir_all(out_dir)?;
    let planets = load_cache(cache_dir)?;
    let lcs = load_lightcurves(cache_dir)?;
    let ttv = load_holczer(cache_dir)?;
    let (train_rows, holdout_rows) = build_dataset(&planets, &lcs, &ttv)?;
    let n_ttv = planets
        .iter()
        .filter(|p| lookup_holczer(&ttv, p).is_some())
        .count();
    let hek_v_demo = hek_v_photometry_only_caution(&planets, &lcs);
    let n_injected = train_rows
        .iter()
        .filter(|r| r.kind == crate::labels::ExampleKind::Injected)
        .count();
    let n_po = train_rows
        .iter()
        .filter(|r| r.kind == crate::labels::ExampleKind::PlanetOnly)
        .count();

    let (fit_set, eval_set) = stratified_holdout(&train_rows, 0.25, RNG_SEED + 7);
    let fit_owned: Vec<FeatureRow> = fit_set.into_iter().cloned().collect();
    let eval_owned: Vec<FeatureRow> = eval_set.into_iter().cloned().collect();
    let model = train(&fit_owned)?;
    let mut eval_held = evaluate(&model, &eval_owned);
    eval_held.n_train = fit_owned.len();
    let mut eval_tr = evaluate(&model, &fit_owned);
    eval_tr.n_train = fit_owned.len();

    let cards = attach_catalog_and_model(locked_cards(), &planets, &holdout_rows, &model);

    let report = PipelineReport {
        n_catalog_planets: planets.len(),
        n_train_planets: planets.iter().filter(|p| !p.is_holdout_host()).count(),
        n_holdout_planets: planets.iter().filter(|p| p.is_holdout_host()).count(),
        n_train_rows: train_rows.len(),
        n_injected_rows: n_injected,
        n_planet_only_rows: n_po,
        n_cached_lightcurves: n_cached_lightcurves(&lcs),
        n_published_ttv_matches: n_ttv,
        hek_v_demo,
        eval_held_fraction: eval_held,
        eval_train: eval_tr,
        holdouts: cards,
        model: model.clone(),
    };

    model.save(&out_dir.join("model.json"))?;
    write_json(&out_dir.join("train_rows.json"), &train_rows)?;
    write_json(&out_dir.join("holdout_rows.json"), &holdout_rows)?;
    write_json(&out_dir.join("report.json"), &report)?;
    write_json(&out_dir.join("holdout_scorecards.json"), &report.holdouts)?;
    Ok(report)
}

pub fn run_fetch(cache_dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    fetch_cache(cache_dir)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    let f = File::create(path)?;
    serde_json::to_writer_pretty(BufWriter::new(f), value)?;
    Ok(())
}

pub fn print_report(report: &PipelineReport) {
    println!("catalog planets: {}", report.n_catalog_planets);
    println!(
        "cached MAST Kepler/K2/TESS light curves: {}",
        report.n_cached_lightcurves
    );
    println!(
        "Holczer+2016 Table 4 planet-only O-C matches: {} (not moons)",
        report.n_published_ttv_matches
    );
    println!(
        "HEK V photometry-only caution demo: {}/{} cached confirmed-planet LCs would trip the extra-dip cut (published HEK V ~1/4 of KOIs; this cache is not that experiment). {}",
        report.hek_v_demo.n_would_flag_photometry_only,
        report.hek_v_demo.n_planet_only_lightcurves,
        report.hek_v_demo.note
    );
    println!(
        "train planets / holdout hosts: {} / {}",
        report.n_train_planets, report.n_holdout_planets
    );
    println!(
        "train rows: {} (planet_only={}, injected={})",
        report.n_train_rows, report.n_planet_only_rows, report.n_injected_rows
    );
    println!(
        "held-out synthetic eval: acc={:.3} inj_recall={:.3} planet_recall={:.3} mean_p(inj|inj)={:.3} mean_p(inj|planet)={:.3}",
        report.eval_held_fraction.accuracy,
        report.eval_held_fraction.injected_recall,
        report.eval_held_fraction.planet_only_recall,
        report.eval_held_fraction.mean_p_injected_on_injected,
        report.eval_held_fraction.mean_p_injected_on_planet_only
    );
    println!("holdout score cards (locked statuses):");
    for c in &report.holdouts {
        println!(
            "  {} [{}] host={} catalog={} P(injected-like)={:?} — {}",
            c.object_id,
            c.status_text,
            c.host_planet,
            c.catalog_found,
            c.model_p_injected_like,
            c.locked_note
        );
    }
    println!("This is not a moon detection pipeline. HEK II–V are null.");
}
