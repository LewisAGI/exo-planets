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
| `kepler11b_kic6541920_q1_llc.csv` | Kepler-11 b | KIC 6541920 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0065/006541920/kplr006541920-2009166043257_llc.fits |
| `kepler2b_kic10666592_q1_llc.csv` | Kepler-2 b (HAT-P-7b) | KIC 10666592 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0106/010666592/kplr010666592-2009166043257_llc.fits |
| `kepler3b_kic10748390_q1_llc.csv` | Kepler-3 b (HAT-P-11b) | KIC 10748390 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0107/010748390/kplr010748390-2009166043257_llc.fits |
| `kepler4b_kic11853905_q1_llc.csv` | Kepler-4 b | KIC 11853905 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0118/011853905/kplr011853905-2009166043257_llc.fits |
| `kepler5b_kic8191672_q1_llc.csv` | Kepler-5 b | KIC 8191672 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0081/008191672/kplr008191672-2009166043257_llc.fits |
| `kepler6b_kic10874614_q1_llc.csv` | Kepler-6 b | KIC 10874614 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0108/010874614/kplr010874614-2009166043257_llc.fits |
| `kepler7b_kic5780885_q1_llc.csv` | Kepler-7 b | KIC 5780885 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005780885/kplr005780885-2009166043257_llc.fits |
| `kepler8b_kic6922244_q1_llc.csv` | Kepler-8 b | KIC 6922244 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0069/006922244/kplr006922244-2009166043257_llc.fits |
| `kepler9b_kic3323887_q1_llc.csv` | Kepler-9 b | KIC 3323887 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0033/003323887/kplr003323887-2009166043257_llc.fits |
| `kepler22b_kic10593626_q1_llc.csv` | Kepler-22 b | KIC 10593626 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0105/010593626/kplr010593626-2009166043257_llc.fits |
| `kepler18b_kic8644288_q1_llc.csv` | Kepler-18 b | KIC 8644288 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0086/008644288/kplr008644288-2009166043257_llc.fits |
| `kepler19b_kic2571238_q1_llc.csv` | Kepler-19 b | KIC 2571238 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0025/002571238/kplr002571238-2009166043257_llc.fits |
| `k2_3_epic201367065_c01_llc.csv` | K2-3 b | EPIC 201367065 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits |
| `k2_18_epic201912552_c01_llc.csv` | K2-18 b | EPIC 201912552 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201900000/12000/ktwo201912552-c01_llc.fits |
| `kepler1625b_kic4760478_q8_llc.csv` | Kepler-1625 b | KIC 4760478 | Q8 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits |
| `kepler1708b_kic7906827_q1_llc.csv` | Kepler-1708 b | KIC 7906827 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0079/007906827/kplr007906827-2009166043257_llc.fits |
| `kepler167e_kic3239945_q1_llc.csv` | Kepler-167 e | KIC 3239945 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0032/003239945/kplr003239945-2009166043257_llc.fits |

Columns: `time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality` (finite PDCSAP only).
TESS FITS uses `QUALITY` (not `SAP_QUALITY`); the extract header stays the same.

Honesty:

- Kepler-10 b, Kepler-1 b, Kepler-2 b, Kepler-3 b, Kepler-4–8 b, Kepler-9 b/c,
  Kepler-11 b/c/d/e, Kepler-18 b, Kepler-19 b, Kepler-22 b, K2-3 b/c, and
  K2-18 b are **confirmed planets**, used as LC-backed training hosts. They
  are not moon detections. Kepler-12 b’s prior catalog epoch (≈166.57)
  misses Q1; no extract was invented.
  Kepler-22 b Q1 covers the catalog epoch (t0≈133.70 BKJD); Kepler-11 b/c/d/e
  cover catalog t0≈138.50 / 138.18 / 148.46 / 154.16; Kepler-9 b/c cover
  previous catalog epochs (≈163.27 / ≈136.52). Those transits were not
  invented. Kepler-11 c/d/e and Kepler-9 c reuse the host-star Q1 extract.
  Kepler-11 f/g catalog epochs do not fall in Q1; no transit was invented.
- Kepler-1625 b, Kepler-1708 b, and Kepler-167 e are **holdouts**.
  Q8 (1625; BKJD ≈ 735–802) and Q1 (1708, 167e; BKJD ≈ 131–165) do **not**
  cover a catalog transit (P ≈ 287 / 737 / 1071 d). No transit was invented.
  Statuses stay **CANDIDATE / SEARCH**.
- Kepler-1708 has a public Kepler LLC (preferred over FFI-only TESS).
  Kepler-167 e likewise. TESS for 1708 is FFI-only (TIC 272716898); no
  TESSCut photometry was invented.
- K2-3 b/c cached PS rows have no transit epoch; extra-dip is unwindowed.
  K2-3 c reuses the K2-3 b C1 extract. Do not invent an epoch.
- No Hubble or Columbia 1625 photometry (Academic Commons `/download` 404).
  JWST GO 6491 is metadata only (`jwst_go6491_*.json/csv` in `data/cache/`),
  not a NIRSpec light curve.
- Extra-dip flags computed from these files are **not** LUNA and **not**
  confirmations. HEK V: photometry-only false-claims moons in ~1/4 of KOIs.
  The crate’s HEK V demo on these LCs is that **caution**, not a detection.

Portals: https://mast.stsci.edu/ · https://archive.stsci.edu/missions-and-data/kepler · https://archive.stsci.edu/missions-and-data/k2 · https://archive.stsci.edu/missions-and-data/tess
