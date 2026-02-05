#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/.." && pwd)"
TARGET_BIN="${PROJECT_DIR}/target/release/amlich-rs"
INSTALL_PATH="${HOME}/.local/bin/amlich"

WAYBAR_CONFIG="${HOME}/.config/waybar/config.jsonc"
WAYBAR_STYLE="${HOME}/.config/waybar/style.css"
BACKUP_DIR="${HOME}/.config/waybar/backups"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

CONFIG_PATCH="${PROJECT_DIR}/patches/waybar-config.patch"
STYLE_PATCH="${PROJECT_DIR}/patches/waybar-style.patch"

echo "[amlich] Building release binary..."
cargo build --release --manifest-path "${PROJECT_DIR}/Cargo.toml"

echo "[amlich] Installing binary to ${INSTALL_PATH}"
install -Dm755 "${TARGET_BIN}" "${INSTALL_PATH}"

# Remove old wrapper if it exists
if [[ -f "${HOME}/.local/bin/amlich-waybar" ]]; then
    echo "[amlich] Removing old wrapper script..."
    rm -f "${HOME}/.local/bin/amlich-waybar"
fi

# Create backup directory
mkdir -p "${BACKUP_DIR}"

# Function to check if patch is already applied
is_patch_applied() {
    local file="$1"
    local patch="$2"
    
    if patch -p0 --dry-run -R "${file}" < "${patch}" &>/dev/null; then
        return 0
    else
        return 1
    fi
}

# Function to apply patch with backup
apply_patch_if_needed() {
    local file="$1"
    local patch="$2"
    local name="$3"
    
    if [[ ! -f "${file}" ]]; then
        echo "[amlich] Warning: ${file} not found, skipping..."
        return 0
    fi
    
    if is_patch_applied "${file}" "${patch}"; then
        echo "[amlich] ${name} patch already applied, skipping..."
        return 0
    fi
    
    echo "[amlich] Backing up ${file} to ${BACKUP_DIR}/"
    cp "${file}" "${BACKUP_DIR}/$(basename ${file}).${TIMESTAMP}.backup"
    
    echo "[amlich] Applying ${name} patch..."
    if patch -p0 "${file}" < "${patch}"; then
        echo "[amlich] ${name} patch applied successfully"
        return 0
    else
        echo "[amlich] Error: Failed to apply ${name} patch"
        echo "[amlich] Restoring from backup..."
        cp "${BACKUP_DIR}/$(basename ${file}).${TIMESTAMP}.backup" "${file}"
        return 1
    fi
}

# Apply patches
if [[ -f "${CONFIG_PATCH}" ]]; then
    apply_patch_if_needed "${WAYBAR_CONFIG}" "${CONFIG_PATCH}" "config.jsonc"
else
    echo "[amlich] Warning: config patch not found at ${CONFIG_PATCH}"
fi

if [[ -f "${STYLE_PATCH}" ]]; then
    apply_patch_if_needed "${WAYBAR_STYLE}" "${STYLE_PATCH}" "style.css"
else
    echo "[amlich] Warning: style patch not found at ${STYLE_PATCH}"
fi

# Restart waybar (kill and let hyprland restart it, or use omarchy command)
echo "[amlich] Restarting waybar..."
if command -v omarchy-restart-waybar &>/dev/null; then
    omarchy-restart-waybar
elif pgrep -x waybar > /dev/null; then
    pkill -x waybar
    sleep 0.5
    waybar &
    echo "[amlich] Waybar restarted"
else
    echo "[amlich] Waybar not running, skipping restart"
fi

echo "[amlich] Done."
echo "[amlich] Backups saved to: ${BACKUP_DIR}"
