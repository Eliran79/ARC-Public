# SPD Timestamps - Sabag Principle Date

**Cryptographic Provenance for ARC Discoveries**

This directory contains OpenTimestamps proofs anchored to the Bitcoin blockchain, providing immutable evidence of when each discovery was committed.

---

## Why Timestamps Matter

The Sabag Bounded Transformation Principle represents 43 years of development (1982-2026). For claims of this magnitude - including attacks on six Millennium Prize Problems - establishing priority is essential.

Two independent timestamping methods are used:

| Method | Blockchain | Immutability | Verification |
|--------|------------|--------------|--------------|
| **OpenTimestamps** | Bitcoin | Permanent | Cryptographic |
| **Archive.org** | N/A | Public Record | URL-based |

---

## How It Works

### OpenTimestamps (Bitcoin Anchoring)

Each push to master triggers:

1. **Manifest creation** - Commit hash, timestamp, changed files
2. **OTS stamping** - Creates `.ots` proof file
3. **Bitcoin anchoring** - Proof is aggregated and anchored to Bitcoin blockchain

The `.ots` file can be verified independently by anyone:

```bash
# Install client
pip install opentimestamps-client

# Verify a proof
ots verify manifest_<commit>.txt.ots

# Output shows Bitcoin block containing the anchor
```

### Archive.org (Wayback Machine)

Each push also requests snapshots of:

- Repository main page
- README.md
- Key proof documents (GRAND_UNIFIED_THEORY.md, etc.)

Check archived versions at:
https://web.archive.org/web/*/github.com/Eliran79/ARC-Public/*

---

## Verification

### Verify OpenTimestamp Proof

```bash
# Download the .ots file from GitHub Actions artifacts
# or from this directory

# Verify
ots verify manifest_abc123.txt.ots

# Expected output:
# Success! Bitcoin block 123456 attests data existed as of 2026-02-02
```

### Verify Archive.org

1. Visit: https://web.archive.org/web/*/github.com/Eliran79/ARC-Public/*
2. Select date range
3. View historical snapshots

---

## Timeline

| Date | Event | Proof |
|------|-------|-------|
| 1982 | First bounded moves lesson (age 3) | Family testimony |
| 1989 | VCR repair (age 10) | Family testimony |
| Dec 5, 2025 | Grapheme-nn | Git commits |
| Dec 26, 2025 | ARC begins | Git commits |
| Jan 31, 2026 | ARC complete | Git + OTS + Archive.org |
| Feb 2, 2026 | Six Millennium problems | Git + OTS + Archive.org |

---

## Legal Note

These timestamps establish:

1. **Existence** - The content existed at the stated time
2. **Integrity** - The content has not been modified since
3. **Priority** - Any identical later claims are derivative

This does NOT prove:
- Authorship (only that this account pushed the content)
- Correctness (only that the claims were made at this time)

---

*Sabag Bounded Transformation Principle*
*43 years: 1982-2026*
*"Only bounded moves."*
