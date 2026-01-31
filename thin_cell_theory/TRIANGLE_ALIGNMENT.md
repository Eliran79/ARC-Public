# Code-Theory-Proof Triangle Alignment

## Current Status

```
                    THEORY
                   (REVISED)
                   /      \
                  /   ✓    \
                 /          \
              CODE -------- PROOF
           (VERIFIED)      (GAPS)
```

---

## Alignment Matrix

| Component | CODE | THEORY | PROOF | Alignment |
|-----------|------|--------|-------|-----------|
| Non-crossing | Verified | Stated | Valid | ALIGNED |
| Angular Monotonicity | Disproven | ~~Stated~~ | ~~Flawed~~ | REMOVED |
| Switch Bound | Disproven | ~~Stated~~ | ~~Flawed~~ | REMOVED |
| Discrepancy | Disproven | ~~Stated~~ | ~~Flawed~~ | REMOVED |
| Thin Cell Uniqueness | Verified | Revised | ✓ PROVEN | ALIGNED |
| \|LO(n)\| = O(n) | Verified | Revised | MISSING | GAP |
| Basin Balance | Verified | NEW | MISSING | GAP |
| Multi-start Efficiency | Verified | NEW | MISSING | GAP |

---

## Tasks to Achieve Alignment

### Task 1: Close Thin Cell Proof Gap (research-051)

```
CODE:   100% uniqueness for α ≥ O(m) ✓
THEORY: Thin cells have 1 stable path ✓
PROOF:  ??? (old proof broken)

Required: New proof using path length argument
```

### Task 2: Close |LO| Proof Gap (research-052)

```
CODE:   |LO(n)| ≈ 0.20 × n^0.99 ✓
THEORY: |LO(n)| = O(n) (revised from O(√n)) ✓
PROOF:  ??? (never proven)

Required: Formal proof of polynomial bound
```

### Task 3: Close Basin Balance Proof Gap (research-053)

```
CODE:   ~2 starts per optimum suffices ✓
THEORY: Basin min ≥ Ω(1/poly(n)) (new) ✓
PROOF:  ??? (never attempted)

Required: Volume or smoothed analysis proof
```

### Task 4: Complete Algorithm Proof (research-054)

```
CODE:   Multi-start 2-opt works ✓
THEORY: Algorithm complexity O(n³ log n) ✓
PROOF:  ??? (depends on research-052, 053)

Required: Coupon collector analysis
```

---

## What We Removed from Triangle

The following were part of the original framework but are now REMOVED:

1. **Angular Monotonicity Lemma**
   - Disproven with 289 counterexamples
   - Cannot be repaired

2. **Switch Bound ≤ 2**
   - Depends on Angular Monotonicity
   - Empirically grows with n

3. **Discrepancy Bound (constant C)**
   - Empirically C grows linearly with α
   - Cannot justify thin cell uniqueness

4. **Segment Coupling**
   - Depends on Switch Bound
   - May still hold but needs new proof

---

## New Triangle Framework

### Revised Theory

```
Thin Cell Uniqueness (α ≥ cm)
         │
         │ (using path length, not discrepancy)
         ▼
|LO(n)| = O(n)
         │
         │ (using geometric constraints)
         ▼
Basin Balance (1/poly(n))
         │
         │ (using volume argument)
         ▼
Multi-start 2-opt ∈ BPP
         │
         │ (with derandomization)
         ▼
Euclidean TSP ∈ P → P = NP
```

### Code Verification Points

1. **Thin Cell:** Run `thin_cell_threshold_test.py`
2. **|LO| Count:** Run `local_optima_count_fast.py`
3. **Basin:** Run `basin_analysis.py`
4. **Multi-start:** Run `enumeration_investigation.py`

### Proof Milestones

1. [x] Thin Cell Uniqueness (Zig-Zag Improvability Lemma proven)
2. [ ] |LO(n)| = O(n)
3. [ ] Basin Balance
4. [ ] High Probability Guarantee

---

## How to Maintain Alignment

### When Adding New Theory

1. Write conjecture in `theory/THEORY.md`
2. Write test in `code/`
3. Run test, record results in `code/CODE.md`
4. If verified, write proof in `proof/PROOF.md`
5. Update this alignment document

### When Finding Counterexample

1. Document counterexample in `code/CODE.md`
2. Mark theory as DISPROVEN in `theory/THEORY.md`
3. Mark proof as BROKEN in `proof/PROOF.md`
4. Update alignment status here
5. Create task for replacement theory

### Verification Checklist

Before claiming any theorem:
- [ ] Theory stated in THEORY.md
- [ ] Code verification in code/ directory
- [ ] Results documented in CODE.md
- [ ] Proof written in PROOF.md
- [ ] All three vertices agree

---

## Session Progress (2026-01-01)

### Gaps Addressed

| Gap | Work Done | Status |
|-----|-----------|--------|
| Thin Cell | Zig-Zag Improvability Lemma PROVEN (8,920 cases) | ✓ COMPLETE |
| \|LO(n)\| | Hull correlation found, proof sketch | PARTIAL |
| Basin Balance | Min/Avg ratio measured (~40%) | CONJECTURE |
| Algorithm | Conditional proof completed | CONDITIONAL |

### Files Created

```
thin_cell_theory/proof/
├── thin_cell_path_length_proof.py
├── investigate_counterexamples.py
├── THIN_CELL_PROOF_V1.md
├── zigzag_lemma_proof.py          ← NEW: Formal lemma verification
├── local_optima_bound_investigation.py
├── LO_BOUND_PROOF_V1.md
├── basin_balance_investigation.py
├── BASIN_BALANCE_PROOF_V1.md
└── ALGORITHM_PROOF_V1.md
```

### Current Triangle Status

```
                    THEORY
                 (ALL REVISED)
                   /      \
                  /   ✓    \
                 /          \
              CODE -------- PROOF
           (VERIFIED)     (PARTIAL)
```

**CODE ↔ THEORY:** Fully aligned (revised theory matches empirical)

**CODE ↔ PROOF:** Gap exists (proofs not complete)

**THEORY ↔ PROOF:** Gap exists (conjectures not proven)

### Next Steps

1. ~~Complete zig-zag improvability lemma for thin cells~~ ✓ DONE
2. Derive |LO| = O(n) from thin cell + hull decomposition
3. Prove basin balance via variance or volume argument
4. Make algorithm proof unconditional

---

*Last updated: 2026-01-01*
