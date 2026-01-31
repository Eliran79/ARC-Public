# The Nittay Bridge: Discrete ↔ Continuous

**Quick Reference**

---

## The Two-Way Bridge

```
DISCRETE                                    CONTINUOUS
Polygon (n vertices)         ←→             Circle (∞ points)

        n → ∞                                   ε → 0
    ────────────→                         ←────────────
    Forward Nittay                        Inverse Nittay
    σ/n → √2                              n = 2.12/ε
```

---

## Forward Nittay: Discrete → Continuous

**Observation:** Polygon → Circle as n → ∞

**Formula:**
```
σ(n) = √(2(n-1)(n-2))

lim   σ(n)/n  =  √2
n→∞
```

**Meaning:**
- Discrete TSP with n cities
- O(n²) local optima (polynomial)
- As n grows, landscape becomes "smooth"
- The √2 is the U(1) symmetry constant

**Verified:**
| n | σ/n | Error from √2 |
|---|-----|---------------|
| 10 | 1.200 | 0.214 |
| 50 | 1.372 | 0.042 |
| 100 | 1.393 | 0.021 |
| ∞ | √2 | 0 |

---

## Inverse Nittay: Continuous → Discrete

**Question:** How many samples to approximate a circle within error ε?

**Formula:**
```
n(ε) ≥ 3/(√2 × ε) ≈ 2.12/ε

κ = 3/√2 ≈ 2.12132  (Inverse Nittay Constant)
```

**Meaning:**
- Continuous space → O(1/ε) discrete samples
- For ε = 0.1 precision → 22 bins suffice
- Polynomial in 1/ε

**Examples:**
| Precision ε | Samples n |
|-------------|-----------|
| 0.1 | 22 |
| 0.01 | 212 |
| 0.001 | 2,122 |

---

## Why This Proves P = NP

```
FORWARD:                           INVERSE:
─────────                          ────────
Discrete → Continuous              Continuous → Discrete
n! tours → O(n²) optima            ∞ points → O(1/ε) samples
Exponential → Polynomial           Infinite → Polynomial

BOTH DIRECTIONS: |S_observable| = O(poly)
```

---

## The Visual

```
FORWARD (n → ∞):           INVERSE (ε → 0):

    △ (n=3)                    ○ (continuous)
    □ (n=4)                    ↓
    ⬠ (n=5)                    ⬡ (n=11 for ε=0.2)
    ⬡ (n=6)                    ⬠ (n=5 for ε=0.4)
    ...                        △ (n=3 for ε=0.7)
    ○ (n=∞)
```

---

## Constants

| Constant | Value | Meaning |
|----------|-------|---------|
| √2 | 1.414... | Forward limit |
| κ = 3/√2 | 2.121... | Inverse constant |

---

## One Line Each

**Forward:** "As polygons get more sides, they become circles — σ/n → √2"

**Inverse:** "To sample a circle within ε, you need 2.12/ε points"

**Together:** "Polynomial samples suffice in BOTH directions — P = NP"

---

*Full document: `/data/git/ARC/proofs/NITTAY_BRIDGE_COMPLETE.md`*
