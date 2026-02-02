# Domain 35: Flow Networks

## Principle: Ford-Fulkerson Foundation

> Augmenting path = local move on residual graph.

## Key Formula

```
Augmenting path = bounded move
Residual = bound
```

## Connection to P = NP

Max flow is polynomial because:
- Augmenting path is a bounded local move
- Residual graph constrains search space
- Flow conservation → κ = 0

Ford-Fulkerson IS Dijkstra for flow networks.

---

*Sabag Framework*
