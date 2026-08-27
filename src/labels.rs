//! Label taxonomy. There is **no** `ConfirmedMoon` variant.
//!
//! Injected moons are synthetic training only. Named objects are holdout
//! score cards (candidate / false-positive / search).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Split {
    Train,
    Holdout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExampleKind {
    /// Confirmed planet / KOI used as a planet-only row. Timing is Holczer
    /// Table 4 S(O−C) when the KOI matches (planet-only, not a moon), else
    /// synthetic white noise — not a published moon non-detection.
    PlanetOnly,
    /// Synthetic moon TTV/TDV on real planet parameters. Training only.
    Injected,
    /// Locked named object. Never a training target.
    Holdout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HoldoutStatus {
    Candidate,
    FalsePositive,
    Search,
}

impl HoldoutStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            HoldoutStatus::Candidate => "CANDIDATE",
            HoldoutStatus::FalsePositive => "FALSE POSITIVE",
            HoldoutStatus::Search => "SEARCH",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainTarget {
    PlanetOnly = 0,
    MoonLikeTiming = 1,
}

impl TrainTarget {
    pub fn as_bool(self) -> bool {
        matches!(self, TrainTarget::MoonLikeTiming)
    }

    pub fn from_kind(kind: ExampleKind) -> Option<Self> {
        match kind {
            ExampleKind::PlanetOnly => Some(TrainTarget::PlanetOnly),
            ExampleKind::Injected => Some(TrainTarget::MoonLikeTiming),
            ExampleKind::Holdout => None,
        }
    }
}

/// Locked statuses from the 2026-08-27 science note. Do not "upgrade".
pub fn locked_holdout_status(object_id: &str) -> Option<HoldoutStatus> {
    match object_id {
        "Kepler-1625b-i" | "Kepler-1708 b-i" => Some(HoldoutStatus::Candidate),
        "Kepler-90g moon" => Some(HoldoutStatus::FalsePositive),
        "JWST Kepler-167e (GO 6491)" => Some(HoldoutStatus::Search),
        _ => None,
    }
}

pub fn is_holdout_host(name: &str) -> bool {
    crate::constants::HOLDOUT_HOSTS
        .iter()
        .any(|h| h.eq_ignore_ascii_case(name.trim()))
}
