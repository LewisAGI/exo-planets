//! Columbia Academic Commons 1625 products (Teachey & Kipping 2018).
//!
//! Landing DOI https://doi.org/10.7916/D8795NHS resolves, but this environment
//! hits an Anubis bot-challenge page and `/download` is HTTP 404. **No file
//! was cached.** Kepler-1625b-i stays **CANDIDATE** (Hubble-dependent,
//! unconfirmed).

use serde::{Deserialize, Serialize};

pub const COLUMBIA_DOI: &str = "10.7916/D8795NHS";
pub const COLUMBIA_DOI_URL: &str = "https://doi.org/10.7916/D8795NHS";
pub const COLUMBIA_LANDING: &str = "https://academiccommons.columbia.edu/doi/10.7916/D8795NHS";
pub const COLUMBIA_DOWNLOAD_TRIED: &str =
    "https://academiccommons.columbia.edu/doi/10.7916/D8795NHS/download";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Columbia1625Skip {
    pub object_id: String,
    pub host_planet: String,
    pub status: String,
    pub doi: String,
    pub cached: bool,
    pub http_notes: Vec<String>,
    pub note: String,
}

pub fn columbia_1625_skip() -> Columbia1625Skip {
    Columbia1625Skip {
        object_id: "Kepler-1625b-i".into(),
        host_planet: "Kepler-1625 b".into(),
        status: "CANDIDATE".into(),
        doi: COLUMBIA_DOI.into(),
        cached: false,
        http_notes: vec![
            "doi.org resolves to academiccommons.columbia.edu (200 HTML)".into(),
            "page body is Anubis 1.21.3 bot-challenge; no asset URLs in HTML".into(),
            format!("{COLUMBIA_DOWNLOAD_TRIED} → HTTP 404"),
        ],
        note: "No Columbia Academic Commons product cached. Hubble dip remains \
model-dependent; authors call the moon unconfirmed. Status stays CANDIDATE."
            .into(),
    }
}
