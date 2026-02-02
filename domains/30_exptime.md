# Domain 30: EXPTIME Collapse

## Principle: State Space Bound

> s = poly(n) → PSPACE

## Key Formula

```
P = NP(c) = PSPACE(d) = EXPTIME(s)
```

## Connection to P = NP

Three bounding parameters:
1. **c** (moves): Bounds NP
2. **d** (depth): Bounds PSPACE
3. **s** (state bits): Bounds EXPTIME

Example: Generalized chess
- Standard 8×8 board: PSPACE (bounded s)
- n×n board: EXPTIME (exponential s)

Board size is the third bounding parameter.

---

*Sabag Framework*
