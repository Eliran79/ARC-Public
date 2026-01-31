# Intersection W1×G2: SampleSpace × 2-opt

## The Threads

**W1 (SampleSpace):** O(n) orthogonal vectors spanning observable combinations
**G2 (2-opt):** Local optimality - no edge swap improves tour

---

## Theory

### Claim

SampleSpace's "observable space collapse" parallels 2-opt's "local optima collapse."

### The Analogy

```
SampleSpace:
  Theoretical space: All 2^n subsets
  Observable space: O(n) actual combinations
  Collapse ratio: 2^n → O(n)

2-opt for TSP:
  Theoretical space: All n! tours
  Observable space: 2-opt stable tours (local optima)
  Collapse ratio: n! → ???
```

### The Question

Does |local optima| = O(n^c) for some constant c?

If yes, this would be the TSP analog of SampleSpace collapse.

---

## What's Good

1. **Correct analogy:** Both are about "observable << theoretical"
2. **Core insight preserved:** Constraints reduce the space
3. **Motivates the approach:** Look for geometric collapse

---

## What's Bad

1. **No mechanism:** SampleSpace collapse comes from data sparsity
   - 2-opt collapse would need geometric argument
   - These are different mechanisms

2. **No proof:** We observe few local optima empirically
   - But no theorem bounds them

3. **Different structures:**
   - SampleSpace: orthogonal basis (linear algebra)
   - 2-opt: swap graph (combinatorics)

### The Gap

SampleSpace collapse is PROVEN (for selection problems).
2-opt collapse is CONJECTURED (for TSP).

The analogy inspires but doesn't prove.

---

## Honest Assessment

| Aspect | Status |
|--------|--------|
| Analogy valid | Yes - both are space collapse |
| Mechanism same | **NO** - different reasons |
| Proof transfers | **NO** - different structures |
| Useful insight | Yes - motivates the search |

### Verdict

This intersection provides **INSPIRATION**, not proof.

The insight "observable << theoretical" is valuable.
But the MECHANISM for collapse must be found separately for TSP.

---

## The Real Question

For 2-opt on Euclidean TSP:

**Conjecture:** |LO(n)| = O(n^c) for some c.

Where LO(n) = number of 2-opt local optima on n points.

**Evidence:**
- Empirical: few optima observed
- No proof exists
- No counterexample known

**What would prove it:**
- Geometric argument (thin cells?)
- Counting argument (coupling?)
- Topological argument (basin structure?)

---

## Code Reference

```python
# The analogy in code form:

# SampleSpace (proven)
observable_combinations = compute_samplespace(data)  # O(n)
# This is PROVEN to be O(n)

# 2-opt (conjectured)
local_optima = find_all_2opt_optima(points)  # O(n^c)?
# This is CONJECTURED, not proven
```

---

*Documented: 2026-01-01*
