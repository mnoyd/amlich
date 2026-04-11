# Kim Lâu (金樓)

**Source ID:** `ngoc-hap-ky`
**Citation:** Ngọc Hạp Ký (玉匣記), "Trạch Cát Tu Tạo" (擇吉修造) section
**Confidence:** HIGH
**Decision:** DEC-0015
**Note:** NOT found in KHCBPPT. Vietnamese folk geomancy tradition from Ngọc Hạp Ký lineage.

---

## Definition

Kim Lâu ("Golden Tower") is a Vietnamese age-based taboo system marking certain ages as inauspicious for major life events, primarily marriage and house construction.

## Formula

```
tuoi_mu = current_lunar_year - birth_lunar_year + 1
remainder = tuoi_mu % 9
if remainder in {1, 3, 6, 8} → Kim Lâu year
```

## The 4 Categories

| Remainder | Category | Vietnamese | Harms | Severity |
|---|---|---|---|---|
| 1 | Kim Lâu Thân | 金樓身 | The person themselves | Most severe |
| 3 | Kim Lâu Thê | 金樓妻 | The spouse | Severe |
| 6 | Kim Lâu Tử | 金樓子 | Children | Moderate |
| 8 | Kim Lâu Súc | 金樓畜 | Livestock / property | Lightest |

Safe remainders: 0 (=9), 2, 4, 5, 7.

## Gender Application

- **Marriage/wedding:** Check the **woman's** lunar age
- **House construction:** Check the **man's** (homeowner's) lunar age
- Vietnamese proverb: "Lấy vợ xem tuổi đàn bà, làm nhà xem tuổi đàn ông"

## Activities Prohibited

- Marriage / wedding ceremony
- House construction / major renovation
- Land purchase

## Known Disagreements

1. **Mod-9 vs digit-sum:** Most sources use mod-9. Digit-sum produces same results for ages 1-9 but can diverge for higher ages. Use mod-9.
2. **Kim Lâu Súc (remainder 8):** Some sources suggest it can be disregarded if one does not raise livestock professionally. Conservative approach: always flag it.

## Implementation Notes

- Input: birth_lunar_year, current_lunar_year, gender
- Output: { in_kim_lau: bool, category: Option<than|the|tu|suc>, remainder: u8 }
- Gender determines which person to check for which activity type
