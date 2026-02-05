# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ARC (The Sabag Bounded Transformation Principle) is a mathematical research documentation repository by Eliran Sabag. It contains formal proofs, empirical verifications, and applications related to the claim that P = NP = PSPACE = BQP for problems with bounded local moves, along with attacks on six of seven Millennium Prize Problems through a unified Laplace framework.

This is a **pure documentation repository** — there is no compiled source code, no build system, and no test suite. All content is Markdown and CSV.

## Repository Structure

- `proofs/` — 136+ formal proof documents including the core `GRAND_UNIFIED_THEORY.md` (P=NP via 12 paths) and `BOURBAKI_LAPLACE_UNIFIED.md` (six Millennium problems unified framework)
- `proofs/*.csv` — Primary tracking CSVs at the proofs root: `discoveries.csv`, `paths.csv`, `triangles.csv`, `verifications.csv`, `domains.csv`, `constants.csv`, `ethers.csv`, plus supplementary CSVs (`theorems.csv`, `lemmas.csv`, `equations.csv`, `citations.csv`, etc.)
- `proofs/csv/` — Additional CSV copies/subsets: `discoveries.csv`, `triangles.csv`, `verifications.csv`, `domains.csv`, `constants.csv`, `ethers.csv`, `particle_ethers.csv`
- `theory/` — Core mathematical framework (`BOUNDED_TRANSFORMATION_PRINCIPLE.md`, `CRYPTOGRAPHY_SAFETY.md`)
- `domains/` — 42 independent domain validations (physics, information, quantum, biology, etc.)
- `verifications/` — 53 empirical proofs (V01 through V53)
- `applications/` — Industry applications and real-world solutions
- `thin_cell_theory/` — TSP thin-cell lemma proofs with `proof/` and `intersections/` subdirectories
- `presentations/` — Cross-domain connections
- `timestamps/` — OpenTimestamps + Archive.org provenance files

## Key Entry Points

- `README.md` — Project overview and Millennium Prize Problems status table
- `00_START_HERE.md` — Practical quick start (Path 23 bounded displacement sorting)
- `MASTER_INDEX.md` — Complete navigation guide with reading paths by audience
- `INDEX.md` — Path 23 sorting applications index

## CI/CD

The only CI workflow is `.github/workflows/spd-timestamp.yml` (SPD Timestamp). It runs on push to main/master and on release:
1. **OpenTimestamps** — Anchors commits to Bitcoin blockchain for cryptographic provenance
2. **Archive.org** — Creates Wayback Machine snapshots of key documents
3. **Summary** — Generates a timestamp report in the GitHub Actions summary

This workflow auto-commits `.ots` proof files back to the repo under `timestamps/`.

## Conventions

- All proof documents are Markdown files with mathematical notation in code blocks
- Discoveries are numbered sequentially (Discovery 0–126+) and tracked in `proofs/csv/discoveries.csv`
- Domain validations follow the pattern `domains/NN_name.md` (01 through 42)
- Verification documents follow the pattern `verifications/VNN_name.md` (V01 through V53)
- The CSV tracking files are the canonical registry for discoveries, paths, triangles, and verifications — keep them in sync when adding new content

## Core Mathematical Concepts

The central insight distinguishes:
- **S_complete**: All syntactically valid states (exponential, O(2^n))
- **S_observable**: States reachable via bounded local moves (polynomial, O(n^c))

Key constants: √2 (Nittay Limit, discrete→continuous boundary), log₂(√2) = ½ (Riemann critical line), κ = 3/√2 ≈ 2.12 (inverse Nittay sampling parameter).

## License

MIT (2026) — collaborative work attributed to Eliran Sabag and Claude (Anthropic).
