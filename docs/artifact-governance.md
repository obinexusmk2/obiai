# Artifact Governance Policy

## Purpose
This policy defines how repository artifacts are named, classified, and updated so downstream references are stable and auditable.

## Naming Rules
- Do not commit filenames with duplicate suffix patterns such as ` (1)`, ` (2)`, etc.
- Do not commit filenames that differ only by trailing spaces before extensions (for example: `name .pdf` vs `name.pdf`).
- Prefer normalized extensions (for example `.txt`, `.md`, `.pdf`) and avoid ambiguous extensionless exports when a typed format is available.
- When importing from external systems, rename files into canonical form before merge; keep original names only in provenance notes.

## Canonical vs Mirror Directories
- Canonical files are the single source of truth listed in:
  - `OBIAI_PATENTS/CANONICAL_INDEX.md`
  - `TRANSCRIPTS/CANONICAL_INDEX.md`
- Mirror, derived, or archive copies may exist for traceability, but are non-authoritative.
- Suggested interpretation for current layout:
  - `OBIAI_PATENTS/` root patent files are canonical unless index marks them otherwise.
  - `TRANSCRIPTS/TRANSCRIPT/` topical directories generally host canonical transcripts.
  - Alternate folders (for example archival snapshots, sealed bundles, import dumps, or duplicate suffix variants) are mirror/archive unless explicitly marked canonical in an index.

## Canonical Artifact Replacement Workflow
1. Identify the existing canonical artifact in the relevant `CANONICAL_INDEX.md`.
2. Add or update the candidate replacement file using canonical naming rules.
3. Compute and verify SHA256 checksums for both old and new files.
4. Update the relevant `CANONICAL_INDEX.md` entry:
   - canonical path,
   - canonical checksum,
   - previous canonical moved to derived/mirror/archive column,
   - provenance note with date and source.
5. Update references in docs or code to the new canonical path if it changed.
6. In the commit message and PR description, include a brief provenance summary (source system, ingestion date, and reason for replacement).

## Minimum Provenance Requirements
Each canonical index entry must include:
- Canonical file path.
- SHA256 checksum of canonical copy.
- List of known derived/mirror/archive variants.
- A dated provenance note (source mirror/system and canonicalization date).
