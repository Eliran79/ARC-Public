# THE MEMORY CONSTRAINT: What You Observed

## The Observation

**While testing n=12 to n=30:**
- Optima count grew: 4 → 9 → 33 → 104 → 356
- Memory usage: **REMAINED FIXED**

This is a critical clue I missed.

---

## What Fixed Memory Means

If memory is fixed while finding more optima, then:

1. **We're not storing all optima simultaneously**
   - Multi-start finds them one at a time
   - Each tour is O(n) space
   - Total memory = O(n), not O(|optima|)

2. **OR: The optima share structure**
   - They differ in only a few positions
   - Can be represented compactly
   - Suggests LOCAL differences, not GLOBAL

3. **OR: The search space is constrained**
   - We're exploring a fixed-size region
   - Not all n! permutations
   - Some structure limits exploration

---

## The Key Insight

**The 356 "optima" at n=30 are NOT 356 independent tours.**

They must share structure. What structure?

### Hypothesis: ROPE Segments Are Fixed

If ROPE decomposition is fixed (hull order invariant), then:
- k segments, each with m_i points
- Only the INTERNAL ordering of each segment varies
- Total combinations = ∏ (internal orderings)

But the optima SHARE the segment structure.

### Hypothesis: Most Segments Have Few Options

Even if segments aren't "thin" (α ≥ m), they might have limited orderings.

```
If each segment has O(1) stable internal orderings:
  Total optima = O(k) = O(hull size) = O(log n)

If each segment has O(m) stable orderings:
  Total optima = O(k × avg_m) = O(n)

If each segment has O(m²) stable orderings:
  Total optima = O(k × avg_m²) = O(n²)
```

The data shows ~n^5. Where does that come from?

---

## Re-examining The Data

| n | Hull k | Interior n-k | Optima | Optima/(n-k)² |
|---|--------|--------------|--------|---------------|
| 12 | 6 | 6 | 4 | 0.11 |
| 15 | 7 | 8 | 9 | 0.14 |
| 20 | 8 | 12 | 33 | 0.23 |
| 25 | 8 | 17 | 104 | 0.36 |
| 30 | 9 | 21 | 356 | 0.81 |

Optima/(n-k)² is growing but slowly.

Let me check Optima vs (n-k)^c for various c:

```
n=12: 4 vs 6^c
n=30: 356 vs 21^c

Ratio: 356/4 = 89
Ratio: 21/6 = 3.5

89 = 3.5^c
c = log(89)/log(3.5) = 4.49/1.25 = 3.6
```

So: **Optima ≈ (n - hull)^3.6**

If hull = O(log n), then interior = n - O(log n) ≈ n.

So Optima ≈ n^3.6, not n^5.

---

## The Constraint Structure

### What's Fixed (Memory-Constant)

1. **The point set** - O(n) space
2. **The hull** - O(log n) vertices
3. **The segment structure** - O(k) segments
4. **Current tour being evaluated** - O(n)

### What Varies (But Is Bounded)

1. **Internal segment orderings**
2. **Which orderings are stable**
3. **The count of stable configurations**

### The Structural Constraint

The fixed memory suggests we're exploring a **FIXED STRUCTURE** with **LIMITED VARIATIONS**.

```
Structure = ROPE decomposition (fixed for a point set)
Variations = internal orderings within segments
Constraint = 2-opt stability
```

---

## Tying More Constraints Together

### Constraint 1: Hull Order (PROVEN)

Hull vertices appear in cyclic order in any optimal tour.
This is FIXED - not a source of variation.

### Constraint 2: Non-Crossing (PROVEN)

Tour edges don't cross.
This constrains which internal orderings are valid.

### Constraint 3: Segment Independence (PARTIAL)

2-opt moves within a segment don't affect other segments.
Cross-segment moves create crossings → forbidden.

### Constraint 4: Internal Ordering Bound (NEEDED)

**THIS IS THE MISSING CONSTRAINT.**

We need: For a segment with m points, how many 2-opt stable orderings exist?

The data suggests: O(m^c) for some c ≤ 4.

### Constraint 5: Coupling at Boundaries (NEEDED)

When segments share a hull vertex:
- The exit edge of segment i
- The entry edge of segment i+1
- Must form a stable configuration

This COUPLES segments and reduces combinations.

---

## The New Proof Approach

### Step 1: Bound Internal Orderings

**Conjecture:** A segment with m interior points has at most O(m^c) stable orderings.

Evidence: Optima ≈ (interior)^3.6

### Step 2: Bound Via Coupling

**Conjecture:** Coupling reduces product to sub-multiplicative.

If segments have p₁, p₂, ..., p_k orderings:
- Naive: ∏ p_i (exponential in k)
- With coupling: something smaller

### Step 3: Memory Constraint

**Key:** The fact that memory is fixed means we're NOT storing all optima.

We're finding them via SEARCH, not ENUMERATION.

This suggests: **The optima form a CONNECTED structure** in some search space.

---

## What To Investigate Next

### Question 1: Why (n-hull)^3.6?

What geometric property causes this exponent?

Possibilities:
- Pairs of points that can swap: O(m²)
- Chains of swaps: O(m³)
- ??? giving O(m^3.6)

### Question 2: What Does Coupling Actually Do?

Measure: For adjacent segments, how many (ordering_i, ordering_j) pairs are stable together?

Is it: O(p_i × p_j)? O(p_i + p_j)? O(√(p_i × p_j))?

### Question 3: Why Fixed Memory?

The search process explores optima one at a time.
Each optimum is O(n) space.
We're not storing all of them.

Does this mean there's a POLYNOMIAL-TIME ENUMERATION?

---

## Revised Assessment

### What I Missed

I reported "n^5 growth" but:
1. The exponent is closer to 3.6 (vs interior points)
2. Memory being fixed is a strong structural constraint
3. The optima share ROPE structure, not independent

### What This Suggests

The polynomial bound might be REAL, with exponent ~4.

The MECHANISM is:
1. ROPE fixes segment structure
2. Each segment has polynomial internal orderings
3. Coupling limits total combinations
4. Memory is O(n) because optima are found sequentially

---

## Next Steps

1. **Measure coupling directly**
   - For each pair of adjacent segments
   - Count compatible orderings
   - See if it's sub-multiplicative

2. **Understand the exponent**
   - Why ~3.6 for interior points?
   - What geometric property causes this?

3. **Prove or disprove internal ordering bound**
   - Can a segment with m points have more than O(m^4) stable orderings?
   - Find explicit bound or counterexample

---

*Analysis updated: 2026-01-01*
*Key insight: Memory fixed → structure shared → bound may be real*
