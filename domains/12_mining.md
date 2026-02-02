# Domain 12: Mining

## Principle: Share Difficulty

> Verification is O(1), mining is O(2^d).

## Key Formula

```
H(block || nonce) < target
```

## Connection to P = NP

| Operation | Complexity | Why |
|-----------|------------|-----|
| Verify share | O(1) | One hash comparison |
| Find share | O(2^d) | Brute force search |

Mining pools exploit this asymmetry:
- Miners search S_complete (exponential work)
- Pool verifies in S_observable (O(1) per share)

This is P vs NP in action—verification is polynomial, search appears exponential.

---

*Sabag Framework*
