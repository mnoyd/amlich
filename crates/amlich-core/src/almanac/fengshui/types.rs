//! Locked v1 Phi Tinh types. See ADR-0002 and ADR-0003.
//!
//! FIELD SET LOCKED — any changes require a superseding ADR.
//! Phase 13 fills algorithms. Phase 14 fills star-pair aspects.

use serde::{Deserialize, Serialize};

use crate::reasoning::{ReasoningEvidenceEnvelope, ReasoningEvidenceSourceFamily};

/// Canonical Lo Shu palace numbering (FS-02).
///
/// Variant values correspond to their Lo Shu numbers 1..=9.
/// The ordering N=1, SW=2, E=3, SE=4, Center=5, NW=6, W=7, NE=8, S=9
/// is used for `[FlyingStar; 9]` array indexing in `FlyingStarLayout`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum Palace {
    N = 1,
    SW = 2,
    E = 3,
    SE = 4,
    Center = 5,
    NW = 6,
    W = 7,
    NE = 8,
    S = 9,
}

impl Palace {
    /// All palaces in Lo Shu number order (1..=9).
    /// Index i in this array corresponds to the palace with Lo Shu number i+1.
    pub const ALL: [Palace; 9] = [
        Palace::N,
        Palace::SW,
        Palace::E,
        Palace::SE,
        Palace::Center,
        Palace::NW,
        Palace::W,
        Palace::NE,
        Palace::S,
    ];
}

/// Direction-string slug for a palace.
///
/// Phase 10 stub returning English compass abbreviations.
/// Phase 13 may refine to add Vietnamese localisation while keeping this
/// function signature intact (it is part of the frozen Phase 10 API surface).
pub fn palace_to_direction(p: Palace) -> &'static str {
    match p {
        Palace::N => "N",
        Palace::SW => "SW",
        Palace::E => "E",
        Palace::SE => "SE",
        Palace::Center => "Center",
        Palace::NW => "NW",
        Palace::W => "W",
        Palace::NE => "NE",
        Palace::S => "S",
    }
}

/// Nine canonical Phi Tinh stars (FS-03).
///
/// Variant values are the star's classical number 1..=9.
/// Metadata (element / polarity / auspice) is loaded by Phase 13 from
/// `data/almanac/flying_stars.json` and a dedicated loader. Phase 10
/// declares the enum only — no metadata logic here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FlyingStar {
    /// 1 White (Nhất Bạch)
    NhatBach = 1,
    /// 2 Black (Nhị Hắc)
    NhiHac = 2,
    /// 3 Jade/Green (Tam Bích)
    TamBich = 3,
    /// 4 Green (Tứ Lục)
    TuLuc = 4,
    /// 5 Yellow — danger per FS-14 (Ngũ Hoàng)
    NguHoang = 5,
    /// 6 White (Lục Bạch)
    LucBach = 6,
    /// 7 Red (Thất Xích)
    ThatXich = 7,
    /// 8 White (Bát Bạch)
    BatBach = 8,
    /// 9 Purple (Cửu Tử)
    CuuTu = 9,
}

/// Period discriminator for `FlyingStarLayout` (FND-02).
///
/// - `Van` = era-level base palace (Vận 7 / Vận 8 / Vận 9 …).
/// - `Yearly` = annual Niên Tử Bạch anchored at Lập Xuân per ADR-0003.
/// - `Monthly` = monthly Nguyệt Tử Bạch anchored at solar terms per ADR-0002.
/// - `Daily` = Lưu Nhật Phi Tinh (日紫白) pivoted on 6 Trung Khí (Đông Chí / Vũ Thuỷ / Cốc Vũ / Hạ Chí / Xử Thử / Sương Giáng), seeded at the first Giáp Tý per pivot per ADR-0004.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FlyingStarPeriod {
    Van { van: u8 },
    Yearly { year: i32 },
    Monthly { year: i32, month: u8 },
    Daily { date: (i32, u32, u32) },
}

/// Frozen v1 Phi Tinh layout shape (FND-02).
///
/// FIELD SET LOCKED — any field-set change requires a superseding ADR.
///
/// `palaces` is indexed in `Palace::ALL` order:
/// index 0 = N (Lo Shu 1), index 1 = SW (2), index 2 = E (3),
/// index 3 = SE (4), index 4 = Center (5), index 5 = NW (6),
/// index 6 = W (7), index 7 = NE (8), index 8 = S (9).
///
/// NOTE: per PITFALLS CRIT-3 and CONTEXT.md, `FlyingStarLayout` is a
/// palace-layout descriptor and is NEVER wired into
/// `interaction/direction_merge.rs` in v1.5.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlyingStarLayout {
    pub period: FlyingStarPeriod,
    /// Flying star assigned to each palace, indexed per `Palace::ALL` order.
    pub palaces: [FlyingStar; 9],
    pub center_star: FlyingStar,
    pub evidence: ReasoningEvidenceEnvelope,
}

/// Sibling layout struct for the daily Phi Tinh (Lưu Nhật / 日紫白) layer.
///
/// The locked v1 `FlyingStarLayout` field set (above) is NOT mutated — this sibling
/// carries the daily-layer-specific period variant while sharing the `palaces` /
/// `center_star` / `evidence` shape. Phase 18-01 (FS-17) schema-lock step; populated
/// by `compute_daily_flying_stars` (Plan 18-02) and surfaced via
/// `DaySnapshot.daily_flying_stars` (Plan 18-04).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyFlyingStarLayout {
    pub period: FlyingStarPeriod,
    /// Flying star assigned to each palace, indexed per `Palace::ALL` order.
    pub palaces: [FlyingStar; 9],
    pub center_star: FlyingStar,
    pub evidence: ReasoningEvidenceEnvelope,
}

/// Construct a minimal `ReasoningEvidenceEnvelope` for use in tests and stubs.
pub fn minimal_evidence() -> ReasoningEvidenceEnvelope {
    ReasoningEvidenceEnvelope {
        source_family: ReasoningEvidenceSourceFamily::AlmanacRule,
        source_id: crate::sources::SOURCE_HUYEN_KHONG.to_string(),
        method: "stub".to_string(),
        note: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Palace enum carries canonical Lo Shu numbering.
    #[test]
    fn test_palace_lo_shu_numbering() {
        assert_eq!(Palace::N as u8, 1, "N should be Lo Shu 1");
        assert_eq!(Palace::Center as u8, 5, "Center should be Lo Shu 5");
        assert_eq!(Palace::S as u8, 9, "S should be Lo Shu 9");
        // Verify full set covers 1..=9 exactly
        let mut seen = [false; 10];
        for &p in &Palace::ALL {
            let n = p as u8;
            assert!((1..=9).contains(&n));
            assert!(!seen[n as usize], "Duplicate Lo Shu number {n}");
            seen[n as usize] = true;
        }
        for (n, &present) in seen.iter().enumerate().skip(1) {
            assert!(present, "Lo Shu number {n} missing from Palace::ALL");
        }
    }

    /// Test 2: FlyingStar enum has variants 1..=9.
    #[test]
    fn test_flying_star_numbering() {
        assert_eq!(FlyingStar::NhatBach as u8, 1);
        assert_eq!(FlyingStar::CuuTu as u8, 9);
        // spot-check middle values
        assert_eq!(FlyingStar::NguHoang as u8, 5);
        assert_eq!(FlyingStar::BatBach as u8, 8);
    }

    /// Test 3: palace_to_direction returns a non-empty string for every palace.
    #[test]
    fn test_palace_to_direction_stub() {
        for &p in &Palace::ALL {
            let dir = palace_to_direction(p);
            assert!(
                !dir.is_empty(),
                "palace_to_direction returned empty string for {p:?}"
            );
        }
        // Spot-check the canonical North palace
        assert_eq!(palace_to_direction(Palace::N), "N");
    }

    /// Test 4: FlyingStarLayout can be constructed (compile-level proof field set is frozen).
    #[test]
    fn test_flying_star_layout_construction() {
        let layout = FlyingStarLayout {
            period: FlyingStarPeriod::Yearly { year: 2024 },
            palaces: [FlyingStar::CuuTu; 9],
            center_star: FlyingStar::CuuTu,
            evidence: minimal_evidence(),
        };
        assert_eq!(layout.center_star as u8, 9);
        assert_eq!(layout.palaces.len(), 9);
        if let FlyingStarPeriod::Yearly { year } = layout.period {
            assert_eq!(year, 2024);
        } else {
            panic!("Wrong period variant");
        }
    }

    /// Test 5: FlyingStarPeriod serde round-trip for Van / Yearly / Monthly / Daily.
    #[test]
    fn test_flying_star_period_serde_round_trip() {
        let cases = [
            FlyingStarPeriod::Van { van: 9 },
            FlyingStarPeriod::Yearly { year: 2025 },
            FlyingStarPeriod::Monthly {
                year: 2025,
                month: 3,
            },
            FlyingStarPeriod::Daily {
                date: (2024, 12, 25),
            },
        ];
        for original in cases {
            let json = serde_json::to_string(&original).expect("serialization failed");
            let roundtripped: FlyingStarPeriod =
                serde_json::from_str(&json).expect("deserialization failed");
            assert_eq!(original, roundtripped, "round-trip failed for {original:?}");
        }
    }

    /// Phase 18-01 (FS-17): DailyFlyingStarLayout serde round-trip.
    #[test]
    fn test_daily_flying_star_layout_period_serde() {
        let layout = DailyFlyingStarLayout {
            period: FlyingStarPeriod::Daily {
                date: (2024, 12, 25),
            },
            palaces: [FlyingStar::NhatBach; 9],
            center_star: FlyingStar::NhatBach,
            evidence: minimal_evidence(),
        };
        let json = serde_json::to_string(&layout).expect("serialize");
        let roundtripped: DailyFlyingStarLayout = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(layout.center_star as u8, roundtripped.center_star as u8);
        if let FlyingStarPeriod::Daily { date: (y, m, d) } = roundtripped.period {
            assert_eq!((y, m, d), (2024_i32, 12_u32, 25_u32));
        } else {
            panic!("Expected Daily period variant");
        }
    }
}
