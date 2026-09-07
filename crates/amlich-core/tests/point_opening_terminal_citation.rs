//! Terminal-citation golden and safety suite for the v1.11
//! point-opening surface wording (bead `amlich-xlag.2.3.2`).
//!
//! The canonical citation block rendered by
//! `amlich_core::point_opening::point_opening_citation_lines` is the
//! single source of the surfaced wording: the terminal renders it
//! verbatim (this bead), the desktop inspector reuses the same strings
//! (`amlich-xlag.2.3.3`), and the cross-surface parity locks compare
//! against it (`amlich-xlag.2.3.4`).
//!
//! This suite locks the deterministic fixtures the whole v1.11 track
//! shares — 2024-02-10 (JD 2460351, Giáp day) at 19:30 (open 甲/戌 →
//! 竅陰 GB44) and 00:30 (explicit closed 甲/子) — against the committed
//! golden texts `data/ty-ngo-luu-chu/terminal-citation-open.txt` and
//! `terminal-citation-closed.txt`, asserts every disclosure the bead
//! requires stays visible on both states, and proves the surfaced
//! wording carries no clinical or action language (BOUND-02): the
//! disclaimer v2 lines are stripped before scanning, mirroring
//! `strip_disclaimers` in `point_opening_finite_core_golden.rs`.

use amlich_core::point_opening::{
    point_opening_citation_lines, point_opening_citation_text, resolve_day_point_opening_context,
    DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN, DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN,
    DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR,
};

const GOLDEN_OPEN: &str = include_str!("../data/ty-ngo-luu-chu/terminal-citation-open.txt");
const GOLDEN_CLOSED: &str = include_str!("../data/ty-ngo-luu-chu/terminal-citation-closed.txt");

/// 2024-02-10 — the verified Giáp-day Julian fixture shared with the
/// civil-time, provenance, DTO, and desktop suites.
const JD_GIAP_DAY: i32 = 2_460_351;

fn open_context() -> amlich_core::point_opening::DayPointOpeningContext {
    resolve_day_point_opening_context(JD_GIAP_DAY, 19, 30).expect("19:30 resolves on the fixture")
}

fn closed_context() -> amlich_core::point_opening::DayPointOpeningContext {
    resolve_day_point_opening_context(JD_GIAP_DAY, 0, 30).expect("00:30 resolves on the fixture")
}

/// The same extended action/efficacy lexicon the finite-core golden
/// guard polices (`point_opening_finite_core_golden.rs`); the citation
/// wording must survive it identically.
const FORBIDDEN_PHRASES: &[&str] = &[
    "best time to treat",
    "best hour to treat",
    "best hour",
    "should be needled",
    "should be pressed",
    "should needle",
    "should stimulate",
    "recommended point",
    "optimal point",
    "nên châm",
    "nên bấm",
    "nên cứu",
    "nên kích thích",
    "hãy châm",
    "hãy bấm",
    "điểm nên ",
    "công dụng",
    "chữa",
    "điều trị",
    "thải độc",
    "liều lượng",
    "hoạt động mạnh nhất",
    "đạt đỉnh",
];

/// Remove the disclaimer v2 lines — the only permitted clinical-verb
/// context, byte-locked separately against the REVIEWER-PACK and the
/// corpus (mirrors `strip_disclaimers`).
fn strip_disclaimer_lines(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .filter(|line| !line.starts_with("Miễn trừ ("))
        .cloned()
        .collect()
}

fn assert_citation_is_safe(label: &str, lines: &[String]) {
    let stripped = strip_disclaimer_lines(lines);
    let text = stripped.join("\n").to_lowercase();
    for phrase in FORBIDDEN_PHRASES {
        assert!(
            !text.contains(phrase),
            "{label}: prohibited action/efficacy phrasing `{phrase}` in surfaced citation wording"
        );
    }
}

#[test]
fn open_citation_matches_the_committed_golden_byte_for_byte() {
    let text = point_opening_citation_text(&open_context());
    assert_eq!(
        text + "\n",
        GOLDEN_OPEN,
        "the open-state citation wording must match the committed golden; \
         regeneration is the documented re-freeze procedure"
    );
}

#[test]
fn closed_citation_matches_the_committed_golden_byte_for_byte() {
    let text = point_opening_citation_text(&closed_context());
    assert_eq!(
        text + "\n",
        GOLDEN_CLOSED,
        "the closed-state citation wording must match the committed golden; \
         regeneration is the documented re-freeze procedure"
    );
}

#[test]
fn both_states_keep_every_disclosure_visible() {
    for (label, context) in [("open", open_context()), ("closed", closed_context())] {
        let text = point_opening_citation_text(&context);

        // Pending review stays visible (row and nomenclature gates).
        assert!(
            text.contains("Duyệt hàng bảng: ExternalReviewPending("),
            "{label}: row review marker missing"
        );
        assert!(
            text.contains("Duyệt danh pháp: ExternalReviewPending("),
            "{label}: nomenclature review marker missing"
        );
        // Evidence stays visible — including the pending edition leg.
        assert!(text.contains("Chứng cứ:"), "{label}: evidence missing");
        assert!(
            text.contains("PENDING_CLASSICAL_REVIEW") || text.contains("closed slot (閉穴)"),
            "{label}: frozen evidence source missing"
        );
        // Disclosed time basis.
        assert!(
            text.contains("cơ sở thời gian local_civil_hour_branch"),
            "{label}: time basis missing"
        );
        // Known divergences.
        assert!(
            text.contains("Dị biệt đã ghi nhận: TNLC-DIV-01"),
            "{label}: divergence references missing"
        );
        // Disclaimer v2 verbatim, both languages.
        assert!(
            text.contains(DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_VN),
            "{label}: Vietnamese disclaimer v2 not verbatim"
        );
        assert!(
            text.contains(DISCLAIMER_HISTORICAL_PROCEDURAL_CITATION_EN),
            "{label}: English disclaimer v2 not verbatim"
        );
        assert!(
            text.contains(DISCLAIMER_ID_HISTORICAL_PROCEDURAL_CITATION_STR),
            "{label}: disclaimer id missing"
        );
        // Safety class and policy contract.
        assert!(
            text.contains("Lớp an toàn: historical_procedural_citation"),
            "{label}: safety class missing"
        );
        assert!(
            text.contains("TY_NGO_LUU_CHU_POLICY_V1"),
            "{label}: policy id missing"
        );
    }
}

#[test]
fn the_citation_wording_stays_citation_framed() {
    let open_text = point_opening_citation_text(&open_context());
    // The canonical §D framing sentence, not an action recommendation.
    assert!(open_text.contains("được ghi tương ứng với huyệt"));
    assert!(open_text.contains("hạng 井 (井金)"));
    assert!(open_text.contains("[GB44]"));

    let closed_text = point_opening_citation_text(&closed_context());
    // Closed slots stay explicitly closed and never list points.
    assert!(closed_text.contains("là ổ đóng (閉穴)"));
    assert!(!closed_text.contains("được ghi tương ứng với huyệt"));
    assert!(!closed_text.contains("[GB"));
}

#[test]
fn the_surfaced_citation_wording_carries_no_clinical_or_action_language() {
    // Both deterministic fixtures.
    assert_citation_is_safe("open", &point_opening_citation_lines(&open_context()));
    assert_citation_is_safe("closed", &point_opening_citation_lines(&closed_context()));

    // Every hour block of the fixture day — all 24 surfaced states.
    for hour in 0u8..=23 {
        let context =
            resolve_day_point_opening_context(JD_GIAP_DAY, hour, 30).expect("valid moment");
        assert_citation_is_safe(
            &format!("hour {hour}:30"),
            &point_opening_citation_lines(&context),
        );
    }

    // The committed golden texts themselves.
    for (label, golden) in [
        ("open golden", GOLDEN_OPEN),
        ("closed golden", GOLDEN_CLOSED),
    ] {
        let lines: Vec<String> = golden.lines().map(str::to_string).collect();
        assert_citation_is_safe(label, &lines);
    }
}
