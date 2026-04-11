# Can Chi Giờ — Ngũ Thử Độn Thời (五鼠遁時)

**Source ID:** `khcbppt`
**Citation:** KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — 日上起時法 / 五鼠遁日起時表
**Confidence:** HIGH
**Decision:** DEC-0017

---

## Mnemonic Verse

```
甲己還加甲    Giáp Kỷ hoàn gia Giáp
乙庚丙作初    Ất Canh Bính tác sơ
丙辛從戊起    Bính Tân tùng Mậu khởi
丁壬庚子居    Đinh Nhâm Canh Tý cư
戊癸何方發    Mậu Quý hà phương phát
壬子是真途    Nhâm Tý thị chân đồ
```

## Lookup Table: Day Stem → Starting Hour Stem for Tý (子) Hour

| Day Stem Pair | Starting Stem for Giờ Tý |
|---|---|
| Giáp (甲) / Kỷ (己) | **Giáp** → Giáp Tý (甲子) |
| Ất (乙) / Canh (庚) | **Bính** → Bính Tý (丙子) |
| Bính (丙) / Tân (辛) | **Mậu** → Mậu Tý (戊子) |
| Đinh (丁) / Nhâm (壬) | **Canh** → Canh Tý (庚子) |
| Mậu (戊) / Quý (癸) | **Nhâm** → Nhâm Tý (壬子) |

## Hour Sequence Generation

From the starting stem, advance through the 10 stems in order for each subsequent branch:

```
Tý → Sửu → Dần → Mão → Thìn → Tỵ → Ngọ → Mùi → Thân → Dậu → Tuất → Hợi
```

Since 12 hours > 10 stems, the last 2 hours (Tuất, Hợi) wrap around to the beginning of the stem cycle.

## Example

Day Stem = Đinh → Starting stem = Canh:
```
Canh-Tý, Tân-Sửu, Nhâm-Dần, Quý-Mão, Giáp-Thìn, Ất-Tỵ,
Bính-Ngọ, Đinh-Mùi, Mậu-Thân, Kỷ-Dậu, Canh-Tuất, Tân-Hợi
```

## Underlying Logic

Day Stems pair by Hợp relationship (Giáp-Kỷ, Ất-Canh, Bính-Tân, Đinh-Nhâm, Mậu-Quý). Starting stems advance by 2 positions per pair: Giáp→Bính→Mậu→Canh→Nhâm (all Yang stems, because Tý is Yang branch). Full cycle = 60 stem-branch combinations over 5 days.

## Tý Hour Boundary Convention (DEC-0017)

**v1 convention: 23:00 = start of new day (整子時 / "whole Tý")**

| School | Tý Starts At | Day Changes At | Note |
|---|---|---|---|
| **整子時 (Whole Tý) ← v1** | 23:00 | 23:00 | Vietnamese mainstream practice |
| 早子時/夜子時 (Split Tý) | 23:00 | 00:00 | Creates 13 possible hour pillars/day |

The split-Tý method is documented as a known variant but not implemented in v1.
