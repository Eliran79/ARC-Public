# DTW FRAMEWORK: Dynamic Time Warping for TSP Segments

## The Insight

**Thin Cell:** Forces ONE path (bandwidth = 0) → Too rigid
**DTW-like:** Allows MULTIPLE paths with BOUNDED deviation → Matches data

---

## Dynamic Time Warping Review

### Classic DTW

Given two sequences A[1..n] and B[1..m]:
- Find optimal alignment (warping path)
- Path through cost matrix C[i,j] = d(A[i], B[j])
- Monotonic: can only move right, down, or diagonal
- Sakoe-Chiba band: limit warping to bandwidth w

```
Without band: O(nm) possible alignments
With band w:  O(n × w) alignments
```

### Key Property

Bandwidth w constrains deviation from diagonal.
Smaller w → fewer paths → more constrained alignment.

---

## DTW for TSP Segments

### Setup

Segment with:
- Entry point E
- Exit point X
- Interior points P₁, P₂, ..., Pₘ

**Reference ordering:** Sort by projection onto E→X axis
Let π(Pᵢ) = projection value, giving order π₁ < π₂ < ... < πₘ

### The Warping

A tour visits points in some order σ = (σ₁, σ₂, ..., σₘ).

**Warping distance:** How far σ deviates from π.

```
warp(σ) = max |position_in_σ(Pᵢ) - position_in_π(Pᵢ)|
        = max_i |σ⁻¹(i) - π⁻¹(i)|
```

### The Constraint

**2-opt stability limits warping!**

If warp(σ) is too large, a zig-zag pattern emerges.
Zig-zag → 2-opt improvable → not stable.

**Theorem (to prove):**
For a segment with aspect ratio α:
```
warp(σ) ≤ w(α, m) for any stable ordering σ
```

where w(α, m) is the **stability bandwidth**.

---

## The Bandwidth Function

### Thin Cell Case (α ≥ m)

Points are well-separated along axis.
Any deviation creates detectable zig-zag.
```
w(α, m) = 0 when α ≥ cm
```
Only the reference ordering π is stable.
**Result:** 1 path.

### Fat Cell Case (α < m)

Points can be "clumped" perpendicular to axis.
Some local reordering doesn't create zig-zag.
```
w(α, m) = f(m/α) for some increasing f
```
Multiple orderings within bandwidth are stable.

### The Formula (Conjecture)

```
w(α, m) ≈ m / α

When α = m:   w = 1 (minimal warping)
When α = 1:   w = m (maximal warping)
When α → ∞:  w → 0 (thin cell)
```

---

## Counting Paths with Bounded Warping

### Combinatorics

Number of permutations of m elements with max displacement ≤ w:

```
|{σ : warp(σ) ≤ w}| = ?
```

This is related to counting permutations with bounded displacement.

### Known Results

For permutations where each element moves at most w positions:

```
|Perm(m, w)| ≈ (2w+1)^m / m! × m!  for small w
            = (2w+1)^m              which is exponential in m
```

Wait, that's still exponential. Let me reconsider...

### Refined Constraint

2-opt stability isn't just "bounded displacement."
It's a GEOMETRIC constraint that's stronger.

The zig-zag lemma says: certain patterns are forbidden.
This is more restrictive than simple displacement bound.

---

## The Geometric DTW Constraint

### What 2-opt Forbids

Not just "large displacement" but "crossing patterns."

If σ has points Pᵢ and Pⱼ where:
- π(Pᵢ) < π(Pⱼ)  (i before j in projection)
- σ(Pᵢ) > σ(Pⱼ)  (j before i in tour)

Then the edges might cross or be improvable.

### The Inversion Bound

**Conjecture:** A 2-opt stable ordering has at most O(1) inversions
relative to the reference ordering.

```
inversions(σ, π) = |{(i,j) : π(i) < π(j) and σ(i) > σ(j)}|
```

If inversions ≤ k for constant k:
```
|stable orderings| ≤ C(m, k) = O(m^k)
```

For k ≈ 0.67: matches our m^0.67 observation!

---

## The DTW-TSP Connection

### Traditional DTW

```
Input: Two sequences
Constraint: Monotonicity + bandwidth
Output: Optimal alignment
Count: O(n × w) paths
```

### TSP-DTW

```
Input: Segment with m points
Constraint: 2-opt stability (≈ bounded inversions)
Output: All stable orderings
Count: O(m^c) orderings where c ≈ 0.67
```

### The Unification

**Thin Cell = DTW with bandwidth 0**
**Fat Cell = DTW with bandwidth w(α, m)**

The aspect ratio α determines the bandwidth.
2-opt stability is the constraint that limits warping.

---

## Why This Explains m^0.67

### The Inversion Argument

If stable orderings have ≤ k inversions:

For k = 0: 1 ordering (monotonic only)
For k = 1: O(m) orderings (one swap)
For k = 2: O(m²) orderings (two swaps)

Our observed m^0.67 suggests k < 1 on average.

**Interpretation:** Most points can't be swapped at all.
Only a "sparse" set of swaps are geometrically valid.

### The Sparse Swap Structure

Not every pair (i, j) can be inverted.
Only pairs where the perpendicular distance is large enough.

In a segment with aspect ratio α:
- Perpendicular spread ≈ L/α (where L = length)
- Number of "swappable" pairs ≈ f(m, α)

This gives a SPARSER structure than arbitrary inversions.

---

## Formalization Needed

### Definition (Stable Bandwidth)

```
w*(S) = max{warp(σ) : σ is 2-opt stable for segment S}
```

### Theorem (to prove)

```
w*(S) ≤ c × m / α  for some constant c
```

### Corollary

```
|stable orderings| ≤ Paths(m, w*(S)) = O(m^{f(w*)})
```

### The Exponent

If w* = c × m / α and counting formula gives m^{g(w*)}:
```
|stable orderings| = O(m^{c/α})
```

For typical random segments α ≈ 1.5:
```
c/α ≈ 0.67  ✓ Matches observation!
```

---

## Next Steps

1. **Formalize inversion bound**
   - Prove: 2-opt stable → bounded inversions
   - Find exact relationship to aspect ratio

2. **Count bounded-inversion permutations**
   - Not standard combinatorics (geometric constraint)
   - May need new counting techniques

3. **Connect to global bound**
   - With DTW-bounded segments
   - How do they combine?
   - Is there a DTW-like global constraint?

---

## The Vision

```
Old: Thin Cell (1 hall, 1 path) → Too rigid

New: DTW Band (corridor with bounded warping) → Flexible but bounded

         Thin Cell        DTW Band           Fat Cell
         ─────────       ══════════          ▓▓▓▓▓▓▓▓
         1 path          O(m^c) paths        O(m!) paths
         α ≥ m           1 < α < m           α ≈ 1
```

The DTW framework INTERPOLATES between thin (unique) and fat (exponential),
giving the polynomial bound we observe.

---

*Framework proposed: 2026-01-01*
*Key insight: 2-opt stability ≈ bounded warping/inversions*
