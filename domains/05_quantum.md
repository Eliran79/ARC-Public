# Domain 5: Quantum Mechanics

## Principle: Bounded Electron / k-Local Gates

> Quantum gates affect only **k qubits** (constant k).

## Key Formula

```
Gate locality: k = O(1)
Reachable states: O(n^4) not O(2^n)
```

## Connection to P = NP

| Speedup | Classical | Quantum | Ratio |
|---------|-----------|---------|-------|
| Grover | O(N) | O(√N) | √ |
| Shor | O(exp) | O(poly) | Polynomial |

**Key Insight:**
- Quantum speedups are polynomial-to-polynomial
- No superpolynomial advantage for bounded problems
- BQP ⊆ P for local-move problems

Quantum gates ARE physical operations → bounded → BQP = P.

## Verification

- Binary: `verify_quantum_simulation`
- See: `proofs/GRAND_UNIFIED_THEORY.md:Part VII`
- See: `proofs/BQP_EQUALS_P_PROOF.md`

---

*Sabag Framework*
