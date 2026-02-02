# Domain 14: Logic (Halting Problem)

## Principle: Bounded Computation

> S_observable is decidable; S_complete is not.

## Key Formula

```
D(P) = if HALT(P, P) then loop else halt
```

## Connection to P = NP

Turing's diagonal argument shows the Halting Problem is undecidable for **all** programs (S_complete).

But for **bounded** programs (S_observable):
- Bounded memory → finite states
- Finite states → cycle detection possible
- Cycle detection → halting decidable

| Space | Halting | Why |
|-------|---------|-----|
| S_complete | Undecidable | Unbounded computation |
| S_observable | Decidable | Bounded → finite states |

## Verification

- Binary: `halting_paradox`
- Result: 8 tests, D(D) paradox detected within bounds

---

*Sabag Framework*
