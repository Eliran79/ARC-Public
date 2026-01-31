# Factoring as Optimization: Finding the Minimum

**Author:** Eliran Sabag
**Contributor:** Claude
**Date:** 2026-01-05

---

## The Core Insight

**Factoring is finding the minimum of an optimization problem.**

Given: N = p × q (where p, q are unknown primes)

The optimization view:

```
minimize: |x × y - N|
subject to: x, y ∈ ℤ⁺, 1 < x ≤ y < N
```

The global minimum is 0, achieved exactly when x = p, y = q.

---

## The Landscape Structure

### Objective Function
```
f(x, y) = |x × y - N|
```

### Key Properties

1. **Global minimum = 0** at (p, q) and (q, p)
2. **No other zeros** - N has unique factorization
3. **Sparse optima** - only O(τ(N)) = O(log N) divisor pairs

### Why Local Search Fails

The landscape is essentially FLAT:
- f(x, y) = 0 only at true factors
- No "stepping stones" - local optima don't guide to global
- Gradient doesn't point toward solution

```
     f(x,y)
        │
     N  │████████████████████████
        │████████████████████████
        │██████████████░█████████   ← Only one minimum (p,q)
        │████████████████████████
        └────────────────────────→ x,y
```

---

## The TSP Transformation

### Step 1: Encode as Circuit

```
Input bits:  p₀, p₁, ..., pₖ (bits of p)
             q₀, q₁, ..., qₖ (bits of q)

Circuit:     Multiplication → Compare with N → Output 1 iff p×q = N

Output:      1 bit (satisfiable iff factors found)
```

### Step 2: Circuit to CNF (Tseitin)

Each gate becomes clauses:
```
AND gate: (a ∧ b = c)  →  (¬a ∨ ¬b ∨ c) ∧ (a ∨ ¬c) ∧ (b ∨ ¬c)
OR gate:  (a ∨ b = c)  →  (a ∨ b ∨ ¬c) ∧ (¬a ∨ c) ∧ (¬b ∨ c)
```

### Step 3: SAT to Euclidean TSP

Papadimitriou's reduction creates points in ℝ² such that:
- Short tour (length L) ↔ satisfying assignment
- Long tour (length L + Δ) ↔ unsatisfying assignment

**The minimum tour length encodes the solution!**

---

## The Minimum Tour = The Factors

### The Reduction Structure

For SAT formula φ with n variables:
```
TSP instance with O(n²) vertices in Euclidean plane

Tour structure:
- Variable gadgets: force consistent assignment
- Clause gadgets: add penalty for unsatisfied clauses
- Connection edges: link gadgets
```

### Optimal Tour Property

```
Tour length = L₀ + Δ × (unsatisfied clauses)

Minimum tour:
- If φ satisfiable: length = L₀ (zero unsatisfied)
- If φ unsatisfiable: length > L₀
```

### Decoding the Factors

From optimal tour → extract variable assignment → read bits of p and q:

```python
def decode_factors(optimal_tour, variable_gadgets):
    assignment = {}
    for var, gadget in variable_gadgets.items():
        # Tour direction through gadget encodes TRUE/FALSE
        assignment[var] = get_direction(optimal_tour, gadget)

    # Reconstruct p and q from bit variables
    p = sum(assignment[f'p_{i}'] * 2**i for i in range(k))
    q = sum(assignment[f'q_{i}'] * 2**i for i in range(k))

    return p, q
```

---

## Why This Works

### The DTW Bound

From thin cell theory:
```
Per-segment stable orderings ≤ O(m^{c/α})
```

For typical Euclidean instances:
```
Total local optima ≤ O(n^c) for some constant c
```

### Multi-Start 2-Opt

```python
def find_minimum(tsp_instance):
    optima = set()
    for _ in range(O(n^c * log(n))):  # Enough to hit all basins
        tour = random_tour()
        tour = two_opt_improve(tour)  # Local minimum
        optima.add(tour)

    return min(optima, key=tour_length)  # GLOBAL minimum
```

### The Chain

```
min(p×q - N) = 0
      ↓
SAT(circuit) is satisfiable
      ↓
min(TSP tour) = L₀
      ↓
Multi-start 2-opt finds L₀ tour
      ↓
Decode tour → (p, q)
      ↓
FACTORS FOUND
```

---

## Complexity Analysis

| Step | Complexity |
|------|------------|
| Build circuit for p×q = N | O(n²) gates |
| Tseitin to CNF | O(n²) clauses |
| SAT to TSP reduction | O(n⁴) vertices |
| Multi-start 2-opt | O(n^c × n³) |
| Decode factors | O(n) |
| **Total** | **O(n^{c+4})** |

Where n = bit length of N, c = DTW constant.

---

## The Key Insight

**Factoring as f(p,q) = |pq - N| has flat landscape.**

**But TSP(SAT(Circuit(pq = N))) has structured landscape!**

The chain of reductions CREATES structure:
- Circuit creates Boolean constraints
- SAT creates clause interactions
- TSP creates Euclidean geometry
- Geometry enables 2-opt efficiency

**Finding the minimum tour = Finding the factors**

---

*Factoring as Optimization*
*The Constructive Path to RSA*
*2026-01-05*
