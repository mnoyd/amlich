# Recommendation Source Actionability Matrix (v1)

## Goal

Decide which computed subsystems should directly influence day recommendations and which should remain informational until profile/event context exists.

## Decisions

| Source | v1 Decision | Rationale |
|---|---|---|
| `stars` | **Direct modifier** | Emits concrete favorable/adverse star buckets that can map to known activities. |
| `day_deity` | **Direct modifier** | Hoàng đạo/hắc đạo classification gives day-level support/caution signal. |
| `taboos` | **Direct modifier (with hard-stop support)** | Structured taboo severity (`hard`/`soft`) naturally maps to override behavior. |
| `xung_hop` | **Direct modifier (limited)** | Conflict/harmony graph is actionable for social/legal/commitment risk framing. |
| `hours` (`gio_hoang_dao`) | **Direct modifier (limited)** | High/low good-hour density is actionable for timing-sensitive activities. |
| `travel` | **Direct modifier (travel-only)** | Output already expresses directional travel signal; keep bounded to travel. |
| `tiet_khi` | **Direct modifier (eligible terms only)** | Seasonal transitions/extreme terms can influence selected activities, not full policy. |

## Informational-Only (for now)

These remain insight-only until personal/event inputs are available:

- broad temperament phrases from `dayGuidance`
- personal astrology signals requiring birth data
- event-specific constraints not derivable from date-only context

Contract note:

- `dayGuidance` is a legacy informational surface and must not seed default `daily_recommendations` synthesis in v1 alignment work.

## Guardrails

- No modifier may invent a new canonical activity ID ad hoc.
- Ambiguous mappings are dropped instead of guessed.
- Hard-stop behavior is reserved for explicit taboo severity unless a later decision explicitly promotes another rule family into policy-approved hard-stop authority.
- All emitted reasons must carry evidence source/code metadata.
