# WARP-WOOF INTERSECTION MATRIX

## STATUS: CLOSED

**See:** `WARP_WOOF_CLOSURE.md` for final assessment.

**Verdict:** Framework provides intuition, NOT proof. Four fatal gaps identified.

---

## Framework Overview

**Warp threads** (vertical) are data structure components.
**Woof threads** (horizontal) are geometric constraint components.
**Hardness dies at the intersections**, not the individual components.

---

## WARP Threads (Data Structure)

| ID | Thread | What It Provides | Code Location |
|----|--------|------------------|---------------|
| W1 | SampleSpace | O(n) orthogonal basis vectors | `prior_art/original_samplespace/` |
| W2 | DualTree | O(log n) dominance encoding | `prior_art/original_samplespace/portfolio_selector.py` |
| W3 | Pareto Frontier | Non-dominated solution set | `thin_cell_verification.py` |
| W4 | ROPE Segments | Hull-based decomposition | `rope_decomposition.py` |
| W5 | Bipartite Hierarchy | Recursive subdivision | `bipartite_hierarchy.py` |

## WOOF Threads (Geometric Constraints)

| ID | Thread | What It Provides | Code Location |
|----|--------|------------------|---------------|
| G1 | Euclidean Metric | Triangle inequality | `thin_cell_verification.py` |
| G2 | 2-opt Stability | Local optimality condition | `zigzag_lemma_proof.py` |
| G3 | Non-crossing | Simple polygon property | `improved_verification.py` |
| G4 | Thin Cell Geometry | Aspect ratio → uniqueness | `thin_cell_verification.py` |
| G5 | Coupling Bounds | Composition limits | `coupling_analysis.py` |

---

## THE 5×5 INTERSECTION MATRIX

```
        │ G1 (Euclidean) │ G2 (2-opt) │ G3 (Non-cross) │ G4 (Thin Cell) │ G5 (Coupling) │
────────┼────────────────┼────────────┼────────────────┼────────────────┼───────────────┤
W1      │ research-067   │ research-072│ [TBD]         │ [TBD]          │ [TBD]         │
Sample  │ Cluster sep    │ Ortho-opt   │                │                │               │
────────┼────────────────┼────────────┼────────────────┼────────────────┼───────────────┤
W2      │ [TBD]          │ research-068│ research-073  │ [TBD]          │ [TBD]         │
DualTree│                │ Dom→improve │ Cross detect  │                │               │
────────┼────────────────┼────────────┼────────────────┼────────────────┼───────────────┤
W3      │ [TBD]          │ [TBD]       │ research-069  │ research-074   │ [TBD]         │
Pareto  │                │             │ Cross=domin   │ Thin=singleton │               │
────────┼────────────────┼────────────┼────────────────┼────────────────┼───────────────┤
W4      │ [TBD]          │ [TBD]       │ [TBD]         │ research-070   │ research-075  │
ROPE    │                │             │               │ ★CRITICAL★     │ Hull couples  │
────────┼────────────────┼────────────┼────────────────┼────────────────┼───────────────┤
W5      │ [TBD]          │ [TBD]       │ [TBD]         │ [TBD]          │ research-071  │
Bipart  │                │             │               │                │ Pareto comp   │
────────┴────────────────┴────────────┴────────────────┴────────────────┴───────────────┘
```

---

## INTERSECTION STATUS

### Critical Path (Must Prove First)

| Task | Intersection | Status | Theory | Code | Test |
|------|--------------|--------|--------|------|------|
| research-070 | W4×G4 | ✅ VERIFIED | ✅ | ✅ | ✅ |
| research-071 | W5×G5 | ✅ VERIFIED | ✅ | ✅ | ✅ |
| research-075 | W4×G5 | TODO | ⬜ | ⬜ | ⬜ |

### High Priority

| Task | Intersection | Status | Theory | Code | Test |
|------|--------------|--------|--------|------|------|
| research-067 | W1×G1 | TODO | ⬜ | ⬜ | ⬜ |
| research-068 | W2×G2 | TODO | ⬜ | ⬜ | ⬜ |
| research-069 | W3×G3 | TODO | ⬜ | ⬜ | ⬜ |
| research-072 | W1×G2 | TODO | ⬜ | ⬜ | ⬜ |
| research-073 | W2×G3 | TODO | ⬜ | ⬜ | ⬜ |
| research-074 | W3×G4 | TODO | ⬜ | ⬜ | ⬜ |

---

## VERIFIED INTERSECTION RESULTS

### W4×G4: ROPE meets Thin Cell (2026-01-01)

**Test file:** `tests/intersections/test_w4_g4_rope_thincell.py`

**Empirical Findings (NOT PROOFS):**
1. 50/50 tested thin segments had 1 stable path
2. 25-30/50 tested fat segments had >1 stable path
3. Bipartite converges in O(log m) levels

**MATHEMATICAL GAPS:**
- "Monotonic" never formally defined
- Bipartite gives O(n) cells, not O(√n)
- No proof that thin → unique

### W5×G5: Bipartite meets Coupling (2026-01-01)

**Test file:** `tests/intersections/test_w5_g5_bipartite_coupling.py`

**Empirical Findings (NOT PROOFS):**
1. Bipartite creates log(n) depth hierarchy
2. Adjacent cells share vertices

**MATHEMATICAL GAPS:**
- Pareto test was on RANDOM pairs, not tour constraints
- "Compatible" never defined
- No valid coupling argument exists
- The O(√n) bound is not established

### Remaining Intersections (To Be Created)

| Intersection | What to Prove |
|--------------|---------------|
| W1×G3 | SampleSpace × Non-crossing |
| W1×G4 | SampleSpace × Thin Cell |
| W1×G5 | SampleSpace × Coupling |
| W2×G1 | DualTree × Euclidean |
| W2×G4 | DualTree × Thin Cell |
| W2×G5 | DualTree × Coupling |
| W3×G1 | Pareto × Euclidean |
| W3×G2 | Pareto × 2-opt |
| W3×G5 | Pareto × Coupling |
| W4×G1 | ROPE × Euclidean |
| W4×G2 | ROPE × 2-opt |
| W4×G3 | ROPE × Non-crossing |
| W5×G1 | Bipartite × Euclidean |
| W5×G2 | Bipartite × 2-opt |
| W5×G3 | Bipartite × Non-crossing |
| W5×G4 | Bipartite × Thin Cell |

---

## THE PROOF CHAIN

The proof flows through specific intersections:

```
     SampleSpace (W1)
           │
           │ Creates orthogonal basis
           ▼
     DualTree (W2) ───────────────────────┐
           │                              │
           │ Encodes dominance            │ G3: detects crossings
           ▼                              ▼
     Pareto Frontier (W3) ◄──── Non-crossing (G3)
           │                              │
           │ Defines optimal set          │
           ▼                              ▼
     ROPE (W4) ◄────────────────── Thin Cell (G4)
           │        research-070          │
           │        ★CRITICAL★            │
           │                              │
           │ Hull decomposition           │ Uniqueness via α≥m
           ▼                              ▼
     Bipartite (W5) ◄──────────── Coupling (G5)
           │        research-071          │
           │                              │
           │ Recursive subdivision        │ Limits composition
           ▼                              ▼
     ┌─────────────────────────────────────┐
     │  OBSERVABLE SPACE COLLAPSE          │
     │  |LO(n)| = O(√n)                    │
     │  P = NP                             │
     └─────────────────────────────────────┘
```

---

## VERIFICATION PROTOCOL

For each intersection [Wi×Gj]:

### 1. Theory Statement
- Write formal lemma
- State assumptions clearly
- Prove or identify as gap

### 2. Code Verification
- Implement test in `tests/intersections/test_wi_gj.py`
- Test on controlled cases
- Test on random instances

### 3. Integration Test
- Verify Theory + Code match
- Run on n = 100, 500, 1000
- Document results in this matrix

### Checkmark Legend
- ⬜ Not started
- 🟡 In progress
- ✅ Verified (Theory + Code + Test pass)
- ❌ Falsified (found counterexample)

---

## HOW TO UPDATE THIS DOCUMENT

1. Complete an intersection task (research-XXX)
2. Update status in the matrix above
3. Add checkmarks: Theory ✅, Code ✅, Test ✅
4. Add notes if intersection revealed new insights

---

*Document created: 2026-01-01*
*Part of Warp-Woof Framework (research-066)*
