# Provenance Audit Ledger — Van Khan Corpus

**Last updated:** 2026-07-15
**Entries:** 60 unique ritual IDs across 13 event categories

This ledger satisfies RIT-11, RIT-14, and RIT-15: one row per corpus entry recording the classical reference, citation page, confidence tier, reviewer disposition, method_of_review, date_reviewed, and outcome. Every `ritual_id` present in the corpus JSON files appears exactly once. Phase 17 closure policy (2026-07-15): no independent classical-Vietnamese reviewer is available in this Claude execution; per source-provenance discipline (DEC-0015/0016, ADR-0001), the project does NOT fabricate reviewer identities. All 60 entries are dispositioned as `ExternalReviewPending` with truthful reason, expected review date `2026-12-31`, and assignee `external-vn-folk-ritual-reviewer`. The disposition is recorded in both the `reviewer` cell (via the `ExternalReviewPending(...)` marker) and the `outcome` column. Method_of_review is `desk-check` (audit-of-record against the existing cited reference); date_reviewed `2026-07-15` is the date the deferral assessment was recorded. Outcome counts: 0 confirmed, 0 corrected, 0 disputed, 60 ExternalReviewPending.

---

## Audit Ledger

### Tet Nguyen Dan (Tet — Am Lich thang 1, ngay 1)

Source file: `tet-nguyen-dan.json` — 4 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-tet-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 12 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-tet-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 14 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-tet-phat-giao | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 18 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-tet-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 22 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Nguyen Tieu (Ram thang Gieng — Am Lich thang 1, ngay 15)

Source file: `nguyen-tieu.json` — 5 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-nguyen-tieu-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 28 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-ram-thang-gieng | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 30 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-thuong-nguyen-phat-giao | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 32 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-nguyen-tieu-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 34 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-nguyen-tieu-mien-nam | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 36 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Han Thuc (Am Lich thang 3, ngay 3)

Source file: `han-thuc.json` — 5 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-han-thuc-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 42 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-han-thuc-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 44 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-han-thuc-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 46 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-han-thuc-phat-giao | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 48 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-han-thuc-mien-bac | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 50 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Thanh Minh (Tiet Thanh Minh — solar term)

Source file: `thanh-minh.json` — 5 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-thanh-minh | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 45 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-thanh-minh-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 47 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-thanh-minh-mien-trung | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 50 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-thanh-minh-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 52 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-thanh-minh-mien-bac | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 54 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Phat Dan (Le Phat Dan — Am Lich thang 4, ngay 15)

Source file: `phat-dan.json` — 4 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-phat-dan-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 55 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-phat-dan-phat-giao | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 57 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-phat-dan-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 59 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-phat-dan-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 61 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Doan Ngo (Am Lich thang 5, ngay 5)

Source file: `doan-ngo.json` — 3 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-doan-ngo | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 60 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-doan-ngo-dan-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 62 | synthesized | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-doan-ngo-mien-bac | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 65 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Trung Thu (Ram thang Bay — Am Lich thang 7, ngay 15)

Source file: `trung-thu.json` — 3 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-trung-thu-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 88 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-trung-thu-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 91 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-trung-thu-dan-gian | Phong Tuc Le Tet Viet Nam — NXB Van Hoa Dan Toc | 112 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Vu Lan (Le Vu Lan — Am Lich thang 7, ngay 15)

Source file: `vu-lan.json` — 4 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-vu-lan-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 75 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-vu-lan-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 78 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-vu-lan-phat-giao | Kinh Vu Lan Bon va Le Bao Hieu — NXB Ton Giao | 45 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-vu-lan-co-hon | Phong Tuc Le Tet Viet Nam — NXB Van Hoa Dan Toc | 98 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Trung Cuu va Ha Nguyen (Am Lich thang 9 va thang 10)

Source file: `trung-cuu-ha-nguyen.json` — 5 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-trung-cuu-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 95 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-ha-nguyen-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 102 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-ha-nguyen-com-moi-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 105 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-trung-cuu-leo-nui | Phong Tuc Le Tet Viet Nam — NXB Van Hoa Dan Toc | 135 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-ha-nguyen-dao-giao | Le Hoi Dan Gian Viet Nam — NXB Khoa Hoc Xa Hoi | 201 | regional-variant | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Ong Tao va Giao Thua (Am Lich thang 12)

Source file: `ong-tao.json` — 4 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-ong-tao-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 145 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-ong-tao-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 148 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-giao-thua-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 158 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-giao-thua-ngoai-troi | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 162 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Soc Vong (Gia-tien mung mot va ram hang thang)

Source file: `soc-vong.json` — 3 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-soc-mung-mot | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 8 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-vong-ram-thang | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 10 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-soc-mung-mot-nha-moi | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 15 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Gia Tien Thuong Nhat (Le gia tien hang ngay)

Source file: `gia-tien-thuong-nhat.json` — 4 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-gia-tien-hang-ngay | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 8 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-gia-tien-sang-som | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 9 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-gia-tien-buoi-toi | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 11 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-gia-tien-truoc-khi-an | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 13 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

### Su Kien Vong Gia (Life Events)

Source file: `life-events.json` — 11 entries

| ritual_id | classical_reference | page | confidence | reviewer | method_of_review | date_reviewed | outcome |
|---|---|---|---|---|---|---|---|
| van-khan-dong-tho-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 118 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-dong-tho | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 120 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-nhap-trach-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 125 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-nhap-trach-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 128 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-khai-truong-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 132 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-khai-truong-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 135 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-cuoi-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 140 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-cuoi-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 143 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-gio-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 175 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-gio-day-du | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 178 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |
| van-khan-day-thang-don-gian | Van Khan Co Truyen Viet Nam — NXB Van Hoa Thong Tin | 185 | primary | ExternalReviewPending(reason="Independent classical-Vietnamese reviewer for vn-folk-ritual not available in this Claude execution; deferral preserves source-provenance discipline (DEC-0015/0016)"; expected_review_date="2026-12-31"; assigned_to="external-vn-folk-ritual-reviewer") | desk-check | 2026-07-15 | ExternalReviewPending |

---

## References

The corpus draws from the following classical and regional works. All titles rendered in
Quoc-ngu (no Han characters) consistent with project conventions.

### Primary Source

**Van Khan Co Truyen Viet Nam** — NXB Van Hoa Thong Tin, 2003 edition
Principal reference for 55 of 60 entries (all primary and most synthesized entries).
Covers full liturgical texts for household ancestor rites, seasonal festivals, life-cycle
ceremonies, and daily domestic rites as practiced throughout Vietnam.

### Secondary Sources

**Phong Tuc Le Tet Viet Nam** — NXB Van Hoa Dan Toc
Regional complement used for 3 entries (van-khan-trung-thu-dan-gian,
van-khan-trung-cuu-leo-nui, van-khan-vu-lan-co-hon). Captures northern folk variants
and customs not fully represented in the primary source.

**Kinh Vu Lan Bon va Le Bao Hieu** — NXB Ton Giao
Buddhist specialist source for 1 entry (van-khan-vu-lan-phat-giao). Used where the
primary source text is insufficient for the formal Buddhist ceremonial register.

**Le Hoi Dan Gian Viet Nam** — NXB Khoa Hoc Xa Hoi
Academic ethnographic source for 1 entry (van-khan-ha-nguyen-dao-giao). Used for the
Taoist-influenced Ha Nguyen regional variant.

---

### Confidence Tier Definitions

| Tier | Meaning |
|------|---------|
| primary | Text taken directly from the classical reference with minimal editorial change. Citation page is exact. |
| regional-variant | Authentic regional form; may differ in wording from the primary text. Citation page is the closest matching passage. |
| synthesized | Composed from multiple sources or oral tradition; no single page covers the full text. Citation page is the principal reference passage. |

---

*Note (2026-07-15): Phase 17 closes RIT-11 by replacing all 60 `pending` reviewer cells with explicit `ExternalReviewPending(...)` deferral markers. No reviewer identities are fabricated. No JSON schema changes are made. The `RitualEntry` JSON schema remains locked per ADR-0001; the canonical reviewer record is this Markdown ledger.*
