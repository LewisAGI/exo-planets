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
| `kepler18b_kic8644288_q1_llc.csv` | Kepler-18 b/c/d | KIC 8644288 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0086/008644288/kplr008644288-2009166043257_llc.fits |
| `kepler19b_kic2571238_q1_llc.csv` | Kepler-19 b | KIC 2571238 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0025/002571238/kplr002571238-2009166043257_llc.fits |
| `kepler20b_kic6850504_q1_llc.csv` | Kepler-20 b–f | KIC 6850504 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0068/006850504/kplr006850504-2009166043257_llc.fits |
| `kepler21b_kic3632418_q1_llc.csv` | Kepler-21 b | KIC 3632418 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0036/003632418/kplr003632418-2009166043257_llc.fits |
| `kepler30b_kic3832474_q1_llc.csv` | Kepler-30 b/d | KIC 3832474 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0038/003832474/kplr003832474-2009166043257_llc.fits |
| `kepler36b_kic11401755_q1_llc.csv` | Kepler-36 b/c | KIC 11401755 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0114/011401755/kplr011401755-2009166043257_llc.fits |
| `kepler48b_kic5735762_q1_llc.csv` | Kepler-48 b–d | KIC 5735762 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0057/005735762/kplr005735762-2009166043257_llc.fits |
| `kepler51b_kic11773022_q1_llc.csv` | Kepler-51 b | KIC 11773022 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0117/011773022/kplr011773022-2009166043257_llc.fits |
| `kepler79b_kic8394721_q1_llc.csv` | Kepler-79 b–e | KIC 8394721 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0083/008394721/kplr008394721-2009166043257_llc.fits |
| `kepler68c_kic11295426_q1_llc.csv` | Kepler-68 c | KIC 11295426 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0112/011295426/kplr011295426-2009166043257_llc.fits |
| `kepler89b_kic6462863_q1_llc.csv` | Kepler-89 b–e | KIC 6462863 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0064/006462863/kplr006462863-2009166043257_llc.fits |
| `kepler102b_kic10187017_q1_llc.csv` | Kepler-102 b–f | KIC 10187017 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0101/010187017/kplr010187017-2009166043257_llc.fits |
| `kepler62c_kic9002278_q1_llc.csv` | Kepler-62 c/d/e | KIC 9002278 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0090/009002278/kplr009002278-2009166043257_llc.fits |
| `kepler37b_kic8478994_q1_llc.csv` | Kepler-37 b/c/d | KIC 8478994 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0084/008478994/kplr008478994-2009166043257_llc.fits |
| `kepler444b_kic6278762_q1_llc.csv` | Kepler-444 b–f | KIC 6278762 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0062/006278762/kplr006278762-2009166043257_llc.fits |
| `kepler42b_kic8561063_q1_llc.csv` | Kepler-42 b/c/d | KIC 8561063 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0085/008561063/kplr008561063-2009166043257_llc.fits |
| `kepler138b_kic7603200_q1_llc.csv` | Kepler-138 b/c/d | KIC 7603200 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0076/007603200/kplr007603200-2009166043257_llc.fits |
| `kepler65b_kic5866724_q1_llc.csv` | Kepler-65 b/c/d | KIC 5866724 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0058/005866724/kplr005866724-2009166043257_llc.fits |
| `kepler32b_kic9787239_q1_llc.csv` | Kepler-32 b–f | KIC 9787239 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0097/009787239/kplr009787239-2009166043257_llc.fits |
| `kepler33b_kic9458613_q1_llc.csv` | Kepler-33 b–f | KIC 9458613 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0094/009458613/kplr009458613-2009166043257_llc.fits |
| `kepler26b_kic9757613_q1_llc.csv` | Kepler-26 b/c/d | KIC 9757613 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0097/009757613/kplr009757613-2009166043257_llc.fits |
| `kepler186b_kic8120608_q1_llc.csv` | Kepler-186 b–e | KIC 8120608 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0081/008120608/kplr008120608-2009166043257_llc.fits |
| `kepler80d_kic4852528_q1_llc.csv` | Kepler-80 c–f | KIC 4852528 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0048/004852528/kplr004852528-2009166043257_llc.fits |
| `kepler29b_kic10358759_q1_llc.csv` | Kepler-29 b/c | KIC 10358759 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0103/010358759/kplr010358759-2009166043257_llc.fits |
| `kepler93b_kic3544595_q1_llc.csv` | Kepler-93 b | KIC 3544595 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0035/003544595/kplr003544595-2009166043257_llc.fits |
| `k2_3_epic201367065_c01_llc.csv` | K2-3 b | EPIC 201367065 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201300000/67000/ktwo201367065-c01_llc.fits |
| `k2_18_epic201912552_c01_llc.csv` | K2-18 b | EPIC 201912552 | K2 C1 LLC | https://archive.stsci.edu/pub/k2/lightcurves/c1/201900000/12000/ktwo201912552-c01_llc.fits |
| `kepler1625b_kic4760478_q8_llc.csv` | Kepler-1625 b | KIC 4760478 | Q8 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits |
| `kepler1708b_kic7906827_q1_llc.csv` | Kepler-1708 b | KIC 7906827 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0079/007906827/kplr007906827-2009166043257_llc.fits |
| `kepler167e_kic3239945_q1_llc.csv` | Kepler-167 e | KIC 3239945 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0032/003239945/kplr003239945-2009166043257_llc.fits |

Columns: `time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality` (finite PDCSAP only).
TESS FITS uses `QUALITY` (not `SAP_QUALITY`); the extract header stays the same.

Honesty:

- Kepler-10 b, Kepler-1 b, Kepler-2 b, Kepler-3 b, Kepler-4–8 b, Kepler-9 b/c,
  Kepler-11 b/c/d/e, Kepler-18 b/c/d, Kepler-19 b, Kepler-20 b–f, Kepler-21 b,
  Kepler-26 b/c/d, Kepler-29 b/c, Kepler-30 b/d, Kepler-32 b–f, Kepler-33 b–f,
  Kepler-36 b/c, Kepler-37 b/c/d, Kepler-42 b/c/d, Kepler-48 b–d, Kepler-51 b,
  Kepler-62 c/d/e, Kepler-65 b/c/d, Kepler-68 c, Kepler-79 b–e, Kepler-80 c–f,
  Kepler-89 b–e, Kepler-93 b, Kepler-102 b–f, Kepler-138 b/c/d, Kepler-186 b–e,
  Kepler-444 b–f, Kepler-22 b, K2-3 b/c, and K2-18 b are
  **confirmed planets**, used as LC-backed training hosts. They are not
  moon detections. Kepler-12 b, 25 b, 30 c, 51 c/d, 62 b, 62 f, 68 b, 80 b,
  and 186 f miss Q1; no extracts were invented. Kepler-20 c–f reuse the Kepler-20 b
  Q1 file; Kepler-30 d reuses Kepler-30 b; Kepler-36 c reuses Kepler-36 b;
  Kepler-37 c/d reuse Kepler-37 b; Kepler-48 c/d reuse Kepler-48 b;
  Kepler-79 c–e reuse Kepler-79 b; Kepler-89 c–e reuse Kepler-89 b;
  Kepler-102 c–f reuse Kepler-102 b; Kepler-62 d/e reuse Kepler-62 c;
  Kepler-444 c–f reuse Kepler-444 b; Kepler-42 c/d reuse Kepler-42 b;
  Kepler-138 c/d reuse Kepler-138 b; Kepler-65 c/d reuse Kepler-65 b;
  Kepler-32 c–f reuse Kepler-32 b; Kepler-33 c–f reuse Kepler-33 b;
  Kepler-26 c/d reuse Kepler-26 b; Kepler-186 c–e reuse Kepler-186 b;
  Kepler-80 c/e/f reuse Kepler-80 d; Kepler-29 c reuses Kepler-29 b.
  Kepler-22 b Q1 covers the catalog epoch (t0≈133.70 BKJD); Kepler-11 b/c/d/e
  cover catalog t0≈138.50 / 138.18 / 148.46 / 154.16; Kepler-9 b/c cover
  previous catalog epochs (≈163.27 / ≈136.52); Kepler-37 b/c/d cover
  previous catalog epochs (≈143.97 / ≈149.24 / ≈135.46); Kepler-62 d
  covers the previous catalog epoch (≈162.65); Kepler-138 c/d cover
  previous catalog epochs (≈136.51 / ≈147.92); Kepler-33 d/f cover
  previous catalog epochs (≈146.09 / ≈131.55); Kepler-186 d covers the
  previous catalog epoch (≈136.87); Kepler-80 c covers the previous catalog
  epoch (≈139.40). Kepler-42 b, Kepler-32 b/c, Kepler-26 b, Kepler-29 b/c,
  and Kepler-186 c use folded catalog transits. Those transits were not
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
