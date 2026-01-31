# Domain 10: Cryptography

## Principle: UTXO Verification

> Verification is polynomial even when generation is hard.

## Key Formula

```
S_utxo = O(n)           verification space
S_hash = O(2^256)       preimage space
```

## Connection to P = NP

| Operation | Complexity | Space |
|-----------|------------|-------|
| Verify signature | O(1) | S_observable |
| Forge signature | O(2^256) | S_complete |
| Verify block | O(n) | S_observable |
| Mine block | O(2^d) | S_complete |

**Critical:** P = NP does NOT break cryptography because:
- Verification uses S_observable (polynomial)
- Breaking requires S_complete (bit-level randomness)
- Bit-level randomness is Kolmogorov-incompressible

**Banks are safe. Bitcoin is safe.**

## Verification

- See: `proofs/PATH_20_TWO_RANDOMNESS_THEOREM.md`

---

*Sabag-Claude Framework*
