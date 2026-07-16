# Tam Sát (三殺) Directional Rule — Provenance Ledger

**Status:** PendingExternalReview for exact KHCBPPT edition/page pin.
**Module:** `crates/amlich-core/src/almanac/tam_sat.rs`
**Requirement:** XLK-02 (Phase 23 Plan 23-01 Task 2)
**Last reviewed:** 2026-07-16

## Rule statement

For each year Earthly Branch, the Tam Sát (三殺 — classical "three killings"
directional taboo) is the set of three 8-point directions obtained by:

1. Identifying the year's **Tam Hợp triad** (the same four triads used by
   `tam_tai.rs::TAI_YEARS` and `xung_hop::tam_hop`):
   - **Water (Thủy):** Thân · Tý · Thìn
   - **Wood  (Mộc):** Hợi · Mão · Mùi
   - **Fire  (Hỏa):** Dần · Ngọ · Tuất
   - **Metal (Kim):** Tỵ · Dậu · Sửu
2. Taking the **opposite** (lục-xung / 六冲) Tam Hợp triad — each branch is
   shifted by 6 modulo 12. This mirrors `tam_tai.rs:58-63` `TAI_YEARS`
   exactly.
3. Mapping each opposite-triad branch to its 8-point `Direction` cell per
   the locked branch-to-direction collapse:

   | Branches                | Direction (VN)  | `Direction` variant |
   | ----------------------- | --------------- | ------------------- |
   | Tý(0)                   | Bắc             | `North`             |
   | Sửu(1), Dần(2)          | Đông Bắc        | `Northeast`         |
   | Mão(3)                  | Đông            | `East`              |
   | Thìn(4), Tỵ(5)          | Đông Nam        | `Southeast`         |
   | Ngọ(6)                  | Nam             | `South`             |
   | Mùi(7), Thân(8)         | Tây Nam         | `Southwest`         |
   | Dậu(9)                  | Tây             | `West`              |
   | Tuất(10), Hợi(11)       | Tây Bắc         | `Northwest`         |

## Locked mapping table

| Tam Hợp triad      | Element | Tam Sát branches (opposite) | Tam Sát directions                  |
| ------------------ | ------- | --------------------------- | ----------------------------------- |
| Thân · Tý · Thìn   | Thủy    | Dần · Ngọ · Tuất            | Đông Bắc, Nam, Tây Bắc              |
| Hợi · Mão · Mùi    | Mộc     | Tỵ · Dậu · Sửu              | Đông Nam, Tây, Đông Bắc             |
| Dần · Ngọ · Tuất   | Hỏa     | Thân · Tý · Thìn            | Tây Nam, Bắc, Đông Nam              |
| Tỵ · Dậu · Sửu     | Kim     | Hợi · Mão · Mùi             | Tây Bắc, Đông, Tây Nam              |

## Source citation

**Tradition:** KHCBPPT — *Khâm Định Hiệp Kỷ Biện Phương Thư* (欽定協紀辨方書),
the Qian-long-era imperial Vietnamese almanac reference. Cited by title and
tradition throughout the codebase (`SOURCE_KHCBPPT = "khcbppt"` in
`crates/amlich-core/src/sources.rs`).

**Exact edition / page pin:** **PendingExternalReview.**

### Why deferred

- The Tam Sát triad → 3-direction mapping is the classical, widely-attested
  rule (lục-xung opposite triad); the algorithmic shape is uncontroversial
  and is mirrored verbatim from the existing `tam_tai.rs::TAI_YEARS`
  precedent.
- The exact KHCBPPT *Quyển* + *Trang* + *Câu* pin for the directional rule
  is **not** in the codebase, and external WebSearch attempts against
  vi.wikipedia.org/wiki/Tam_tai and related pages returned 404 / CAPTCHA
  during Phase 23 research.
- A future reviewer with access to a physical KHCBPPT copy should locate
  the directional section (typically Quyển 9, Lập Thành 立成 — the same
  section cited for the directional Thái Tuế in `thai_tue.rs`) and update
  the `evidence.profile` text in `tam_sat.rs` to cite the exact page.

### Upgrade path

Once verified, the upgrade lands as:

1. Update `evidence.profile` in `tam_sat.rs::tam_sat_direction` to drop the
   `PendingExternalReview` marker and insert the exact `Quyển N, trang M`
   citation.
2. Update this ledger's **Status** line to `Confirmed — KHCBPPT Quyển N, trang M`.
3. Supersede with an ADR amendment if the citation reveals a sub-school
   variance (mirrors `ADR-0006 §5` page-citation deferral pattern).

## Related artifacts

- `crates/amlich-core/src/almanac/tam_sat.rs` — runtime Rust implementation.
- `crates/amlich-core/src/almanac/tam_tai.rs:58-63` — `TAI_YEARS` precedent
  for the lục-xung opposite-triad concept (Tam Tai three-year cycle).
- `crates/amlich-core/src/almanac/xung_hop.rs::tam_hop` — Tam Hợp triad lookup.
- `.planning/phases/23-th-i-tu-tam-s-t-phi-tinh-cross-link/23-CONTEXT.md`
  §"Tam Sát triad → 3-direction mapping" — locked mapping decision.

## Search criteria (for future reviewer)

When verifying against a physical KHCBPPT copy, search for:

- "三殺" / "Tam Sát" in the directional chapter (typically Quyển 9).
- The four Tam Hợp triad labels paired with directional proscriptions.
- Confirm the rule is the lục-xung opposite-triad mapping (not Tam Tai's
  three-year cycle, despite the shared Chinese name 三殺).
