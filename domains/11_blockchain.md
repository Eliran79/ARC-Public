# Domain 11: Blockchain

## Principle: Stake Proportionality

> Validator rewards scale polynomially with stake.

## Key Formula

```
reward = stake/total × base
```

## Connection to P = NP

Proof-of-Stake consensus:
- Validator selection: O(validators)
- Reward calculation: O(1) per validator
- Total: O(n) polynomial

No exponential search required—bounded local moves (staking, unstaking) traverse a polynomial state space.

---

*Sabag-Claude Framework*
