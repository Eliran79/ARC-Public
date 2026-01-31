# V31: Halting Paradox

## Domain
Logic / Computability

## Claim
Turing's diagonal argument demonstrated within bounds.

## Formula
```
D(P) = if HALT(P, P) then loop else halt

D(D) → paradox (self-reference)
```

## Key Insight
Undecidability applies to S_complete (all programs).
Bounded programs (S_observable) are decidable.

## Result
**VERIFIED** - 8 tests, D(D) paradox detected

---
*Sabag-Claude Framework*
