# CONNECTING THE BOUNDS: From Segments to Total

## The Puzzle

**Per-segment:** O(m^0.67) stable orderings
**Total optima:** O(n^4-5) observed

**Question:** How do these connect?

---

## The Naive Product

If we have k segments, each with O(m^0.67) orderings:

```
Total = ∏ᵢ O(mᵢ^0.67)
```

For random points:
- k = hull size ≈ O(log n)
- Each segment has mᵢ ≈ n/k = n/log n points

Naive product:
```
Total = (n/log n)^{0.67 × log n}
      = (n/log n)^{0.67 log n}
      = n^{0.67 log n} / (log n)^{0.67 log n}
```

This is **SUPER-POLYNOMIAL** (almost exponential).

But we observe O(n^4-5). Why?

---

## The Key Insight: NOT ALL SEGMENTS ARE INDEPENDENT

### What We Measured

We counted stable orderings per segment IN ISOLATION.

But in a full tour:
- Segments share hull vertices
- A tour chooses ONE ordering per segment
- These choices must be CONSISTENT

### Constraint 1: Entry/Exit Must Match

Segment i exits at vertex v.
Segment i+1 enters at vertex v.

The PATH through v is determined by:
- Last point of segment i's ordering
- First point of segment i+1's ordering

Not all combinations are stable at v.

### Constraint 2: Global 2-opt Stability

Even if each segment is locally stable,
the FULL TOUR might have a 2-opt improvement
that involves edges from DIFFERENT segments.

This creates GLOBAL constraints on which segment orderings can combine.

---

## Re-examining Coupling

Our coupling test showed 100% compatibility.
But this might be misleading.

### What We Tested

```python
def count_compatible_pairs(...):
    # For now, consider all pairs compatible if they're individually stable
    compatible += 1
```

We didn't actually CHECK cross-segment 2-opt moves!

### What We Should Test

For each pair (ordering_i, ordering_j):
1. Build the full tour with these orderings
2. Check if there's a 2-opt improvement involving edges from both segments
3. Only count as compatible if NO such improvement exists

---

## A Better Coupling Test

Let me think about what cross-segment improvements look like.

### Cross-Segment 2-opt

Edges (a, b) from segment i and (c, d) from segment j.

2-opt swap: (a, c) and (b, d).

For this to be an improvement:
```
d(a,c) + d(b,d) < d(a,b) + d(c,d)
```

For this to be VALID (non-crossing):
- The new edges (a,c) and (b,d) must not cross other edges
- The tour must remain a simple polygon

### Geometric Constraint

If segments i and j are NOT adjacent:
- They are separated by hull edges
- Any cross-segment edge would cross a hull edge
- This creates a crossing → NOT ALLOWED

Therefore: **Cross-segment 2-opt only possible between ADJACENT segments.**

### Adjacent Segment Coupling

For adjacent segments sharing vertex v:
- Segment i: path ending at v
- Segment j: path starting at v

The only "cross-segment" edges are those adjacent to v.

Edge (last_of_i, v) and edge (v, first_of_j).

A 2-opt move involving these would:
- Remove (last_of_i, v) and (v, first_of_j)
- Add (last_of_i, first_of_j) and ... wait, that's not a valid 2-opt

**2-opt swaps two non-adjacent edges.** Edges sharing vertex v are ADJACENT.

Therefore: **NO cross-segment 2-opt is possible!**

---

## The Independence Theorem

**Theorem:** ROPE segments are 2-opt independent.

**Proof sketch:**
1. Non-adjacent segments: cross-segment edges would cross hull edges → not allowed
2. Adjacent segments: edges sharing hull vertex are adjacent → not valid 2-opt

Therefore: Any 2-opt improvement involves edges from a SINGLE segment.

Therefore: Segment orderings can be chosen INDEPENDENTLY.

**Corollary:** Total optima = ∏ (orderings per segment).

---

## But This Contradicts Our Observation!

If segments are truly independent:
```
Total = ∏ (n/k)^0.67 where k = O(log n)
      = super-polynomial
```

But we observe O(n^4-5).

**EITHER:**
1. The independence theorem is WRONG
2. The segment bound is WRONG
3. Something else limits the product

---

## Resolution: The Segments Overlap!

Wait. Let me re-check the ROPE decomposition.

### How Points Are Assigned

In our test:
```python
for p in interior:
    t = ((p.x - entry.x) * dx + (p.y - entry.y) * dy) / seg_len_sq
    if 0 < t < 1:
        seg_interior.append(p)
```

A point p is assigned to segment i if it projects onto segment i.

**But what if a point projects onto MULTIPLE segments?**

If segments share interior points, they're NOT independent!

### The Overlap Constraint

Points near hull vertices might project onto adjacent segments.
These points create DEPENDENCIES between segment orderings.

**This is the COUPLING we missed!**

---

## New Hypothesis

### Coupling via Shared Points

If point p projects onto both segment i and segment i+1:
- p must appear in EXACTLY ONE segment in the tour
- This creates a constraint: if p is in segment i, it can't be in segment i+1

The number of ways to assign shared points to segments is LIMITED.

### Counting the Constraint

Let s = number of points that could go to multiple segments.

Naive: 2^s choices of assignment.
But: each assignment determines orderings in those segments.

Total combinations:
```
∏ (orderings per segment) × 2^s
```

Wait, this makes it WORSE, not better.

---

## Back to Basics: What DOES Limit the Count?

Let me think differently.

### Observation 1: Memory is Fixed

The test found optima one at a time.
Memory = O(n) for current tour.
Not storing all optima.

### Observation 2: Optima Grow as n^4-5

This is polynomial, not exponential.

### Observation 3: Per-Segment is O(m^0.67)

Much smaller than m!.

### The Missing Link

The product of per-segment bounds is super-polynomial.
But actual count is polynomial.

**THEREFORE:** Segments are NOT independent.

The coupling we measured (100%) was the WRONG coupling.
There's a DIFFERENT constraint we haven't identified.

---

## What Could It Be?

### Possibility 1: Global 2-opt Across Segments

Despite my earlier argument, maybe cross-segment 2-opt IS possible
in some configurations.

### Possibility 2: Point Assignment Creates Constraints

The way points are assigned to segments creates dependencies.

### Possibility 3: Geometric Constraints Beyond 2-opt

Maybe 3-opt or k-opt stability creates additional constraints.

### Possibility 4: Our Counting Is Wrong

Maybe the n^4-5 observed count is UNDER-counting
because multi-start doesn't find all optima.

---

## What To Test Next

1. **Verify segment independence claim**
   - Generate random points
   - Find all 2-opt stable tours
   - Check if cross-segment improvements exist

2. **Check point assignment overlap**
   - Count points that project onto multiple segments
   - See if this explains coupling

3. **Verify multi-start finds all optima**
   - For small n, compare exhaustive vs multi-start
   - If multi-start under-counts, true count might be higher

---

*Analysis in progress: 2026-01-01*
*The connection between segment bounds and total is NOT YET CLEAR*
