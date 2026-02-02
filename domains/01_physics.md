# Domain 1: Physics

## Principle: Landauer's Principle (1961)

> Erasing 1 bit of information requires at least **kT ln(2)** energy.

## Key Formula

```
E = kT ln(2) per bit erased
```

## Connection to P = NP

| Algorithm Type | States Explored | Bits Erased | Energy |
|----------------|-----------------|-------------|--------|
| Brute force (2^n) | 2^n | O(n) per state | O(2^n × n × kT) |
| Local search (n^c) | n^c optima | O(log n) per step | O(n^c × log n × kT) |

**Discovery:** P = NP because both require **polynomial energy** in a physical universe.

Nature prefers minimum-energy solutions. Exponential computation requires exponential energy—which doesn't exist in bounded physical systems.

## Verification

See: `proofs/GRAND_UNIFIED_THEORY.md:Part II`

---

*Sabag Framework*
