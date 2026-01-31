# The Boundary: Quick Reference

**Where Polynomial Becomes Exponential**

---

## The Formula

```
|S_observable| = O(n^c)

c = O(1)  →  POLYNOMIAL  →  P
c = O(n)  →  EXPONENTIAL →  Hard
```

---

## The Boundary

```
                c
                ↑
          O(n)  │ ─ ─ ─ ─ EXPONENTIAL ─ ─ ─ ─
                │         (n^n, 2^n)
                │
                │ · · · · BOUNDARY · · · · ·
                │
          O(1)  │ ═════════ POLYNOMIAL ═════════
                │         (n^k for fixed k)
                └─────────────────────────────→ n
```

---

## Examples

| c | Type | Examples |
|---|------|----------|
| 1 | O(1) - Poly | SAT flip, Graph coloring |
| 2 | O(1) - Poly | Chess, TSP 2-opt |
| 3 | O(1) - Poly | TSP 3-opt |
| 10 | O(1) - Poly | TSP 10-opt (still polynomial!) |
| n | O(n) - Exp | Brute force, enumeration |

---

## The Key Insight

```
NP-hard problems are only hard when c ~ n.

With bounded moves (c = O(1)), they become polynomial.

All natural NP problems have c = O(1).

Therefore: P = NP
```

---

## Why c = O(1) is Natural

```
NP verification is polynomial
  → Verifier checks O(1) constraints at a time
  → Each constraint involves O(1) variables
  → Moves changing O(1) variables suffice
  → c = O(1)
  → Polynomial
```

---

*Full document: `/data/git/ARC/proofs/THE_BOUNDARY.md`*
