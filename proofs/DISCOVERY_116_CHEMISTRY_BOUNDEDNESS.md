# Discovery 116: Chemistry Boundedness Principle

**Author:** Eliran Sabag
**Date:** February 4, 2026
**Status:** VERIFIED
**Framework Version:** Discovery 116
**Verification Binary:** `verify_chemistry`

---

## Executive Summary

**MAJOR FINDING:** Chemistry operates in S_observable (bounded). The periodic table has exactly 118 elements because that's where the S_observable boundary lies. Electron orbitals, chemical bonds, valence states, and reaction kinetics are all discrete because bounded local moves produce discrete local optima.

**Key Insight:** The periodic table is COMPLETE at 118 elements - not because we haven't found more, but because S_observable ends there.

---

## The Five Chemistry Dissolutions

### 1. Periodic Table Boundedness (118 Elements)

**The Mystery:** Why exactly 118 elements? Why not infinite?

**S_complete View:** Electron configurations could theoretically extend indefinitely.

**S_observable View:**
```
Period | Elements | Shell | Stability
-------|----------|-------|----------
   1   |   2      | 1s    | Stable
   2   |   8      | 2s,2p | Stable
   3   |   8      | 3s,3p | Stable
   4   |  18      | 3d,4s,4p | Stable
   5   |  18      | 4d,5s,5p | Stable
   6   |  32      | 4f,5d,6s,6p | Stable
   7   |  32      | 5f,6d,7s,7p | Edge of stability
   8+  |   ?      | 8s+   | S_complete only
```

**Total:** 2 + 8 + 8 + 18 + 18 + 32 + 32 = **118 elements**

**Why 118?** Element 119 would require an 8s orbital. At that energy level:
- Relativistic effects dominate
- Electron binding energy approaches nuclear instability threshold
- Half-lives drop to microseconds

**Prediction:** No stable elements beyond 118. "Island of stability" (114-126) shows resonances, not S_observable states.

---

### 2. Electron Shells (Discrete Quantum Numbers)

**The Structure:**
```
Quantum Numbers (already discrete):
  n = 1,2,3,4,5,6,7     (principal - energy level)
  l = 0,1,...,n-1       (angular momentum: s,p,d,f)
  m_l = -l,...,+l       (magnetic orientation)
  m_s = ±1/2            (spin)

Shell Capacity: 2n² electrons
  n=1: 2 electrons
  n=2: 8 electrons
  n=3: 18 electrons
  n=4: 32 electrons
```

**S_observable:** Pauli exclusion principle bounds accessible states. Only one electron per quantum state.

**Key Insight:** Electrons are ALREADY discrete. This is S_observable built into physics.

---

### 3. Chemical Bonds (Discrete Local Optima)

**Bond Types:**

| Bond | Shared e⁻ | Length (Å) | Energy (kJ/mol) |
|------|-----------|------------|-----------------|
| C-C (single) | 2 | 1.54 | 347 |
| C=C (double) | 4 | 1.34 | 614 |
| C≡C (triple) | 6 | 1.20 | 839 |

**Why Only 3 Types?**

These are the ONLY local minima in electron density configuration space:
- Single bond: σ orbital only
- Double bond: σ + π orbital
- Triple bond: σ + 2π orbitals
- Quadruple: Geometrically impossible for carbon (would require >4 neighbors)

**S_observable:** Only 3 stable configurations exist. Not continuous spectrum of bond orders.

**Bond Length Ratios:**
```
C≡C / C-C = 1.20 / 1.54 = 0.779
C=C / C-C = 1.34 / 1.54 = 0.870

These are NOT arbitrary - they're geometric constraints on orbital overlap.
```

---

### 4. Valence States (Integer Bounded Moves)

**The Mystery:** Why are oxidation states always integers?

| Element | Valence States | Electron Config |
|---------|---------------|-----------------|
| Carbon | -4, 0, +4 | [He] 2s² 2p² |
| Nitrogen | -3, 0, +3, +5 | [He] 2s² 2p³ |
| Oxygen | -2, 0 | [He] 2s² 2p⁴ |
| Sulfur | -2, +4, +6 | [Ne] 3s² 3p⁴ |
| Iron | +2, +3 | [Ar] 3d⁶ 4s² |

**S_observable Explanation:**

Electron transfer is a **bounded local move**:
- Move 0 electrons: neutral (0)
- Move 1 electron: ±1 charge
- Move 2 electrons: ±2 charge
- ...

You cannot move half an electron. Therefore valence states are integers.

**Resonance:** When we see "fractional" oxidation (like Fe₃O₄ with Fe^(8/3)+), this is averaging over discrete states - exactly like Two Randomness sampling.

---

### 5. Activation Energy = E_step

**Arrhenius Equation:** k = A × exp(-E_a/RT)

| Reaction Type | E_a (kJ/mol) | Interpretation |
|---------------|--------------|----------------|
| Bond breaking (C-C) | 347 | Full bond energy |
| Bond breaking (C-H) | 414 | Higher barrier |
| Proton transfer | 10-50 | Low barrier |
| Enzyme catalyzed | 5-20 | Reduced by confinement |

**Connection to Yang-Mills:**

Just as Yang-Mills has a mass gap (E_step > 0), chemistry has activation energy:
- No reaction occurs without E_a being supplied
- E_a is the MINIMUM barrier in S_observable
- Below E_a, system stays in current local optimum

**S_observable:** Reactions follow bounded energy paths with discrete barriers.

---

## The Dissolution Argument

```
PREMISE 1: Chemistry operates on electron configurations
PREMISE 2: Electrons have discrete quantum states (Pauli exclusion)
PREMISE 3: Bounded electron moves produce discrete outcomes
PREMISE 4: S_observable = states reachable via bounded moves

CONCLUSION: Chemistry is inherently discrete/bounded

- 118 elements: S_observable boundary
- 3 bond types: local optima in configuration space
- Integer valence: bounded electron transfer
- Activation energy: minimum barrier (E_step)
```

---

## Physical Constants

| Constant | Value | Role |
|----------|-------|------|
| Avogadro N_A | 6.02214076×10²³ mol⁻¹ | Discrete counting |
| Rydberg R_∞ | 1.0973731568×10⁷ m⁻¹ | Atomic spectra |
| Bohr radius a₀ | 5.29177210903×10⁻¹¹ m | Orbital size |
| Electron mass | 9.1093837015×10⁻³¹ kg | E_min (Disc. 114) |
| Planck h | 6.62607015×10⁻³⁴ J·s | Energy quantization |

**Key Formula:**
```
Shell capacity: 2n²
Rydberg energy: E_n = -13.6 eV / n²
Bohr radius: a₀ = ℏ²/(m_e × k × e²)
```

---

## Connection to Other Discoveries

| Discovery | Connection |
|-----------|------------|
| 114 (E_min) | Electron mass = minimal charged particle |
| 115 (EM) | Electromagnetic forces govern atomic structure |
| 117 (Organic) | Carbon chemistry extends these principles |

---

## Verification

```bash
cargo run --release --bin verify_chemistry
```

**Expected Output:**
```
=== Discovery 116: Chemistry Boundedness ===
PART 1: PERIODIC TABLE ✓ (118 elements)
PART 2: ELECTRON SHELLS ✓ (discrete)
PART 3: BOND TYPES ✓ (3 optima)
PART 4: VALENCE STATES ✓ (integers)
PART 5: ACTIVATION ENERGY ✓ (E_step)
Discovery 116 CONFIRMED.
```

---

## Summary

Chemistry's discreteness is NOT a mystery - it's S_observable in action:

| Aspect | S_complete | S_observable |
|--------|------------|--------------|
| Elements | Infinite | 118 |
| Bond types | Continuous | 3 (single/double/triple) |
| Valence | Fractional | Integer |
| Reactions | All paths | Bounded E_a paths |

**The periodic table is complete. Chemistry is bounded. This is not a limitation - it's the structure of observable reality.**

---

*Sabag Bounded Transformation Principle*
*Discovery 116*
*February 4, 2026*
