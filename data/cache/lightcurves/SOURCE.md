# Light-curve cache (MAST / Kepler / K2 / TESS public HTTP)

Pulled 2026-08-27. **No secrets.** These are small PDCSAP extracts from real
Kepler / K2 long-cadence FITS and a TESS SPOC 2-min FITS on the public STScI
archive (the same bits MAST serves). Full mission light curves are huge;
this is a cached slice.

Re-fetch (needs network; Rust FITS reader, no Python):

```bash
cargo run -- fetch --cache data/cache
```

TESS TIME is BTJD (BJD−2457000). Cached TESS times are shifted to BKJD
(+2167 d = 2457000−2454833). TESS S14 is mid-span 2500 PDCSAP points.

| Cache CSV | Host | ID | File | URL |
|---|---|---|---|---|
| `kepler10b_kic11904151_q1_llc.csv` | Kepler-10 b | KIC 11904151 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0119/011904151/kplr011904151-2009166043257_llc.fits |
| `kepler10b_tic377780790_tess_s14_pdcsap.csv` | Kepler-10 b | TIC 377780790 | TESS S14 SPOC 2-min | https://archive.stsci.edu/missions/tess/tid/s0014/0000/0003/7778/0790/tess2019198215352-s0014-0000000377780790-0150-s_lc.fits |
| `kepler1b_kic11446443_q1_llc.csv` | Kepler-1 b (TrES-2b) | KIC 11446443 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0114/011446443/kplr011446443-2009166043257_llc.fits |
| `k2_3_epic201367065_c01_llc.csv` | K2-3 b | EPIC 201367065 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits |
| `kepler1625b_kic4760478_q8_llc.csv` | Kepler-1625 b | KIC 4760478 | Q8 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits |
| `kepler1708b_kic7906827_q1_llc.csv` | Kepler-1708 b | KIC 7906827 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0079/007906827/kplr007906827-2009166043257_llc.fits |
| `kepler167e_kic3239945_q1_llc.csv` | Kepler-167 e | KIC 3239945 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0032/003239945/kplr003239945-2009166043257_llc.fits |

Columns: `time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality` (finite PDCSAP only).
TESS FITS uses `QUALITY` (not `SAP_QUALITY`); the extract header stays the same.

Honesty:

- Kepler-10 b, Kepler-1 b, and K2-3 b are **confirmed planets**, used as
  LC-backed training hosts. They are not moon detections.
- Kepler-1625 b, Kepler-1708 b, and Kepler-167 e are **holdouts**.
  Q8 (1625; BKJD ≈ 735–802) and Q1 (1708, 167e; BKJD ≈ 131–165) do **not**
  cover a catalog transit (P ≈ 287 / 737 / 1071 d). No transit was invented.
  Statuses stay **CANDIDATE / SEARCH**.
- Kepler-1708 has a public Kepler LLC (preferred over FFI-only TESS).
  Kepler-167 e likewise. TESS for 1708 is FFI-only (TIC 272716898); no
  TESSCut photometry was invented.
- K2-3 b cached PS row has no transit epoch; extra-dip is unwindowed.
- No Hubble, JWST, or Columbia 1625 photometry products.
- Extra-dip flags computed from these files are **not** LUNA and **not**
  confirmations. HEK V: photometry-only false-claims moons in ~1/4 of KOIs.
  The crate’s HEK V demo on these LCs is that **caution**, not a detection.

Portals: https://mast.stsci.edu/ · https://archive.stsci.edu/missions-and-data/kepler · https://archive.stsci.edu/missions-and-data/k2 · https://archive.stsci.edu/missions-and-data/tess
