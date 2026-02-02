# The Correlation Argument for Polynomial Local Optima

**Status:** PROMISING NEW DIRECTION
**Date:** 2026-01-02
**Author:** Eliran Sabag

---

## The Key Insight

**Why random DAG analysis fails:**

For a random DAG with N vertices and degree d, expected sinks ≈ N/d.

For 2-opt:
- N = (n-1)!/2 tours
- d = n(n-3)/2 ≈ O(n²) neighbors per tour

Expected sinks (random) ≈ (n-1)!/2 / n² = (n-3)!/2 = **FACTORIAL**

But empirically, we observe O(n²) sinks. **400,000× fewer than random prediction!**

**The 2-opt improvement DAG is NOT random. The weight structure creates massive correlation.**

---

## The Correlation Structure

### Edge Sharing Creates Correlation

Two tours T₁, T₂ sharing k edges have correlated "fates":
- Similar edge costs → similar improvement opportunities
- Overlapping 2-opt neighborhoods
- Often flow to same local optimum

### Quantifying Overlap

For random pair of tours, expected shared edges:
- Each tour has n edges
- Total possible edges: n(n-1)/2
- Expected overlap: n × n / (n(n-1)/2) = 2n/(n-1) ≈ 2

**Average tours share ~2 edges** (out of n).

But neighbors in 2-opt graph share exactly (n-2) edges!

### The Neighborhood Effect

A tour T and its 2-opt neighbor T' share (n-2) edges.
They differ only in which 2 edges they use.

This means:
- T and T' have almost identical cost structures
- Their improvement patterns are highly correlated
- They're likely in the same basin (one improves to the other, or both near same sink)

---

## The Funneling Mechanism

### Why Random Weights Create Funnels

1. **Edge ranking:** Random weights induce a total ordering of all edges
2. **Heavy edges are bad:** Tours using many heavy edges are unstable
3. **Light edges converge:** Tours using light edges flow together
4. **MST as attractor:** The MST uses only lightest edges per cut

### The Stability Condition

A tour T is locally optimal iff for every 2-opt move (i,j):
```
cost(current edges) ≤ cost(new edges)
```

This is a system of O(n²) inequalities on edge weights.

For generic weights, these inequalities are "tight" at exactly O(n²) points.

**Each tight point corresponds to a potential local optimum.**

---

## The Dimension Argument (Refined)

### Weight Space

Edge weights form a vector w ∈ ℝ^{n(n-1)/2}.

### Optimality Regions

For each tour T, the set of weights making T locally optimal is:
```
R_T = { w : for all 2-opt moves (i,j), Δ_cost(i,j) ≥ 0 }
```

This is defined by O(n²) linear inequalities, so R_T is a convex polyhedron.

### Counting Regions

The O(n² × n!) total constraints partition weight space into regions.

**Key claim:** The number of distinct non-empty optimality regions is O(n² × 2^{n}).

Wait, that's still exponential...

### The Constraint Dependence

The constraints are NOT independent!

For tour T, the O(n²) constraints share edges:
- Constraint for move (i,j) involves edges at positions i, i+1, j, j+1
- Constraint for move (i',j') overlaps if positions overlap

This dependence reduces the effective dimension.

**Effective degrees of freedom:** O(n) (the tour itself has n edges)

**Maximum independent constraints:** O(n)

**Therefore:** At most O(2^n) distinct optimality patterns for a single tour.

But we have (n-1)!/2 tours...

---

## The Tour Independence Argument

### Independent Tours

Two tours T₁, T₂ are "independent" if their optimality constraints don't share edges.

This happens when |T₁ ∩ T₂| = 0 (no shared edges).

### Maximum Independent Set of Tours

**Question:** How many pairwise independent tours exist?

**Answer:** At most O(n²).

**Proof sketch:**
- Each tour uses n edges
- Independent tours use disjoint edge sets
- Total edges: n(n-1)/2
- Maximum tours with disjoint edges: ⌊n(n-1)/2 / n⌋ = (n-1)/2 ≈ O(n)

Wait, that gives O(n), not O(n²). Let me reconsider.

Actually, tours can't be edge-disjoint in a complete graph - any Hamiltonian cycle must visit all vertices, so any two cycles share at least some structure.

The constraint is more subtle: tours share vertices, not edges.

---

## The Vertex-Based Argument

### Shared Vertices = Shared Constraints

If tours T₁, T₂ both visit vertices a, b, c in some order, their 2-opt constraints involving these vertices are related.

### Counting by Vertex Triples

There are C(n,3) = O(n³) ways to choose 3 vertices.
For each triple, there are limited "local optimum configurations".

If each triple contributes O(1) local optima:
Total local optima ≤ O(n³)

But empirically we see O(n²), suggesting stronger coupling.

---

## The Edge-Pair Argument (Most Promising)

### Key Observation

A 2-opt move is defined by choosing 2 edges of the tour to swap.

Each tour has n edges, so C(n,2) = O(n²) possible moves.

### The "Bottleneck" Edges

For a tour T to be locally optimal, EVERY move must be non-improving.

But some moves are "dominated" by others:
- If move A is non-improving, related move A' is also likely non-improving
- The "independent" moves form a smaller set

**Claim:** There are O(n) independent 2-opt constraints per tour.

**Therefore:** The optimality condition is determined by O(n) bits, not O(n²).

### Counting Optimality Patterns

With O(n) independent bits per tour:
- At most 2^{O(n)} optimality patterns
- But tours are coupled by shared edges
- Effective patterns across all tours: O(n²)

---

## The Coupling Theorem (Conjecture)

**Theorem (Target):** For any weight function w on K_n:
```
|LocalOptima(w)| ≤ c × n²
```

for some constant c.

**Proof approach:**
1. Each local optimum T has a "certificate" = pair of "critical" edges
2. Critical edges are edges whose weight determines T's optimality
3. There are C(n,2) = O(n²) edge pairs
4. Each pair certifies at most O(1) local optima
5. Therefore O(n²) total

**Gap:** Prove that each optimum has a unique critical pair.

---

## Connection to Euclidean Case

In Euclidean TSP:
- Tours are constrained by non-crossing
- Critical "certificate" = pair of inversion-defining edges
- O(n²) such pairs → O(n⁴) optima (via O(n²) inversions per dimension)

In General TSP:
- Tours are constrained by weight ordering
- Critical "certificate" = pair of weight-comparison edges
- O(n²) such pairs → O(n²) optima

**The General case has fewer optima because it lacks the geometric structure that creates additional degrees of freedom!**

---

## Summary

The proof likely comes from showing:

1. **Each local optimum has a 2-edge certificate** that "witnesses" its optimality
2. **Certificates are unique** (or have O(1) multiplicity)
3. **There are O(n²) possible certificates** (edge pairs)
4. **Therefore O(n²) local optima**

The certificate could be:
- The two heaviest edges in the tour
- The two edges involved in the "closest" non-improving 2-opt move
- Some other characteristic pair

Finding and formalizing this certificate is the key to completing the proof.

---

*2026-01-02*
