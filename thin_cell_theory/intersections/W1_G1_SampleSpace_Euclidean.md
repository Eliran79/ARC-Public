# Intersection W1×G1: SampleSpace × Euclidean

## The Threads

**W1 (SampleSpace):** Creates O(n) orthogonal basis vectors from categorical data
**G1 (Euclidean):** Triangle inequality d(a,c) ≤ d(a,b) + d(b,c)

---

## Theory

### Claim

SampleSpace's orthogonal basis respects Euclidean cluster structure.

### Formal Statement

**Conjecture W1×G1:**
For n points with categorical features embedded in Euclidean space:
1. Points in same category have bounded pairwise distance
2. Orthogonal categories create separable clusters
3. SampleSpace basis vectors align with cluster directions

### The Connection

```
SampleSpace:
  Input: n items with categorical attributes
  Output: O(n) orthogonal vectors spanning observable combinations

Euclidean Metric:
  Input: points in R^d
  Property: triangle inequality holds

Intersection Question:
  If we embed categorical data in Euclidean space,
  does SampleSpace structure respect Euclidean geometry?
```

---

## What's Good

1. **Orthogonality is well-defined:** Gram-Schmidt gives O(n) orthogonal vectors
2. **Euclidean is well-defined:** Standard metric with triangle inequality
3. **Embedding exists:** Categories → one-hot → Euclidean vectors

### Code Evidence

```python
# From portfolio_selector.py
# Categories become orthogonal dimensions
# sector: Tech, Finance, Energy → 3 orthogonal directions
# Each stock is a point in this space
```

---

## What's Bad

1. **No TSP connection:** SampleSpace was designed for selection, not routing
2. **Categories ≠ Geometry:** Orthogonal categories don't imply geometric clustering
3. **Dimension explosion:** n categories → n dimensions, not useful for TSP

### The Gap

SampleSpace works for **multi-objective selection** (choose best portfolio).
TSP is a **routing problem** (visit all points once).

These are fundamentally different:
- Selection: which subset to choose
- Routing: what order to visit

**SampleSpace does not apply to TSP directly.**

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| Theory defined | Partially - conjecture stated |
| Code exists | Yes - portfolio_selector.py |
| Applies to TSP | **NO** |
| Math proven | No |

### Verdict

This intersection is **NOT USEFUL** for P=NP via TSP.

SampleSpace solves a different problem (selection, not routing).
The orthogonal basis has no clear connection to tour optimization.

---

## Code Reference

```python
# Location: thin_cell_theory/prior_art/original_samplespace/portfolio_selector.py

class SampleSpace:
    """
    Observable combinations form O(n) orthogonal vectors.
    Used for: Portfolio selection (multi-objective)
    NOT used for: TSP routing
    """
```

---

## Next Steps

Either:
1. Find a way to connect SampleSpace to TSP (unlikely)
2. Mark this intersection as NOT APPLICABLE
3. Use SampleSpace insight (observable << theoretical) in different way

---

*Documented: 2026-01-01*
