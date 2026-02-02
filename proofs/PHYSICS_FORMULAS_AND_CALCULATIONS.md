# Physics Formulas and Calculations for Bounded Sorting

**Author:** Eliran Sabag
**Date:** 2026-01-31
**Status:** REFERENCE DOCUMENT
**Framework Version:** Discovery 98

---

## I. LANDAUER'S PRINCIPLE: DETAILED CALCULATIONS

### 1.1 Basic Formula

**Landauer's Principle:**
```
E = k_B × T × ln(2) × (number of bits erased)

where:
  k_B = Boltzmann constant = 1.380649 × 10^-23 J/K
  T = absolute temperature in Kelvin
  ln(2) ≈ 0.693147

Energy per bit at 300K:
  E_bit = 1.38 × 10^-23 J/K × 300 K × 0.693
        ≈ 2.87 × 10^-21 joules
        ≈ 2.87 zeptojoules
```

### 1.2 Sorting Information Requirements

**Adversarial Input (n! permutations):**

```
Information to distinguish n! permutations:
I_adversarial = log₂(n!) bits

Stirling's approximation:
ln(n!) ≈ n ln(n) - n + 0.5 ln(2πn)
log₂(n!) = ln(n!) / ln(2)
         ≈ (n ln(n) - n) / ln(2)
         ≈ 1.443 × n × (ln(n) - 1)

For specific values:
  n=10:  log₂(10!) ≈ 33.2 bits
  n=100: log₂(100!) ≈ 525.8 bits
  n=1000: log₂(1000!) ≈ 8528.6 bits
  n=10^6: log₂((10^6)!) ≈ 1.93 × 10^7 bits
```

**Bounded Displacement Input (D(n,d) permutations):**

```
For d-bounded permutations where d = O(1):

Upper bound on D(n,d):
D(n,d) ≤ (2d+1)^n

Proof:
  - Element i can be in any position [i-d, i+d]
  - For each element: at most (2d+1) choices
  - Total: (2d+1)^n permutations

Information needed:
I_bounded = log₂(D(n,d))
          ≤ log₂((2d+1)^n)
          = n × log₂(2d+1)

For specific values (d=3):
  I_bounded = n × log₂(7) ≈ 2.807 × n bits

Ratio for n=10^6, d=3:
  I_adversarial / I_bounded = 1.93×10^7 / (2.807 × 10^6)
                            ≈ 6.87×
```

### 1.3 Energy Calculations

**Energy Dissipated: Adversarial**

```
E_adversarial = log₂(n!) × k_B × T × ln(2) joules

At T = 300K (room temperature):
E_bit = 1.38 × 10^-23 × 300 × 0.693 = 2.87 × 10^-21 J

For n = 10^6:
I_adversarial ≈ 1.93 × 10^7 bits
E_adversarial = 1.93 × 10^7 × 2.87 × 10^-21 J
              ≈ 5.5 × 10^-14 joules
              ≈ 55 femtojoules

Per element:
E_per_element = 5.5 × 10^-14 J / 10^6
              ≈ 5.5 × 10^-20 joules
              ≈ 55 attojoules
```

**Energy Dissipated: Bounded (d=3)**

```
E_bounded = n × log₂(2d+1) × k_B × T × ln(2)
          = 10^6 × log₂(7) × 2.87 × 10^-21 J
          = 10^6 × 2.807 × 2.87 × 10^-21 J
          ≈ 8.1 × 10^-15 joules
          ≈ 8.1 femtojoules

Per element:
E_per_element = 8.1 × 10^-15 J / 10^6
              ≈ 8.1 × 10^-21 joules
              ≈ 8.1 zeptojoules
```

**Energy Reduction Ratio**

```
Ratio = E_adversarial / E_bounded
      = (1.93 × 10^7 bits) / (2.807 × 10^6 bits)
      ≈ 6.87×

Meaning: Sorting 10^6 elements with bounded displacement
requires ~7× less energy dissipation than adversarial sorting.
```

### 1.4 Practical Implications

**Memory Access Energy:**

```
Modern DRAM access energy: ~100-1000 pJ per bit

For adversarial sorting (n = 10^6):
  Thermodynamic minimum: 55 fJ
  Actual DRAM: ~100 pJ per access

  Ratio: 100 pJ / 55 fJ ≈ 1.8 × 10^6×

Huge gap between physics limit and actual hardware!
(Hardware dissipates ~2 million times the thermodynamic minimum)

For bounded sorting (d=3):
  Thermodynamic minimum: 8 fJ
  Ratio: 100 pJ / 8 fJ ≈ 1.25 × 10^7×

Still far from limit, but structure helps reduce the gap.
```

---

## II. ENTROPY CALCULATIONS

### 2.1 Shannon Entropy

**Adversarial:**
```
H_adversarial = log₂(n!) bits
              ≈ n log₂(n) - n log₂(e) bits
              ≈ 1.443 × n × (ln(n) - 1) bits

For n = 10^6:
H_adversarial ≈ 1.93 × 10^7 bits
              ≈ 1.93 × 10^7 × 8 bits
              ≈ 1.54 × 10^8 bits
              ≈ 19.25 megabytes of information
```

**Bounded (d=3):**
```
H_bounded = n × log₂(2d+1) bits
          = 10^6 × log₂(7) bits
          = 10^6 × 2.807 bits
          ≈ 2.807 megabits
          ≈ 0.351 megabytes of information

Ratio: 19.25 MB / 0.351 MB ≈ 54.9×
```

### 2.2 Thermodynamic Entropy

**Boltzmann Entropy:**

```
S = k_B × ln(Ω)

where Ω = number of microstates

Adversarial:
S_adversarial = k_B × ln(n!)
              ≈ k_B × (n ln(n) - n)

At T = 300K, n = 10^6:
S_adversarial ≈ 1.38 × 10^-23 × (10^6 × ln(10^6) - 10^6)
              ≈ 1.38 × 10^-23 × (10^6 × 13.816 - 10^6)
              ≈ 1.38 × 10^-23 × (13.816 × 10^6 - 10^6)
              ≈ 1.38 × 10^-23 × 12.816 × 10^6
              ≈ 1.77 × 10^-16 J/K
```

**Bounded (d=3):**
```
S_bounded = k_B × ln(D(n,d))
          ≈ k_B × ln((2d+1)^n)
          = k_B × n × ln(2d+1)

For d=3:
S_bounded = 1.38 × 10^-23 × 10^6 × ln(7)
          = 1.38 × 10^-23 × 10^6 × 1.946
          ≈ 2.69 × 10^-17 J/K

Ratio: S_adversarial / S_bounded ≈ 6.6×
```

### 2.3 Entropy Reduction During Sorting

**For Adversarial Input:**

```
Initial: Random permutation, S_i ≈ k_B ln(n!)
Final: Sorted array, S_f = 0

ΔS = S_f - S_i = -k_B ln(n!) ≈ -k_B (n ln(n) - n)

At T = 300K, n = 10^6:
ΔS ≈ -1.77 × 10^-16 J/K

Heat generated:
Q = T × |ΔS| ≈ 300 × 1.77 × 10^-16
  ≈ 5.3 × 10^-14 joules (matches Landauer calculation!)
```

**For Bounded Input (d=3):**

```
Initial: d-bounded permutation, S_i ≈ k_B ln((2d+1)^n)
Final: Sorted array, S_f = 0

ΔS = -k_B × n × ln(2d+1)
   = -1.38 × 10^-23 × 10^6 × ln(7)
   ≈ -2.69 × 10^-17 J/K

Heat generated:
Q = T × |ΔS| ≈ 300 × 2.69 × 10^-17
  ≈ 8.1 × 10^-15 joules

Entropy reduction ratio:
|ΔS_adversarial| / |ΔS_bounded| ≈ 6.6×
```

---

## III. PERMUTATION COUNTING

### 3.1 Bounded Permutation Enumeration

**Definition (d-Bounded Permutation):**
```
A permutation σ where |σ(i) - i| ≤ d for all i

Meaning: Element i is within distance d of its correct position.
```

**Counting Formula (Upper Bound):**

```
For d-bounded permutations of n elements:

D(n,d) ≤ (2d+1)^n

Reasoning:
  - Position i can contain elements from [i-d, i+d]
  - That's 2d+1 choices
  - Each of n positions: (2d+1)^n choices
  - This overcounts (not all combinations are valid)

Tighter bounds (Bona & Zare, 2012):
  D(n,1) = 2n - 1
  D(n,2) ≤ 6n² + 2n + 1 (approximately)
  D(n,d) ≈ Θ(n^(2d+1))

For d = O(1): D(n,d) = O(n^c) for constant c
```

**Examples:**

```
n=10, d=1:
  D(10,1) = 2×10 - 1 = 19 permutations
  vs n! = 3,628,800
  Reduction: 191,000×

n=10, d=2:
  D(10,2) ≤ 6×100 + 20 + 1 = 621 permutations
  vs n! = 3,628,800
  Reduction: 5,800×

n=100, d=3:
  D(100,3) ≤ 7^100 ≈ 3.2 × 10^84 permutations
  vs 100! ≈ 9.3 × 10^157
  Reduction: 2.9 × 10^73×
```

### 3.2 Information Content

```
Information to specify arbitrary permutation:
I_full = log₂(n!) ≈ n log₂(n) bits

Information to specify d-bounded permutation:
I_bounded = log₂(D(n,d))
          ≤ log₂((2d+1)^n)
          = n × log₂(2d+1) bits

Savings:
ΔI = I_full - I_bounded
   ≈ n log₂(n) - n log₂(2d+1)
   = n × (log₂(n) - log₂(2d+1))
   = n × log₂(n / (2d+1))

For n=10^6, d=3:
ΔI = 10^6 × log₂(10^6 / 7)
   = 10^6 × log₂(142857)
   = 10^6 × 17.124 bits
   ≈ 1.71 × 10^7 bits saved
   ≈ 2.1 megabytes saved!
```

---

## IV. DIFFUSION AND RANDOM WALK

### 4.1 Random Walk in 1D

**Discrete Random Walk:**
```
Position at time t: X_t = X_0 + Σ ε_i
where ε_i ∈ {-1, +1} with equal probability

Expected displacement:
E[|X_t|] = √(2t/π) for large t

Variance:
Var(X_t) = t

Standard deviation:
σ(t) = √t
```

**Continuous Diffusion:**

```
Diffusion equation: ∂u/∂t = D ∂²u/∂x²

Solution (point source at origin):
u(x,t) = (1/√(4πDt)) × exp(-x²/(4Dt))

RMS displacement:
x_rms = √(2Dt)

At various times (D = 10^-5 cm²/s, typical liquid):
  t = 0.1 ms: x_rms ≈ 1.4 μm
  t = 1 ms:   x_rms ≈ 4.5 μm
  t = 10 ms:  x_rms ≈ 14 μm
  t = 1 s:    x_rms ≈ 140 μm
  t = 10 s:   x_rms ≈ 450 μm
```

### 4.2 Bounded Motion from Bounded Initial Displacement

**Theorem:**
If particles start within distance d of correct positions and undergo diffusion for time T:

```
Maximum displacement at time T:
d(T) = d_0 + √(2DT)

where d_0 = initial displacement bound

For bounded displacement to remain bounded:
d(T) ≤ d_0 + √(2DT) = O(1)

This requires:
√(2DT) = O(1)
T = O(1) (constant time)

Implication: Bounded motion creates bounded initial condition
for next algorithm phase!
```

### 4.3 Reversal Time

**To reverse diffusion of displacement d:**

```
Diffusion created: σ ~ √(D × T_diffuse)

So if d ~ √(D × T_diffuse), then:
T_diffuse ~ d² / D

To reverse via gradient (annealing):
- Energy to overcome: E ~ kT ln(2) × # bits
- bits to flip: log₂(# states) ~ log₂(states) ~ n log d
- Work: W ~ n × d (propagation sort)
- Time: T_reverse ~ n × d / velocity

Ratio:
T_reverse / T_diffuse ~ (n × d × D) / d²
                      ~ (n × D) / d

For n = 10^6, D = 10^-5 cm²/s, d = 1 cm:
T_reverse / T_diffuse ~ (10^6 × 10^-5) / 1 = 10
```

---

## V. CRYSTALLOGRAPHY: DEFECT ENERGETICS

### 5.1 Point Defect Formation Energy

**Vacancy Formation:**

```
E_v ≈ 0.5-2 eV (typical metals and ionic crystals)

Examples:
  Copper: E_v ≈ 1.1 eV
  Silver: E_v ≈ 1.14 eV
  Aluminum: E_v ≈ 0.76 eV
  NaCl: E_v ≈ 2.3 eV

At equilibrium:
  N_v / N = exp(-E_v / (k_B T))

At T = 300K (k_B T ≈ 0.026 eV):
  N_v / N = exp(-E_v / 0.026)

For Cu (E_v = 1.1 eV):
  N_v / N = exp(-1.1 / 0.026) ≈ exp(-42) ≈ 10^-18
  (virtually zero at room temperature!)

At T = 900K (k_B T ≈ 0.078 eV):
  N_v / N = exp(-1.1 / 0.078) ≈ exp(-14) ≈ 10^-6
  (significant concentration)
```

**Interstitial Formation:**

```
E_i ≈ 3-5 eV (typically 2-3× higher than vacancy)

Frenkel pair (vacancy + interstitial):
E_Frenkel ≈ E_v + E_i ≈ 4-7 eV
```

### 5.2 Defect Migration Energy

**Activation Energy for Motion:**

```
Mechanism: Atom hops to adjacent site

Migration energy E_m:
E_m ≈ 0.1-0.5 eV (typically lower than formation!)

Reason: Atom in saddle point needs less energy
because neighboring atoms help.

Examples:
  Cu vacancy migration: E_m ≈ 0.7 eV
  Cu self-diffusion: E_v + E_m ≈ 1.1 + 0.7 = 1.8 eV
  Al self-diffusion: E_v + E_m ≈ 0.76 + 0.55 = 1.31 eV
```

**Migration Time:**

```
Attempt frequency: ν ≈ 10^12 - 10^13 Hz (lattice vibration)

Migration time constant:
τ_m = (1/ν) × exp(E_m / k_B T)

At T = 300K:
τ_m = 10^-12 s × exp(E_m / 0.026)

For E_m = 0.7 eV:
τ_m = 10^-12 s × exp(27) ≈ 10^-12 s × 5×10^11 ≈ 500 seconds

For E_m = 0.2 eV:
τ_m = 10^-12 s × exp(7.7) ≈ 10^-12 s × 2200 ≈ 2 μs
```

**Temperature Dependence:**

```
τ_m(T) = τ_m(300K) × exp((E_m/k_B) × (1/T - 1/300K))

At T = 600K:
τ_m(600) = 500 s × exp(0.7/0.026 × (1/600 - 1/300))
         = 500 s × exp(27 × (-0.0056))
         = 500 s × exp(-0.15)
         = 500 s × 0.86
         ≈ 430 s (minimal change at T=600K!)

At T = 1000K (k_B T ≈ 0.086 eV):
τ_m(1000) = 10^-12 s × exp(0.7 / 0.086)
          = 10^-12 s × exp(8.14)
          = 10^-12 s × 3400
          ≈ 3.4 ns (much faster!)
```

### 5.3 Dislocation Motion

**Line Defect Velocity:**

```
Dislocation velocity:
v_disl = B × (τ - τ_c) / η

where:
  B = dislocation mobility
  τ = applied stress
  τ_c = critical stress (Peierls stress)
  η = viscous drag coefficient

Under controlled annealing:
v_disl ~ 1-10 nm/s (for small driving force)

Time to remove dislocation across distance d:
T_disl = d / v_disl ~ 100-1000 seconds (for d = 1 μm)

But: much faster than vacancy migration at same T!
Reason: Dislocation motion involves many atoms cooperatively
```

---

## VI. ANNEALING SCHEDULES

### 6.1 Linear Annealing

**Temperature schedule:**
```
T(t) = T_0 × (1 - t / T_total)

At t = 0: T = T_0
At t = T_total: T = 0

Cooling rate: |dT/dt| = T_0 / T_total
```

**Energy relaxation:**
```
For system with energy barriers {E_i}:

Time for barrier E to be overcome:
t_E ≈ τ_0 × exp(E / k_B T(t))

For linear cooling to reach T where k_B T ~ E/2:
t_E ~ polynomial(E, T_0, cooling_rate)

Critical slowing down avoided by slow enough cooling!
```

### 6.2 Logarithmic Annealing (Simulated Annealing)

**Schedule:**
```
T(k) = T_0 / ln(1 + k)

or equivalently:

T(k) = T_0 / ln(k) for k > 1
```

**Acceptance Probability:**
```
P_accept(ΔE) = min(1, exp(-ΔE / T(k)))

Move cost: ΔE (positive for energy increase)

For bounded problems (ΔE ≤ E_max = O(1)):
  P_accept(k) ~ exp(-O(1) / T(k))

Total probability to accept all moves:
  P_total = ∏_k P_accept(k)

For d-bounded problem:
  # moves = O(n × d)
  P_accept(k) ≥ constant for k ≤ K (K polynomial)
  After K moves: ∏_k P_accept(k) ≥ constant^(n×d)
```

### 6.3 Exponential Annealing

**Schedule:**
```
T(t) = T_0 × exp(-t / τ_cool)

Cooling time constant: τ_cool

Faster than logarithmic, but risks "freezing in" defects
```

---

## VII. WORKED EXAMPLES

### Example 1: Sorting 10^6 Elements with d=3

**Problem:** Sort array where each element is at most 3 positions from correct spot.

**Computation:**

```
Standard sort (comparison-based):
  Time: O(n log n) = 10^6 × log₂(10^6) ≈ 10^6 × 20 ≈ 2 × 10^7 operations
  Comparisons: ~20 million

Propagation sort (for bounded input):
  Time: O(n × d) = 10^6 × 3 = 3 × 10^6 operations
  Speedup: 6.67×

Information theory:
  Adversarial: log₂(10^6!) ≈ 1.93 × 10^7 bits
  Bounded (d=3): log₂((2×3+1)^10^6) = 10^6 × log₂(7) ≈ 2.81 × 10^6 bits
  Information ratio: 6.87×

Energy (Landauer principle at 300K):
  Adversarial: 1.93 × 10^7 bits × 2.87 × 10^-21 J/bit ≈ 5.5 × 10^-14 J
  Bounded: 2.81 × 10^6 bits × 2.87 × 10^-21 J/bit ≈ 8.1 × 10^-15 J
  Energy ratio: 6.8×

Entropy (at 300K):
  ΔS_adversarial = -k_B ln(10^6!) ≈ -1.77 × 10^-16 J/K
  ΔS_bounded = -k_B × 10^6 × ln(7) ≈ -2.69 × 10^-17 J/K
  Entropy ratio: 6.6×
```

**Physical Analogy (Crystallography):**

```
Defects: 3-point displacement = Frenkel pair at each position

Defect energy: E_defect ≈ E_v + E_i ≈ 3 eV

Annealing temperature to overcome:
  Probability ≈ exp(-3 eV / (k_B T))
  At T = 600K: P ~ exp(-3 / 0.052) ~ exp(-58) ~ 10^-25 (no motion)
  At T = 1200K: P ~ exp(-3 / 0.103) ~ exp(-29) ~ 10^-13 (slight motion)
  At T = 1500K: P ~ exp(-3 / 0.129) ~ exp(-23) ~ 10^-10 (heating!)

For d=3 (not too severe): Annealing at T = 900K takes ~100 seconds
To remove 10^6 defects: Total time ~ 10 seconds (parallel removal)

This matches O(n × d) = O(3 × 10^6) operations!
```

### Example 2: Protein Folding with Displacement Bound

**Biological Context:**

```
Protein with 300 amino acids
Native (folded) state: Some configuration C_native

Misfolded state: Each amino acid at most 5 positions from native
  (5-residue displacement: local structural error)

State space:
  All sequences: 20^300 ≈ 10^390
  With displacement ≤ 5: D(300, 5) ≈ (2×5+1)^300 ≈ 11^300 ≈ 10^313

Information to fold to native:
  Adversarial: log₂(20^300) ≈ 300 × log₂(20) ≈ 300 × 4.32 ≈ 1296 bits
  Bounded: log₂(11^300) = 300 × log₂(11) ≈ 300 × 3.46 ≈ 1038 bits
  Savings: 258 bits
```

**Why Proteins Fold Fast:**

```
Observation: Proteins fold in milliseconds to seconds
If they needed to search all 20^300 configurations: time ≈ 10^370 age of universe

Explanation:
  - Native state creates attractive basin
  - Other residues already mostly correct
  - Local Gibbs free energy funnel
  - Displacement from folded path: d = O(1) residues

Energy landscape:
  - Arbitrary misfolded: ~1296 bits to specify
  - Partially folded (d≤5): ~1038 bits to specify
  - Native: 0 bits (no specification needed)

Folding kinetics:
  - Search bounded subspace: D(300, 5) ≈ 10^313
  - With thermal noise: folding time ≈ poly(300, d=5)
  - Observed: milliseconds to seconds ✓ MATCHES
```

---

## VIII. PHYSICAL LIMITS AND ENGINEERING

### 8.1 Computing Power and Heat Dissipation

**Thermal Design Power (TDP):**

```
Modern CPU (Intel Core i9): TDP ≈ 125-253 watts
  Number of operations: ~10^9 - 10^10 per second
  Energy per operation: 125 W / 10^10 ops = 12.5 nanojoules

Landauer minimum for n log n bits (n = 10^9):
  Bits: 10^9 × log₂(10^9) ≈ 10^9 × 30 ≈ 3 × 10^10 bits
  Energy: 3 × 10^10 × 2.87 × 10^-21 J ≈ 86 picojoules per sort
  Total per sort: 3 × 10^10 × 2.87 × 10^-21 J / 10^9 ≈ 86 nJ per element sorted

Ratio: 12.5 nJ (actual) / 86 pJ (minimum) ≈ 145×

For bounded problem (d=3):
  Energy: 10^9 × log₂(7) × 2.87 × 10^-21 J ≈ 8.1 pJ per element
  Ratio: 12.5 nJ / 8.1 pJ ≈ 1500×
```

**Heat Dissipation in Server:**

```
Data center sorting 1 petabyte (10^15 bytes = 10^15 elements)

Adversarial sort:
  Energy: 10^15 × 5.5 × 10^-20 J = 5.5 × 10^-5 J = 55 μJ
  At CPU speed: 55 μJ / (10^10 ops/s) ≈ 5.5 microseconds
  Heat in room: dissipate 125 W for 5.5 μs ≈ 68 × 10^-6 joules ≈ 0.68 mJ

Bounded sort (d=3):
  Energy: 10^15 × 8.1 × 10^-21 J = 8.1 × 10^-6 J = 8.1 μJ
  Time: 8.1 μJ / (10^10 ops/s) ≈ 0.81 microseconds
  Heat: 68× less
```

### 8.2 Quantum Computing Advantage

**For d-Bounded Problems:**

```
Classical: O(n × d) operations
Quantum: O(√(n × d)) operations (Grover search on solution space)

For n = 10^6, d = 3:
  Classical: 3 × 10^6 operations
  Quantum: √(3 × 10^6) ≈ 1732 operations
  Speedup: 1732×

BUT: For adversarial:
  Classical: 10^6 × 20 = 2 × 10^7 operations
  Quantum: √(2 × 10^7) ≈ 4472 operations
  Speedup: 4472×

So quantum helps less for bounded problems (they're already fast!)
```

---

## IX. SUMMARY TABLE: ENERGY AND ENTROPY

| Aspect | Adversarial (n=10^6) | Bounded (d=3) | Ratio |
|--------|----------------------|---------------|-------|
| **Information** | 1.93 × 10^7 bits | 2.81 × 10^6 bits | 6.87× |
| **Entropy ΔS** | 1.77 × 10^-16 J/K | 2.69 × 10^-17 J/K | 6.6× |
| **Landauer Energy** | 5.5 × 10^-14 J | 8.1 × 10^-15 J | 6.8× |
| **Time (ops)** | 2 × 10^7 | 3 × 10^6 | 6.67× |
| **Heat dissipated** | ~68 μJ | ~10 μJ | 6.8× |

---

**Conclusion:** Bounded structure reduces computational energy by ~7× for n=10^6. For larger n, the ratio grows logarithmically.

