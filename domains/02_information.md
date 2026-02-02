# Domain 2: Information Theory

## Principle: Shannon Entropy

> Entropy measures the information content of a probability distribution.

## Key Formula

```
H = -Σ p(x) log p(x)
```

## Connection to P = NP

High compression ratio → Low entropy → Polynomial structure

| Problem | State Entropy | Optima Entropy | Compression |
|---------|---------------|----------------|-------------|
| TSP n=7 | 17.47 bits | 5.29 bits | **69.7%** |
| SAT n=9 | 9.00 bits | 5.70 bits | **36.7%** |

Local optima are **describable** in O(log n) bits:
> "Run local search from seed X"

This is why NP problems are tractable—the solutions are compressible.

## Verification

- Binary: `verify_entropy`
- See: `proofs/GRAND_UNIFIED_THEORY.md:Part III`

---

*Sabag Framework*
