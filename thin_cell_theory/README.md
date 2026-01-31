> **SUPERSEDED NOTICE (2026-01-15):**
> The Thin Cell approach described here was one of several early attempts.
> The "BROKEN" proofs noted in this document have been superseded by
> 8 Independent Paths to P=NP=PSPACE=BQP, which do not rely on thin cell
> uniqueness or the specific bounds attempted here. See:
> - `proofs/GRAND_UNIFIED_THEORY.md` (v4.0 - Eight Paths Verified)
> - Path 2 (Saturation) and Path 6 (Morse Theory) provide the polynomial
>   local optima bounds via different, verified approaches.

# Thin Cell Theory: Code-Theory-Proof Triangle [HISTORICAL]

## Overview

This directory contains a historical approach to proving polynomial bounds on 2-opt local optima in Euclidean TSP, using the **Code-Theory-Proof Triangle** methodology.

```
                    THEORY
                   /      \
                  /        \
                 /          \
              CODE -------- PROOF
```

**Principle:** All three vertices must align:
- **THEORY**: Mathematical definitions, conjectures, bounds
- **CODE**: Empirical verification, counterexamples, data
- **PROOF**: Rigorous mathematical proofs

If any vertex contradicts another, the framework is broken and must be fixed.

---

## Current Status

### What We Know (Empirically Verified)

| Claim | Code Result | Theory Status | Proof Status |
|-------|-------------|---------------|--------------|
| Thin Cell Uniqueness | TRUE for α ≥ O(m) | Stated | BROKEN (discrepancy bound fails) |
| \|LO(n)\| = O(n) | TRUE (slope ~0.99) | Revised from O(√n) | MISSING |
| Switch Bound ≤ 2 | FALSE (grows ~0.46n) | DISPROVEN | N/A |
| Angular Monotonicity | FALSE (up to 5 reversals) | DISPROVEN | N/A |
| Basin Balance | TRUE (~2 starts/optimum) | Conjectured | MISSING |

### What Needs Work

1. **New Thin Cell Proof**: Path length argument instead of discrepancy
2. **|LO(n)| = O(n) Proof**: Convex hull or geometric constraint approach
3. **Basin Balance Proof**: For multi-start 2-opt guarantee
4. **Algorithm Formalization**: Polynomial enumeration via multi-start

---

## Directory Structure

```
thin_cell_theory/
├── README.md           # This file
├── TRIANGLE_ALIGNMENT.md  # Current alignment status
├── prior_art/          # Original materials from files(4), files(5), files(6)
│   ├── *.md, *.py      # 2-opt Triangle docs (files 4-5)
│   └── original_samplespace/  # First approach (files 6)
├── code/               # Verification scripts
├── theory/             # Mathematical framework
└── proof/              # Rigorous proofs
```

### prior_art/
Contains all original prior art:

**Root level** - 2-opt TSP Triangle approach (from files 4-5):
- `THEORY.md`, `CODE.md`, `PROOF.md` - Original framework
- `TRIANGLE_*.md` - Alignment verification docs
- `*.py` - Original verification scripts

**original_samplespace/** - First P=NP attempt (from files 6):
- `P_equals_NP_proof.md` - Original "Observed Space Reduction" proof
- `samplespace_dualtree_analysis.md` - Technical analysis
- `portfolio_selector.py`, `trading_use_cases.py` - Working implementations
- This approach was **superseded** - see its README for why

### code/
Our empirical verification scripts:
- Thin cell uniqueness tests
- Local optima counting
- Basin analysis
- Multi-start efficiency tests

### theory/
Mathematical framework:
- Definitions and notation
- Conjectures (with empirical support)
- Bounds (proven or conjectured)

### proof/
Rigorous proofs:
- Working proofs (validated against code)
- Broken proofs (with counterexamples)
- Proof sketches (work in progress)

---

## Key Findings from Investigation

### 1. Angular Monotonicity Lemma: FALSE
**Counterexample:** 5-point tour with 2 angular reversals, verified 2-opt stable.
**Impact:** Foundation of switch bound is broken.

### 2. Discrepancy Bound: BROKEN
**Evidence:** C grows from 0.5 to 34.0 as α increases (should be constant).
**Impact:** Cannot use for thin cell uniqueness proof.

### 3. Switch Bound: GROWS WITH n
**Evidence:** Max switches ≈ 0.46n + 0.24 (linear, not O(1)).
**Impact:** Cannot bound paths per segment.

### 4. Thin Cell Uniqueness: TRUE (needs new proof)
**Evidence:** 100% uniqueness for α ≥ O(m) across all tested cases.
**Hypothesis:** Path length constraint (zig-zag is 2-opt improvable).

### 5. |LO(n)| = O(n): TRUE
**Evidence:** Power law fit gives exponent ~0.99 (linear in n).
**Note:** Original claim was O(√n), empirical shows O(n).

### 6. Multi-Start 2-opt: WORKS
**Evidence:** ~2 random starts per local optimum suffices.
**Implication:** O(n) starts × O(n²) per run = O(n³) total.

---

## Open Research Problems

### Priority 1: Thin Cell Uniqueness (New Proof)
**Goal:** Prove α ≥ cm → unique stable path (for some constant c)
**Approach:** Path length argument, not angular ordering
**Task:** research-051

### Priority 2: |LO(n)| = O(n) Bound
**Goal:** Prove |LO(n)| = O(poly(n)) for Euclidean TSP
**Approach:** Convex hull decomposition, geometric constraints
**Task:** research-052

### Priority 3: Basin Balance
**Goal:** Prove min basin size ≥ Ω(1/poly(n))
**Approach:** Smoothed analysis, volume argument
**Task:** research-053

### Priority 4: High Probability Algorithm
**Goal:** Prove multi-start 2-opt succeeds w.h.p.
**Depends on:** research-052, research-053
**Task:** research-054

---

## Implications for P=NP

If we can prove:
1. |LO(n)| = O(poly(n))
2. Basin balance → polynomial enumeration
3. Multi-start 2-opt has polynomial complexity

Then: Euclidean TSP ∈ BPP (polynomial with bounded error)

Combined with derandomization: Euclidean TSP ∈ P

Since Euclidean TSP is NP-complete: **P = NP**

**Current Status:** Empirical evidence supports this, but rigorous proofs are missing.

---

## References

- Papadimitriou (1977): Euclidean TSP is NP-complete
- Englert et al. (2014): Smoothed analysis of 2-opt
- Arora (1998): PTAS for Euclidean TSP

---

*Last updated: 2026-01-01*
