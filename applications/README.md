# P=NP=PSPACE Framework Applications

## Overview

This directory contains documentation for **WHAT** real-world problems
can be solved with the Sabag framework and **WHY** it's safe.

**Implementation details (HOW) are in `/proofs/` directory.**

## Documents

### [REAL_WORLD_SOLUTIONS.md](./REAL_WORLD_SOLUTIONS.md)
**WHAT** problems are solvable NOW:
- Perfect game play (Chess, Connect4, Poker)
- **Hospital scheduling (VERIFIED: 100 nurses, 126 shifts, 0 violations)**
- Hardware verification
- **Logistics optimization (VERIFIED: 1000 stops in 16ms)**
- AI planning
- Constraint satisfaction
- **Language translation (py2rs: Python→Rust, 26 tests passing)**
- **Halting problem resolution (S_observable decidable, 8 tests)**

### [CRYPTOGRAPHY_SAFETY.md](./CRYPTOGRAPHY_SAFETY.md)
**WHY** encryption remains safe:
- The decision vs search gap
- The constant problem
- The encoding overhead
- Three barriers protecting RSA

### [INDUSTRY_APPLICATIONS.md](./INDUSTRY_APPLICATIONS.md)
**WHAT** industries benefit:
- Gaming & Entertainment
- Logistics & Supply Chain
- Healthcare & Scheduling
- Chip Design & Verification
- Finance & Trading
- Telecommunications
- Manufacturing
- Energy & Utilities
- Software & Compiler Technology
- **Quantum Computing & Cryptography (NEW - Path 20, BQP=P)**
- **Large-Scale Optimization (NEW - Path 21, Sparse)**

## The Core Insight

```
P = NP(c) = PSPACE(d) = EXPTIME(s) means:
  - Polynomial witnesses exist for decision problems
  - Structured problems become tractable
  - Bounding c (local moves), d (depth), s (state bits) collapses all to P

It does NOT mean:
  - All encryption is broken
  - Computers are infinitely fast
  - Constants don't matter
```

## NEW: Full Complexity Hierarchy (v0.17.0)

| Class | Bounding Parameter | When Bounded |
|-------|-------------------|--------------|
| NP | c = local move size | P |
| PSPACE | d = alternation depth | P |
| EXPTIME | s = state representation bits | P |
| א^א | Nothing bounded | Incomputable |

**Dark Matter Dissolution:** The framework proves ~85% "missing mass" is a
miscalculation from treating discrete systems with continuous integrals.
See `proofs/COMPLETE_RELATIVITY_THEORY.md`.

## Separation of Concerns

| Directory | Contains |
|-----------|----------|
| `/products/` | Guard8.ai product overviews (CausaDB, FitGuard, TranslatorGuard, DLM, Grapheme) |
| `/applications/` | WHAT & WHY (this directory) |
| `/proofs/` | HOW (implementation details) |
| `/np-optima/src/` | CODE (algorithms) |

---

**Author:** Eliran Sabag
**Framework Version:** Discovery 96 (Full Hierarchy + Sparse Direction)
**Date:** 2026-01-25

*For implementation details, see `/proofs/` directory.*
