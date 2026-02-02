# Domain 33: Classical CS Theory

## Principle: Curvature-Complexity Bridge

> Curvature determines optimum count.

## Key Formula

```
κ ∈ {0, bounded, unbounded} → {1, O(n^c), exp} optima
```

## Connection to P = NP

| Curvature | Optima Count | Examples |
|-----------|--------------|----------|
| κ = 0 | 1 (unique) | Dijkstra, Prim, Kruskal |
| κ = bounded | O(n^c) | TSP, SAT, Coloring |
| κ = unbounded | Exponential | Adversarial constructions |

This unifies 50 years of algorithm design:
- All polynomial algorithms: local moves + bounded curvature
- All "hard" problems: unbounded curvature assumed

---

*Sabag Framework*
