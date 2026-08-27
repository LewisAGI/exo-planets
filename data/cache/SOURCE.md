# Cache source (NASA Exoplanet Archive TAP)

Pulled 2026-08-27 over public HTTP. **No secrets.** Full MAST / Kepler light-curve
archives are huge; this repo keeps a small TAP/CSV slice so `cargo test` and
`exo-planets all` work offline.

Endpoint: `https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=...&format=csv`

Re-fetch (needs network):

```bash
cargo run -- fetch --cache data/cache
```

Same queries are hardcoded in `src/ingest.rs`.

| File | Table | What it is |
|---|---|---|
| `nasa_koi_confirmed_geometry_sample.csv` | `cumulative` | TOP 60 confirmed KOIs with period, impact, duration, depth, SMA, R*, M* |
| `nasa_koi_long_period_sample.csv` | `cumulative` | TOP 25 confirmed KOIs with P > 100 d and the same geometry columns |
| `nasa_koi_named_systems.csv` | `cumulative` | Kepler-1625 b, Kepler-167 e, Kepler-90 g, Kepler-10 b, Kepler-22 b. **Kepler-1708 b is not in this cumulative pull** (no `kepler_name` match; KIC hunt did not return a 737 d KOI). |
| `nasa_ps_kepler_transiting_sample.csv` | `ps` | TOP 80 Kepler transiting default rows (masses often empty) |
| `nasa_ps_named_systems.csv` | `ps` | Named systems including Kepler-1625 b, Kepler-1708 b, Kepler-167 e. Kepler-1708 b `pl_bmasselim=1` (upper limit 4.6 M_J). **Kepler-90 g has no `default_flag=1` PS row** — host parameters come from the KOI cumulative row. |

Units we rely on (NASA TAP docs, not guessed fills):

- `koi_depth`: ppm
- `pl_trandep`: percent → converted to ppm in ingest
- `koi_duration`, `pl_trandur`: hours
- `koi_sma`, `pl_orbsmax`: AU
- `koi_prad`, `pl_rade`: Earth radii
- `pl_bmasse`: Earth masses; `pl_bmasselim=1` means upper limit

Empty cells stay empty. Missing SMA is derived from Kepler III only when P and M* exist, and is flagged `a_from_kepler3`.

Public portals (not downloaded in full):

- MAST https://mast.stsci.edu/
- Kepler https://archive.stsci.edu/missions-and-data/kepler
- NASA Exoplanet Archive https://exoplanetarchive.ipac.caltech.edu/
- Kepler-1625 / 1708 / 167 overview pages there
- Columbia 1625 products https://academiccommons.columbia.edu/doi/10.7916/D8795NHS
- JWST GO 6491 DOI https://archive.stsci.edu/doi/resolve/resolve.html?doi=10.17909/e50n-4y96
