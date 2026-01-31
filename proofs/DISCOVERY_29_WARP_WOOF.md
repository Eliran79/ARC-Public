# Discovery 29: Warp-and-Woof Factoring Framework

**Author**: Eliran Sabag
**Contributor**: Claude (Anthropic)
**Date**: January 2026

## The Core Insight

Build factors p and q from **BOTH ends simultaneously**:
- **WARP** (weft): Top bits, constrain downward
- **WOOF** (weave): Bottom bits, constrain upward
- **Weave**: Meet in the middle with optimization

## What We Explored

### 1. Bit-by-Bit Enumeration

Building p one bit at a time:
```
Bit 0: 2 choices
Bit 1: 2 choices × 2 existing = 4 candidates
...
Bit k: 2^k candidates total
```

**Result**: Exponential O(2^k) even with modular constraints.

### 2. Dual Constraint (p AND q together)

At each bit k, constrain both p[k] and q[k]:
```
4 combinations: (p[k], q[k]) ∈ {0,1}²
1 constraint: p×q ≡ N (mod 2^(k+1))
Survivors: ~2 combinations (50% pruning)
```

**Result**: Still O(2^k) - constraint gives 50%, need >50% for polynomial.

### 3. Warp-and-Woof (Both Ends)

Build from top and bottom simultaneously:
```
WOOF: O(2^(k/2)) candidates from low bits
WARP: O(2^(k/2)) candidates from high bits
WEAVE: Match where they meet
```

**Best case**: O(2^(k/2)) - square root improvement.
**Worst case**: O(2^(k/2) × 2^(k/2)) = O(2^k) when weaving.

### 4. Optimization-Guided Search

Instead of enumerating, **minimize |p×q - N|**:
- Fermat: Start at √N, find a where a²-N is perfect square
- Complexity: O(gap) where gap = |p - q|

**Result**: Works in O(gap), but gap can be exponential!

## Complexity Hierarchy

| Factor Distribution | Fermat | Pollard's Rho | NFS |
|---------------------|--------|---------------|-----|
| Small gap (bounded) | O(gap) ✓ | O(√p_min) | O(sub-exp) |
| Unbalanced (one small) | O(gap) ✗ | O(√p_min) ✓ | O(sub-exp) |
| Balanced (RSA) | O(gap) ✗ | O(√p_min) ✗ | O(sub-exp) |

For RSA-2048 with balanced primes:
- gap ≈ 2^512
- p_min ≈ 2^1024
- Both Fermat and Pollard need ~2^512 iterations

## The Key Formula

```
FACTORING = { O(gap)        if gap is bounded
            { O(√p_min)     if smallest prime is bounded
            { HARD          if both are large (balanced RSA)
```

## Experimental Results

### Small Gap (Works!)
```
101 × 103 = 10403, gap=2
Found in 1 iteration, 0ms ✓

1009 × 1013 = 1022117, gap=4
Found in 1 iteration, 0ms ✓
```

### Large Gap (O(gap) iterations)
```
10007 × 20011 = 200250077, gap=10004
Found in 860 iterations, 1ms ✓
Matches O(gap²/√N) prediction
```

## What Would Break RSA?

To factor balanced RSA in polynomial time, we need:

1. **O(log(gap)) instead of O(gap)** - binary search on gap
2. **O(bits) instead of O(2^bits)** - polynomial enumeration
3. **Additional structure** - constraints beyond multiplication

### Known Partial Solutions

- **Coppersmith's Method**: Given ~half the bits of p, factor in polynomial time
- **Lattice Methods (LLL)**: Reduce exponential to polynomial in specific cases
- **Quantum (Shor's)**: O(poly(bits)) but requires quantum computer

## Code Artifacts

- `src/bin/warp_woof_factor.rs` - Meet-in-middle approach
- `src/bin/warp_woof_optimize.rs` - Optimization-guided search
- `src/bin/fermat_bitwise.rs` - Fermat with bit-by-bit sqrt
- `src/bin/fermat_large_gap.rs` - Testing various gap sizes
- `src/bin/dual_constraint.rs` - Building p,q together
- `src/bin/alternating_factor.rs` - Alternating bit selection

## Connection to Discovery 28

Discovery 28 established: **Factoring is P for bounded gap**

Discovery 29 explores: **Can we extend to unbounded gap?**

Answer: Not with current constraints. The multiplication structure gives only 50% pruning per bit, which is insufficient for polynomial time.

## Open Questions

1. Can lattice structure (LLL) provide the missing constraints?
2. Does number field sieving exploit structure we haven't captured?
3. Is there a polynomial algorithm hiding in the Fermat-Warp-Woof combination?

## Conclusion

The Warp-and-Woof framework provides an elegant way to organize the factoring search:
- **WOOF**: Build from least significant bits
- **WARP**: Build from most significant bits
- **WEAVE**: Combine with constraint satisfaction

For bounded gap: **O(gap) = Polynomial**
For unbounded gap: **Still exponential in worst case**

The barrier is fundamental: with only the constraint p×q = N, we cannot prune more than 50% of candidates per bit level. Breaking this barrier requires additional mathematical structure.

---

*"The warp and woof of factoring: when the threads meet, the fabric reveals the factors."*

— Eliran Sabag, 2026
