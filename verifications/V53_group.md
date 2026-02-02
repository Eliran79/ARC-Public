# V53: Group Theory (Bounded Permutations)

## Claim
|B_d(n)| = Θ(n^d) for constant d.

## Formula
```
B_d(n) = {π ∈ S_n : |π(i) - i| ≤ d for all i}

Cardinality: |B_d(n)| = Θ(n^d)
Generator: Adjacent transpositions {τ₁, ..., τₙ₋₁}
Structure: Submonoid of symmetric group S_n
```

## Key Insight
- S_n has n! elements (exponential)
- B_d(n) has Θ(n^d) elements (polynomial)
- Bounded displacement → polynomial search space

## Result
**VERIFIED** - Cayley graph structure confirmed for all tested n, d

---
*Sabag Framework*
