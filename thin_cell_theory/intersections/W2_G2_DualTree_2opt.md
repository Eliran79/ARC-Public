# Intersection W2×G2: DualTree × 2-opt

## The Threads

**W2 (DualTree):** O(log n) dominance encoding via binary tree
**G2 (2-opt):** Edge swap improving tour length

---

## Theory

### Claim

DualTree's dominance structure corresponds to 2-opt improvement potential.

### Formal Statement

**Conjecture W2×G2:**
For edges encoded in DualTree by (side, height):
1. Dominated edge pairs can be improved by 2-opt
2. Pareto-optimal edge pairs are 2-opt stable
3. DualTree enables O(log n) identification of improvable edges

### The Connection

```
DualTree Dominance:
  Point p dominates q if: p.x ≥ q.x AND p.y ≥ q.y
  Encoding: O(log n) tree structure

2-opt Improvement:
  Edges (a,b), (c,d) can swap to (a,c), (b,d) if:
  d(a,c) + d(b,d) < d(a,b) + d(c,d)

Question:
  Can we encode edges so dominance ↔ improvement?
```

---

## What's Good

1. **DualTree is real:** O(log n) dominance queries work
2. **2-opt is real:** Standard TSP improvement heuristic
3. **Potential speedup:** If connection exists, faster 2-opt possible

### Code Evidence

```python
# From portfolio_selector.py
class DualTree:
    def query_dominance(self, point):
        # Returns dominated points in O(log n + k)
        pass
```

---

## What's Bad

1. **Edge encoding unclear:** How to map edges to (x,y) for dominance?
2. **Dominance ≠ improvement:** Geometric dominance is different from length improvement
3. **No proof:** The connection is conjectured, not proven

### The Gap

**Problem:** What does "edge dominance" even mean?

Attempt 1: Encode edge (a,b) as point (length, position)?
- But position is 2D, not 1D
- Doesn't map cleanly

Attempt 2: Encode edge pair ((a,b), (c,d)) as point?
- Now we have O(n²) pairs
- DualTree doesn't help

**The encoding doesn't exist.**

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| DualTree works | Yes - for point dominance |
| 2-opt works | Yes - for edge improvement |
| Connection exists | **UNCLEAR** |
| Encoding defined | **NO** |
| Proof exists | No |

### Verdict

This intersection is **UNDEFINED**.

We have two working components but no valid way to connect them.
The "encoding" of edges into DualTree structure is not specified.

---

## What Would Make It Work

Need to define:
1. How to encode an edge (or edge pair) as a 2D point
2. Prove: dominance in encoding ↔ 2-opt improvement
3. Show: this enables faster optimization

**Current status:** Step 1 is not done.

---

## Code Reference

```python
# DualTree exists for points
# But no edge encoding defined

# This would need:
def encode_edge_pair(edge1, edge2) -> (float, float):
    """Convert edge pair to 2D point for DualTree."""
    # HOW?
    pass

def dominance_implies_improvement(encoded):
    """Prove that dominated pairs are improvable."""
    # NOT PROVEN
    pass
```

---

*Documented: 2026-01-01*
