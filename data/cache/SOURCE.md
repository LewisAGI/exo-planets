# Cache source (NASA Exoplanet Archive TAP + Holczer 2016)

Pulled 2026-08-27 over public HTTP. **No secrets.** Full MAST / Kepler light-curve
archives are huge; this repo keeps a small TAP/CSV slice, a Holczer Table 4
extract, **and** small PDCSAP CSVs so `cargo test` and `exo-planets all`
work offline. Light-curve files: [`lightcurves/SOURCE.md`](lightcurves/SOURCE.md).

Endpoint: `https://exoplanetarchive.ipac.caltech.edu/TAP/sync?query=...&format=csv`

Re-fetch (needs network):

```bash
cargo run -- fetch --cache data/cache
```

Same queries are hardcoded in `src/ingest.rs`. Holczer refresh is CDS HTTP
(`table4.dat`); a 503 leaves this cache in place.

| File | Table | What it is |
|---|---|---|
| `nasa_koi_confirmed_geometry_sample.csv` | `cumulative` | TOP 60 confirmed KOIs with period, impact, duration, depth, SMA, R*, M* |
| `nasa_koi_long_period_sample.csv` | `cumulative` | TOP 25 confirmed KOIs with P > 100 d and the same geometry columns |
| `nasa_koi_named_systems.csv` | `cumulative` | Kepler-1625 b, Kepler-167 e, Kepler-90 g, Kepler-10 b, Kepler-22 b. **Kepler-1708 b is not in this cumulative pull** (no `kepler_name` match; KIC hunt did not return a 737 d KOI). |
| `nasa_koi_lc_hosts.csv` | `cumulative` | Kepler-2/3/4–9/11/18–21/23/24/26–33/36/37/41–44/48–62/65/66/68/69/74–76/79–81/83–85/88/89/92–95/100/102/138/186/444 (letters as listed) KOI epochs for cached Q1 LCs; not invented. Misses Q1: 11 f/g, 12 b, 25 b, 30 c, 51 c/d, 62 b, 62 f, 68 b, 80 b, 186 f. Kepler-67 b Q1 LLC 404; not cached. |
| `nasa_ps_kepler_transiting_sample.csv` | `ps` | TOP 80 Kepler transiting default rows (masses often empty) |
| `nasa_ps_named_systems.csv` | `ps` | Named systems including Kepler-1625 b, Kepler-1708 b, Kepler-167 e. Kepler-1708 b `pl_bmasselim=1` (upper limit 4.6 M_J). **Kepler-90 g has no `default_flag=1` PS row** — host parameters come from the KOI cumulative row. |
| `nasa_ps_k2_hosts.csv` | `ps` | K2-3 b/c and K2-18 b/c default rows (confirmed planets for the K2 C1 LCs). No `pl_tranmid` in this slice — extra-dip is unwindowed. K2-18 c is RV-only (`tran_flag=0`). |
| `jwst_go6491_search.json` | MAST DOI 10.17909/e50n-4y96 | GO 6491 **SEARCH** fixture. Metadata only. NIRSpec time series **not** cached. Residual 7–17 min is lock text, not a moon. |
| `jwst_go6491_mast_caom.csv` | MAST CAOM | One calib_level=3 science row (Kepler-167, NIRSPEC/SLIT, CLEAR;PRISM). Not a light curve. |
| `holczer2016_table4_oc_scatter.csv` | VizieR `J/ApJS/225/9` table4 | Holczer+2016 O−C **scatter** (1.4826×MAD, minutes) and median TTV uncertainty. **Planet-only timing**, often planet–planet. Not moons. NASA TAP has `ttv_flag` on PS, not a TTV time series. Holdout KOIs 351.02 / 490.02 / 5084.01 are absent. Kepler-1708 is 2021 — not in this 2016 table. Source: `https://cdsarc.cds.unistra.fr/ftp/J/ApJS/225/9/table4.dat` |
| `lightcurves/*.csv` | Kepler / K2 LLC + TESS SPOC extract | Real MAST PDCSAP slices. See [`lightcurves/SOURCE.md`](lightcurves/SOURCE.md). |

Units we rely on (NASA TAP docs / Holczer ReadMe, not guessed fills):

- `koi_depth`: ppm
- `pl_trandep`: percent → converted to ppm in ingest
- `koi_duration`, `pl_trandur`: hours
- `koi_sma`, `pl_orbsmax`: AU
- `koi_prad`, `pl_rade`: Earth radii
- `pl_bmasse`: Earth masses; `pl_bmasselim=1` means upper limit
- Holczer `sigTT`: median TTV **uncertainty** (min)
- Holczer `S(O-C)`: O−C scatter (min)

Empty cells stay empty. Missing SMA is derived from Kepler III only when P and M* exist, and is flagged `a_from_kepler3`.

Public portals (not downloaded in full):

- MAST https://mast.stsci.edu/
- Kepler https://archive.stsci.edu/missions-and-data/kepler
- K2 https://archive.stsci.edu/missions-and-data/k2
- TESS https://archive.stsci.edu/missions-and-data/tess
- NASA Exoplanet Archive https://exoplanetarchive.ipac.caltech.edu/
- Holczer+2016 VizieR https://cdsarc.cds.unistra.fr/viz-bin/cat/J/ApJS/225/9
- Kepler-1625 / 1708 / 167 overview pages there
- Columbia 1625 products https://academiccommons.columbia.edu/doi/10.7916/D8795NHS
- JWST GO 6491 DOI https://archive.stsci.edu/doi/resolve/resolve.html?doi=10.17909/e50n-4y96
