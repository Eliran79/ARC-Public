# Intersection W4×G2: ROPE × 2-opt

## The Threads

**W4 (ROPE):** Hull-based decomposition into k segments
**G2 (2-opt):** Local optimality via edge swaps

---

## Theory

### Claim

ROPE segments are 2-opt independent: improvements within segments don't cross boundaries.

### Formal Statement

**Theorem W4×G2 (Segment Independence):**
For a 2-opt stable tour decomposed by ROPE:
1. Each segment is internally 2-opt stable
2. No 2-opt move crosses segment boundaries
3. Segments can be optimized independently

### The Proof Attempt

```
Given: Tour T with ROPE segments S₁, ..., S_k

Claim: No 2-opt move swaps edges from different segments.

Argument:
  Suppose edges (a,b) ∈ S_i and (c,d) ∈ S_j with i ≠ j.
  The 2-opt swap would create edges (a,c) and (b,d).

  Case 1: i and j adjacent
    Then (a,c) or (b,d) would cross the hull edge.
    This creates a crossing → contradicts optimality.

  Case 2: i and j non-adjacent
    Then (a,c) and (b,d) both cross hull edges.
    Multiple crossings → contradicts optimality.

Therefore: 2-opt moves stay within segments.
```

---

## What's Good

1. **Hull order fixed:** Proven via non-crossing argument
2. **Intuition correct:** Cross-segment swaps create crossings
3. **Enables decomposition:** Can analyze segments separately

---

## What's Bad

### Gap: The Argument Is Incomplete

The "proof" assumes swaps across segments create crossings.

But this isn't always true:
- Points near segment boundaries might swap without crossing
- The hull edge isn't necessarily crossed

**Counterexample attempt:**

```
Hull vertices: A -- B -- C -- D (square)
Segment 1: A to B with interior point P near B
Segment 2: B to C with interior point Q near B

Edge (P, B) in S₁, edge (B, Q) in S₂.

Could (P, Q) and (B, B) be a 2-opt swap?
No - (B, B) isn't a valid edge.

But what about edges further inside?
Need careful analysis.
```

### The Real Question

When can edges from different segments be swapped?

**Answer:** Only if the swap doesn't create crossings.

**Constraint:** Tour must remain a simple polygon.

This LIMITS but doesn't ELIMINATE cross-segment swaps.

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| Hull order fixed | **PROVEN** |
| Full independence | **NOT PROVEN** |
| Cross-segment swaps rare | Likely true |
| Formal proof | Missing |

### Verdict

This intersection is **MOSTLY VALID** but needs tightening.

The independence is approximately true but the proof has gaps.

---

## What Would Complete It

Prove: For 2-opt stable tour T, no improving swap uses edges from different segments.

This requires showing: any cross-segment swap either:
1. Creates a crossing (hence not improving), or
2. Isn't improving due to distance increase

Current status: Case 1 argued informally, Case 2 not addressed.

---

*Documented: 2026-01-01*
