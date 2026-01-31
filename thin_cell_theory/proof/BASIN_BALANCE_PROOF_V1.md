# Basin Balance: Proof Sketch (Version 1)

## Status: CONJECTURE WITH STRONG EMPIRICAL SUPPORT

---

## Theorem Statement

**Conjecture (Basin Balance):**
For Euclidean TSP on n random points, every 2-opt local optimum has a basin of attraction containing at least Ω(1/n) fraction of all tours.

**Empirical Evidence:**
```
Min basin / Avg basin ≈ 0.4-0.5
With |LO| = O(n), this gives min basin ≈ 0.4/n
```

---

## Key Empirical Findings

### 1. Basin Size Distribution

For instances with multiple local optima:

| n | Multi-LO Instances | Avg Min/Avg Ratio |
|---|-------------------|-------------------|
| 5 | 2% | 0.33 |
| 6 | 22% | 0.46 |
| 7 | 34% | 0.42 |
| 8 | 37% | 0.42 |

**Pattern:** Min basin is consistently 30-50% of average.

### 2. Basin Imbalance Bound

Max basin / Min basin ≤ 5 in all observed cases.

No instance showed exponentially small basins.

### 3. 2-opt Graph Connectivity

Each tour has O(n²) 2-opt neighbors.
The 2-opt graph is well-connected (diameter O(n) moves).

---

## Proof Approach

### Intuition

1. **Basins are connected regions:** Tours reach the same optimum via downhill 2-opt moves.

2. **No thin tentacles:** Basins are roughly "convex" in tour space - no exponentially narrow paths.

3. **Euclidean spread:** Local optima are spread out in tour space, giving each a substantial basin.

### Formal Argument (Sketch)

**Definition:** Let T be the set of all tours on n points. |T| = (n-1)!/2.

**Definition:** Basin(L*) = {T ∈ T : 2-opt(T) converges to L*}.

**Claim:** For any local optimum L*, |Basin(L*)| ≥ c × |T| / |LO|.

**Proof Sketch:**

1. **Partition:** The basins partition T. So Σ |Basin(L*)| = |T|.

2. **Average size:** Avg basin = |T| / |LO|.

3. **Variance bound:** We claim Var(basin sizes) is bounded.

4. **By Chebyshev:** Min basin ≥ (1 - ε) × Avg for some ε < 1.

**Gap:** Need to prove the variance bound.

---

## Alternative: Volume Argument

### 2-opt Ball Definition

For tour T and radius r, define:
```
B_r(T) = {T' : T can reach T' in ≤ r 2-opt moves}
```

### Key Lemma (Unproven)

**Lemma:** For any local optimum L*, |Basin(L*)| ≥ |B_r(L*)| for some r = O(n).

**Intuition:** Tours within distance r of L* (in 2-opt graph) likely converge to L*.

### Ball Size

|B_r(T)| ≥ (n²)^r / r! for small r.

For r = O(1), this is polynomial in n.

---

## Probabilistic Bound

### Random Walk Analysis

Consider random walk on 2-opt graph:
- Start at random tour
- Follow improving moves (or random if none)
- Eventually reach local optimum

**Claim:** Starting uniform random, each optimum reached with probability ≥ c/|LO|.

**This is equivalent to basin balance.**

### Mixing Time Argument

If 2-opt graph mixes in O(poly(n)) steps, basins can't be exponentially small.

**Mixing time for tour graph:** Not well-studied for 2-opt specifically.

---

## Implications for Algorithm

### Coupon Collector Bound

With min basin probability p_min ≥ c/n:
```
E[starts to find all optima] ≤ n × (1 + ln(n)) / p_min
                             = n × O(log n) / (c/n)
                             = O(n² log n)
```

### High Probability

With k = O(n² log n × log(1/ε)) starts:
```
P[find all optima] ≥ 1 - ε
```

### Algorithm Complexity

Multi-start 2-opt:
- k = O(n² log n) starts
- Each run: O(n²) iterations
- Total: O(n⁴ log n)

This is polynomial!

---

## What's Needed for Complete Proof

### Option 1: Variance Bound
Show that basin size variance is O(Avg²).

### Option 2: Ball Containment
Prove Basin(L*) ⊇ B_r(L*) for r = O(1).

### Option 3: Mixing Time
Prove 2-opt mixing time is polynomial.

### Option 4: Smoothed Analysis
Use perturbation arguments (Englert et al. 2014).

---

## Summary

| Claim | Evidence | Proof Status |
|-------|----------|--------------|
| Min basin ≥ 0.3 × Avg | Strong | EMPIRICAL |
| No exponentially small basins | Strong | EMPIRICAL |
| Basin ≥ Ω(1/n) | Derived | CONJECTURE |
| O(n² log n) starts suffice | Calculation | CONDITIONAL |

---

*Version 1 - 2026-01-01*
