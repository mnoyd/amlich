# Issue tracker: bd (beads)

Issues and PRDs for this repository live in its bd (beads) database. Do not create GitHub Issues or markdown task lists as an alternative tracker. Run `bd prime` for the repository's current workflow instructions.

## Conventions

- Create: `bd create "Title" --description="..." -t bug|feature|task -p 0-4 --json`
- Read: `bd show <id> --json`
- List: `bd list --json` with appropriate status and label filters
- Find ready work: `bd ready --json`
- Claim: `bd update <id> --claim --json`
- Comment: `bd comment <id> "..."` or `bd comment <id> --stdin`
- Apply a label: `bd update <id> --add-label "<label>" --json`
- Remove a label: `bd update <id> --remove-label "<label>" --json`
- Add a blocker: `bd dep add <blocked-id> <blocker-id>`
- Close: `bd close <id> --reason "..." --json`
- Link work discovered during another issue with `--deps discovered-from:<parent-id>`.

Follow the persistence and synchronization requirements in `AGENTS.md`; they vary with each repository's beads configuration.

## When a skill says "publish to the issue tracker"

Create a bead with `bd create`. Use the appropriate issue type, priority, description, acceptance criteria, labels, parent, and dependencies.

## When a skill says "fetch the relevant ticket"

Run `bd show <id> --json` and `bd comments <id> --json`.

## Wayfinding operations

- Map: create an epic labelled `wayfinder:map`.
- Child ticket: create a child with `--parent <map-id>` and a `wayfinder:<type>` label (`research`, `prototype`, `grilling`, or `task`).
- Blocking: run `bd dep add <child-id> <blocker-id>`.
- Frontier: use `bd ready --json`, scoped to the map's children; ignore already assigned work.
- Claim: run `bd update <id> --claim --json` before beginning work.
- Resolve: add the answer as a comment, close the child, then record a context pointer in the map.
