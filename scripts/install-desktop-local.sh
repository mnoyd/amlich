#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
DESKTOP_DIR="${ROOT_DIR}/apps/desktop"
WORKSPACE_TARGET_DIR="${ROOT_DIR}/target/release"
LOCAL_TARGET_DIR="${DESKTOP_DIR}/src-tauri/target/release"

LOCAL_BIN_DIR="${HOME}/.local/bin"
INSTALL_BIN="${LOCAL_BIN_DIR}/amlich-app"
DESKTOP_ENTRY_DIR="${HOME}/.local/share/applications"
DESKTOP_ENTRY_PATH="${DESKTOP_ENTRY_DIR}/amlich.desktop"
ICON_DIR="${HOME}/.local/share/icons/hicolor/128x128/apps"
ICON_PATH="${ICON_DIR}/amlich.png"
SOURCE_ICON="${DESKTOP_DIR}/src-tauri/icons/128x128.png"

info() {
  echo "[amlich-desktop] $1"
}

err() {
  echo "[amlich-desktop] ERROR: $1" >&2
  exit 1
}

require_cmd() {
  command -v "$1" >/dev/null 2>&1 || err "Missing required command: $1"
}

build_desktop() {
  info "Installing desktop dependencies..."
  pnpm install

  info "Syncing SvelteKit..."
  pnpm exec svelte-kit sync

  info "Building Tauri app for Linux binary (no bundle)..."
  pnpm tauri build --no-bundle
}

detect_binary() {
  local target_dir
  for target_dir in "${WORKSPACE_TARGET_DIR}" "${LOCAL_TARGET_DIR}"; do
    [[ -d "${target_dir}" ]] || continue

    local candidate
    for candidate in "am-lich" "AmLich" "amlich"; do
      if [[ -x "${target_dir}/${candidate}" && -f "${target_dir}/${candidate}" ]]; then
        echo "${target_dir}/${candidate}"
        return 0
      fi
    done

    local executables=()
    local f
    for f in "${target_dir}"/*; do
      [[ -f "$f" && -x "$f" ]] || continue
      case "$(basename "$f")" in
        *.d) continue ;;
      esac
      executables+=("$f")
    done

    if [[ ${#executables[@]} -gt 0 ]]; then
      ls -t "${executables[@]}" | head -n 1
      return 0
    fi
  done

  return 1
}

install_binary() {
  local source_bin="$1"
  mkdir -p "${LOCAL_BIN_DIR}"
  install -Dm755 "${source_bin}" "${INSTALL_BIN}"
  info "Installed binary: ${INSTALL_BIN}"
}

install_icon() {
  if [[ -f "${SOURCE_ICON}" ]]; then
    mkdir -p "${ICON_DIR}"
    install -Dm644 "${SOURCE_ICON}" "${ICON_PATH}"
    info "Installed icon: ${ICON_PATH}"
  else
    info "Icon not found at ${SOURCE_ICON}; skipping icon install"
  fi
}

create_desktop_entry_if_missing() {
  mkdir -p "${DESKTOP_ENTRY_DIR}"

  if [[ -f "${DESKTOP_ENTRY_PATH}" ]]; then
    info "Desktop entry already exists: ${DESKTOP_ENTRY_PATH} (skipped)"
    return 0
  fi

  cat > "${DESKTOP_ENTRY_PATH}" <<EOF
[Desktop Entry]
Name=AmLich
Comment=Vietnamese Lunar Calendar
Exec=env GDK_BACKEND=x11 WEBKIT_DISABLE_COMPOSITING_MODE=1 ${INSTALL_BIN}
Icon=amlich
Terminal=false
Type=Application
Categories=Utility;Calendar;
Keywords=lunar;calendar;vietnamese;amlich;
StartupWMClass=AmLich
EOF

  info "Created desktop entry: ${DESKTOP_ENTRY_PATH}"

  if command -v update-desktop-database >/dev/null 2>&1; then
    update-desktop-database "${DESKTOP_ENTRY_DIR}" >/dev/null 2>&1 || true
    info "Refreshed desktop database"
  fi
}

main() {
  require_cmd cargo
  require_cmd node
  require_cmd pnpm

  [[ -d "${DESKTOP_DIR}" ]] || err "Desktop app directory not found: ${DESKTOP_DIR}"

  info "Working directory: ${DESKTOP_DIR}"
  cd "${DESKTOP_DIR}"

  build_desktop

  local built_bin
  built_bin="$(detect_binary)" || err "Unable to find built binary in ${WORKSPACE_TARGET_DIR} or ${LOCAL_TARGET_DIR}"
  info "Detected built binary: ${built_bin}"

  install_binary "${built_bin}"
  install_icon
  create_desktop_entry_if_missing

  info "Done. Launch with: ${INSTALL_BIN}"
}

main "$@"
