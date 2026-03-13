#!/usr/bin/env sh
set -eu

command -v cargo >/dev/null 2>&1
command -v pnpm >/dev/null 2>&1
command -v script >/dev/null 2>&1

if [ -f package.json ] && [ ! -d node_modules ]; then
  pnpm install
fi

cargo fetch --locked
