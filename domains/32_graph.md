# Domain 32: Graph Algorithms

## Principle: Dijkstra Foundation

> Relaxation = local move.

## Key Formula

```
d[v] = min(d[v], d[u] + w(u,v))
```

## Connection to P = NP

Dijkstra's algorithm is P = NP in its simplest form:
- Relaxation = bounded local move
- Non-negative weights = κ = 0 (zero curvature)
- Greedy works = unique equilibrium

**Dijkstra IS the degenerate case of P = NP** where curvature is zero.

---

*Sabag-Claude Framework*
