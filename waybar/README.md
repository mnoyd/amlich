# Waybar Integration

This folder contains Waybar-specific integration assets.

The `amlich-core` crate stays focused on calendar calculations only.
Waybar rendering and relevance filtering live in `amlich-cli`.

## Install CLI

```bash
cargo install --path crates/amlich-cli
```

## Module Config

Copy the module snippet from `waybar/modules/amlich.jsonc` into your Waybar config.

Recommended command wiring:

- `exec`: `amlich today`
- `on-click`: `amlich toggle`

## Styles

Copy or merge rules from `waybar/styles/amlich.css` into your Waybar style file.

## Restart Waybar

```bash
omarchy-restart-waybar
```

Fallback:

```bash
pkill -x waybar && waybar &
```
