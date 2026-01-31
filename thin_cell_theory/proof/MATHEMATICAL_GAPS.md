# MATHEMATICAL GAPS: What Code Cannot Prove

## The Fundamental Problem

**Code shows:** "For these 50 random instances, thin segments had 1 stable path."

**Math requires:** "For ALL possible instances, thin segments have exactly 1 stable path."

These are completely different claims. The Millennium Prize requires the second.

---

## GAP 1: Thin Segment Uniqueness

### What We Claim

**Lemma (Thin Cell Uniqueness):**
For a segment with m interior points and aspect ratio α ≥ m, there is exactly 1 stable path.

### What Code Shows

```python
def test_thin_segment_unique_path():
    # Tested 50 random thin segments
    # All had 1 stable path
    # Counterexamples: 0
```

### What Math Requires

A PROOF that proceeds like:

```
Assume segment S has α ≥ m.
Assume path P through S is 2-opt stable.
We must show P is the unique such path.

Step 1: Define "monotonic" precisely
Step 2: Prove stable → monotonic (Zig-Zag Lemma)
Step 3: Prove α ≥ m → unique monotonic ordering
Step 4: Conclude uniqueness
```

### The Actual Gap

**Step 3 is NOT proven.**

The Zig-Zag Lemma (Step 2) says: "If path has zig-zag, it's improvable."

But we need: "In thin cells, there's only one way to avoid zig-zag."

**Why is this hard?**

Consider 3 points A, B, C with:
- A = (0, 0.1)
- B = (0.5, -0.1)
- C = (1, 0.05)

Entry at (0, 0), Exit at (1, 0).

Is A→B→C monotonic? Is A→C→B monotonic?

"Monotonic" means "no reversal of direction toward exit."

But direction depends on WHERE you measure from!

**The Missing Definition:**
We need a FORMAL definition of monotonicity that:
1. Is computable
2. Implies 2-opt stability
3. Has at most 1 ordering in thin cells

Current status: **We have intuition, not proof.**

---

## GAP 2: Bipartite Thinning

### What We Claim

**Lemma (Bipartite Thinning):**
After O(log m) subdivisions, all cells are thin.

### What Code Shows

```python
# Average levels to make thin: 6.0
# For m=8 points, expected log₂(8) = 3
```

### What Math Requires

Prove: After k subdivisions, cell has:
- Length ≈ L/2^k (halved each time)
- Width ≈ W (unchanged? unclear!)
- Points ≈ m/2^k (halved each time)

Aspect ratio α = L/W.

For α ≥ m we need: (L/2^k) / W ≥ m/2^k

This simplifies to: L/W ≥ m

**Wait.** This says: Original aspect ratio must satisfy L/W ≥ m.

If original segment is "fat" (L/W < m), subdivision DOESN'T HELP with aspect ratio!

**What Actually Happens:**

Points decrease: m → m/2 → m/4 → ... → 1

When m = 1, ANY cell is "thin" (α ≥ 1 always).

So: Thinning happens by REDUCING m, not by IMPROVING α.

**The Real Lemma:**
After log₂(m) subdivisions, each cell has ≤1 point, so trivially thin.

**But This Changes Everything:**
- We get O(m) cells, not O(log m) cells
- Each cell has O(1) points
- Total cells = O(n), not O(√n)

**Where Does O(√n) Come From?**

It doesn't. The bipartite hierarchy gives O(n) leaf cells.

The O(√n) must come from COUPLING, not from cell count.

---

## GAP 3: Coupling Collapse (The Real Gap)

### What We Claim

**Theorem (Coupling Collapse):**
Total stable tours = O(√n), not O(∏ per-cell optima).

### What Code Shows

```python
# Pareto frontier of random (x,y) pairs is O(n^0.23)
# This is just a property of random points, NOT of our structure!
```

### What Math Requires

**The Structure We Have:**
- n points
- O(√n) convex hull vertices (expected for random points)
- O(√n) ROPE segments
- Each segment has O(√n) interior points (on average)

**The Coupling Constraint:**
Adjacent segments share a hull vertex.
The path through segment S_i exits at vertex v.
The path through segment S_{i+1} enters at the same vertex v.

**The Question:**
Does this constraint reduce the product to a sum?

**Attempt at Proof:**

Let p_i = number of stable paths in segment S_i.

Naive bound: Total = ∏ p_i = p_1 × p_2 × ... × p_k

Coupling says: Only COMPATIBLE pairs count.

Two paths (P_i in S_i, P_{i+1} in S_{i+1}) are compatible if:
- P_i's exit point = P_{i+1}'s entry point (always true, both = hull vertex)
- P_i's exit EDGE and P_{i+1}'s entry EDGE form a valid tour edge

**The Problem:**

The hull vertex is FIXED. Both paths go through it.
But the EDGES into/out of the vertex can vary.

Path P_i ends with edge (last_interior_point, hull_vertex).
Path P_{i+1} starts with edge (hull_vertex, first_interior_point).

For 2-opt stability of the FULL tour, we need:
- The edge (last_i, first_{i+1}) doesn't create crossings
- The tour through vertex v is locally optimal

**This is NOT a Pareto constraint!**

The "Pareto frontier" test in code was MEANINGLESS:
```python
# Generate random (exit_length, entry_length) pairs
pairs = [(random.random(), random.random()) for _ in range(num_pairs)]
```

This has NOTHING to do with actual tour constraints!

---

## GAP 4: The O(√n) Hull Assumption

### What We Assume

Convex hull has O(√n) vertices.

### The Reality

For n random points in a square: E[hull size] = O(log n)
For n random points in a disk: E[hull size] = O(n^{1/3})
For n points in convex position: hull size = n

**The O(√n) claim is FALSE for random points!**

Where did √n come from? It's the bound for points in a SQUARE:
- Hull size = O(log n) on average
- But ROPE segments = O(hull size), not O(√n)

**Correction:**
For random points in [0,1]²:
- E[hull vertices] = O(log n)
- E[ROPE segments] = O(log n)
- NOT O(√n)

---

## Summary: What's Actually Proven vs. Claimed

| Claim | Status | What's Missing |
|-------|--------|----------------|
| Thin cells have 1 stable path | UNPROVEN | Formal definition of monotonicity |
| Bipartite makes cells thin | MISLEADING | Cells become thin by m→1, giving O(n) cells |
| Coupling gives O(√n) | UNPROVEN | No valid Pareto argument constructed |
| Hull has O(√n) vertices | FALSE | For random points, it's O(log n) |
| Total optima = O(√n) | UNPROVEN | No valid proof chain |

---

## What Would A Real Proof Need?

### Step 1: Fix the Definitions

Define precisely:
- What is a "thin" cell? (α ≥ cm for what c?)
- What is "monotonic"? (w.r.t. what reference?)
- What is "stable"? (2-opt? 3-opt? k-opt?)

### Step 2: Prove Thin Cell Uniqueness

Theorem: For cells with α ≥ cm, there is exactly 1 stable path.

Proof must:
- Work for ALL point configurations, not just random
- Not depend on probabilistic arguments
- Give an explicit construction of the unique path

### Step 3: Prove Coupling Bound

Theorem: For k segments with p_1, ..., p_k stable paths each,
the total stable tours is at most f(k, max p_i).

Proof must:
- Define "compatible" precisely
- Show incompatibility is common enough
- Give explicit bound, not just "Pareto hand-wave"

### Step 4: Prove Algorithm

Given the structure, show:
- How to ENUMERATE all stable tours
- In polynomial time
- Without missing any

---

## Honest Assessment

**Current state:** We have empirical observations and intuitions.

**What's needed:** Rigorous definitions and proofs.

**The gap:** The entire proof chain is missing. Code tests don't bridge it.

---

*Document created: 2026-01-01*
*This is a critical self-assessment of the proof attempt*
