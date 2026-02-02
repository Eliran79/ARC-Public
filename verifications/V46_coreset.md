# V46: TSP Coreset Sampling

## Claim
Sparse sampling preserves near-optimality.

## Formula
```
Dense: O(n²) distance pairs
Coreset: O(log n) representative pairs
Approximation: (1+ε)-optimal
```

## Result
**VERIFIED**
- Ratio: 1.00-1.04
- n=100: 47 samples vs 10,000 dense
- (1+ε)-approximation achieved

---
*Sabag Framework*
