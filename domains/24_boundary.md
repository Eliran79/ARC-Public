# Domain 24: Computational Boundary

## Principle: Structure vs Anti-Structure

> SHA destroys locality; RSA preserves it.

## Key Formula

```
SHA: avalanche destroys locality → intractable
RSA: preserves structure → tractable
```

## Connection to P = NP

| Algorithm | Structure | Tractability |
|-----------|-----------|--------------|
| SHA-256 | Anti-local (avalanche) | Preimage intractable |
| RSA | Preserves multiplicative | Factoring tractable |

**Critical insight:** SHA is intentionally designed to destroy locality. This makes it resistant even to P=NP.

**Banks are safe** because they use both:
- RSA for key exchange (structure exploitable but keys are bit-random)
- SHA for hashing (anti-structure, inherently hard)

---

*Sabag Framework*
