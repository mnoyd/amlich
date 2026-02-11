# Shared Data Contract

This directory is the single source of truth for static calendar data consumed by:

- `crates/amlich-core` (Rust)
- `packages/core` (Node/CommonJS)
- `apps/desktop` (Svelte/TypeScript)

## Files

- `holidays/solar-holidays.json`: Solar observances and national days
- `holidays/lunar-festivals.json`: Lunar festivals and related observances
- `canchi.json`: Heavenly Stems/Earthly Branch metadata for insights
- `tiet-khi.json`: Solar term metadata for insights

## Schema

Schema files are in `data/schemas/`:

- `solar-holidays.schema.json`
- `lunar-festivals.schema.json`

The project uses an additional validator script for cross-file integrity and business rules.

## Validation

Run from repository root:

```bash
pnpm check:data
```

This validates:

- required fields and basic value ranges
- kebab-case IDs and uniqueness
- allowed categories
- duplicate date/name collisions
- lunar `yearOffset` constraints (`0` or `-1`)
- cross-file ID uniqueness

## Category Guidance

To keep dataset balance, prefer incremental additions and avoid overloading `international`.
Track category counts after each enrichment batch.
