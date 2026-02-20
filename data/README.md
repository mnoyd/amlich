# Shared Data Contract

This directory is the source of truth for static calendar datasets consumed by:

- `crates/amlich-core`
- `packages/core`
- `apps/desktop`

## Files

- `holidays/solar-holidays.json`: Solar observances and national days
- `holidays/lunar-festivals.json`: Lunar festivals and observances
- `canchi.json`: Heavenly stem/earthly branch metadata
- `tiet-khi.json`: Solar term metadata

## Schemas

Schema files are in `data/schemas/`:

- `solar-holidays.schema.json`
- `lunar-festivals.schema.json`

## Validation

Run from repository root:

```bash
pnpm check:data
```

Optional parity check:

```bash
pnpm check:parity
```

`check:data` validates schema compliance and cross-file integrity rules enforced by project validators.
