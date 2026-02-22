#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PAIRS=(
  "data/canchi.json crates/amlich-core/data/canchi.json"
  "data/tiet-khi.json crates/amlich-core/data/tiet-khi.json"
  "data/holidays/solar-holidays.json crates/amlich-core/data/holidays/solar-holidays.json"
  "data/holidays/lunar-festivals.json crates/amlich-core/data/holidays/lunar-festivals.json"
)

status=0

for pair in "${PAIRS[@]}"; do
  src="${pair%% *}"
  dst="${pair#* }"
  src_abs="${ROOT_DIR}/${src}"
  dst_abs="${ROOT_DIR}/${dst}"

  if [[ ! -f "${src_abs}" ]]; then
    echo "Missing source file: ${src}" >&2
    status=1
    continue
  fi

  if [[ ! -f "${dst_abs}" ]]; then
    echo "Missing vendored file: ${dst}" >&2
    status=1
    continue
  fi

  if ! cmp -s "${src_abs}" "${dst_abs}"; then
    echo "Out of sync: ${src} != ${dst}" >&2
    status=1
  else
    echo "OK: ${src}"
  fi
done

if [[ ${status} -ne 0 ]]; then
  echo
  echo "amlich-core vendored data is out of sync." >&2
  echo "Sync by copying root data files into crates/amlich-core/data/." >&2
  exit "${status}"
fi

echo
echo "amlich-core vendored data is in sync."
