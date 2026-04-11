# Tam Tai (三災 / 三殺)

**Source ID:** `khcbppt` (as 三殺 / San Sha) + Vietnamese adaptation
**Citation:** KHCBPPT, Quyển 3–8, Nghĩa Lệ (義例) — directional taboos; Vietnamese folk rendering as 三災殃
**Confidence:** HIGH
**Decision:** DEC-0021

---

## Definition

Tam Tai ("Three Calamities") is a 3-year affliction period recurring every 12 years. Based on the Tam Hợp (三合) triads and directional opposition. In KHCBPPT, the technical term is 三殺 (San Sha / "Three Killings"), a directional affliction that Vietnamese tradition adapted into a personal year-based system.

## Formula

The 12 Earthly Branches form 4 Tam Hợp triads. Each triad encounters Tam Tai in the 3 years of the **opposite directional group**.

## Lookup Table

| Birth Year Branch (Tam Hợp Group) | Element | Tam Tai Falls in Years | Direction Afflicted |
|---|---|---|---|
| Thân (申), Tý (子), Thìn (辰) | Thủy | Dần, Mão, Thìn | Đông (East) |
| Dần (寅), Ngọ (午), Tuất (戌) | Hỏa | Thân, Dậu, Tuất | Tây (West) |
| Tỵ (巳), Dậu (酉), Sửu (丑) | Kim | Hợi, Tý, Sửu | Bắc (North) |
| Hợi (亥), Mão (卯), Mùi (未) | Mộc | Tỵ, Ngọ, Mùi | Nam (South) |

## Severity of the 3 Years

| Year | Vietnamese | Chinese | Severity |
|---|---|---|---|
| Year 1 | Nhập Tai | 入災 | Lightest — entering calamity |
| Year 2 | Cư Tai | 居災 | **Heaviest** — residing in calamity |
| Year 3 | Xuất Tai | 出災 | Moderate — exiting calamity |

## Application

- Applies to **birth year Earthly Branch** of the person
- Check against the **current calendar year's Earthly Branch**
- Middle year (Cư Tai) is unanimously considered most severe

## Implementation Notes

- Input: birth_year_chi_index (0-11), current_year_chi_index (0-11)
- Output: { in_tam_tai: bool, year_position: Option<1|2|3>, severity: Option<nhap|cu|xuat> }
- The triad grouping uses `chi_index % 4` (same as existing `tam_hop` in xung_hop.rs)
