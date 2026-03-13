# User Testing

Testing surface, commands, setup steps, and known quirks for manual validation.

**What belongs here:** user-facing entrypoints, test commands, smoke-test steps, validation limitations.

---

## Validation Surface

### Interactive TUI surface

- `amlich` in TTY mode
- `amlich tui --date <date>`
- Use bounded PTY/script-driven smoke and flow checks only; `tuistory` is not installed.

Preferred smoke commands:

- `printf 'q' | timeout 15 script -qec "cargo run --package amlich-cli -- tui --date 2026-03-13" /dev/null`
- `printf 'q' | timeout 15 script -qec "cargo run --package amlich-cli --" /dev/null`

Use the second command whenever launch-path parity changes.

### Headless / machine-output surface

- Default non-TTY `cargo run --package amlich-cli --`
- `cargo run --package amlich-cli -- day 2026-03-13 --format json --pretty`
- `cargo run --package amlich-cli -- range --start 2026-03-13 --end 2026-03-15 --format json --pretty`
- `cargo run --package amlich-cli -- range --start 2026-03-13 --end 2026-03-15 --format ndjson`
- Discovery helpers:
  - `cargo run --package amlich-cli -- lookup rulesets --format json`
  - `cargo run --package amlich-cli -- lookup recommendation-packs --format json`

## Validation Concurrency

Machine snapshot at planning time:

- CPUs: `12`
- Memory available: about `16 GiB`
- 70% planning budget: about `11.2 GiB`

### Interactive TUI validators

- Recommended max concurrent validators: `6`
- Rationale: memory footprint is low, but PTY/script orchestration and terminal-state contention are the real limit.

### Headless validators

- Recommended max concurrent validators: `12`
- Rationale: these are lightweight CLI/JSON checks with low steady-state footprint, so CPU/core count becomes the practical ceiling before memory does.

## Known Limitations

- `tuistory` is not installed, so semantic terminal playback is unavailable.
- Interactive evidence will come from PTY captures, interaction transcripts, and focused TUI tests rather than full terminal automation.
- Timestamp freshness fields such as `generated_at` should be checked for presence/format rather than exact equality across separate commands.
- When validating selector-bearing parity, compare canonical metadata fields explicitly: `schema_version`, `ruleset_id`, `ruleset_version`, `profile`, contextual activation, and active packs.
