//! v1.11 Point-Opening Context — the canonical terminal citation block
//! (bead `amlich-xlag.2.3.2`).
//!
//! Single source of truth for the point-opening citation *wording*:
//! the terminal (this bead), the desktop inspector
//! (`amlich-xlag.2.3.3`), and the cross-surface parity locks
//! (`amlich-xlag.2.3.4`) all render [`point_opening_citation_lines`]
//! verbatim so the surfaced wording stays byte-identical across
//! surfaces (ADR-0004 / REVIEWER-PACK §D).
//!
//! Framing contract (REVIEWER-PACK §D, Gate 3): an open-point
//! citation is always framed as historical citation ("theo bảng, giờ X
//! ngày Y được ghi tương ứng với huyệt Z"), never as an action
//! recommendation. The block carries, in order: the policy header, the
//! hour-slot identity with the disclosed time basis, the day-attribution
//! markers, the open or explicit-closed citation, the per-row evidence
//! (pending classical review stays visible), both review markers, the
//! safety class, the known-divergence ids, and disclaimer v2 verbatim
//! in both languages. Nothing here adds function, effect, indication,
//! or endorsement language.

use super::snapshot::DayPointOpeningContext;
use super::state::{PointOpeningContext, PointOpeningSlotState};

/// Render the canonical, deterministic citation block for one resolved
/// point-opening context as plain lines (no styling, no wrapping —
/// terminals and the desktop wrap the same strings themselves).
///
/// The output is a pure function of the context: the same context
/// always renders the same lines, byte for byte, on every surface.
pub fn point_opening_citation_lines(context: &DayPointOpeningContext) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Tý Ngọ Lưu Chú — trích dẫn lịch sử ({})",
        context.context.policy_id
    ));
    lines.push(format!(
        "Khung giờ {} ({}) · trụ giờ {} · cơ sở thời gian {}",
        context.hour_branch_vi,
        context.hour_time_range,
        context.hour_pillar_zh,
        context.context.time_basis.as_str()
    ));
    if context.late_night_day_transition {
        lines.push(
            "Giờ Tý 23:00–23:59 xếp vào ngày dân sự kế tiếp theo quy ước đóng băng (TNLC-DIV-03)"
                .to_string(),
        );
    }
    if context.cross_day_spillover {
        lines.push("Hàng bảng ghi tràn sang ngày kế tiếp (cross-day spillover)".to_string());
    }
    push_state_lines(&mut lines, context);
    push_evidence_lines(&mut lines, context);
    push_disclosure_lines(&mut lines, &context.context);
    lines
}

/// The canonical citation text as one newline-joined string — the
/// shape the golden files and cross-surface parity locks compare.
pub fn point_opening_citation_text(context: &DayPointOpeningContext) -> String {
    point_opening_citation_lines(context).join("\n")
}

fn push_state_lines(lines: &mut Vec<String>, context: &DayPointOpeningContext) {
    let day_stem_vi = context.slot_day_canchi.can.as_str();
    match &context.context.state {
        PointOpeningSlotState::Open {
            slot_class_zh_as_printed,
            phase_annotation_as_printed,
            points,
            substitution,
        } => {
            let primary = points
                .first()
                .expect("open slots carry at least one identity triple");
            lines.push(format!(
                "Theo bảng Châm Cứu Đại Thành, giờ {} ngày {} ({}) được ghi tương ứng với huyệt {} ({}) [{}] — kinh {}, hạng {} ({}).",
                context.hour_branch_vi,
                day_stem_vi,
                context.day_stem_zh,
                primary.xue_ming_zh,
                primary.huyet_danh_vi,
                primary.standard_code_gloss,
                primary.channel_vi,
                slot_class_zh_as_printed,
                phase_annotation_as_printed
            ));
            for companion in points.iter().skip(1) {
                lines.push(format!(
                    "Huyệt ghi kèm: {} ({}) [{}] — kinh {}",
                    companion.xue_ming_zh,
                    companion.huyet_danh_vi,
                    companion.standard_code_gloss,
                    companion.channel_vi
                ));
            }
            if let Some(substitution) = substitution {
                lines.push(format!("Cách ghi hàng bảng: {substitution}"));
            }
        }
        PointOpeningSlotState::Closed {
            running_tables,
            doctrine_zh,
            ..
        } => {
            lines.push(format!(
                "Theo bảng Châm Cứu Đại Thành, giờ {} ngày {} ({}) là ổ đóng (閉穴) — không huyệt nào được ghi cho khung giờ này.",
                context.hour_branch_vi, day_stem_vi, context.day_stem_zh
            ));
            lines.push(format!("Lời gốc: {doctrine_zh}"));
            lines.push(format!("Bảng đang chạy: {}", running_tables.join(", ")));
        }
    }
}

fn push_evidence_lines(lines: &mut Vec<String>, context: &DayPointOpeningContext) {
    if context.provenance.work_evidence.is_empty() {
        if let PointOpeningSlotState::Closed { note, .. } = &context.context.state {
            lines.push(format!("Chứng cứ: closed slot (閉穴): {note}"));
        }
        return;
    }
    for citation in &context.provenance.work_evidence {
        lines.push(format!(
            "Chứng cứ: {} — {} ({}) · bản in: {}",
            citation.work_title,
            citation.volume_or_chapter,
            citation.passage_key,
            citation.edition_or_facsimile_uri
        ));
    }
    if let Some(table) = &context.provenance.table_evidence {
        lines.push(format!(
            "Vị trí bảng: {} · hàng {}",
            table.table_id, table.row_index
        ));
    }
}

fn push_disclosure_lines(lines: &mut Vec<String>, context: &PointOpeningContext) {
    lines.push(format!(
        "Duyệt hàng bảng: {}",
        context.review_state.to_marker()
    ));
    lines.push(format!(
        "Duyệt danh pháp: {}",
        context.nomenclature_review_state.to_marker()
    ));
    lines.push(format!("Lớp an toàn: {}", context.safety_class));
    if !context.known_divergence_ids.is_empty() {
        lines.push(format!(
            "Dị biệt đã ghi nhận: {}",
            context.known_divergence_ids.join(" · ")
        ));
    }
    lines.push(format!(
        "Miễn trừ ({}) · vi: {}",
        context.disclaimer.id.as_str(),
        context.disclaimer.vi
    ));
    lines.push(format!(
        "Miễn trừ ({}) · en: {}",
        context.disclaimer.id.as_str(),
        context.disclaimer.en
    ));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point_opening::resolve_day_point_opening_context;

    /// 2024-02-10, the verified Giáp-day Julian fixture shared with
    /// the civil-time, provenance, and DTO suites.
    const JD_GIAP_DAY: i32 = 2_460_351;

    fn open_context() -> DayPointOpeningContext {
        resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30)
            .expect("19:30 on the Giáp day resolves")
    }

    fn closed_context() -> DayPointOpeningContext {
        resolve_day_point_opening_context(JD_GIAP_DAY, 0, 30)
            .expect("00:30 on the Giáp day resolves")
    }

    #[test]
    fn open_citation_carries_identity_class_and_all_disclosures() {
        let lines = point_opening_citation_lines(&open_context());
        let text = lines.join("\n");

        assert!(text.contains("trích dẫn lịch sử (TY_NGO_LUU_CHU_POLICY_V1)"));
        // Slot identity and disclosed time basis.
        assert!(text.contains("Khung giờ Tuất (19:00-21:00)"));
        assert!(text.contains("trụ giờ 甲戌"));
        assert!(text.contains("cơ sở thời gian local_civil_hour_branch"));
        // Citation framing sentence with the identity triple and class.
        assert!(text.contains(
            "giờ Tuất ngày Giáp (甲) được ghi tương ứng với huyệt 竅陰 (Kiếu âm) [GB44] — kinh Đởm, hạng 井 (井金)"
        ));
        // Evidence with the pending edition legible.
        assert!(text.contains("Chứng cứ: Zhenjiu Dacheng (針灸大成) — 卷七"));
        assert!(text.contains("bản in: PENDING_CLASSICAL_REVIEW"));
        assert!(text.contains("Vị trí bảng: jia · hàng 1"));
        // Both review markers stay visible.
        assert!(text.contains("Duyệt hàng bảng: ExternalReviewPending("));
        assert!(text.contains("Duyệt danh pháp: ExternalReviewPending("));
        assert!(text.contains("Lớp an toàn: historical_procedural_citation"));
        assert!(text.contains("Dị biệt đã ghi nhận: TNLC-DIV-01"));
        // Disclaimer v2 verbatim in both languages.
        assert!(text.contains(crate::point_opening::DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN));
        assert!(text.contains(crate::point_opening::DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN));
    }

    #[test]
    fn closed_citation_is_explicit_and_never_lists_points() {
        let lines = point_opening_citation_lines(&closed_context());
        let text = lines.join("\n");

        assert!(text.contains("là ổ đóng (閉穴) — không huyệt nào được ghi"));
        assert!(text.contains("Lời gốc: 得時為之開"));
        assert!(text.contains("Bảng đang chạy: gui, jia"));
        assert!(text.contains("Chứng cứ: closed slot (閉穴):"));
        assert!(!text.contains("được ghi tương ứng với huyệt"));
        assert!(!text.contains("Vị trí bảng"));
        // Pending review and disclaimer stay visible on closed slots too.
        assert!(text.contains("Duyệt hàng bảng: ExternalReviewPending("));
        assert!(text.contains(crate::point_opening::DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN));
    }

    #[test]
    fn rendering_is_deterministic() {
        for context in [open_context(), closed_context()] {
            let first = point_opening_citation_lines(&context);
            let second = point_opening_citation_lines(&context);
            assert_eq!(first, second);
            assert_eq!(point_opening_citation_text(&context), first.join("\n"));
        }
    }

    #[test]
    fn late_night_and_spillover_markers_ride_their_slots() {
        // 23:30 is the TNLC-DIV-03 late-night transition; the Giáp-day
        // Tý slot is explicitly closed.
        let late = resolve_day_point_opening_context(JD_GIAP_DAY, 23, 30).expect("23:30 resolves");
        let text = point_opening_citation_text(&late);
        assert!(text.contains("Giờ Tý 23:00–23:59 xếp vào ngày dân sự kế tiếp"));

        // A spillover row: 01:30 on the Giáp day resolves into the
        // previous stem's 癸 (gui) table, row 2.
        let spillover =
            resolve_day_point_opening_context(JD_GIAP_DAY, 1, 30).expect("01:30 resolves");
        assert!(spillover.cross_day_spillover);
        let text = point_opening_citation_text(&spillover);
        assert!(text.contains("Hàng bảng ghi tràn sang ngày kế tiếp"));
        assert!(text.contains("Vị trí bảng: gui · hàng 2"));
    }
}
