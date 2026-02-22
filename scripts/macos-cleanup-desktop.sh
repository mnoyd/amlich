#!/usr/bin/env bash
set -euo pipefail

MODE="${1:-safe}"

APP_SYSTEM="/Applications/AmLich.app"
APP_USER="${HOME}/Applications/AmLich.app"
CACHE_DIR="${HOME}/Library/Caches/com.amlich.calendar"
PREF_GLOB="${HOME}/Library/Preferences/com.amlich.calendar"*
SUPPORT_DIR="${HOME}/Library/Application Support/com.amlich.calendar"

usage() {
  cat <<'EOF'
Usage: scripts/macos-cleanup-desktop.sh [safe|full]

safe (default):
  - Removes AmLich.app from /Applications and ~/Applications (if present)
  - Removes updater/app cache directory
  - Preserves preferences and Application Support data

full:
  - Performs safe cleanup, plus removes preferences and Application Support data
EOF
}

remove_if_exists() {
  local path="$1"
  if [[ -e "$path" ]]; then
    echo "Removing: $path"
    rm -rf "$path"
  else
    echo "Not found: $path"
  fi
}

case "$MODE" in
  safe|full) ;;
  -h|--help)
    usage
    exit 0
    ;;
  *)
    echo "Unknown mode: $MODE" >&2
    usage >&2
    exit 1
    ;;
esac

echo "AmLich macOS cleanup mode: $MODE"
remove_if_exists "$APP_SYSTEM"
remove_if_exists "$APP_USER"
remove_if_exists "$CACHE_DIR"

if [[ "$MODE" == "full" ]]; then
  remove_if_exists "$SUPPORT_DIR"
  shopt -s nullglob
  pref_matches=($PREF_GLOB)
  shopt -u nullglob
  if (( ${#pref_matches[@]} > 0 )); then
    for pref in "${pref_matches[@]}"; do
      remove_if_exists "$pref"
    done
  else
    echo "No matching preferences found for com.amlich.calendar"
  fi
else
  echo "Preserved user data:"
  echo "  - $SUPPORT_DIR"
  echo "  - ~/Library/Preferences/com.amlich.calendar*"
fi

echo "Cleanup complete."
