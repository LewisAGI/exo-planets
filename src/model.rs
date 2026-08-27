//! Trainable model: **linfa logistic regression**.
//!
//! Why linfa, not candle/burn: the label set is a few hundred tabular rows
//! (real KOI/PS parameters + synthetic TTV/TDV). A linear classifier on
//! those features is the honest capacity. A neural net would overfit the
//! injections and imply a photodynamical detector we did not build.

use crate::error::{ExoError, Result};
use crate::features::{FeatureRow, FEATURE_NAMES};
use crate::labels::TrainTarget;
use linfa::prelude::*;
use linfa_logistic::LogisticRegression;
use ndarray::{Array1, Array2};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainedModel {
    pub intercept: f64,
    pub weights: Vec<f64>,
    pub feature_names: Vec<String>,
    pub l2_penalty: f64,
    pub backend: String,
    pub task: String,
    pub caveat: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalReport {
    pub n_train: usize,
    pub n_eval: usize,
    pub accuracy: f64,
    pub injected_recall: f64,
    pub planet_only_recall: f64,
    pub mean_p_injected_on_injected: f64,
    pub mean_p_injected_on_planet_only: f64,
    pub notes: Vec<String>,
}

fn matrix(rows: &[&FeatureRow]) -> Result<(Array2<f64>, Array1<bool>)> {
    if rows.is_empty() {
        return Err(ExoError::Model("no rows".into()));
    }
    let n = rows.len();
    let d = FEATURE_NAMES.len();
    let mut x = Array2::<f64>::zeros((n, d));
    let mut y = Array1::<bool>::from_elem(n, false);
    for (i, row) in rows.iter().enumerate() {
        if row.vector.len() != d {
            return Err(ExoError::Model(format!(
                "feature length {} != {}",
                row.vector.len(),
                d
            )));
        }
        for (j, v) in row.vector.iter().enumerate() {
            x[[i, j]] = if v.is_finite() { *v } else { 0.0 };
        }
        y[i] = row.target == Some(TrainTarget::MoonLikeTiming);
    }
    Ok((x, y))
}

fn sigmoid(z: f64) -> f64 {
    1.0 / (1.0 + (-z).exp())
}

impl TrainedModel {
    pub fn predict_proba(&self, vector: &[f64]) -> f64 {
        let mut z = self.intercept;
        for (w, v) in self.weights.iter().zip(vector.iter()) {
            z += w * if v.is_finite() { *v } else { 0.0 };
        }
        sigmoid(z)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let f = File::create(path)?;
        serde_json::to_writer_pretty(BufWriter::new(f), self)?;
        Ok(())
    }

    pub fn load(path: &Path) -> Result<Self> {
        let f = File::open(path)?;
        Ok(serde_json::from_reader(BufReader::new(f))?)
    }
}

pub fn train(rows: &[FeatureRow]) -> Result<TrainedModel> {
    let usable: Vec<&FeatureRow> = rows
        .iter()
        .filter(|r| r.target.is_some() && r.split == crate::labels::Split::Train)
        .collect();
    let (x, y) = matrix(&usable)?;
    let dataset = Dataset::new(x, y);
    let l2 = 1.0;
    let fitted = LogisticRegression::default()
        .max_iterations(200)
        .alpha(l2)
        .fit(&dataset)
        .map_err(|e| ExoError::Model(format!("{e}")))?;
    Ok(TrainedModel {
        intercept: fitted.intercept(),
        weights: fitted.params().iter().copied().collect(),
        feature_names: FEATURE_NAMES.iter().map(|s| (*s).to_string()).collect(),
        l2_penalty: l2,
        backend: "linfa-logistic 0.7 (L2-regularized logistic regression)".into(),
        task: "P(injected-like TTV/TDV | features). Not P(real exomoon).".into(),
        caveat: "Trained on synthetic nulls + synthetic moons. HEK II–V are null. Holdouts never trained as confirmed moons.".into(),
    })
}

pub fn evaluate(model: &TrainedModel, rows: &[FeatureRow]) -> EvalReport {
    let mut n_inj = 0usize;
    let mut n_po = 0usize;
    let mut hit_inj = 0usize;
    let mut hit_po = 0usize;
    let mut sum_p_inj = 0.0;
    let mut sum_p_po = 0.0;
    let mut n = 0usize;
    for row in rows {
        let Some(target) = row.target else { continue };
        n += 1;
        let p = model.predict_proba(&row.vector);
        let pred = p >= 0.5;
        match target {
            TrainTarget::MoonLikeTiming => {
                n_inj += 1;
                sum_p_inj += p;
                if pred {
                    hit_inj += 1;
                }
            }
            TrainTarget::PlanetOnly => {
                n_po += 1;
                sum_p_po += p;
                if !pred {
                    hit_po += 1;
                }
            }
        }
    }
    let acc = if n == 0 {
        0.0
    } else {
        (hit_inj + hit_po) as f64 / n as f64
    };
    EvalReport {
        n_train: 0,
        n_eval: n,
        accuracy: acc,
        injected_recall: if n_inj == 0 {
            0.0
        } else {
            hit_inj as f64 / n_inj as f64
        },
        planet_only_recall: if n_po == 0 {
            0.0
        } else {
            hit_po as f64 / n_po as f64
        },
        mean_p_injected_on_injected: if n_inj == 0 {
            0.0
        } else {
            sum_p_inj / n_inj as f64
        },
        mean_p_injected_on_planet_only: if n_po == 0 {
            0.0
        } else {
            sum_p_po / n_po as f64
        },
        notes: vec![
            "Accuracy is synthetic-signal recovery, not moon discovery.".into(),
            "A high score means 'looks like the injected TTV/TDV class', not 'has a moon'.".into(),
        ],
    }
}
