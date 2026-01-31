# THE PARADOX: Actual Tours > Product of Segment Orderings

## The Data

| n | Actual stable tours | Product of segments | Ratio |
|---|---------------------|---------------------|-------|
| 6 | 1.1 | 1.0 | 1.1 |
| 7 | 1.3 | 1.0 | 1.3 |
| 8 | 1.7 | 1.0 | 1.7 |
| 9 | 1.6 | 1.2 | 1.5 |
| 10 | 2.2 | 1.2 | 1.9 |

**Paradox:** Actual > Product. This "shouldn't" happen if ROPE is a valid decomposition.

---

## Why Does This Happen?

### The ROPE Assumption

ROPE assumes: Every optimal tour visits hull vertices in order,
and the interior points within each segment can be analyzed independently.

### What We're Missing

**Not all 2-opt stable tours follow the ROPE structure!**

A tour can be 2-opt stable WITHOUT having interior points
grouped by their "assigned" segment.

### Example

Consider 6 points: 4 on hull (A, B, C, D) and 2 interior (P, Q).

ROPE assigns P to segment AB, Q to segment CD.

ROPE-compatible tours:
- A → P → B → C → Q → D → A

But a 2-opt stable tour might be:
- A → Q → B → C → P → D → A

Here Q is visited in segment AB's "territory" and P in CD's.
This tour doesn't fit ROPE decomposition but might still be stable!

---

## The Critical Error in ROPE

### ROPE Theorem Claims

"Any optimal tour visits hull vertices in cyclic order."

This is TRUE.

### But We Extended It Too Far

We assumed: "Interior points are visited within their assigned segment."

This is FALSE.

A stable tour might visit point P anywhere along the hull order,
not just between its "assigned" hull vertices.

---

## What This Means

### ROPE Decomposition Is Incomplete

ROPE captures SOME stable tours (those following segment structure).
But MISSES others (where interior points are visited "out of segment").

### The True Count

Actual stable tours = ROPE-compatible + non-ROPE-compatible

Since Actual > Product, there are non-trivial non-ROPE-compatible tours.

### Implications for the Proof

1. **ROPE alone doesn't bound the optima count**
2. **Need a different decomposition or approach**
3. **The "segment independence" argument fails**

---

## Re-Examining the Data

### What's Constant

- Hull order is fixed (proven)
- Non-crossing is required (proven)

### What's NOT Constant

- Point assignment to segments (not proven)
- Segment independence (FALSE)

### The Real Structure

A 2-opt stable tour on n points with k hull vertices:
- Visits hull in cyclic order
- Interior points can be ANYWHERE along the cycle
- Still must be non-crossing
- Still must be 2-opt stable

The constraint is WEAKER than ROPE assumed.

---

## New Bound Attempt

### Hull-Ordered Tours

A tour is hull-ordered if hull vertices appear in cyclic order.

Count of hull-ordered tours:
- Choose where to insert each interior point: k positions each
- k^(n-k) arrangements
- Minus those with crossings

For random points: k ≈ log n, so k^(n-k) is still HUGE.

### Non-Crossing Constraint

Most arrangements create crossings.
How many don't?

This is related to the number of simple polygons on n points.
Known: Ω(4.64^n) to O(56^n).

Still exponential.

### 2-opt Stability

Among non-crossing hull-ordered tours, how many are 2-opt stable?

This is what limits the count to polynomial.

---

## The Real Constraint

**Theorem (to prove):** Among hull-ordered non-crossing tours,
the number of 2-opt stable ones is O(n^c).

This would be the REAL theorem, not ROPE decomposition.

### Why Might This Hold?

2-opt stability + non-crossing + hull-order together
create strong geometric constraints.

Each additional point has limited "stable positions" in the tour.

---

## Updated Proof Strategy

### Old Strategy (ROPE-based, FAILS)

```
1. Decompose into segments
2. Bound orderings per segment
3. Multiply to get total
→ FAILS because tours aren't segment-decomposable
```

### New Strategy (Direct)

```
1. Fix hull order (no choice here)
2. Insert interior points one by one
3. Each point has O(1) stable positions
4. Total: O(n) × O(1)^(n-k) = polynomial
```

### The Key Claim

**Conjecture:** For each interior point, there are O(1) positions
where it can be inserted while maintaining 2-opt stability.

If this holds: |LO(n)| = O(k × 1^{n-k}) = O(log n) per starting config.

But we observe more optima than that...

---

## What We Actually Observe

| n | Hull k | Interior n-k | Stable tours | Tours/(n-k) |
|---|--------|--------------|--------------|-------------|
| 6 | 4.6 | 1.4 | 1.1 | 0.79 |
| 7 | 5.1 | 1.9 | 1.3 | 0.68 |
| 8 | 5.8 | 2.2 | 1.7 | 0.77 |
| 9 | 6.0 | 3.0 | 1.6 | 0.53 |
| 10 | 6.0 | 4.0 | 2.2 | 0.55 |

Tours per interior point is roughly O(1).

This suggests: **O(n-k) = O(n) stable tours?**

But earlier large-n tests showed n^4-5 growth. Contradiction?

---

## Resolving the Contradiction

### Small n: Few optima (~O(n))

| n | Optima | Optima/n |
|---|--------|----------|
| 6 | 1.1 | 0.18 |
| 10 | 2.2 | 0.22 |

### Large n: Many optima (~O(n^4-5))

| n | Optima | Optima/n |
|---|--------|----------|
| 20 | 32.5 | 1.6 |
| 30 | 356 | 11.9 |

### The Transition

Something changes between n=10 and n=20.

Possibilities:
1. Multi-start misses optima for small n
2. Structure changes as n grows
3. The exponent decreases for small n

---

## Conclusion

### What We Learned

1. **ROPE decomposition is INCOMPLETE** - misses non-segment-structured tours
2. **Actual > Product** - segments aren't the right unit of analysis
3. **Small n: optima ≈ O(n)** - but this changes at larger n

### What We Need

A proof that works WITHOUT segment decomposition.

Direct argument: 2-opt stability + non-crossing + hull-order → polynomial count.

---

*Paradox documented: 2026-01-01*
*ROPE-based approach needs revision*
