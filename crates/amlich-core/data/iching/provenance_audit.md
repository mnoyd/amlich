# Provenance Audit Ledger — IChing (Kinh Dịch) Corpus

**Last updated:** 2026-07-16
**Entries:** 64 hexagrams (King Wen sequence 1..=64)
**Source:** *Kinh Dịch Trọn Bộ* — Ngô Tất Tố (SOURCE_KINH_DICH = "kinh-dich")

Lifecycle policy: [`docs/architecture/external-review-lifecycle.md`](../../../../docs/architecture/external-review-lifecycle.md).

This ledger satisfies ICH-01 success criterion 3: one row per corpus entry recording the classical reference, confidence tier, reviewer disposition, method_of_review, date_reviewed, and outcome. Every `king_wen_index` present in `hexagrams.json` appears exactly once. Phase 21 closure policy (2026-07-16): no independent classical-Vietnamese reviewer for the kinh-dich source is available in this Claude execution; per source-provenance discipline (DEC-0015/0016) and AF-05 ("never silently filled from another translator"), the project does NOT fabricate reviewer identities or silently fill interpretive text from another translator. All 64 entries are dispositioned as `ExternalReviewPending` with truthful reason, expected review date `2026-12-31`, and assignee `external-kinh-dich-reviewer`. The disposition is recorded in both the `reviewer` cell (via the `ExternalReviewPending(...)` marker) and the `outcome` column. Method_of_review is `desk-check` (audit-of-record against the existing cited reference); date_reviewed `2026-07-16` is the date the deferral assessment was recorded. Outcome counts: 0 confirmed, 0 corrected, 0 disputed, 64 ExternalReviewPending.

Per ADR-0005 §4, the canonical per-entry reviewer record is the `reviewer: String` field on each `HexagramEntry` in `hexagrams.json`. This Markdown ledger mirrors that data for human-readable audit; the two must be consistent (same ExternalReviewPending marker string, same expected_review_date, same assignee).

---

## Audit Ledger

### Hexagrams #1-8

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 1 | Thuần Kiền | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 2 | Thuần Khôn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 3 | Truân | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 4 | Mông | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 5 | Nhu | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 6 | Tụng | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 7 | Sư | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 8 | Tỷ | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #9-16

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 9 | Tiểu Súc | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 10 | Lữ | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 11 | Thái | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 12 | Bỉ | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 13 | Đồng Nhân | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 14 | Đại Hữu | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 15 | Khiêm | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 16 | Dự | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #17-24

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 17 | Tùy | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 18 | Cổ | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 19 | Lâm | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 20 | Quan | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 21 | Phệ Hạp | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 22 | Bí | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 23 | Bác | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 24 | Phục | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #25-32

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 25 | Vô Vọng | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 26 | Đại Súc | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 27 | Di | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 28 | Đại Quá | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 29 | Thuần Khảm | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 30 | Thuần Ly | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 31 | Hàm | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 32 | Hằng | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #33-40

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 33 | Độn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 34 | Đại Tráng | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 35 | Tấn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 36 | Minh Di | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 37 | Gia Nhân | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 38 | Khuê | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 39 | Kiển | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 40 | Giải | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #41-48

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 41 | Tổn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 42 | Ích | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 43 | Quải | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 44 | Cấu | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 45 | Tụy | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 46 | Thăng | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 47 | Khốn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 48 | Tỉnh | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #49-56

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 49 | Cách | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 50 | Đỉnh | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 51 | Thuần Chấn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 52 | Thuần Cấn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 53 | Tiệm | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 54 | Quy Muội | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 55 | Phong | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 56 | Lữ | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

### Hexagrams #57-64

| king_wen_index | vi_name | classical_reference | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| 57 | Thuần Tốn | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 58 | Thuần Đoài | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 59 | Hoán | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 60 | Tiết | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 61 | Trung Phu | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 62 | Tiểu Quá | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 63 | Ký Tế | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |
| 64 | Vị Tế | Kinh Dịch Trọn Bộ — Ngô Tất Tố | pending | ExternalReviewPending(reason="Ngô Tất Tố Kinh Dịch Trọn Bộ interpretive text not available in this Claude execution; AF-05 forbids filling from another translator"; expected_review_date="2026-12-31"; assigned_to="external-kinh-dich-reviewer") | desk-check | 2026-07-16 | ExternalReviewPending |

---

## References

### Primary Source

**Kinh Dịch Trọn Bộ** — Ngô Tất Tố (SOURCE_KINH_DICH = `"kinh-dich"`)
Principal (and sole) reference for all 64 hexagram corpus entries. Covers the thoái từ (彖辭 judgment), hào từ (爻辭 line texts — six per hexagram, plus the seventh dụng cửu/dụng lục line for hexagrams #1 Kiền and #2 Khôn), and cát hung (吉凶 verdict) for each of the 64 King Wen hexagrams. Pending external review: the interpretive text is not available in this Claude execution and per AF-05 is NOT silently filled from another translator (Richard Wilhelm, Gregory Whincup, et al.).

### Closure-Pattern + Schema Precedents

- **ADR-0005 (HexagramEntry schema v1)** — locks the field set, the `hao_tu` length rule (6 for #3..=64; 7 for #1 & #2), the reviewer free-text marker shape, the `HauThienTrigram` Lo Shu encoding pin, and the naming-convention divergence from the rituals schema.
- **AF-05** — the v1.7 anti-fabrication rule: interpretive text gaps are logged as `PendingExternalReview`, never silently filled from another translator.
- **`data/rituals/provenance_audit.md` (Phase 17 closure)** — the closure-pattern precedent mirrored here: 60/60 entries dispositioned `ExternalReviewPending` when no independent reviewer is available in the current Claude execution.

### Confidence Tier Definitions

| Tier | Meaning |
|------|---------|
| pending | Interpretive text not yet sourced from the primary reference; entry is a structural skeleton awaiting external review. |

---

*Note (2026-07-16): Phase 21 Plan 21-01 closes the data half of ICH-01 by authoring all 64 corpus entries with explicit `ExternalReviewPending(...)` deferral markers. No reviewer identities are fabricated. The canonical reviewer record is the per-entry `reviewer: String` field on `HexagramEntry`; this Markdown ledger is the aggregate audit view.*
