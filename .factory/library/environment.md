# Environment

Environment variables, external dependencies, and setup notes.

**What belongs here:** required tools, env vars, platform constraints, dependency/setup notes.
**What does NOT belong here:** service ports/commands (use `.factory/services.yaml`).

---

- Mission scope is Rust-only; no external credentials are required.
- Required tools observed during planning: `cargo`, `pnpm`, `just`, `rustfmt`, `clippy`, `script`, `timeout`.
- `tuistory` is not installed.
- `wasm-pack` is not installed; WASM validation is out of scope for this mission.
- This mission should not require any listening ports or additional services.
- Avoid interfering with already-active ports on this machine, including `3389`, `8006`, `8080`, `8317`, `18789-18792`, `39221`, and `54622`.
- A local `target/debug/amlich tui` process may already be running on the machine and must not be touched by workers.
- The working tree already contains unrelated uncommitted changes. Workers must avoid reverting or broad-formatting untouched files.
