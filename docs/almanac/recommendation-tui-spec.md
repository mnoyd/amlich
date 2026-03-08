# TUI Recommendation Rendering Spec (v1)

## Target

Applies to `crates/amlich-tui/src/widgets/guidance.rs` using `DayBundleDto.daily_recommendations`.

## Bucket Order

Always render in this order:

1. `Nên`
2. `Có thể`
3. `Tránh`
4. `Kỵ mạnh`

Each section header shows item count: `── <bucket> (<count>)`.

## Collapsed Behavior

When details are collapsed:

- small layout: max 2 rows per bucket
- medium layout: max 4 rows per bucket
- large layout: max 6 rows per bucket

If truncated, render `+N mục ẩn`.

When expanded (`a` toggle), show all rows.

## Row Shape

Row content is recommendation label (`label.vi`) plus optional chip:

- chip format: `[<severity> • <source>]`
- severity: `override | primary | support`
- source: one of `guidance | trực | sao | thần sát | kiêng kỵ | xung-hợp | tiết khí | giờ tốt | xuất hành | mở rộng`

Primary emphasis marker:

- first row in each bucket uses `★`
- other rows use `•`

## Summary and Footer

- header section shows `daily_recommendations.summary_vi`
- footer uses top 3 good-hour ranges when available (`Giờ đẹp tham chiếu: ...`)

## Provenance Visibility

Provenance is visible at row level through reason chips derived from strongest reason evidence.
