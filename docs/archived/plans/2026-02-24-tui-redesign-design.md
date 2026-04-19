# TUI Redesign: Info-First Dashboard

**Date:** 2026-02-24
**Status:** Approved

## Problem

The current TUI is calendar-dominant — the calendar grid gets 50-60% of screen width while the detail panel (the information users actually want) is squeezed into 25%. Day guidance (good/bad activities) is hidden behind an overlay despite being the #2 use case. The hours panel wastes 25% of screen for 12 items. There is no clear visual hierarchy — users don't know where to focus. The experience is inconsistent across screen sizes.

## User Priorities

1. **Primary:** "What lunar date is today?" — quick glance at solar-to-lunar conversion
2. **Secondary:** "Is today a good day?" — Can Chi, day guidance (Nen/Tranh)
3. **Tertiary:** Browse calendar for planning, check holidays

## Approach: Info-First Dashboard

Flip the layout. Today's information is the hero. Calendar becomes a navigation sidebar.

## Information Hierarchy

**Tier 1 — Instant glance (always visible, most prominent):**
- Solar date to lunar date conversion (the headline)
- Day quality verdict (good/bad summary)

**Tier 2 — Quick scan (always visible, secondary):**
- Can Chi for day/month/year with elements
- Top 3-4 good-for and avoid-for activities
- Auspicious hours (compact one-liner)
- Current Tiet Khi (solar term + season)

**Tier 3 — On-demand (behind one keypress):**
- Full activity guidance lists
- Festival/holiday details with origin, food, customs
- Full 12-hour breakdown
- Ngu Hanh detail

**Tier 4 — Navigation/utility:**
- Calendar grid
- Month/year navigation
- Bookmarks, search, help

## Layout System

Three responsive breakpoints. Info panel is always the majority of screen real estate. Small screens hide the calendar, not the info.

### Small Screen (<80 cols): Info Card

No calendar by default. Press `c` to toggle calendar overlay.

```
+----------------------------------------+
|  Am Lich                  Th2  2026 >  |
+----------------------------------------+
|                                        |
|   24 thang 2              Thu Ba       |
|   Mung 7 thang Gieng, Binh Ngo        |
|                                        |
| -- Can Chi -------------------------   |
|   Nham Thin . Canh Dan . Binh Ngo      |
|   Thuy.Tho    Kim.Moc    Hoa.Hoa      |
|                                        |
| -- Nen / Tranh ----------------------  |
|   V Cuoi hoi    V Khai truong          |
|   V Xuat hanh                          |
|   X Dong tho    X An tang              |
|                                [i] xem |
|                                        |
| -- Gio tot -------------------------   |
|   Ty . Suu . Mao . Ngo                |
|                                        |
| -- Tiet khi ------------------------   |
|   Vu Thuy . Mua Xuan                  |
|                                        |
+----------------------------------------+
|  c lich . t nay . g nhay . ? . q       |
+----------------------------------------+
```

### Medium Screen (80-119 cols): Sidebar Calendar

Mini calendar ~35%, info panel ~65%.

```
+--------------------------------------------------------------+
|  Am Lich                                  < Th2  2026 >      |
+---------------+----------------------------------------------+
|               |                                              |
|   Th2  2026   |   24 thang 2                      Thu Ba     |
|  T2 T3 .. CN  |   Mung 7 thang Gieng, Binh Ngo              |
|   .  .     .  |                                              |
|   .  .     .  |  -- Can Chi ----------------------------     |
|   .  .     .  |   Nham Thin . Canh Dan . Binh Ngo            |
|   .  .     .  |   Thuy.Tho    Kim.Moc    Hoa.Hoa            |
|   .  .     .  |                                              |
|               |  -- Nen / Tranh -------------------------    |
|               |   V Cuoi hoi     V Khai truong               |
|               |   V Xuat hanh    V Ky ket                    |
|               |   X Dong tho     X An tang                   |
|               |                                     [i] xem  |
|               |                                              |
|               |  -- Gio tot ---- Ty . Suu . Mao . Ngo       |
|               |  -- Tiet khi --- Vu Thuy . Mua Xuan         |
+---------------+----------------------------------------------+
|  hjkl nav . t nay . / tim . g nhay . b bm . ? help . q      |
+--------------------------------------------------------------+
```

### Large Screen (120+ cols): Full Dashboard

Full calendar ~40%, spacious info panel ~60%.

```
+--------------------------------------------------------------------------------+
|  Am Lich                                                < Thang 2  2026 >      |
+---------------------------------+----------------------------------------------+
|                                 |                                              |
|   Thang 2, 2026                 |   24 thang 2, 2026                  Thu Ba   |
|  +----------------------------+ |   Mung 7 thang Gieng, Binh Ngo              |
|  | T2   T3   T4   T5   T6  T7| |                                              |
|  |                          CN| |  -- Can Chi ----------------------------     |
|  |  2    3    4    5    6   7 | |   Ngay  Nham Thin     Thuy . Tho            |
|  | 14   15   16   17   18  19| |   Thang Canh Dan      Kim . Moc             |
|  |                            | |   Nam   Binh Ngo      Hoa . Hoa  (Ngua)     |
|  |  9   10   11   12   13  14| |                                              |
|  | 21   22   23   24   25  26| |  -- Nen / Tranh ----------------------------  |
|  |                            | |   V Cuoi hoi       V Khai truong             |
|  | 16   17   18   19  [20] 21| |   V Xuat hanh      V Ky ket                  |
|  | 28   29   30    1    2   3 | |   X Dong tho       X An tang                 |
|  |                            | |   X Pha tho                                  |
|  | 23  [24]  25   26   27  28| |                                     [i] xem  |
|  |  5  [ 7]   8    9   10  11| |                                              |
|  +----------------------------+ |  -- Gio tot ----------------------------     |
|                                 |   Ty(23-1) Suu(1-3) Mao(5-7) Ngo(11-13)    |
|                                 |                                              |
|                                 |  -- Tiet khi ---------------------------     |
|                                 |   Vu Thuy . Mua Xuan                        |
|                                 |   Mua nhieu, chuan bi gieo ma               |
+---------------------------------+----------------------------------------------+
|  hjkl nav . t hom nay . / tim . g nhay . b bookmark . ? tro giup . q          |
+--------------------------------------------------------------------------------+
```

## Visual Style

**Color palette — terminal-native with selective accents:**
- Background: Terminal default (respects user's dark/light theme)
- Primary text: Terminal default foreground
- Secondary text: Dimmed/gray
- Accent: Warm amber (#D4A855) for section headers, highlights
- Good: Soft green (#6DBF8B)
- Bad/Avoid: Soft coral (#E07070)
- Today marker: Inverted (swap fg/bg)
- Selected: Bold + underline (no background color)
- Weekend: Coral tint
- Holiday: Amber

**Typography:**
- Emoji: only moon (lunar context) and check/cross (guidance markers)
- No emoji in section headers
- Bold for emphasis, not color
- Section dividers: `-- Section Name -----` pattern
- No colored header/footer bars — just text

**Calendar grid:**
- Selected day: bold + underline
- Today: inverted colors
- Lunar date below solar date in dimmed text
- Holidays: amber, weekends: coral

## Interaction Model

All existing keybindings preserved. One addition:

| Key | Action | Notes |
|-----|--------|-------|
| c | Toggle calendar | New. Small screens only — shows/hides calendar overlay |
| hjkl / arrows | Navigate days | Unchanged |
| n/p | Next/prev month | Unchanged |
| N/P | Next/prev year | Unchanged |
| t | Jump to today | Unchanged |
| g | Date jump popup | Unchanged |
| / | Search holidays | Unchanged |
| b/B | Bookmark/list | Unchanged |
| H | Holidays overlay | Unchanged |
| i | Full insight overlay | Unchanged — now expands the Nen/Tranh summary to full list |
| ? | Help | Unchanged |
| q/Esc | Quit/close | Unchanged |

**Key behavior changes:**
- Day guidance (Nen/Tranh) visible by default as summary (top 3-4 items), `i` expands to full list
- Hours panel removed as separate column, shown inline as one-liner
- Calendar hidden by default on small screens, `c` toggles it

## What This Is NOT

- No new features. Purely layout/styling reorganization of existing data.
- No new data sources or API changes.
- No changes to the core calendar logic.
- No changes to overlay content (just where overlays appear over).

## Minimum Terminal Size

- 40x15 remains the minimum (shows error if smaller)
- Below 80 cols: info-card mode (no calendar)
- 80-119: sidebar calendar mode
- 120+: full dashboard mode
