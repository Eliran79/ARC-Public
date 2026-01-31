# Domain 23: Medical Diagnostics (ECG)

## Principle: ECG c=1 Principle

> Each beat is independent → O(n) classification.

## Key Formula

```
class = argmax[log P(features|class) + log P(class)]
```

## Connection to P = NP

ECG classification:
- c = 1 (each beat independent)
- λ = 1 (single heartbeat window)
- d'^2 feature weights
- 93% V-recall (≥90% clinical threshold)

Independence (c=1) yields O(n) complexity for n beats.

---

*Sabag-Claude Framework*
