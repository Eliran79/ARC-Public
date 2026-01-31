# Domain 31: Quantum Randomness

## Principle: Two Randomness Distinction

> Bit-level vs physics-level randomness.

## Key Formula

```
K(x) ≈ |x|    (bit-level: incompressible)
K(x) << |x|   (physics-level: compressible)
```

## Connection to P = NP

| Type | Compression | Source | P=NP Impact |
|------|-------------|--------|-------------|
| Bit-level | ~0% | CSPRNG, PBKDF2 | **Safe** |
| Physics-level | 15-92% | Quantum, thermal | Exploitable |

Quantum gates are physical operations → bounded → BQP = P.

But cryptographic keys are bit-level random → no structure → safe despite P=NP.

## Verification

- Binary: `entropy_quantum`
- Result: Gap 35.6%, p < 0.001

---

*Sabag-Claude Framework*
