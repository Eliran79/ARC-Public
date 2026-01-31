# Structuring Continuous Problems: Quick Reference

**Continuous → Discrete via Inverse Nittay**

---

## The Recipe

```
1. Choose precision ε (acceptable error)
2. Quantize: n = ⌈2.12/ε⌉ bins per dimension
3. Count changes per move → c
4. λ = √c, |S_obs| = O(n^c) = O((1/ε)^c)
```

---

## Inverse Nittay Table

| ε | n = 2.12/ε | Application |
|---|------------|-------------|
| 0.1 | 22 | Audio/phonemes |
| 0.01 | 212 | Fine control |
| 0.004 | 530 | 8-bit images |
| 0.001 | 2,122 | Scientific |

---

## Examples

| Domain | ε | n | c | Polynomial? |
|--------|---|---|---|-------------|
| Audio | 0.1 | 22 | 1 | ✓ O(n) |
| Images (3×3 conv) | 1/256 | 256 | 9 | ✓ O(n⁹) |
| Neural Net (SGD) | 10⁻³ | 2122 | O(1)* | ✓ sparse |
| High-dim (d~n) | any | n | d~n | ✗ Exponential |

*Sparse gradients make c = O(1) despite many parameters

---

## Key Insight

```
Polynomial when: c = O(1) AND dimensions d = O(1)
Exponential when: c = O(n) OR d = O(n)

Curse of dimensionality = d scaling with n
Solution: Reduce dimensions first, then quantize
```

---

## The Pattern

```
Continuous (infinite)
      ↓
Inverse Nittay: n = 2.12/ε
      ↓
Discrete (n bins)
      ↓
Count changes → c
      ↓
λ = √c
      ↓
|S_obs| = O(n^c)
      ↓
Polynomial if c = O(1)
```

---

*Full document: `/data/git/ARC/proofs/STRUCTURING_CONTINUOUS_PROBLEMS.md`*
