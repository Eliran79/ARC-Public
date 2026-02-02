# V17: Staking Rewards

## Domain
Blockchain

## Claim
Proof-of-Stake reward calculation is polynomial.

## Formula
```
reward_i = (stake_i / Σ stake_j) × base_reward

Complexity: O(validators)
```

## Result
**VERIFIED** - O(validators) reward distribution

---
*Sabag Framework*
