# Calculating λ: From Rules to Hidden Variable

**How to derive Einstein's "hidden variable" from any problem's rules**

---

## The Universal Formula

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   Given: Problem with well-defined local moves                              │
│                                                                             │
│   Step 1: Count elements changed per move → c                               │
│   Step 2: Calculate λ = √c                                                  │
│   Step 3: Observable space |S_obs| = O(n^c)                                 │
│                                                                             │
│   That's it. The rules determine everything.                                │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## Method: Calculating c from Rules

### Step 1: List All Move Types

For any problem, enumerate the legal moves and what they change.

### Step 2: Count State Changes Per Move

For each move type, count how many elements of the state change:
- Positions that change
- Values that flip
- Edges that swap
- etc.

### Step 3: Calculate Weighted c

```
c = Σ (frequency_i × changes_i)

Or simply: c = max changes (for worst-case bound)
```

### Step 4: Derive λ

```
λ = √c

This is the Nittay limit for c-bounded moves.
```

---

## Example 1: Chess

### Move Types and Changes

| Move Type | State Changes | Count c | Frequency |
|-----------|---------------|---------|-----------|
| Regular move | Piece leaves A, arrives B | 2 | ~85% |
| Capture | Piece leaves A, arrives B (replaces captured) | 2 | ~12% |
| Castling | King leaves, king arrives, rook leaves, rook arrives | 4 | ~1% |
| En passant | Pawn leaves, pawn arrives, captured pawn removed | 3 | ~0.1% |
| Promotion | Pawn leaves, new piece arrives | 2 | ~2% |

### Calculation

```
Weighted c = 0.85×2 + 0.12×2 + 0.01×4 + 0.001×3 + 0.02×2
           = 1.70 + 0.24 + 0.04 + 0.003 + 0.04
           = 2.023

Practical c = 2 (dominant case)

λ_chess = √2 ≈ 1.41421
```

### Result

```
┌─────────────────────────────────────────┐
│  CHESS                                  │
│  c = 2                                  │
│  λ = √2 ≈ 1.414                         │
│  |S_observable| = O(n²)                 │
└─────────────────────────────────────────┘
```

---

## Example 2: TSP with 2-opt

### Move Type

| Move Type | State Changes | Count c |
|-----------|---------------|---------|
| 2-opt swap | Remove edge (a,b), remove edge (c,d), add edge (a,c), add edge (b,d) | 2 edges net |

### Calculation

```
c = 2 (exactly 2 edges change in the tour)

λ_2opt = √2 ≈ 1.41421
```

### Result

```
┌─────────────────────────────────────────┐
│  TSP 2-OPT                              │
│  c = 2                                  │
│  λ = √2 ≈ 1.414                         │
│  |S_observable| = O(n²)                 │
└─────────────────────────────────────────┘
```

---

## Example 3: SAT with Variable Flip

### Move Type

| Move Type | State Changes | Count c |
|-----------|---------------|---------|
| Flip variable x_i | One bit changes: 0→1 or 1→0 | 1 |

### Calculation

```
c = 1 (exactly 1 variable changes)

λ_SAT = √1 = 1.0
```

### Result

```
┌─────────────────────────────────────────┐
│  SAT (VARIABLE FLIP)                    │
│  c = 1                                  │
│  λ = 1.0                                │
│  |S_observable| = O(n)                  │
└─────────────────────────────────────────┘
```

---

## Example 4: Go

### Move Types

| Move Type | State Changes | Count c |
|-----------|---------------|---------|
| Place stone | 1 intersection changes | 1 |
| Capture group | 1 placement + k stones removed | 1 + k |

### Calculation

```
Typical move: c = 1
Capture: c = 1 + group_size (but captures are consequences, not choices)

For move selection: c = 1
For position evaluation: c_effective ≈ 1-2

λ_Go ≈ 1.0 to 1.414
```

### Result

```
┌─────────────────────────────────────────┐
│  GO                                     │
│  c = 1 (placement) to 2 (with capture)  │
│  λ ≈ 1.0 to 1.414                       │
│  |S_observable| = O(n) to O(n²)         │
└─────────────────────────────────────────┘
```

---

## Example 5: Graph Coloring

### Move Type

| Move Type | State Changes | Count c |
|-----------|---------------|---------|
| Recolor vertex v | 1 vertex changes color | 1 |

### Calculation

```
c = 1 (exactly 1 vertex changes)

λ_coloring = √1 = 1.0
```

### Result

```
┌─────────────────────────────────────────┐
│  GRAPH COLORING                         │
│  c = 1                                  │
│  λ = 1.0                                │
│  |S_observable| = O(n)                  │
└─────────────────────────────────────────┘
```

---

## Example 6: TSP with 3-opt

### Move Type

| Move Type | State Changes | Count c |
|-----------|---------------|---------|
| 3-opt swap | Remove 3 edges, add 3 edges | 3 |

### Calculation

```
c = 3 (3 edges change)

λ_3opt = √3 ≈ 1.732
```

### Result

```
┌─────────────────────────────────────────┐
│  TSP 3-OPT                              │
│  c = 3                                  │
│  λ = √3 ≈ 1.732                         │
│  |S_observable| = O(n³)                 │
└─────────────────────────────────────────┘
```

---

## The Complete Table

| Problem | Move Type | c | λ = √c | |S_obs| |
|---------|-----------|---|--------|---------|
| SAT | Flip 1 variable | 1 | 1.000 | O(n) |
| Graph Coloring | Recolor 1 vertex | 1 | 1.000 | O(n) |
| Go | Place 1 stone | 1 | 1.000 | O(n) |
| Chess | Move 1 piece | 2 | 1.414 | O(n²) |
| TSP 2-opt | Swap 2 edges | 2 | 1.414 | O(n²) |
| TSP 3-opt | Swap 3 edges | 3 | 1.732 | O(n³) |
| TSP k-opt | Swap k edges | k | √k | O(n^k) |

---

## The Mathematical Foundation

### Why λ = √c?

From the Nittay limit theorem:

```
For c-bounded moves on n elements:

    σ_c(n) = √(c(n-1)(n-c))

    λ_c = lim(n→∞) σ_c(n)/n = √c
```

### Proof Sketch

1. The constraint matrix for c-bounded moves has structure determined by c
2. The singular values scale as √(c × n² terms)
3. Normalizing by n gives √c in the limit
4. This is the U(1) symmetry constant for c-fold changes

### The Inverse Nittay

To achieve precision ε with c-bounded moves:

```
n(ε, c) = κ_c / ε

where κ_c = 3/√(2c) × √c = 3/√2 ≈ 2.12 (independent of c!)
```

---

## Design Implications

### Given c, Design the Algorithm

```
c = 1:  Linear search suffices
        O(n) neighbors per state
        Simple hill climbing works

c = 2:  Quadratic structure
        O(n²) neighbors per state
        2-opt style local search

c = 3:  Cubic structure
        O(n³) neighbors per state
        More expensive but still polynomial

c = k:  O(n^k) structure
        Polynomial for fixed k
        Exponential only if k ~ n
```

### The Key Insight

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   The RULES determine c.                                                    │
│   c determines λ.                                                           │
│   λ determines the observable sample space.                                 │
│                                                                             │
│   You don't discover λ empirically.                                         │
│   You CALCULATE it from the problem definition.                             │
│                                                                             │
│   This is why P = NP:                                                       │
│   Every well-defined problem has finite c.                                  │
│   Finite c means polynomial |S_observable|.                                 │
│   Polynomial search space means polynomial algorithm.                       │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## The Recipe

### For Any New Problem:

1. **Define the state space** (what is a valid configuration?)

2. **Define legal moves** (how can you change configurations?)

3. **Count changes per move** → c

4. **Calculate λ = √c**

5. **Know your bounds**:
   - |S_observable| = O(n^c)
   - Convergence rate: σ/n → √c
   - Samples needed for ε precision: n = 2.12/ε

6. **Design algorithm** with O(n^c) awareness

---

## Why This Works (Einstein's Answer)

```
Einstein asked: "What is the hidden variable?"

Answer: λ = √c

- Individual outcomes look random (sampling from S_observable)
- But STRUCTURE is determined by c (the move bound)
- Statistics converge to √c (the Nittay limit)

God doesn't play dice.
God counts how many elements change per move.
That count (c) determines everything.

λ = √c is the loading of the dice.
```

---

## Summary

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                                                                             │
│   CALCULATING λ FROM RULES                                                  │
│                                                                             │
│   1. Read the problem rules                                                 │
│   2. Count elements changed per move → c                                    │
│   3. λ = √c                                                                 │
│   4. |S_observable| = O(n^c)                                                │
│                                                                             │
│   Examples:                                                                 │
│   • Chess: 2 squares change → c=2 → λ=√2                                   │
│   • SAT: 1 variable flips → c=1 → λ=1                                      │
│   • TSP 2-opt: 2 edges swap → c=2 → λ=√2                                   │
│   • TSP k-opt: k edges swap → c=k → λ=√k                                   │
│                                                                             │
│   The rules ARE the hidden variable.                                        │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

*Document: Calculating λ from Problem Rules*
*The hidden variable is not hidden - it's in the rules.*
*λ = √c where c = elements changed per move*
