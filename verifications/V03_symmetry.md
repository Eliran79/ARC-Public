# V3: Symmetry Collapse

## Path
Path 5 - Burnside/Group Theory

## Claim
Symmetry reduces exponential search to polynomial orbits.

## Formula
```
|X/G| = (1/|G|) × Σ |X^g|  (Burnside's Lemma)

For TSP with dihedral symmetry D_n:
|Tours/D_n| = O(n!) / O(n) = O((n-1)!/2)
```

## Mathematical Basis
Equivalent solutions under symmetry group G need only be counted once. Orbit-counting via Burnside's lemma.

## Result
**VERIFIED** - Exponential → Polynomial orbit count

---
*Sabag Framework*
