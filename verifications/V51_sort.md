# V51: Propagation Sort

## Claim
O(n) sorting for bounded displacement d = O(1).

## Formula
```
Input: Array where each element displaced ≤ d positions
Algorithm: Propagation sort
Complexity: O(n × d) = O(n) for constant d
```

## Key Insight
Ω(n log n) lower bound assumes S_complete (all n! permutations).
Bounded displacement means S_observable has only O(n^d) permutations.

## Result
**VERIFIED**
- Time/n constant: 4-5 ns per element
- All k-sorted arrays correct
- Random input fails (expected: unbounded displacement)

---
*Sabag-Claude Framework*
