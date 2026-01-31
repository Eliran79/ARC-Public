# Intersection W5×G5: Bipartite × Coupling (CRITICAL)

## The Threads

**W5 (Bipartite):** Recursive subdivision into cells
**G5 (Coupling):** Shared vertices limit composition freedom

---

## Theory

### Claim

Bipartite hierarchy + coupling constraints give O(√n) total optima.

### Formal Statement

**Conjecture W5×G5:**
1. Bipartite creates O(n) leaf cells
2. Adjacent cells share entry/exit vertex (coupling)
3. Coupling limits compatible combinations
4. Total stable tours = O(√n), not O(∏ per-cell)

---

## What's Good

### Bipartite Hierarchy (Valid)

```
Algorithm:
  Split cell along diameter
  Recurse on each half
  Stop when cell has ≤ c points

Properties:
  - Depth = O(log n)
  - Leaf cells = O(n)
  - Each leaf has O(1) points
```

This is correct and well-defined.

### Shared Vertices (Valid)

```
Adjacent cells share boundary vertex.
Left cell's exit = Right cell's entry.
This is true by construction.
```

### Code Evidence

```python
def test_shared_vertex_coupling():
    # Adjacent pairs with shared vertex: 221/221
    # Coupling rate: 100.0%
```

---

## What's Bad

### Gap 1: O(n) Cells, Not O(√n)

Bipartite gives O(n) leaf cells, not O(√n).

```
Start: 1 cell, n points
After log₂(n) levels: n cells, 1 point each
```

The O(√n) must come from COUPLING, not from cell count.

### Gap 2: Coupling Argument Invalid

The test we ran was meaningless:

```python
# WRONG: Random pairs have nothing to do with tours
pairs = [(random.random(), random.random()) for _ in range(num_pairs)]
frontier = pareto_frontier(pairs)
```

This tests Pareto frontier of RANDOM POINTS.
It doesn't test coupling in TOUR STRUCTURE.

### Gap 3: "Compatible" Undefined

What makes two segment paths compatible?

**Attempt 1:** They share a vertex (always true by construction)
**Attempt 2:** The combined path is 2-opt stable (need to verify)
**Attempt 3:** Some Pareto condition (not defined)

We never defined the compatibility relation.

### Gap 4: Sum vs Product Not Proven

We claim: coupling turns product into sum.

```
Product: ∏ p_i = exponential
Sum: Σ p_i = polynomial
```

But why does coupling give SUM instead of PRODUCT?

No proof exists.

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| Bipartite structure | **VALID** |
| Shared vertices | **VALID** |
| O(n) cells | **CORRECT** (not O(√n)) |
| Coupling defined | **NO** |
| Sum-not-product | **NOT PROVEN** |
| O(√n) total | **NOT PROVEN** |

### Verdict

This intersection is **STRUCTURALLY VALID but MATHEMATICALLY INCOMPLETE**.

The data structure works. The counting argument doesn't exist.

---

## What Would Complete It

### Step 1: Define Compatibility

```
Definition: Paths P in cell C₁ and Q in cell C₂ are compatible iff:
  1. P ends at shared vertex v
  2. Q starts at shared vertex v
  3. The combined path P·Q is 2-opt stable
     (no improvement crossing the C₁-C₂ boundary)
```

### Step 2: Bound Incompatibility

```
Theorem: For most (P, Q) pairs, they are incompatible.

Specifically: If C₁ has p₁ stable paths and C₂ has p₂ stable paths,
the number of compatible pairs is at most f(p₁, p₂) << p₁ × p₂.
```

### Step 3: Prove the Bound

```
What is f(p₁, p₂)?

Options:
  - f = min(p₁, p₂)           → sum bound
  - f = √(p₁ × p₂)            → geometric mean
  - f = O(1)                  → constant

We claimed "Pareto frontier" but never proved ANY of these.
```

### Step 4: Telescope

```
If f(p₁, p₂) = min(p₁, p₂), then:
  Total = f(f(f(...))) = O(max single cell)

If f(p₁, p₂) = √(p₁ × p₂), then:
  Total = O(∏ pᵢ^{1/2^depth}) = O(n^c) for some c
```

---

## The Real Question

**Is there geometric structure that limits compatible pairs?**

Intuition says YES:
- Paths ending "high" at v conflict with paths starting "low"
- The angle of entry/exit matters
- This might form a Pareto-like structure

But this is INTUITION, not PROOF.

---

## Empirical Status

```
Tested: n up to 50
Cells created: O(n) as expected
Shared vertices: 100%
Product bound: ~1 (cells have ~1 path each for small n)
Sum bound: O(n)

Limitation:
  - Tests too small for asymptotic claims
  - Coupling never actually measured
  - Pareto test was on wrong data
```

---

*Documented: 2026-01-01*
*Status: INCOMPLETE - coupling argument missing*
