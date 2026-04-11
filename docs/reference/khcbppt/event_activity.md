# Event/Activity Classification — KHCBPPT 用事

**Source ID:** `khcbppt`
**Citation:** KHCBPPT, Quyển 10 (宜忌) + Quyển 11 (用事)
**Confidence:** HIGH (activity lists from Wikisource/Siku Quanshu edition)
**Decision:** DEC-0019, DEC-0020

---

## v1 Baseline: 37 Dân Dụng (民用三十七事)

| # | Chinese | Sino-Vietnamese | Vietnamese Common | Current Activity ID |
|---|---|---|---|---|
| 1 | 祭祀 | Tế tự | Cúng tế | — |
| 2 | 上表章 | Thượng biểu chương | Trình văn thư | — |
| 3 | 上官 | Thượng quan | Nhậm chức | — |
| 4 | 入學 | Nhập học | Nhập học | — |
| 5 | 冠帶 | Quan đới | Lễ trưởng thành | — |
| 6 | 結婚姻 | Kết hôn nhân | Đính hôn | WeddingEngagement |
| 7 | 會親友 | Hội thân hữu | Gặp gỡ bạn bè | MeetingSocial |
| 8 | 嫁娶 | Giá thú | Cưới hỏi | WeddingEngagement |
| 9 | 進人口 | Tấn nhân khẩu | Nhận thêm người | — |
| 10 | 出行 | Xuất hành | Đi xa | Travel |
| 11 | 移徙 | Di tỉ | Dọn nhà | MoveRelocation |
| 12 | 安牀 | An sàng | Kê giường | — |
| 13 | 沐浴 | Mộc dục | Tắm gội (lễ) | — |
| 14 | 剃頭 | Thế đầu | Cắt tóc | — |
| 15 | 療病 | Liệu bệnh | Chữa bệnh | MedicalTreatment |
| 16 | 裁衣 | Tài y | May quần áo | — |
| 17 | 修造動土 | Tu tạo động thổ | Xây/sửa nhà | ConstructionGroundbreaking |
| 18 | 豎柱上梁 | Thụ trụ thượng lương | Dựng cột, gác đòn tay | ConstructionGroundbreaking |
| 19 | 經絡 | Kinh lạc | Đặt đường đi | — |
| 20 | 開市 | Khai thị | Khai trương | OpeningStart |
| 21 | 立券 | Lập khoán | Ký hợp đồng | ContractAgreement |
| 22 | 交易 | Giao dịch | Mua bán | FinanceInvestment |
| 23 | 納財 | Nạp tài | Thu tiền | FinanceInvestment |
| 24 | 修置產室 | Tu trí sản thất | Sửa cơ sở sản xuất | — |
| 25 | 開渠穿井 | Khai cừ xuyên tỉnh | Đào kênh/giếng | — |
| 26 | 安碓磑 | An đối ngại | Lắp máy xay | — |
| 27 | 掃舍宇 | Tảo xá vũ | Quét dọn nhà | — |
| 28 | 平治道塗 | Bình trị đạo đồ | San đường | — |
| 29 | 破屋壞垣 | Phá ốc hoại viên | Phá dỡ nhà | — |
| 30 | 伐木 | Phạt mộc | Đốn cây | — |
| 31 | 捕捉 | Bổ tróc | Bắt giữ | — |
| 32 | 畋獵 | Điền liệp | Săn bắn | — |
| 33 | 栽種 | Tài chủng | Trồng trọt | — |
| 34 | 牧養 | Mục dưỡng | Chăn nuôi | — |
| 35 | 破土 | Phá thổ | Động thổ (an táng) | — |
| 36 | 安葬 | An táng | Chôn cất | — |
| 37 | 啟攢 | Khải toản | Cải táng | — |

Activities marked with `—` need new activity IDs.

## Precedence Rules (KHCBPPT Q10 — 宜忌)

### 3-Tier Resolution

| Tier | Chinese | Vietnamese | Rule |
|---|---|---|---|
| 1 | 吉足勝凶 | Cát thắng Hung | Auspicious spirits outnumber/outweigh → follow auspicious |
| 2 | 吉凶相抵 | Cát Hung cân bằng | Balanced → **avoid** for important events (hung wins ties) |
| 3 | 吉不足勝凶 | Hung thắng Cát | Inauspicious prevails → follow taboo |

### Absolute Hard-Stops (no override possible)

| Spirit | Chinese | Effect |
|---|---|---|
| Tuế Phá | 歲破 | Year-branch opposition — all activities taboo |
| Nguyệt Phá | 月破 | Month-branch opposition — all activities taboo |

### Major Auspicious Spirits (high weight)

Thiên Đức (天德), Nguyệt Đức (月德), Thiên Ân (天恩), Thiên Quý (天貴)

### Five Historical Schools (harmonized by KHCBPPT)

1. 建除家 Kiến Trừ (12 Trực) — day officers
2. 堪輿家 Kham Dư (Feng Shui) — directional analysis
3. 叢辰家 Tùng Thần — complex star combinations
4. 五行家 Ngũ Hành — Five Elements
5. 曆家 Lịch — calendar/astronomical

## Implementation Notes

- The KHCBPPT system is fundamentally **qualitative**, not quantitative
- Current numeric scoring (DEC-0020) remains as operational layer
- Add validation: when numeric score says "good" but qualitative says "hung thắng cát", flag divergence
- Hard-stops (Tuế Phá, Nguyệt Phá) override everything, aligns with DEC-0014
