# Cửu Diệu (九曜 / Nine Stars)

**Source ID:** `cuu-dieu`
**Citation:** Buddhist/Indian astronomical tradition (宿曜道 Sukuyōdō); Vietnamese practice
**Confidence:** HIGH (lookup tables), MEDIUM (female "reverse" characterization)
**Decision:** DEC-0016
**Note:** NOT directly from KHCBPPT. Originates from Indian Navagraha system transmitted via Buddhism.

---

## The Nine Stars

| # | Vietnamese | Sino-Vietnamese | Chinese | Celestial Body | Element | Quality |
|---|---|---|---|---|---|---|
| 1 | La Hầu | 羅睺 | Rahu | Ascending lunar node | Kim | **HUNG** |
| 2 | Thổ Tú | 土宿 | Saturn | Saturn | Thổ | Trung |
| 3 | Thủy Diệu | 水曜 | Mercury | Mercury | Thủy | Trung |
| 4 | Thái Bạch | 太白 | Venus | Venus | Kim | **HUNG** |
| 5 | Thái Dương | 太陽 | Sun | Sun | Hỏa | **CÁT** |
| 6 | Vân Hớn / Hỏa Tinh | 雲漢 / 火曜 | Mars | Mars | Hỏa | Trung |
| 7 | Kế Đô | 計都 | Ketu | Descending lunar node | Thổ | **HUNG** |
| 8 | Thái Âm | 太陰 | Moon | Moon | Thủy | **CÁT** |
| 9 | Mộc Đức | 木德 | Jupiter | Jupiter | Mộc | **CÁT** |

**Quality summary:** 3 Cát (Thái Dương, Thái Âm, Mộc Đức), 3 Trung (Thổ Tú, Thủy Diệu, Vân Hớn), 3 Hung (La Hầu, Thái Bạch, Kế Đô).

## Formula

```
tuoi_mu = current_lunar_year - birth_lunar_year + 1
remainder = tuoi_mu % 9
if remainder == 0 → remainder = 9
star = lookup[gender][remainder]
```

## Male Lookup Table (Nam — thuận)

| Remainder | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|---|---|---|---|---|---|---|---|---|---|
| Star | La Hầu | Thổ Tú | Thủy Diệu | Thái Bạch | Thái Dương | Vân Hớn | Kế Đô | Thái Âm | Mộc Đức |

## Female Lookup Table (Nữ — specific mapping, not simple reverse)

| Remainder | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 |
|---|---|---|---|---|---|---|---|---|---|
| Star | Kế Đô | Vân Hớn | Mộc Đức | Thái Âm | Thổ Tú | La Hầu | Thái Dương | Thái Bạch | Thủy Diệu |

## Star Affliction Details (Sao Hạn)

| Star | Worst Months (Lunar) | Folk Saying |
|---|---|---|
| La Hầu | 3, 7 | "Nam La Hầu" — hits men hardest |
| Kế Đô | 3, 9 | "Nữ Kế Đô" — hits women hardest |
| Thái Bạch | 5, 11 | Financial ruin, mourning |

## Remediation Days (Cúng Sao)

| Star | Day of Lunar Month |
|---|---|
| La Hầu | 8th |
| Kế Đô | 18th |
| Thái Bạch | 15th |

## Implementation Notes

- Input: birth_lunar_year, current_lunar_year, gender
- Output: { star_index: 1-9, star_name: String, quality: cat|trung|hung, is_han: bool }
- `is_han = true` when star is La Hầu, Kế Đô, or Thái Bạch
- Cycle repeats every 9 years; age 1 always starts at La Hầu (male) / Kế Đô (female)
