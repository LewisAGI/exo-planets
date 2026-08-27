//! exo-planets: Kipping-signature features + a small linfa trainer.
//!
//! Science lock (2026-08-27): only opened Kipping / HEK / FORECASTER papers.
//! Cool Worlds videos are not results. Named moons stay candidate / FP / search.

pub mod constants;
pub mod error;
pub mod features;
pub mod forecaster;
pub mod geometry;
pub mod hek;
pub mod holdout;
pub mod ingest;
pub mod inject;
pub mod labels;
pub mod model;
pub mod pipeline;
pub mod tdv;
pub mod ttv;

pub use error::{ExoError, Result};
pub use pipeline::{run_fetch, run_train_score, PipelineReport};
