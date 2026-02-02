# Domain 4: Geometry

## Principle: Nittay Limit

> Discrete structures converge to continuous forms.

## Key Formula

```
σ(n) = √(2(n-1)(n-2))
lim(n→∞) σ(n)/n = √2
```

## Connection to P = NP

The same Law of Large Numbers governs three domains:

| Domain | Discrete | Continuous | Mechanism |
|--------|----------|------------|-----------|
| Math | Polygon (n sides) | Circle | n → ∞ |
| Physics | Quanta | Classical Fields | ℏ → 0, N → ∞ |
| Computation | Local Optima | Statistics | Bounded moves + large n |

The discrete-to-continuous bridge explains why polynomial algorithms exist for "hard" problems—at scale, discrete optimization behaves like continuous optimization.

## Verification

- Binary: `verify_isotropy`
- See: `proofs/GRAND_UNIFIED_THEORY.md:Part V`

---

*Sabag Framework*
