# Domain 21: Project Management

## Principle: TaskGuard Bounded Workflow

> N! task orderings → O(N²) via bounded status transitions.

## Key Formula

```
States(5) × Priority(4) × DAG(N)
```

## Connection to P = NP

Traditional project management: N! possible task orderings.

TaskGuard bounded workflow:
- 5 status states: todo → doing → review → done (or blocked)
- 4 priority levels
- DAG dependency structure

Bounded transitions reduce N! to O(N²).

---

*Sabag Framework*
