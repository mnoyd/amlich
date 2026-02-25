/// Thập nhị trực (十二建除) — Twelve Duty Stars
///
/// Each day is governed by one of 12 duty stars (trực) determined by the
/// relationship between the day's earthly branch and the month's earthly branch.
///
/// **Month branch assignment (nguyệt kiến):**
/// - Tháng 1 (Giêng) → Dần (2)
/// - Tháng 2        → Mão (3)
/// - …
/// - Tháng 11       → Tý  (0)
/// - Tháng 12       → Sửu (1)
/// Formula: `month_chi = (lunar_month + 1) % 12`
///
/// **Trực formula:**
/// `truc_index = (day_chi_index − month_chi_index + 12) % 12`
///
/// **Source:** Khâm Định Hiệp Kỷ Biện Phương Thư (欽定協紀辨方書), method: formula.
use super::types::TrucInfo;

pub const TRUC_NAMES: [&str; 12] = [
    "Kiến", "Trừ", "Mãn", "Bình", "Định", "Chấp", "Phá", "Nguy", "Thành", "Thu", "Khai", "Bế",
];

/// Auspicious quality for each trực (in order 0..11).
/// cat=auspicious, hung=ominous, binh=neutral
pub const TRUC_QUALITY: [&str; 12] = [
    "cat",  // Kiến
    "cat",  // Trừ
    "hung", // Mãn
    "binh", // Bình
    "cat",  // Định
    "binh", // Chấp
    "hung", // Phá
    "hung", // Nguy
    "cat",  // Thành
    "hung", // Thu
    "cat",  // Khai
    "hung", // Bế
];

/// Compute the month's earthly-branch index from the lunar month number (1–12).
///
/// Uses the nguyệt-kiến table:
/// month 1→Dần(2), month 2→Mão(3), …, month 11→Tý(0), month 12→Sửu(1).
pub fn month_chi_index(lunar_month: i32) -> usize {
    (lunar_month as usize + 1) % 12
}

/// Compute the trực index (0–11) for a day.
pub fn truc_index(day_chi_index: usize, lunar_month: i32) -> usize {
    let m = month_chi_index(lunar_month);
    (day_chi_index + 12 - m) % 12
}

/// Get the full `TrucInfo` for a day.
pub fn get_truc(day_chi_index: usize, lunar_month: i32) -> TrucInfo {
    let idx = truc_index(day_chi_index, lunar_month);
    TrucInfo {
        index: idx,
        name: TRUC_NAMES[idx].to_string(),
        quality: TRUC_QUALITY[idx].to_string(),
        evidence: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- month_chi_index ---

    #[test]
    fn month_chi_thang_1_is_dan() {
        // Tháng 1 (Giêng) → Dần (index 2)
        assert_eq!(month_chi_index(1), 2);
    }

    #[test]
    fn month_chi_thang_11_is_ty() {
        // Tháng 11 → Tý (index 0)
        assert_eq!(month_chi_index(11), 0);
    }

    #[test]
    fn month_chi_thang_12_is_suu() {
        // Tháng 12 → Sửu (index 1)
        assert_eq!(month_chi_index(12), 1);
    }

    // --- Structural invariant: day chi == month chi → Kiến ---

    #[test]
    fn kien_when_day_chi_equals_month_chi() {
        // For every lunar month, a day whose chi equals the month chi → Kiến (0)
        for m in 1..=12 {
            let mchi = month_chi_index(m);
            assert_eq!(
                truc_index(mchi, m),
                0,
                "month {} (mchi={}) with matching day should be Kiến",
                m,
                mchi
            );
        }
    }

    // --- Full cycle: 12 consecutive chi values yield all 12 trực ---

    #[test]
    fn consecutive_days_cycle_all_12_truc() {
        // Starting from the month-chi day, the next 11 chi offsets produce Trừ..Bế
        for start_month in 1..=12i32 {
            let mchi = month_chi_index(start_month);
            for offset in 0..12usize {
                let day_chi = (mchi + offset) % 12;
                assert_eq!(
                    truc_index(day_chi, start_month),
                    offset,
                    "month={} offset={} expected truc={}",
                    start_month,
                    offset,
                    offset
                );
            }
        }
    }

    // --- Known-date assertions ---

    #[test]
    fn tet_2024_is_man() {
        // Feb 10, 2024: Tết Giáp Thìn
        // Lunar: day 1, month 1 → month_chi = Dần (2)
        // Day can chi: "Giáp Thìn" → Thìn (4)
        // truc = (4 - 2 + 12) % 12 = 2 → Mãn
        let result = get_truc(4, 1);
        assert_eq!(result.name, "Mãn");
        assert_eq!(result.index, 2);
        assert_eq!(result.quality, "hung");
    }

    #[test]
    fn tet_2025_is_thanh() {
        // Jan 29, 2025: Tết Ất Tỵ
        // Lunar: day 1, month 1 → month_chi = Dần (2)
        // Day can chi: "Mậu Tuất" → Tuất (10)
        // truc = (10 - 2 + 12) % 12 = 8 → Thành
        let result = get_truc(10, 1);
        assert_eq!(result.name, "Thành");
        assert_eq!(result.index, 8);
        assert_eq!(result.quality, "cat");
    }

    // --- All 12 trực names present ---

    #[test]
    fn all_12_truc_names_reachable() {
        let seen: std::collections::HashSet<String> =
            (0..12).map(|i| get_truc(i, 1).name).collect();
        assert_eq!(seen.len(), 12, "All 12 trực names must be reachable");
    }
}
