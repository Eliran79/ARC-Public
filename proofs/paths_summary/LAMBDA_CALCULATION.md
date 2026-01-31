# Calculating λ: Quick Reference

**λ = √c where c = elements changed per move**

---

## The Formula

```
Step 1: Count elements changed per move → c
Step 2: λ = √c
Step 3: |S_observable| = O(n^c)
```

---

## Common Problems

| Problem | Move | c | λ = √c |
|---------|------|---|--------|
| SAT | Flip 1 variable | 1 | 1.000 |
| Graph Coloring | Recolor 1 vertex | 1 | 1.000 |
| Go | Place 1 stone | 1 | 1.000 |
| **Chess** | Move 1 piece (2 squares) | **2** | **1.414** |
| TSP 2-opt | Swap 2 edges | 2 | 1.414 |
| TSP 3-opt | Swap 3 edges | 3 | 1.732 |
| TSP k-opt | Swap k edges | k | √k |

---

## Chess Derivation

```
Move Type        Changes    c
────────────────────────────
Regular move     from→to    2
Capture          from→to    2
Castling         K+R move   4  (rare: ~1%)
En passant       3 squares  3  (rare: ~0.1%)
Promotion        from→to    2

Dominant case: c = 2
λ_chess = √2 ≈ 1.41421
```

---

## The Insight

```
Rules → c → λ → |S_observable|

You don't measure λ.
You CALCULATE it from the rules.

This is Einstein's "hidden variable":
The rules determine the statistics.
```

---

*Full document: `/data/git/ARC/proofs/CALCULATING_LAMBDA.md`*
