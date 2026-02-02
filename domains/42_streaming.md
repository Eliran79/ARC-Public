# Domain 42: Streaming Data

## Principle: Bounded Displacement

> Ω(n log n) bound assumes adversarial input.

## Key Formula

```
disp(A) ≤ d → O(n) sort
```

## Connection to P = NP

Classical sorting lower bound: Ω(n log n) comparisons.

**But this assumes S_complete** (all n! permutations possible).

For bounded displacement d (S_observable):
- Only O(n^d) permutations reachable
- O(n) sorting via propagation
- Applications: time-series, log aggregation, sensor fusion

## Verification

- Binary: `sparse_propagate_sort`
- Result: Time/n constant (4-5 ns) for d = O(1)

---

*Sabag Framework*
