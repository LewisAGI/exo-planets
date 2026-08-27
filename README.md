# exo-planets

Rust crate for **Kipping transit / TTV / TDV signatures** on a **public NASA
Exoplanet Archive TAP slice**, **cached MAST Kepler / K2 / TESS light-curve
extracts**, a **Holczer+2016 planet-only O−C catalog**, LUNA-**style flags**
(not a LUNA port), plus a small **linfa** logistic model.

This is **not** a moon-discovery paper, not a LUNA integrator, and not a
confirmation engine.

Science lock (2026-08-27, opened papers only). Cool Worlds videos are not
results. HEK II–V are **all null**. Named objects stay
**candidate / false positive / search**.

## What this is

1. **Ingest** of a cached NASA TAP/CSV sample (KOI `cumulative` + PS `ps`,
   including K2-3 / K2-18 hosts), Holczer et al. 2016 Table 4 O−C scatter
   (VizieR `J/ApJS/225/9`; **planet-only timing, not moons**), JWST GO 6491
   MAST/DOI **metadata** (SEARCH; no NIRSpec LC), and real PDCSAP extracts:
   Kepler-10 b (Kepler Q1 + TESS S14), Kepler-1 b, Kepler-2 b, Kepler-3 b,
   Kepler-4–8 b Q1, Kepler-9 b/c Q1, Kepler-11 b/c/d/e Q1, Kepler-18 b/c/d,
   Kepler-19 b, Kepler-20 b–f, Kepler-21 b, Kepler-30 b/d, Kepler-36 b/c,
   Kepler-26 b/c/d, Kepler-32 b–f, Kepler-33 b–f, Kepler-37 b/c/d, Kepler-42 b/c/d,
   Kepler-48 b–d, Kepler-51 b, Kepler-62 c/d/e, Kepler-65 b/c/d, Kepler-68 c,
   Kepler-79 b–e, Kepler-89 b–e, Kepler-102 b–f, Kepler-138 b/c/d, Kepler-186 b–e,
   Kepler-444 b–f, Kepler-22 b Q1,
   K2-3 b/c C1, K2-18 b C1, plus holdout-host Q1/Q8 windows
   for Kepler-1625 b,
   Kepler-1708 b, and Kepler-167 e. See
   [`data/cache/lightcurves/SOURCE.md`](data/cache/lightcurves/SOURCE.md).
2. **Feature layer**: transit geometry (Kipping 2009b / Seager & Mallén-Ornelas),
   circular TTV (Kipping 2009a), TDV-V / first-order TDV-TIP, HEK-style
   dynamical cuts and a **timing Bayes-factor proxy**, FORECASTER mass-prior
   *class* (Chen & Kipping 2017) labelled **extrapolation** when discretized
   from radius, **LUNA-style geometric flags** (overlapping disc / syzygy /
   extra-dip *possible*), extra-dip SNR from the cached LCs, and a **HEK V
   photometry-only caution demo** on confirmed-planet LCs (not a detection).
3. **Trainable model in Rust** (`linfa-logistic`). Train/eval on confirmed KOIs
   as planet-only + **injected** TTV/TDV moons. The four locked systems are
   **holdout score cards only**.

## What this is not

- Not LUNA. There is **no** 3-body sky integrator and **no** overlapping-disc
  photometry. Flags only. Do not cite this as a LUNA port.
- Not a HEK Bayes factor. The “BF” number is a timing-RMS χ² / BIC **proxy**.
- Not official FORECASTER posterior draws. Radius bins are a discretization.
- Not a moon TTV catalog. Holczer+2016 Table 4 is **planet-only** O−C scatter
  (often planet–planet). Unmatched KOIs still use synthetic white timing.
- Not a confirmation of Kepler-1625b-i, Kepler-1708 b-i, or a Kepler-167e moon.
- Not a re-estimate of the HEK V ~1/4 false-claim rate. The cached-LC extra-dip
  demo is a **caution**, including when 0/N LCs trip the cut.
- Kepler-90g’s moon is a **false positive** (SPSD / pixel-centroid) no matter
  what the classifier outputs.

## Why linfa (not candle / burn)

The labels are a few hundred **tabular** rows: real KOI/PS parameters plus
analytic TTV/TDV injections. A regularized logistic regression is the honest
capacity. A neural net (candle/burn) would overfit the injections and look
like a photodynamical detector we did not build.

## Fetch / train / score

Needs Rust 1.74+ (`rustc` 1.83 is fine). No API keys.

```bash
# offline: use the in-repo TAP slice
cargo test
cargo run -- all --cache data/cache --out data/out

# optional: re-pull the same TAP queries (public HTTP)
cargo run -- fetch --cache data/cache
```

`data/out/report.json` and `data/out/holdout_scorecards.json` are the human
artifacts. `data/out/` is gitignored.

Cache provenance and the exact TAP SQL: [`data/cache/SOURCE.md`](data/cache/SOURCE.md).

Locked statuses: [`data/labels/holdout_scorecards.json`](data/labels/holdout_scorecards.json).

## Feature math (implemented)

| Piece | Formula / cut | Honesty note |
|---|---|---|
| Depth | δ ≈ (R_p / R_*)² | No limb darkening |
| Impact | b = a cos i / R_* | Uses catalog b when present |
| Chord T₁₄ | Seager & Mallén-Ornelas circular | Missing a → Kepler III, flagged |
| TTV RMS | δTTV = a_W / (√2 v_{B⊥}) | Circular, coplanar |
| HEK I max-dev | 36.0 D (M_S/M⊕) (P_B/yr) (M_J/M_P)^{2/3} (M_☉/M_*)^{1/3} min | Scale, not a detection |
| Moon period | P_S = P_B √(D³/3) so P_S(D=1) = P_B/√3 | See note below |
| Unique P_S | P_S ≲ 0.6 P_P | Else TTV aliases harmonics |
| TDV-V | ∝ M_S a_S^{-1/2}; η_V = 2π T / P_S | π/2 out of phase with TTV |
| TDV-TIP | first-order \|dT/db\| (a_W/R_*)/√2 | Additive prograde, subtractive retrograde. **Not LUNA.** |
| D_max | 0.4895 prograde, 0.9309 retrograde | Domingos 2006 |
| HEK I 4σ | z = δTTV / σ_timing | Proxy threshold |
| HEK V | photometry-only extra-dip flag | Would have false-claimed ~1/4 of KOIs. A fire is a **caution**, not a moon. |
| HEK VI | η < 0.38 (95%), 284 KOIs | A **dearth**, not a detection. BF~2 is a hint. |
| FORECASTER | Terran / Neptunian / Jovian / stellar; 2.0^{+0.7}_{-0.6} M⊕ | Prior class only |
| LUNA-style flags | overlapping disc / syzygy / extra-dip-on-star **possible**; D vs D_max; coplanar syzygy timescale (R_p+R_m)/a_S × P_S/(2π) | Geometry only. Not LUNA. |
| LC extra-dip SNR | box residual on cached Kepler / K2 / TESS PDCSAP | Real photometry; not a detection. |
| Holczer S(O−C) | Table 4 1.4826×MAD of O−C (minutes) | Published **planet-only** timing. Not a moon. |
| HEK V demo | extra-dip cut on confirmed-planet cached LCs | Caution, not a detection; not a 1/4 re-estimate. |

The lock text writes `P_SB = P_B / √(D³/3)`. That is the **reciprocal** of the
Kepler+Hill period. This crate evaluates `P_S = P_B √(D³/3)` and the cut
`P_S ≤ P_B/√3` (then Domingos D_max).

Kepler long-cadence (29.4 min) smears ingress; the `long_cadence_smear` flag
is on when predicted ingress ≲ 2 LC samples. Prefer short cadence for TDV.

Large-moon injections sit at ≥ 0.1 M⊕ (HEK scale), D = 0.25 and 0.40
(prograde, inside D_max).

## Named objects (holdouts)

| Object | Status | Locked note |
|---|---|---|
| Kepler-1625b-i | **CANDIDATE** | Hubble dip model-dependent; authors call unconfirmed. No TTV invented. |
| Kepler-1708 b-i | **CANDIDATE** | Planet validated; moon not. Two Kepler transits. Predicted TTV 1.2–77 min (95%). Archive `pl_bmasse` is an **upper limit**. |
| Kepler-90g moon | **FALSE POSITIVE** | SPSD / pixel-centroid. |
| JWST Kepler-167e (GO 6491) | **SEARCH** | Residual 7–17 min after linear ephemeris. Do not promote a moon. |

They are never trained as confirmed moons. The classifier’s
`P(injected-like)` on a score card is **not** a posterior that a moon exists.

## Honest limits

- Training “planet_only” rows are confirmed **planets**, not confirmed
  moon-nulls. HEK II–V already published those nulls; we do not re-litigate
  them as detections.
- Injected moons are synthetic and labelled `injected`.
- Cached LCs: Kepler-10 b (Kepler Q1 + TESS S14 SPOC), Kepler-1 b Q1,
  Kepler-2 b Q1, Kepler-3 b Q1, Kepler-4–8 b Q1, Kepler-9 b/c Q1 (prior catalog epochs
  ≈163.27 / ≈136.52 in-window), Kepler-11 b/c/d/e Q1 (catalog t0 in-window;
  11 f/g epochs are outside Q1 and were not invented),
  Kepler-18 b/c/d Q1, Kepler-19 b Q1, Kepler-20 b–f Q1, Kepler-21 b Q1
  (folded catalog ephemeris), Kepler-30 b/d Q1, Kepler-36 b/c Q1,
  Kepler-26 b/c/d Q1, Kepler-32 b–f Q1, Kepler-33 b–f Q1, Kepler-37 b/c/d Q1,
  Kepler-42 b/c/d Q1, Kepler-48 b–d Q1, Kepler-51 b Q1, Kepler-62 c/d/e Q1,
  Kepler-65 b/c/d Q1, Kepler-68 c Q1, Kepler-79 b–e Q1, Kepler-89 b–e Q1,
  Kepler-102 b–f Q1, Kepler-138 b/c/d Q1, Kepler-186 b–e Q1, Kepler-444 b–f Q1,
  Kepler-22 b
  Q1 (catalog transit in-window, not invented; Kepler-12 b, 25 b, 30 c,
  51 c/d, 62 b, 62 f, 68 b, 80 b, and 186 f miss Q1 and were not cached), K2-3 b/c C1
  (no catalog epoch; extra-dip unwindowed), K2-18 b C1, Kepler-1625 b Q8,
  Kepler-1708 b Q1, Kepler-167 e Q1.
  JWST GO 6491 is **MAST/DOI metadata only** (SEARCH; no NIRSpec time
  series cached). Columbia Academic Commons 1625 `/download` was **404**
  behind Anubis; no Hubble product cached. No TESSCut / FFI photometry
  was invented.
  Kepler-1625 Q8, Kepler-1708 Q1, and Kepler-167 e Q1 do **not** cover a
  catalog transit; none was invented. Statuses stay **CANDIDATE / SEARCH**.
- Holczer+2016 is Table 4 statistics only (not the 295k-row table3 times).
  KOIs 351.02 / 490.02 / 5084.01 are absent; Kepler-1708 is post-2016.
- LUNA-style items are **flags**, not a photodynamical integrator.
- Kepler-90 g has no `default_flag=1` PS row in the 2026-08-27 TAP pull;
  geometry comes from KOI cumulative.
- Kepler-1708 b is missing from the KOI cumulative pull; it comes from PS.
- A download failure should leave this cache in place rather than invent rows.

## License

MIT.
