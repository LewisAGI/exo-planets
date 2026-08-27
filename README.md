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
   Kepler-4–8 b Q1, Kepler-9 b/c Q1, Kepler-11 b/c/d/e Q1, Kepler-13 b, Kepler-14 b, Kepler-15 b, Kepler-17 b, Kepler-18 b/c/d,
   Kepler-19 b, Kepler-20 b–f, Kepler-21 b, Kepler-30 b/d, Kepler-36 b/c,
   Kepler-23 b/c, Kepler-24 b/c, Kepler-26 b/c/d, Kepler-27 b/c, Kepler-28 b/c,
   Kepler-39 b, Kepler-40 b, Kepler-41 b, Kepler-43 b, Kepler-44 b, Kepler-45 b, Kepler-46 b, Kepler-49 b/c, Kepler-52 b/c/d, Kepler-53 b/c, Kepler-54 b/c/d, Kepler-55 b–e,
   Kepler-56 b/c, Kepler-57 b/c,
   Kepler-58 b/c/d, Kepler-59 b/c,
   Kepler-60 b/c/d,    Kepler-61 b, Kepler-63 b, Kepler-66 b, Kepler-69 b, Kepler-71 b, Kepler-74 b,
   Kepler-75 b, Kepler-76 b, Kepler-77 b, Kepler-82 b/d/e, Kepler-83 b/c/d, Kepler-84 b–e, Kepler-85 b–e,
   Kepler-29 b/c, Kepler-31 b/c, Kepler-32 b–f, Kepler-33 b–f, Kepler-50 b/c,
   Kepler-37 b/c/d, Kepler-42 b/c/d, Kepler-48 b–d, Kepler-51 b, Kepler-62 c/d/e,
   Kepler-65 b/c/d, Kepler-68 c, Kepler-79 b–e, Kepler-80 c–f, Kepler-81 b/c/d,
   Kepler-88 b,
   Kepler-89 b–e, Kepler-90 b/c/d/e/h, Kepler-91 b, Kepler-92 b/c/d, Kepler-93 b, Kepler-94 b, Kepler-95 b, Kepler-96 b, Kepler-97 b, Kepler-98 b, Kepler-99 b, Kepler-100 b/c/d, Kepler-101 b/c, Kepler-102 b–f, Kepler-103 b, Kepler-104 b/c, Kepler-105 b/c, Kepler-106 b–e, Kepler-107 b–e, Kepler-108 b, Kepler-109 b, Kepler-110 b, Kepler-111 b, Kepler-112 b, Kepler-113 b, Kepler-114 b/c, Kepler-115 b/c, Kepler-116 b/c, Kepler-117 b/c, Kepler-118 b, Kepler-119 b, Kepler-120 b, Kepler-121 b/c, Kepler-122 b–f, Kepler-123 b, Kepler-124 b/c, Kepler-125 b, Kepler-126 b/c/d, Kepler-127 b/c, Kepler-128 b, Kepler-129 b, Kepler-130 b, Kepler-131 b, Kepler-132 b, Kepler-133 b, Kepler-134 b, Kepler-135 b, Kepler-136 b/c, Kepler-137 b/c, Kepler-139 b/d, Kepler-140 b, Kepler-141 b/c, Kepler-142 b/c/d, Kepler-143 b/c, Kepler-144 b/c, Kepler-145 b/c, Kepler-146 b, Kepler-147 b/c, Kepler-148 b/c, Kepler-149 b, Kepler-150 b–e, Kepler-151 b/c, Kepler-152 b/c, Kepler-153 b/c, Kepler-154 b/d/e/f, Kepler-155 b, Kepler-156 b/c, Kepler-157 b/c/d, Kepler-158 b/c, Kepler-159 b/c, Kepler-160 b/c, Kepler-138 b/c/d,
   Kepler-186 b–e, Kepler-444 b–f, Kepler-22 b Q1,
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
  Kepler-13 b Q1 (folded catalog transits), Kepler-14 b Q1, Kepler-15 b Q1,
  Kepler-17 b Q1,
  Kepler-18 b/c/d Q1, Kepler-19 b Q1, Kepler-20 b–f Q1, Kepler-21 b Q1
  (folded catalog ephemeris), Kepler-30 b/d Q1, Kepler-36 b/c Q1,
  Kepler-23 b/c Q1, Kepler-24 b/c Q1, Kepler-26 b/c/d Q1,
  Kepler-27 b/c Q1 (27 b catalog t0 after Q1; previous epochs ≈143.33 / ≈158.67
  in-window), Kepler-28 b/c Q1 (catalog t0 after Q1; folded catalog transits),
  Kepler-29 b/c Q1, Kepler-31 b/c Q1 (31 b previous epoch ≈159.16),
  Kepler-32 b–f Q1, Kepler-39 b Q1 (folded catalog transits), Kepler-40 b Q1
  (folded catalog transits), Kepler-41 b Q1, Kepler-43 b Q1, Kepler-44 b Q1,
  Kepler-45 b Q1 (folded catalog transits), Kepler-46 b Q1 (previous epoch
  ≈153.08),
  Kepler-50 b/c Q1 (50 b folded catalog
  transits),
  Kepler-52 b/c/d Q1 (52 b folded catalog transits; 52 c previous epoch
  ≈156.34), Kepler-53 b/c Q1 (53 b previous epoch ≈156.12),
  Kepler-54 b/c/d Q1 (54 b catalog t0 after Q1; previous epoch ≈162.20
  in-window), Kepler-55 b–e Q1 (55 b previous epoch ≈150.78; 55 d folded
  catalog transits), Kepler-56 b/c Q1,
  Kepler-57 b/c Q1, Kepler-58 b/c/d Q1, Kepler-59 b/c Q1, Kepler-60 b/c/d Q1,
  Kepler-49 b/c Q1 (49 b folded catalog transits; 49 c previous epoch
  ≈158.94), Kepler-61 b Q1, Kepler-66 b Q1, Kepler-69 b Q1, Kepler-71 b Q1, Kepler-74 b Q1,
  Kepler-63 b Q1 (folded catalog transits),
  Kepler-75 b Q1 (folded catalog transits), Kepler-76 b Q1, Kepler-77 b Q1,
  Kepler-82 b/d/e Q1 (82 b folded catalog transit ≈141.23; 82 d/e in-window;
  82 c misses Q1 and was not cached),
  Kepler-83 b/c/d Q1 (83 b/d folded catalog transits), Kepler-84 b–e Q1, Kepler-85 b–e Q1,
  Kepler-33 b–f Q1, Kepler-37 b/c/d Q1, Kepler-42 b/c/d Q1, Kepler-48 b–d Q1,
  Kepler-51 b Q1, Kepler-62 c/d/e Q1, Kepler-65 b/c/d Q1, Kepler-68 c Q1,
  Kepler-79 b–e Q1, Kepler-80 c–f Q1, Kepler-81 b/c/d Q1 (81 b/c folded
  catalog transits), Kepler-88 b Q1, Kepler-89 b–e Q1,
  Kepler-91 b Q1 (catalog epoch in-window),
  Kepler-92 b/c/d Q1 (92 b folded catalog transits), Kepler-93 b Q1, Kepler-94 b Q1, Kepler-95 b Q1,
  Kepler-96 b Q1 (previous epoch ≈154.78), Kepler-97 b Q1 (folded catalog
  transits), Kepler-98 b Q1 (folded catalog transits), Kepler-99 b Q1
  (folded catalog transits), Kepler-100 b/c/d Q1, Kepler-101 b/c Q1
  (101 b folded catalog transits), Kepler-102 b–f Q1, Kepler-103 b Q1
  (103 c misses Q1), Kepler-104 b/c Q1 (104 d misses Q1), Kepler-105 b/c Q1,
  Kepler-106 b–e Q1, Kepler-107 b–e Q1, Kepler-108 b Q1, Kepler-109 b Q1
  (folded catalog transits), Kepler-110 b Q1, Kepler-111 b Q1, Kepler-112 b Q1,
  Kepler-113 b Q1 (next catalog epoch ≈133.30), Kepler-114 b/c Q1,
  Kepler-115 b/c Q1, Kepler-116 b/c Q1, Kepler-117 b/c Q1,
  Kepler-90 b/c/d/e/h Q1 (confirmed planets; 90 f misses Q1; 90 g stays a
  holdout, not a moon), Kepler-118 b Q1, Kepler-119 b Q1, Kepler-120 b Q1,
  Kepler-121 b/c Q1, Kepler-122 b–f Q1, Kepler-123 b Q1, Kepler-124 b/c Q1,
  Kepler-125 b Q1 (folded catalog transits), Kepler-126 b/c/d Q1 (126 b
  previous epoch ≈162.30; 126 d previous epoch ≈144.76), Kepler-127 b/c Q1
  (127 c folded catalog transit ≈150.28), Kepler-128 b Q1 (previous epoch
  ≈160.86), Kepler-129 b Q1 (previous epoch ≈161.03), Kepler-130 b Q1,
  Kepler-131 b Q1 (previous epoch ≈154.51), Kepler-132 b Q1 (previous epoch
  ≈162.68), Kepler-133 b Q1, Kepler-134 b Q1 (folded catalog transits),
  Kepler-135 b Q1 (folded catalog transits), Kepler-136 b/c Q1 (136 b
  previous epoch ≈164.01), Kepler-137 b/c Q1 (137 b folded catalog
  transits; 137 c previous epoch ≈158.90), Kepler-139 b/d Q1 (139 b folded
  catalog transits; 139 c misses Q1), Kepler-140 b Q1 (folded catalog
  transits; 140 c misses Q1), Kepler-141 b/c Q1 (141 c folded catalog
  transits), Kepler-142 b/c/d Q1 (142 b/c folded catalog transits),
  Kepler-143 b/c Q1 (143 c previous epoch ≈164.72), Kepler-144 b/c Q1
  (144 b folded catalog transits), Kepler-145 b/c Q1 (145 c previous
  epoch ≈160.77), Kepler-146 b Q1 (previous epoch ≈142.74; 146 c misses
  Q1), Kepler-147 b/c Q1 (147 c previous epoch ≈137.92), Kepler-148 b/c Q1
  (148 c folded catalog transits; 148 d misses Q1), Kepler-149 b Q1
  (previous epoch ≈156.24; 149 c/d miss Q1), Kepler-150 b–e Q1 (150 c
  folded catalog transits; 150 d previous epoch ≈154.23), Kepler-151 b/c Q1
  (151 b previous epoch ≈161.33), Kepler-152 b/c Q1 (152 b folded catalog
  transits), Kepler-153 b/c Q1 (153 b previous epoch ≈159.84), Kepler-154
  b/d/e/f Q1 (154 d previous epoch ≈158.40; 154 c misses Q1), Kepler-155 b
  Q1 (folded catalog transits; 155 c misses Q1), Kepler-156 b/c Q1 (156 b
  folded catalog transits; 156 c previous epoch ≈162.02), Kepler-157 b/c/d
  Q1 (157 c previous epoch ≈158.14), Kepler-158 b/c Q1 (158 b previous
  epoch ≈158.04; 158 c previous epoch ≈156.93), Kepler-159 b/c Q1 (159 b
  folded catalog transits; 159 c previous epoch ≈151.02), Kepler-160 b/c
  Q1 (160 c previous epoch ≈157.78),
  Kepler-138 b/c/d Q1, Kepler-186 b–e Q1, Kepler-444 b–f Q1,
  Kepler-22 b
  Q1 (catalog transit in-window, not invented; Kepler-12 b, 25 b, 30 c,
  51 c/d, 62 b, 62 f, 68 b, 80 b, 82 c, 90 f, 103 c, 104 d, 139 c, 140 c,
  146 c, 148 d, 149 c/d, 154 c, 155 c, and 186 f miss Q1 and were not cached;
  Kepler-67 b Q1 LLC 404 and was not invented), K2-3 b/c C1
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
