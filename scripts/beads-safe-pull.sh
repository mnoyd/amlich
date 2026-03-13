#!/usr/bin/env bash

set -euo pipefail

log() {
  printf '[beads-safe-pull] %s\n' "$*"
}

die() {
  printf '[beads-safe-pull] ERROR: %s\n' "$*" >&2
  exit 1
}

ROOT_DIR="${ROOT_DIR:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
BEADS_DIR="${BEADS_DIR:-$ROOT_DIR/.beads}"
DOLT_DIR="${DOLT_DIR:-$BEADS_DIR/dolt}"
DB_REPO="${DB_REPO:-$DOLT_DIR/beads_amlich}"
BACKUP_DIR="${BACKUP_DIR:-$BEADS_DIR/backup}"
REMOTE_NAME="${REMOTE_NAME:-origin}"
REMOTE_BRANCH="${REMOTE_BRANCH:-main}"
MERGE_MESSAGE="${MERGE_MESSAGE:-Merge ${REMOTE_NAME}/${REMOTE_BRANCH} into local beads history}"
EXPECTED_CONFLICTS="${EXPECTED_CONFLICTS:-events metadata}"
MANAGE_SERVER="${MANAGE_SERVER:-1}"
VERIFY_BD="${VERIFY_BD:-1}"
FETCH_STRATEGY="${FETCH_STRATEGY:-cached}"
SERVER_WAS_RUNNING=0
VERIFY_TIMEOUT="${VERIFY_TIMEOUT:-20}"

mkdir -p "$BACKUP_DIR"

cleanup() {
  local status=$?

  if [[ "$status" -ne 0 && "$MANAGE_SERVER" == "1" && "$SERVER_WAS_RUNNING" == "1" && ! -f "$BEADS_DIR/dolt-server.pid" ]]; then
    log "Restarting Beads Dolt server after failure"
    (cd "$ROOT_DIR" && bd dolt start >/dev/null) || true
  fi

  trap - EXIT
  exit "$status"
}

trap cleanup EXIT

assert_repo() {
  local path="$1"
  [[ -d "$path/.dolt" ]] || die "missing dolt repo at $path"
}

stop_server() {
  [[ "$MANAGE_SERVER" == "1" ]] || return 0
  if [[ -f "$BEADS_DIR/dolt-server.pid" ]]; then
    SERVER_WAS_RUNNING=1
    log "Stopping Beads Dolt server"
    (cd "$ROOT_DIR" && bd dolt stop >/dev/null 2>&1) || true
  fi
}

start_server() {
  [[ "$MANAGE_SERVER" == "1" ]] || return 0
  log "Starting Beads Dolt server"
  (cd "$ROOT_DIR" && bd dolt start >/dev/null)
}

quarantine_nested_backups() {
  local path

  shopt -s nullglob
  for path in "$DOLT_DIR"/*.pre_repair_* "$DOLT_DIR"/*.bak "$DOLT_DIR"/*.backup; do
    if [[ -e "$path" ]]; then
      log "Moving nested backup out of dolt data dir: $(basename "$path")"
      mv "$path" "$BACKUP_DIR/"
    fi
  done
  shopt -u nullglob
}

backup_repo() {
  local repo="$1"
  local stamp="$2"
  local name

  name="$(basename "$repo")"
  cp -a "$repo" "$BACKUP_DIR/${name}.pre_repair_${stamp}"
}

revive_if_needed() {
  local repo="$1"
  local stamp="$2"
  local output

  assert_repo "$repo"
  output="$(cd "$repo" && dolt fsck 2>&1 || true)"
  if grep -Fq "Run \`dolt fsck --revive-journal-with-data-loss\`" <<<"$output"; then
    log "Corrupted journal detected in $repo"
    backup_repo "$repo" "$stamp"
    (cd "$repo" && dolt fsck --revive-journal-with-data-loss >/dev/null)
    return
  fi
  if grep -Fq "No problems found." <<<"$output"; then
    return
  fi
  if [[ -n "$output" ]]; then
    log "$output"
  fi
}

integrate_remote() {
  local output

  if [[ "$FETCH_STRATEGY" == "remote" ]]; then
    output="$(cd "$DB_REPO" && dolt pull "$REMOTE_NAME" "$REMOTE_BRANCH" --no-edit 2>&1 || true)"
  else
    output="$(cd "$DB_REPO" && dolt merge "$REMOTE_NAME/$REMOTE_BRANCH" --no-edit 2>&1 || true)"
  fi
  if [[ -z "$output" ]]; then
    return 0
  fi
  if grep -Fq "Automatic merge failed" <<<"$output"; then
    log "Remote integration produced merge conflicts"
    return 1
  fi
  if grep -Fq "Already up to date." <<<"$output" || grep -Fq "Everything up-to-date" <<<"$output"; then
    log "Beads repo already up to date"
    return 0
  fi
  if grep -Fq "Fast-forward" <<<"$output" || grep -Fq "Updating " <<<"$output"; then
    log "Remote integration completed"
    return 0
  fi
  printf '%s\n' "$output" >&2
  die "unexpected remote integration failure"
}

conflict_tables() {
  (cd "$DB_REPO" && dolt sql -r csv -q "select \`table\` from dolt_conflicts order by \`table\`")
}

assert_known_event_conflicts() {
  local unexpected

  unexpected="$(
    cd "$DB_REPO" &&
      dolt sql -r csv -q "
        select count(*) as unexpected_rows
        from dolt_conflicts_events
        where our_diff_type <> 'added'
           or their_diff_type <> 'added'
           or our_id <> their_id;
      " |
      tail -n +2
  )"
  [[ "$unexpected" == "0" ]] || die "unsupported events conflict shape"
}

assert_known_metadata_conflicts() {
  local unexpected

  unexpected="$(
    cd "$DB_REPO" &&
      dolt sql -r csv -q "
        select count(*) as unexpected_rows
        from dolt_conflicts_metadata
        where their_key not in ('dolt_auto_push_commit', 'dolt_auto_push_last');
      " |
      tail -n +2
  )"
  [[ "$unexpected" == "0" ]] || die "unsupported metadata conflict shape"
}

resolve_known_conflicts() {
  local tables

  tables="$(conflict_tables | tail -n +2 | tr '\n' ' ' | sed 's/[[:space:]]\+$//')"
  [[ "$tables" == "$EXPECTED_CONFLICTS" ]] || die "unsupported conflict set: ${tables:-<none>}"
  assert_known_event_conflicts
  assert_known_metadata_conflicts

  log "Resolving known events/metadata merge conflicts"
  (
    cd "$DB_REPO"
    dolt conflicts resolve --ours events
    dolt conflicts resolve --theirs metadata
    dolt sql -q "
      insert into events (issue_id, event_type, actor, old_value, new_value, comment, created_at)
      select
        their_issue_id,
        their_event_type,
        their_actor,
        their_old_value,
        their_new_value,
        their_comment,
        their_created_at
      from dolt_conflicts_events
      where their_diff_type = 'added';
    "
    dolt add .
    dolt commit -m "$MERGE_MESSAGE" >/dev/null
  )
}

verify_result() {
  [[ "$VERIFY_BD" == "1" ]] || return 0
  (
    cd "$ROOT_DIR"
    timeout "$VERIFY_TIMEOUT" bd status --json --no-activity >/dev/null
  )
}

main() {
  local stamp

  stamp="$(date +%Y%m%d_%H%M%S)"
  assert_repo "$DOLT_DIR"
  assert_repo "$DB_REPO"

  stop_server
  quarantine_nested_backups
  revive_if_needed "$DOLT_DIR" "$stamp"
  revive_if_needed "$DB_REPO" "$stamp"

  if ! integrate_remote; then
    resolve_known_conflicts
  fi

  start_server
  verify_result
  log "Beads pull finished successfully"
}

main "$@"
