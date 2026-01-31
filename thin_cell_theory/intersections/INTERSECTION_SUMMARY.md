# WARP-WOOF INTERSECTION SUMMARY

## Complete 5×5 Matrix Status

```
        │ G1 Euclidean │ G2 2-opt │ G3 Non-cross │ G4 Thin Cell │ G5 Coupling │
────────┼──────────────┼──────────┼──────────────┼──────────────┼─────────────┤
W1      │ N/A          │ INSPIRES │ -            │ -            │ -           │
Sample  │ Wrong domain │ Analogy  │              │              │             │
────────┼──────────────┼──────────┼──────────────┼──────────────┼─────────────┤
W2      │ -            │ UNDEFINED│ -            │ -            │ -           │
DualTree│              │ No encode│              │              │             │
────────┼──────────────┼──────────┼──────────────┼──────────────┼─────────────┤
W3      │ -            │ -        │ ★PROVEN★    │ -            │ -           │
Pareto  │              │          │ Textbook    │              │             │
────────┼──────────────┼──────────┼──────────────┼──────────────┼─────────────┤
W4      │ -            │ PARTIAL  │ ★PROVEN★    │ INCOMPLETE   │ -           │
ROPE    │              │ Gaps     │ Foundation  │ Key gaps     │             │
────────┼──────────────┼──────────┼──────────────┼──────────────┼─────────────┤
W5      │ -            │ -        │ -            │ -            │ INCOMPLETE  │
Bipart  │              │          │              │              │ No argument │
────────┴──────────────┴──────────┴──────────────┴──────────────┴─────────────┘
```

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ★PROVEN★ | Rigorous mathematical proof exists |
| PARTIAL | Some valid parts, gaps remain |
| INCOMPLETE | Structure valid, math missing |
| INSPIRES | Useful analogy, no direct proof |
| UNDEFINED | Key definitions missing |
| N/A | Not applicable |
| - | Not documented / low priority |

---

## Detailed Status

### PROVEN (2 intersections)

| ID | Intersection | What's Proven |
|----|--------------|---------------|
| W3×G3 | Pareto × Non-crossing | Crossing edges suboptimal (textbook) |
| W4×G3 | ROPE × Non-crossing | Hull order fixed (standard result) |

These are **real theorems** that can be cited.

### INCOMPLETE (2 intersections)

| ID | Intersection | What's Missing |
|----|--------------|----------------|
| W4×G4 | ROPE × Thin Cell | Monotonicity undefined, uniqueness unproven |
| W5×G5 | Bipartite × Coupling | Compatibility undefined, counting unproven |

These have **valid structure** but **no math**.

### PARTIAL (1 intersection)

| ID | Intersection | Status |
|----|--------------|--------|
| W4×G2 | ROPE × 2-opt | Independence mostly true, proof incomplete |

### INSPIRATIONAL (1 intersection)

| ID | Intersection | Status |
|----|--------------|--------|
| W1×G2 | SampleSpace × 2-opt | Analogy only, different mechanisms |

### NOT APPLICABLE (1 intersection)

| ID | Intersection | Status |
|----|--------------|--------|
| W1×G1 | SampleSpace × Euclidean | SampleSpace solves selection, not routing |

### UNDEFINED (1 intersection)

| ID | Intersection | Status |
|----|--------------|--------|
| W2×G2 | DualTree × 2-opt | Edge encoding not defined |

---

## The Critical Path

For P=NP, we need this chain:

```
W4×G3 (ROPE - PROVEN)
    ↓
W4×G4 (Thin Cell - INCOMPLETE)
    ↓
W5×G5 (Coupling - INCOMPLETE)
    ↓
|LO(n)| = O(n^c) (NOT PROVEN)
    ↓
P = NP (NOT PROVEN)
```

**Current status:** Chain breaks at W4×G4.

---

## What Each Intersection Needs

### W4×G4 (ROPE × Thin Cell)

1. **Define monotonicity formally**
   ```
   Path is monotonic iff proj(p₁) < proj(p₂) < ... < proj(pₘ)
   ```

2. **Prove thin → unique**
   ```
   Theorem: If α ≥ cm, exactly one monotonic path exists
            and it is 2-opt stable.
   ```

3. **Handle non-thin segments**
   ```
   Theorem: Non-thin segments have O(m²) stable paths.
   (Currently claimed via (L,R) bijection, needs verification)
   ```

### W5×G5 (Bipartite × Coupling)

1. **Define compatibility**
   ```
   Paths P, Q compatible iff combined tour stable at boundary.
   ```

2. **Prove coupling bound**
   ```
   Theorem: Number of compatible pairs ≤ f(p₁, p₂)
            where f << p₁ × p₂.
   ```

3. **Telescope to get total**
   ```
   Theorem: Total stable tours = O(n^c) for some c.
   ```

---

## Empirical vs Proven

| Claim | Empirical | Proven |
|-------|-----------|--------|
| Hull order fixed | N/A | YES |
| Crossing is suboptimal | N/A | YES |
| Thin segments unique | Yes (50/50) | NO |
| Few local optima | Yes (small n) | NO |
| Coupling limits combos | Yes (wrong test) | NO |
| |LO(n)| = O(n^c) | Maybe | NO |

---

## Files Created

| File | Content |
|------|---------|
| `W1_G1_SampleSpace_Euclidean.md` | N/A - wrong domain |
| `W1_G2_SampleSpace_2opt.md` | Inspirational analogy |
| `W2_G2_DualTree_2opt.md` | Undefined encoding |
| `W3_G3_Pareto_NonCrossing.md` | PROVEN |
| `W4_G2_ROPE_2opt.md` | Partial |
| `W4_G3_ROPE_NonCrossing.md` | PROVEN |
| `W4_G4_ROPE_ThinCell.md` | Incomplete |
| `W5_G5_Bipartite_Coupling.md` | Incomplete |
| `INTERSECTION_SUMMARY.md` | This file |

---

## Next Step: Large Scale Tests

Before deciding A/B/C:

Run empirical tests for n = 12, 20, 50, 100, 200, 500, 1000

Measure:
1. Number of 2-opt local optima found
2. Time to find all optima
3. Hull size vs n
4. Thin segment ratio

This will inform whether the conjectures are plausible at scale.

---

*Summary created: 2026-01-01*
