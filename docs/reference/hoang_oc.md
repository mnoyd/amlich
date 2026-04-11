# Hoàng Ốc (荒屋)

**Source ID:** `vn-folk`
**Citation:** Vietnamese folk tradition (phong tục dân gian)
**Confidence:** HIGH
**Decision:** DEC-0015
**Note:** NOT found in KHCBPPT. Vietnamese folk tradition specific to house construction.

---

## Definition

Hoàng Ốc ("Desolate House") is a Vietnamese age-based taboo for house construction. It determines whether a person's current lunar age is auspicious or inauspicious for building/renovating. One of the "Ba Đại Hạn" (Three Great Taboos) alongside Kim Lâu and Tam Tai.

## The 6-Position Cycle

| Position | Vietnamese | Quality | Meaning |
|---|---|---|---|
| 1 | Nhất Cát (Kiết) | **GOOD** | First Luck |
| 2 | Nhị Nghi | **GOOD** | Second Proper |
| 3 | Tam Địa Sát | **BAD** | Third Earth Killing |
| 4 | Tứ Tấn Tài | **GOOD** | Fourth Advancing Wealth |
| 5 | Ngũ Thọ Tử | **BAD** | Fifth Death |
| 6 | Lục Hoàng Ốc | **BAD** | Sixth Desolate House |

## Formula

```
digit_sum = sum of digits of tuoi_mu (repeat until single digit if needed)
position = digit_sum % 6
if position == 0 → position = 6
if position in {3, 5, 6} → BAD (avoid construction)
```

**Anchor points:** Age 10→Nhất Cát, 20→Nhị Nghi, 30→Tam Địa Sát, 40→Tứ Tấn Tài, 50→Ngũ Thọ Tử, 60→Lục Hoàng Ốc, 70→Nhất Cát (restarts).

## Activities Prohibited

- House construction
- Major renovation
- Breaking ground (động thổ)

## Relationship to Other Systems

- Complementary to Kim Lâu (different formula, same domain: construction)
- Complementary to Kua/Bát Trạch (Kua = "which direction?", Hoàng Ốc = "which year?")
- No overlap with direction systems

## Implementation Notes

- Input: tuoi_mu (lunar age)
- Output: { position: 1-6, name: String, is_good: bool }
- Simple digit-sum + mod-6 calculation
