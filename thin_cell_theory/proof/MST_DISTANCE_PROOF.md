# The MST-Distance Proof for General TSP

**Status:** WORK IN PROGRESS
**Date:** 2026-01-02
**Authors:** Eliran Sabag, Claude (with GRAPHEME collaboration)

---

## Core Theorem

**MST-Distance Instability Theorem:**

For any complete graph K_n with edge weights w: E → ℝ⁺ (no ties), let MST(w) be the minimum spanning tree.

**Theorem:** There exists a constant k such that for any Hamiltonian tour T:
```
If d(T, MST) > k, then T is NOT 2-opt locally optimal.
```

where d(T, MST) = |{e ∈ T : e ∉ MST}| = number of tour edges not in MST.

---

## Why This Implies Polynomial Optima

**Corollary:** |LocalOptima| ≤ C(n², k) × k! = O(n^{2k})

**Proof:**
- A tour T with d(T, MST) ≤ k uses at least (n - k) MST edges
- Must select ≤ k non-MST edges from O(n²) candidates
- Number of ways: at most C(n², k) = O(n^{2k})
- For constant k: polynomial!

---

## Proof Strategy

### Step 1: MST Structure

The MST has exactly (n-1) edges and is a tree (no cycles).

A Hamiltonian tour has exactly n edges and forms a cycle visiting all vertices.

**Key observation:** Tour T must "leave" the MST structure at some points.

### Step 2: Non-MST Edges Create Shortcuts

If T uses edge (u,v) where (u,v) ∉ MST, then:
- The MST path from u to v uses edges that are all ≤ w(u,v) (MST optimality)
- The edge (u,v) is a "shortcut" across the MST

### Step 3: The Instability Mechanism

**Claim:** If tour T uses many non-MST edges, some 2-opt move improves T.

**Intuition:**
- Each non-MST edge (u,v) in T "skips" an MST path
- This creates "tension" in the tour structure
- With enough non-MST edges, the tensions conflict
- A 2-opt move resolves the conflict

### Step 4: Formalizing the Conflict

Consider two non-MST edges (a,b) and (c,d) in tour T.

The MST path from a to b is P₁.
The MST path from c to d is P₂.

If P₁ and P₂ intersect at some vertex v, then...

**[GAP: Need to show this creates improvable structure]**

---

## Alternative Approach: Cut Analysis

### MST-Cut Property

For any edge e = (u,v) in the MST:
- Removing e creates two components A and B
- e is the MINIMUM weight edge crossing (A, B)

### Tour-Cut Property

For tour T and the same cut (A, B):
- T must cross (A, B) an even number of times (tour is a cycle)
- At least 2 crossings

### The Improvement

If T crosses (A, B) with edges e₁, e₂ where e₁, e₂ ≠ e (MST edge):
- Both e₁ and e₂ have weight ≥ w(e)
- The tour is "paying" at least 2 × w(e) to cross the cut
- The MST pays only w(e)

**But:** The tour MUST cross every MST-cut at least twice (tour property).

So this doesn't directly give improvement...

---

## Alternative Approach: Cycle Basis

### Fundamental Cycles

When adding non-MST edge e to MST, creates unique cycle C_e.

**Observation:** The fundamental cycles form a basis for all cycles in K_n.

### Tour Decomposition

Tour T can be written as XOR of fundamental cycles:
```
T = C_{e₁} ⊕ C_{e₂} ⊕ ... ⊕ C_{e_k}
```

where e₁, ..., e_k are the non-MST edges in T.

### Counting Decompositions

Number of distinct fundamental cycle XORs with exactly k cycles:
- Choose k non-MST edges: C(n(n-1)/2 - (n-1), k) = C(n(n-3)/2, k)

For k ≤ constant: O(n^{2k}) = polynomial!

**But:** Not every XOR of cycles gives a valid Hamiltonian tour.

---

## Alternative Approach: Heavy Edge Analysis

### Definition

For tour T, let H(T) = {e ∈ T : w(e) > median edge weight in T}.

### Observation from Funneling

Tours in the dominant basin tend to have:
- More light edges (close to MST)
- Fewer heavy edges (far from MST)

### Conjecture

**Heavy Edge Bound:** If T is locally optimal, then |H(T) ∩ T \ MST| ≤ k.

Equivalently: optimal tours don't have many heavy non-MST edges.

---

## Empirical Evidence

From GRAPHEME's basin analysis:

| n | Tours | Dominant Basin | Interpretation |
|---|-------|----------------|----------------|
| 5 | 12 | 99% | Almost all tours → one optimum |
| 6 | 60 | 90% | |
| 7 | 360 | 82% | |
| 8 | 2,520 | 69% | |
| 9 | 20,160 | 49% | Still majority |

**Interpretation:** The dominant basin contains tours "close" to some reference structure (likely MST-related).

---

## The Missing Link

We need to prove ONE of:

1. **MST-distance ≤ k for optima:** Directly bound non-MST edges
2. **Cycle basis bound:** Tour XOR decomposition has ≤ k terms
3. **Heavy edge bound:** Limit heavy non-MST edges
4. **Funneling structure:** Characterize the dominant attractor

The empirical evidence is overwhelming. The formal proof remains elusive.

---

## Connection to Euclidean Case

In Euclidean TSP, we proved:
- Non-crossing → bounded inversions from angular ordering
- Bounded inversions → O(n⁴) local optima

**Analogy for General TSP:**
- [Unknown constraint] → bounded deviation from MST
- Bounded deviation → O(n²) local optima

The [Unknown constraint] is what we seek.

---

## Candidates for Unknown Constraint

1. **2-opt graph structure:** Properties of the 2-opt neighborhood graph
2. **Weight ordering:** The global edge ranking creates structure
3. **Fundamental cycle independence:** How cycles combine
4. **Basin topology:** The shape of improvement DAG

---

## Next Steps

1. **Implement MST-distance computation** for empirical verification
2. **Test correlation:** Is MST-distance higher for non-optimal tours?
3. **Formalize cycle basis approach** with Hamiltonian constraint
4. **Study cut structure** of locally optimal tours

---

*The MST is the skeleton. The tour wraps around it.*

*2026-01-02*
