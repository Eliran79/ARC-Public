# Domain 9: Configuration Space

## Principle: Observable Sample Space Lemma

> The core lemma of the entire framework.

## Key Formula

```
S_observable << S_complete

|S_complete| = O(k^n)     exponential
|S_observable| = O(n^c)   polynomial
```

## Connection to P = NP

This IS the P = NP insight:

| Space | Definition | Size | Searchable? |
|-------|------------|------|-------------|
| S_complete | All syntactically valid states | O(2^n) | No |
| S_observable | States reachable via bounded moves | O(n^c) | **Yes** |

NP-hard problems are only hard when searching S_complete. Constrained to S_observable, they become polynomial.

**The entire framework reduces to this single distinction.**

## Verification

- See: `proofs/OBSERVABLE_SAMPLE_SPACE_LEMMA.md`
- All other domains are applications of this lemma

---

*Sabag Framework*
