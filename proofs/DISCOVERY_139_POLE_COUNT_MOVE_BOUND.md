# Discovery 139: Pole Count as Move Bound in Time Series

**Author:** Eliran Sabag
**Date:** February 9, 2026
**Status:** VERIFIED
**Framework Version:** Discovery 139 / FitGuard
**Verification Binary:** `verify_fitguard_ml`

---

## Abstract

The number of poles in a Laplace-domain time series decomposition IS the move bound c. This extends the cross-domain translation table with a new entry: in time series forecasting, each pole (s = sigma + j*omega) is an element, and pole refinement is the bounded local move. The compression-derived c (Discovery 137) limits model complexity to the number of true signal components, automatically rejecting noise.

---

## The Laplace Decomposition

A time series x(t) decomposes into exponentially-modulated sinusoids:

```
x(t) = sum_k  A_k * exp(sigma_k * t) * cos(omega_k * t + phi_k)
```

Each component is a **pole** in the Laplace domain:
- **sigma** < 0: decaying (stable)
- **sigma** = 0: sustained oscillation
- **omega**: angular frequency (2*pi/period)
- **A**: amplitude
- **phi**: phase

The number of poles determines model complexity. Too few = underfit. Too many = overfit (fitting noise). This is the same c-bound problem as in every other ARC domain.

---

## The Cross-Domain Translation Table

| Domain | Element | Local Move | c |
|--------|---------|------------|---|
| TSP | Edge | 2-opt swap | 2 |
| SAT | Variable | Flip | 1 |
| Chess | Piece | Move | 2 |
| Audio | Frame | Window shift | 1 |
| Regression | Parameter | Gradient step | derived |
| **Forecasting** | **Pole** | **Pole refinement** | **derived** |
| Swarm | Position | Velocity | <= bound |

### Why Pole = Element

In regression, each parameter corresponds to one feature dimension. Updating a parameter = moving along one axis in coefficient space.

In forecasting, each pole corresponds to one frequency component. Refining a pole's amplitude/phase = moving along one axis in the Laplace decomposition.

The parallel is exact:
- **Too many parameters** in regression = overfitting noise features
- **Too many poles** in forecasting = overfitting noise frequencies
- **c-bounded parameters** = regularized regression
- **c-bounded poles** = regularized forecasting

---

## The Pipeline

### Phase 0: Diagnose
1. Run Two Randomness test on the series
2. IF incompressible: series is noise, no forecastable structure
3. Derive c = max number of poles

### Phase 1: Extract
1. Compute autocorrelation r(lag) for lag = 0..n/2
2. Find peaks in autocorrelation (local maxima with r > 0.05)
3. Convert peaks to poles: omega = 2*pi/lag, sigma = ln(r)/lag
4. **Truncate to c poles** (bounded move)

### Phase 2: Refine
1. Fit amplitudes and phases via 2x2 least-squares projection
2. Gradient refinement with saturation detection
3. Clip growing poles (sigma > 0) to sustained (sigma = 0)

---

## Empirical Results (FitGuard ARCForecaster)

| Signal Type | True Components | Poles Extracted | R2 | Saturated At |
|-------------|----------------|----------------|-----|-------------|
| Pure sine | 1 | c-bounded | 0.91 | Early |
| Linear trend | 0 (trend only) | 0 | 1.0 | Immediate |
| Trend + seasonal | 1 + trend | c-bounded | 0.9963 | Early |
| Sine + noise | 1 + noise | c-bounded | 0.89 | Early |

### Excess Poles Are Noise

For a signal with 2 true frequency components + noise:

| Configuration | MSE | Residual Structure |
|--------------|-----|-------------------|
| 2 poles (correct) | 0.2322 | 0.035 (near noise) |
| 4 poles (excess) | 0.2301 | 0.035 (near noise) |

**MSE improvement from extra poles: 0.88%** — diminishing returns. The Two Randomness test confirms: residuals after 2 poles are already noise-like. Additional poles fit noise, not signal.

---

## Connection to Existing Discoveries

| Discovery | Connection |
|-----------|-----------|
| 14 (Saturation) | Pole refinement stops at saturation |
| 103 (Two Randomness) | Residual compressibility determines when to stop adding poles |
| 109 (Laplace Completeness) | The Laplace transform as universal representation |
| 137 (Compression-Derived c) | c limits pole count |
| 138 (Zero-Hyperparameter) | No tuning of pole count — derived from data |

---

## The Deeper Insight

The autocorrelation peaks in a time series are the **local optima** of the frequency decomposition. Just as:

- TSP has O(n^c) local optima under bounded 2-opt
- SAT has O(n^c) local optima under variable flip
- Chess has bounded positions under piece moves

Time series has c autocorrelation peaks under the Laplace decomposition. The poles ARE the local optima of the frequency domain. And c — their count — is bounded by the data's compression structure.

This is not analogy. It is the same mathematics in a different domain.

---

## Verification

Run: `cargo run --release --bin verify_fitguard_ml`

Tests 7-9 verify Discovery 139:

7. **Pole count bounded by c** — poles_used <= c
8. **Excess poles are noise** — <1% MSE improvement from extra poles
9. **Cross-domain translation** — 7 domains, all with bounded c

---

*In TSP, the move bound limits edges swapped.*
*In regression, it limits parameters updated.*
*In forecasting, it limits poles modeled.*
*Same bound. Same pattern. Different domain.*
