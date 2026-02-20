# Waybar Integration

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

Copy or merge rules from `waybar/styles/amlich.css` into your Waybar stylesheet.

## Restart Waybar

```bash
pkill -x waybar && waybar &
```
