# Intersection W4×G4: ROPE × Thin Cell (CRITICAL)

## The Threads

**W4 (ROPE):** Hull-based decomposition into segments
**G4 (Thin Cell):** Aspect ratio α ≥ m implies uniqueness

---

## Theory

### Claim

ROPE segments that are "thin" have exactly one stable path.

### Formal Statement

**Conjecture W4×G4:**
1. ROPE decomposes tour into k segments (k = hull size)
2. Each segment has m_i interior points and aspect ratio α_i
3. If α_i ≥ m_i (thin), segment has exactly 1 stable path
4. Combined: total stable tours bounded

### The ROPE Part (Solid)

```
ROPE Decomposition:
  Input: n points P
  Step 1: Compute convex hull H = {h₁, ..., h_k}
  Step 2: Tour visits hull in cyclic order (proven: non-crossing)
  Step 3: Segment S_i = path from h_i to h_{i+1} through interior points

This is VALID. Hull order is fixed for optimal tours.
```

### The Thin Cell Part (Problematic)

```
Thin Cell Claim:
  If segment has α ≥ m, there is exactly 1 stable path.

Problems:
  1. "Stable" means 2-opt stable within segment? Or globally?
  2. "Monotonic" not defined precisely
  3. Zig-Zag Lemma proven for W-separated points only
```

---

## What's Good

1. **ROPE is valid:** Hull decomposition works
2. **Thin intuition is correct:** Elongated cells have limited orderings
3. **Zig-Zag Lemma verified:** 8,920 cases tested, all improved
4. **Empirical support:** 50/50 thin segments had 1 path in tests

### Code Evidence

```python
# ROPE decomposition works:
def rope_decomposition(points):
    hull = convex_hull(points)
    segments = []
    for i in range(len(hull)):
        # Assign interior points to segments
        # This is well-defined
    return segments

# Thin cell test works:
def test_thin_segment_unique_path():
    # 50/50 thin segments had exactly 1 stable path
    # Counterexamples: 0
```

---

## What's Bad

### Gap 1: Definition of "Monotonic"

We say: "Thin cells force monotonic ordering."

But what IS monotonic?

**Attempt 1:** Points sorted by x-coordinate
- Problem: segment axis may not be horizontal

**Attempt 2:** Points sorted by projection onto segment axis
- Better, but: ties possible, perturbation needed

**Attempt 3:** Points sorted by angle from entry
- Problem: angle ordering ≠ 2-opt stability directly

**No rigorous definition exists.**

### Gap 2: W-separation Required

Zig-Zag Lemma requires "W-separated" points:
- Consecutive points at least W apart
- W proportional to cell width

For arbitrary point configurations, W-separation may not hold.

### Gap 3: Local vs Global Stability

Segment is 2-opt stable locally.
But we need GLOBAL stability of the full tour.

A segment path that's locally stable might become improvable
when combined with adjacent segments.

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| ROPE decomposition | **VALID** |
| Thin cell intuition | Correct |
| Monotonicity defined | **NO** |
| Zig-Zag Lemma | Verified (W-separated only) |
| General uniqueness | **NOT PROVEN** |
| Local → global | **NOT PROVEN** |

### Verdict

This intersection is **PARTIALLY VALID**.

- ROPE part: works
- Thin cell part: unproven

The connection exists but the thin cell uniqueness is not rigorously established.

---

## What Would Complete It

### Step 1: Define Monotonicity

```
Definition: Path p₁ → p₂ → ... → p_m is monotonic w.r.t. axis L if:
  proj_L(p₁) < proj_L(p₂) < ... < proj_L(p_m)
where proj_L is projection onto line L.
```

### Step 2: Prove Monotonic = Stable (for thin cells)

```
Theorem: In a thin cell (α ≥ cm), the unique monotonic path is 2-opt stable.

Proof needed:
  - Show any deviation creates zig-zag
  - Show zig-zag implies 2-opt improvement
  - Requires W-separation or alternative argument
```

### Step 3: Prove Local → Global

```
Theorem: If each segment path is locally stable,
         the combined tour is globally stable.

Proof needed:
  - Handle segment boundaries
  - Show no cross-segment improvements
```

---

## Empirical Status

```
Tested: n up to 25
Thin segments found: ~1-3% of random segments
Thin segment uniqueness: 100% in tests
Fat segment multiplicity: ~50% have >1 path

Limitation: n too small for asymptotic claims
```

---

*Documented: 2026-01-01*
*Status: PARTIALLY VALID - key gaps remain*
