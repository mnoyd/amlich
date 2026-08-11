# Tài liệu Hướng dẫn Mở rộng Hệ thống Chuyên gia Âm lịch (Expansion Framework)

## 1. Tổng quan (Overview)
Tài liệu này định nghĩa các hướng nghiên cứu và tích hợp kỹ thuật để mở rộng thư viện `amlich-core` từ một công cụ tính toán lịch thuần túy thành một hệ sinh thái tri thức phương Đông toàn diện, đáp ứng nhu cầu văn hóa và tâm linh của người Việt.

Khung được viết để kế thừa các quyết định đã chốt:
- **DEC-0015 / 0016**: Cho phép trộn nguồn nhưng phải tách `source_id` cho mỗi truyền thống.
- **DEC-0022**: Mô hình 3-tier dữ liệu sinh (Tier 0 / 1 / 2). Tier mới (Tier 3 — không gian) được đề xuất ở §3.3.
- **Interaction Layer 5 ma trận** (đã hoàn tất): Day-Person, Element Resonance, Personal Hour, Direction Merge, Domain-Day Boost.

---

## 2. Các Trụ cột Tri thức (Knowledge Pillars)

Mỗi trụ cột khai báo: nguồn (`source_id`), tier yêu cầu, ranh giới với module hiện có.

### 2.1. Hệ thống Tử Vi Đẩu Số (Zi Wei Dou Shu)
**Mục tiêu:** Lập lá số và giải đoán vận mệnh cá nhân chi tiết hơn Bát Tự.
- **`source_id`**: `tu-vi-dau-so` (sách kinh điển: *Tử Vi Đẩu Số Tân Biên* – Vân Đằng Thái Thứ Lang). Lưu ý: KHÔNG trộn với `khcbppt`.
- **Tier yêu cầu**: **Tier 2** bắt buộc (cần ngày + giờ + giới tính). Không degrade được — Tier 0/1 phải trả `Unsupported`.
- **Nội dung nghiên cứu:**
    - **Thuật toán An Sao:** Logic xác định vị trí của 14 chính tinh và 90+ phụ tinh dựa trên ngày/giờ sinh và "Cục".
    - **Hệ thống 12 Cung:** Mệnh, Thân, Phụ, Phúc, Điền, Quan, Nô, Di, Tật, Tài, Tử, Phu Thê.
    - **Tứ Hóa:** Cơ chế biến hóa (Lộc, Quyền, Khoa, Kỵ) theo Thiên Can năm.
- **Hướng triển khai (Rust):**
    - Module mới: `crates/amlich-core/src/ziwei/` (sibling với `bazi/`).
    - Structs: `ZiweiChart`, `Star`, `Palace`, `FourTransformations`.
    - Tích hợp graph: Thêm node `ZiweiStar`, edges `LocatedAt`, `Transforms`.

### 2.2. Kinh Dịch & Chiêm Quẻ (I-Ching Divination)
**Mục tiêu:** Hỗ trợ ra quyết định cho các sự vụ cụ thể thông qua quẻ dịch.
- **`source_id`**: `kinh-dich` (sách: *Kinh Dịch Trọn Bộ* – Ngô Tất Tố) cho 64 quẻ; `mai-hoa-dich-so` cho thuật lập quẻ.
- **Tier yêu cầu**: **Tier 0** đủ — chỉ cần thời điểm hỏi quẻ; không phụ thuộc Bazi cá nhân (nhưng có thể enrich nếu Tier 2).
- **Nội dung nghiên cứu:**
    - **Mai Hoa Dịch Số:** Lập quẻ từ số thời gian hoặc ngoại vật.
    - **Hệ thống 64 Quẻ:** Thoán từ, Hào từ, ý nghĩa cát hung.
    - **Biến Quẻ:** Logic chuyển từ quẻ Chủ sang quẻ Biến qua hào động.
- **Hướng triển khai (Rust):**
    - Module mới: `crates/amlich-core/src/reasoning/iching/`.
    - Tích hợp: Thêm `ConsultationIntent::IChing { question }` và một evaluator nhánh trong `reasoning/personal.rs` → trả `ReasoningEvidenceEnvelope { source_id: "kinh-dich", ... }`.

### 2.3. Phong Thủy Chuyên Sâu (Advanced Geomancy)
**Mục tiêu:** Kết hợp yếu tố "Thời gian" (Lịch) với "Không gian" (Hướng).
- **`source_id`**: `huyen-khong` (*Thẩm Thị Huyền Không Học*) cho Phi Tinh; `bat-trach` (*Bát Trạch Minh Cảnh*) cho Kua. Lưu ý Thái Tuế / Tam Sát đã có sẵn dưới `source_id: khcbppt` — không trùng lặp.
- **Tier yêu cầu**:
    - Bát Trạch (Kua) → **Tier 1** (đã có trong `interaction/direction_merge.rs`).
    - Phi Tinh → **Tier 3** mới (xem §3.3) — cần hướng nhà / tọa độ.
- **Ranh giới với code hiện có:**
    - GIỮ trong `almanac/`: `sat_phuong.rs`, `than_huong.rs`, `thai_tue.rs` (dữ liệu thô theo lịch).
    - GIỮ trong `interaction/`: `direction_merge.rs` (tổng hợp cá nhân hóa).
    - THÊM `almanac/fengshui/flying_stars.rs`: Phi Tinh thuần thời gian (Vận/Năm/Tháng).
    - THÊM `interaction/spatial_compose.rs`: Hợp nhất Phi Tinh + hướng nhà người dùng → khuyến nghị từng phòng.
- **Hướng triển khai (Rust):**
    - Data: `data/almanac/flying_stars.json` (bảng phi tinh Vận 8 → Vận 9).

### 2.4. Văn Hóa Nghi Lễ (Rituals & Traditions)
**Mục tiêu:** Số hóa tri thức về thực hành nghi lễ truyền thống.
- **`source_id`**: `vn-folk-ritual` (phân biệt với `vn-folk` đang dùng cho Hoàng Ốc).
- **Tier yêu cầu**: **Tier 0** (lookup theo ngày lịch).
- **Bản chất**: Content corpus + rule mapping, KHÔNG phải reasoning thuần. Phải có metadata rõ ràng để semantic graph trích xuất.
- **Nội dung nghiên cứu:**
    - **Văn Khấn Cổ Truyền:** Văn khấn theo sự kiện (Lễ Tết, Động thổ, Cưới hỏi, Sóc Vọng).
    - **Nghi thức thực hành:** Lễ vật và trình tự hành lễ.
- **Hướng triển khai (Data + thin module):**
    - Storage: `data/rituals/*.json` (frontmatter chuẩn: `event_type`, `season`, `lunar_date`, `source`).
    - Module: `crates/amlich-core/src/rituals/` chỉ lookup + filter; không suy luận.
    - Feature: Gợi ý văn khấn dựa trên `DaySnapshot.event_type` hiện có.

### 2.5. Y Học & Dưỡng Sinh (Lunar Health)
**Mục tiêu:** Cung cấp bối cảnh văn hóa–lịch sử, phi lâm sàng về liên hệ giờ–kinh và dưỡng sinh bốn mùa.
- **`source_id`**: `shi-er-jing-na-di-zhi` cho bảng *Thập nhị kinh nạp địa chi*; `huangdi-neijing-suwen` cho bốn hồ sơ mùa trong thiên *Tứ khí điều thần đại luận*. Giữ `ty-ngo-luu-chu` cho một milestone tương lai triển khai đúng phép khai huyệt theo ngày/giờ; Tier 0 hiện tại KHÔNG phát source ID này.
- **Tier yêu cầu**: **Tier 0**; không dùng Bazi, triệu chứng, giới tính hay lịch sử sức khỏe.
- **Nội dung nghiên cứu:**
    - **Thập nhị kinh nạp địa chi:** Bảng liên hệ lịch sử giữa 12 khung giờ địa chi và 12 kinh chính; chỉ nói “được gắn với”, không nói cơ quan hoạt động mạnh hay khí huyết đạt đỉnh.
    - **Tứ khí điều thần:** Bốn chủ đề sinh hoạt theo xuân/hạ/thu/đông; phép nối 24 Tiết khí vào bốn mùa là composition minh bạch của Amlich, không phải 24 toa dưỡng sinh riêng.
    - **Ngoài phạm vi:** Tý Ngọ Lưu Chú đầy đủ, huyệt/châm/cứu/bấm huyệt, chẩn đoán, phòng ngừa, điều trị, thực phẩm/dược liệu, và cá nhân hóa Bazi.
- **Hướng triển khai (Rust):**
    - Module sibling phi lâm sàng dưới `reasoning/`, trả về `TraditionalWellnessContext` với disclaimer ổn định, provenance tách biệt và `KnownDivergence`.
    - Đầu vào: `DaySnapshot` + giờ dân sự địa phương; không nhận dữ liệu sinh hay dữ liệu y tế.

---

## 3. Kiến trúc Tích hợp (Integration Architecture)

### 3.1. Source Provenance (bắt buộc)
Mọi rule/fact phải đi qua `ReasoningEvidenceEnvelope` với `source_id` đúng theo bảng §2.x. Tham chiếu `semantic_graph/provenance.rs` cho các constructor (`Provenance::almanac_rule`, `Provenance::interaction`, …). KHÔNG tạo `source_id` ad-hoc trong code mở rộng.

### 3.2. Semantic Graph Extension
1. **Nodes mới:** `ZiweiStar`, `Hexagram`, `FlyingStar`, `TraditionalChannel`, `Ritual`.
2. **Edges mới:** `ConflictsWith`, `Supports`, `OccupiesSector`, `AssociatedWithHourBranch` (Thập nhị kinh nạp địa chi).
3. **Reasoning Rules** — dùng API thực:

```rust
// Pseudo-code dựa trên reasoning/personal.rs hiện có
let envelope = ReasoningEvidenceEnvelope {
    source_id: "rule.composite.opening_ceremony".into(),
    method: "v1.compose".into(),
    notes: vec![
        format!("truc={truc:?}"),
        format!("hexagram={hex:?}"),
        format!("direction={dir:?}"),
    ],
    // ...
};
PersonalRecommendation::from_birth(birth, ConsultationIntent::Opening { date })
    .with_evidence(envelope);
```

Mọi rule tổng hợp đa pillar phải emit evidence với `source_id` bắt đầu bằng `rule.composite.` để phân biệt với evidence nguyên thủy.

### 3.3. Tier 3 — Spatial Data (đề xuất mới)
Phi tinh và phong thủy ứng dụng cần hướng nhà. Đề xuất thêm:

```rust
pub struct SpatialInput {
    pub facing_direction: Direction24,  // 24 sơn
    pub period_vận: u8,                  // Vận 8, 9, ...
    pub rooms: Vec<RoomLocation>,        // optional
}
```

Tier 3 = Tier 2 + `SpatialInput`. Pillar nào yêu cầu Tier 3 phải trả `Unsupported` nếu thiếu — KHÔNG fabricate default direction.

---

## 4. Nguồn Tài liệu Tham chiếu (Sources of Truth)

| Trụ cột | Sách / Truyền thống | `source_id` |
|---|---|---|
| Tử Vi | *Tử Vi Đẩu Số Tân Biên* (Vân Đằng Thái Thứ Lang) | `tu-vi-dau-so` |
| Kinh Dịch | *Kinh Dịch Trọn Bộ* (Ngô Tất Tố) | `kinh-dich` |
| Mai Hoa | *Mai Hoa Dịch Số* (Thiệu Khang Tiết) | `mai-hoa-dich-so` |
| Phi Tinh | *Thẩm Thị Huyền Không Học* | `huyen-khong` |
| Bát Trạch | *Bát Trạch Minh Cảnh* | `bat-trach` |
| Văn khấn | Tổng tập văn khấn cổ truyền VN | `vn-folk-ritual` |
| Thập nhị kinh nạp địa chi | *Châm Cứu Đại Toàn* / bảng `十二經納地支歌` | `shi-er-jing-na-di-zhi` |
| Dưỡng sinh bốn mùa | *Hoàng Đế Nội Kinh · Tố Vấn · Tứ khí điều thần đại luận* | `huangdi-neijing-suwen` |

Các `source_id` đã chốt từ trước (`khcbppt`, `ngoc-hap-ky`, `vn-folk`, `cuu-dieu`, `tam-menh-thong-hoi`) KHÔNG được tái sử dụng cho nội dung mới.

---

## 5. Sequencing & Priorities

Thứ tự đề xuất theo (giá trị UX) × (effort) × (tier rào cản):

| Phase | Pillar | Lý do ưu tiên | Effort | Tier |
|---|---|---|---|---|
| P1 | 2.4 Văn khấn | Low risk, high UX, không cần thuật toán mới | S | T0 |
| P2 | 2.2 Kinh Dịch (Mai Hoa) | 64 quẻ là bảng tra hữu hạn; tích hợp `ConsultationIntent` rõ ràng | M | T0 |
| P3 | 2.5 Traditional Wellness Context | Hai corpus hữu hạn (12 liên hệ giờ–kinh + 4 hồ sơ mùa); integration nhẹ, phi lâm sàng | M | T0 |
| P4 | 2.3 Phi Tinh (thời gian) | Bảng phi tinh hữu hạn; chưa cần Tier 3 | M | T0 |
| P5 | 2.3 Phi Tinh + Spatial (Tier 3) | Cần thiết kế lại model birth/spatial | L | T3 |
| P6 | 2.1 Tử Vi Đẩu Số | Thuật toán An Sao rất phức tạp; 100+ sao | XL | T2 |

Không bắt đầu P5/P6 trước khi P1–P4 ổn định và `Tier 3 model` được chốt qua DEC mới.

---

## 6. Quy trình Phát triển (Development Workflow)

1.  **Nghiên cứu (Research):** Tổng hợp thuật toán từ sách cổ; tạo `.planning/research/<pillar>.md`.
2.  **Số hóa (Data):** Bảng tra → JSON dưới `data/`; mỗi file kèm metadata `source_id`.
3.  **Lập trình (Code):** Module Rust + unit test golden (đối chiếu phần mềm uy tín ở §7).
4.  **Tích hợp (Graph):** Thêm nodes/edges; emit evidence qua `ReasoningEvidenceEnvelope`.
5.  **Kiểm chứng (Validation):** Cross-check với danh sách ở §7. Sai số > 0 phải có ADR giải thích.

---

## 7. Validation References (phần mềm/website đối chiếu)

| Pillar | Reference |
|---|---|
| Tử Vi | thienco.com, tuvi.vn, lyhocdongphuong.org.vn |
| Kinh Dịch | nhantu.net (Mai Hoa), divination.com (hexagram texts) |
| Phi Tinh | fengshui.net (vận hành phi tinh), phongthuyhomemy.com |
| Bát Trạch | Đối chiếu với `interaction/direction_merge.rs` test fixtures hiện có |
| Traditional Wellness | Bảng `十二經納地支歌` trong *Châm Cứu Đại Toàn*; `四氣調神大論` trong *Hoàng Đế Nội Kinh · Tố Vấn* |
| Văn khấn | Đối chiếu với *Văn khấn cổ truyền Việt Nam* (NXB Văn Hóa Dân Tộc) |

Golden test required cho mỗi pillar: tối thiểu 10 ca, đối chiếu ≥ 2 nguồn độc lập; sai lệch phải log dưới dạng `KnownDivergence` chứ không "fix" về phía nguồn nào.
