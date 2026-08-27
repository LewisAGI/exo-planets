# Light-curve cache (MAST / Kepler public HTTP)

Pulled 2026-08-27. **No secrets.** These are small PDCSAP extracts from real
Kepler long-cadence FITS on the public STScI archive (the same bits MAST
serves). Full mission light curves are huge; this is a cached slice.

Re-fetch (needs network; Rust FITS reader, no Python):

```bash
cargo run -- fetch --cache data/cache
```

| Cache CSV | Host | KIC | File | URL |
|---|---|---|---|---|
| `kepler10b_kic11904151_q1_llc.csv` | Kepler-10 b | 11904151 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0119/011904151/kplr011904151-2009166043257_llc.fits |
| `kepler1b_kic11446443_q1_llc.csv` | Kepler-1 b (TrES-2b) | 11446443 | Q1 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0114/011446443/kplr011446443-2009166043257_llc.fits |
| `kepler1625b_kic4760478_q8_llc.csv` | Kepler-1625 b | 4760478 | Q8 LLC | https://archive.stsci.edu/pub/kepler/lightcurves/0047/004760478/kplr004760478-2011073133259_llc.fits |

Columns: `time_bkjd,pdcsap_flux,pdcsap_flux_err,sap_quality` (finite PDCSAP only).

Honesty:

- Kepler-10 b and Kepler-1 b are **confirmed planets**, used as LC-backed
  training hosts. They are not moon detections.
- Kepler-1625 b is a **holdout**. Q8 (BKJD ≈ 735–802) does **not** cover a
  catalog transit (P ≈ 287.4 d, t0 ≈ 348.83 BKJD). No transit was invented.
  The moon stays **CANDIDATE**.
- No K2 or TESS time series landed in this cache (Kepler LLC only).
- No Hubble, JWST, or Columbia 1625 photometry products.
- Extra-dip flags computed from these files are **not** LUNA and **not**
  confirmations. HEK V: photometry-only false-claims moons in ~1/4 of KOIs.

Portals: https://mast.stsci.edu/ · https://archive.stsci.edu/missions-and-data/kepler
