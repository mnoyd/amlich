/**
 * Compile-time contract checks for the user-facing v1.7 classical surfaces.
 * Rust source of truth: IChingCastSummary, DirectionCrossLinkSummary, and the
 * desktop ClassicalSurfaceDto command projection.
 */

import type {
    ClassicalSurfaceDto,
    DayPointOpeningContextDto,
    DirectionCellDto,
    DirectionCrossLinkSummaryDto,
    IChingCastSummaryDto,
    PointOpeningContextDto,
    PointOpeningIdentityDto,
    PointOpeningProvenanceDto,
    PointOpeningSourceCitationDto,
    PointOpeningTableEvidenceDto,
} from './types';

type AssertTrue<T extends true> = T;
type Equals<X, Y> =
    (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;

type _SurfaceKeys = AssertTrue<
    Equals<
        keyof ClassicalSurfaceDto,
        | 'iching_cast'
        | 'direction_cross_link'
        | 'traditional_wellness'
        | 'point_opening'
        | 'point_opening_citation_lines'
    >
>;
type _IChingKeys = AssertTrue<
    Equals<
        keyof IChingCastSummaryDto,
        | 'cast'
        | 'bien_que'
        | 'the_dung'
        | 'chu_hexagram_vi_name'
        | 'chu_hexagram_thoai_tu'
        | 'bien_hexagram_vi_name'
        | 'bien_hexagram_thoai_tu'
        | 'cat_hung_summary'
        | 'moving_line'
        | 'question_vi'
        | 'evidence'
    >
>;
type _DirectionSummaryKeys = AssertTrue<
    Equals<
        keyof DirectionCrossLinkSummaryDto,
        | 'cross_link_kind'
        | 'cross_link_source'
        | 'date'
        | 'day_chi_index'
        | 'birth_chi_index'
        | 'cells'
        | 'summary_vi'
        | 'composite_severity'
        | 'evidence'
    >
>;
type _DirectionCellKeys = AssertTrue<
    Equals<
        keyof DirectionCellDto,
        'direction' | 'khcbppt' | 'huyen_khong' | 'agreement' | 'severity'
    >
>;
type _PointOpeningContextKeys = AssertTrue<
    Equals<
        keyof DayPointOpeningContextDto,
        | 'day_stem_zh'
        | 'hour_branch_zh'
        | 'hour_pillar_zh'
        | 'hour_branch_vi'
        | 'hour_time_range'
        | 'hour_slot_index'
        | 'civil_day_canchi'
        | 'slot_day_canchi'
        | 'late_night_day_transition'
        | 'cross_day_spillover'
        | 'context'
        | 'provenance'
        | 'method_evidence'
        | 'calendar_evidence'
    >
>;
type _PointOpeningCarrierKeys = AssertTrue<
    Equals<
        keyof PointOpeningContextDto,
        | 'policy_id'
        | 'state'
        | 'disclaimer'
        | 'review_state'
        | 'nomenclature_review_state'
        | 'safety_class'
        | 'time_basis'
        | 'known_divergence_ids'
    >
>;
type _PointOpeningIdentityKeys = AssertTrue<
    Equals<
        keyof PointOpeningIdentityDto,
        | 'point_key'
        | 'xue_ming_zh'
        | 'huyet_danh_vi'
        | 'standard_code_gloss'
        | 'channel_zh'
        | 'channel_vi'
        | 'channel_en'
        | 'role'
    >
>;
type _PointOpeningProvenanceKeys = AssertTrue<
    Equals<
        keyof PointOpeningProvenanceDto,
        'source_id' | 'work_evidence' | 'table_evidence'
    >
>;
type _PointOpeningCitationKeys = AssertTrue<
    Equals<
        keyof PointOpeningSourceCitationDto,
        | 'source_id'
        | 'work_title'
        | 'volume_or_chapter'
        | 'passage_key'
        | 'edition_or_facsimile_uri'
        | 'transcription_uri'
        | 'cross_reference_uri'
        | 'translation_kind'
    >
>;
type _PointOpeningTableEvidenceKeys = AssertTrue<
    Equals<keyof PointOpeningTableEvidenceDto, 'table_id' | 'row_index'>
>;

export const classicalSurfaceWithoutImplicitCast: ClassicalSurfaceDto = {
    direction_cross_link: {
        cross_link_kind: 'date_only',
        cross_link_source: 'rule.composite.direction_cross_link',
        date: '2024-02-10',
        day_chi_index: 0,
        birth_chi_index: Number.MAX_SAFE_INTEGER,
        cells: [],
        summary_vi: 'Tổng hợp phương hướng',
        composite_severity: 'soft_taboo',
        evidence: [],
    },
};

// ---------------------------------------------------------------------------
// v1.11 (amlich-xlag.2.3.3) — canonical point-opening citation samples.
// Both fixtures mirror the core golden files byte for byte
// (crates/amlich-core/data/ty-ngo-luu-chu/terminal-citation-{open,closed}.txt):
// open is the Giáp-day 19:30 Tuất slot (甲/戌 → 竅陰 GB44), closed the
// 00:30 Tý slot (甲/子 閉穴). The desktop renders these exact lines; it
// never recomposes the wording.
// ---------------------------------------------------------------------------

export const canonicalOpenCitationLines: string[] = [
    'Tý Ngọ Lưu Chú — trích dẫn lịch sử (TY_NGO_LUU_CHU_POLICY_V1)',
    'Khung giờ Tuất (19:00-21:00) · trụ giờ 甲戌 · cơ sở thời gian local_civil_hour_branch',
    'Theo bảng Châm Cứu Đại Thành, giờ Tuất ngày Giáp (甲) được ghi tương ứng với huyệt 竅陰 (Kiếu âm) [GB44] — kinh Đởm, hạng 井 (井金).',
    'Chứng cứ: Zhenjiu Dacheng (針灸大成) — 卷七 (徐氏子午流注逐日按時定穴歌；流注圖) · bản in: PENDING_CLASSICAL_REVIEW',
    'Vị trí bảng: jia · hàng 1',
    'Duyệt hàng bảng: ExternalReviewPending(reason="najia_xu_style_table_row_review_pending"; expected_review_date="2026-12-31"; assigned_to="classical_chinese_reviewer")',
    'Duyệt danh pháp: ExternalReviewPending(reason="vietnamese_nomenclature_and_code_gloss_pending"; expected_review_date="2026-12-31"; assigned_to="vietnamese_nomenclature_reviewer")',
    'Lớp an toàn: historical_procedural_citation',
    'Dị biệt đã ghi nhận: TNLC-DIV-01 · TNLC-DIV-02 · TNLC-DIV-03 · TNLC-DIV-05',
    'Miễn trừ (historical_procedural_citation_v1) · vi: Trích dẫn thuật ngữ y học cổ truyền từ văn bản Châm Cứu Đại Thành; chỉ mang tính văn hóa – lịch sử. Đây không phải hướng dẫn châm, bấm, cứu hay tự điều trị tại bất kỳ thời điểm nào. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.',
    'Miễn trừ (historical_procedural_citation_v1) · en: Citations of classical acupuncture terminology from Zhenjiu Dacheng; provided as historical and cultural information only. This is not instruction or encouragement to needle, press, moxibust, or self-treat at any time. Do not use it to delay or replace care from a qualified health professional.',
];

export const canonicalClosedCitationLines: string[] = [
    'Tý Ngọ Lưu Chú — trích dẫn lịch sử (TY_NGO_LUU_CHU_POLICY_V1)',
    'Khung giờ Tý (23:00-01:00) · trụ giờ 甲子 · cơ sở thời gian local_civil_hour_branch',
    'Theo bảng Châm Cứu Đại Thành, giờ Tý ngày Giáp (甲) là ổ đóng (閉穴) — không huyệt nào được ghi cho khung giờ này.',
    'Lời gốc: 得時為之開，失時為之闔（論子午流注法）；闔者閉也，陽日遇陰時，陰日遇陽時，則前穴已閉（流注時日）',
    'Bảng đang chạy: gui, jia',
    'Chứng cứ: closed slot (閉穴): neither running day-table lists this hour block; the Xu-style tables as printed leave it without an assigned point (閉穴)',
    'Duyệt hàng bảng: ExternalReviewPending(reason="najia_xu_style_table_row_review_pending"; expected_review_date="2026-12-31"; assigned_to="classical_chinese_reviewer")',
    'Duyệt danh pháp: ExternalReviewPending(reason="vietnamese_nomenclature_and_code_gloss_pending"; expected_review_date="2026-12-31"; assigned_to="vietnamese_nomenclature_reviewer")',
    'Lớp an toàn: historical_procedural_citation',
    'Dị biệt đã ghi nhận: TNLC-DIV-01 · TNLC-DIV-02 · TNLC-DIV-03 · TNLC-DIV-05',
    'Miễn trừ (historical_procedural_citation_v1) · vi: Trích dẫn thuật ngữ y học cổ truyền từ văn bản Châm Cứu Đại Thành; chỉ mang tính văn hóa – lịch sử. Đây không phải hướng dẫn châm, bấm, cứu hay tự điều trị tại bất kỳ thời điểm nào. Không dùng để trì hoãn hoặc thay thế chăm sóc từ nhân viên y tế có chuyên môn.',
    'Miễn trừ (historical_procedural_citation_v1) · en: Citations of classical acupuncture terminology from Zhenjiu Dacheng; provided as historical and cultural information only. This is not instruction or encouragement to needle, press, moxibust, or self-treat at any time. Do not use it to delay or replace care from a qualified health professional.',
];

const openPointIdentity: PointOpeningIdentityDto = {
    point_key: 'qiao-yin',
    xue_ming_zh: '竅陰',
    huyet_danh_vi: 'Kiếu âm',
    standard_code_gloss: 'GB44',
    channel_zh: '足少陽膽',
    channel_vi: 'Đởm',
    channel_en: 'Gallbladder',
    role: 'primary',
};

const pointOpeningWorkEvidence: PointOpeningSourceCitationDto = {
    source_id: 'ty-ngo-luu-chu',
    work_title: 'Zhenjiu Dacheng (針灸大成)',
    volume_or_chapter: '卷七',
    passage_key: '徐氏子午流注逐日按時定穴歌；流注圖',
    edition_or_facsimile_uri: 'PENDING_CLASSICAL_REVIEW',
    transcription_uri: 'PENDING_CLASSICAL_REVIEW',
    cross_reference_uri: 'PENDING_CLASSICAL_REVIEW',
    translation_kind: 'verbatim_classical_table_with_project_paraphrase_gloss',
};

const pointOpeningTableEvidence: PointOpeningTableEvidenceDto = {
    table_id: 'jia',
    row_index: 1,
};

const giapThinDayCanchi = {
    can_index: 0,
    chi_index: 4,
    can: 'Giáp',
    chi: 'Thìn',
    full: 'Giáp Thìn',
    con_giap: 'Thìn (Rồng)',
    ngu_hanh: { can: 'Mộc', chi: 'Thổ' },
    sexagenary_index: 2,
};

const pointOpeningDisclaimerVi = canonicalOpenCitationLines[9].slice(
    canonicalOpenCitationLines[9].indexOf('vi: ') + 4,
);
const pointOpeningDisclaimerEn = canonicalOpenCitationLines[10].slice(
    canonicalOpenCitationLines[10].indexOf('en: ') + 4,
);

const pendingRowReviewMarker =
    'ExternalReviewPending(reason="najia_xu_style_table_row_review_pending"; expected_review_date="2026-12-31"; assigned_to="classical_chinese_reviewer")';
const pendingNomenclatureReviewMarker =
    'ExternalReviewPending(reason="vietnamese_nomenclature_and_code_gloss_pending"; expected_review_date="2026-12-31"; assigned_to="vietnamese_nomenclature_reviewer")';

export const classicalSurfaceWithOpenPointOpening: ClassicalSurfaceDto = {
    direction_cross_link: classicalSurfaceWithoutImplicitCast.direction_cross_link,
    point_opening: {
        day_stem_zh: '甲',
        hour_branch_zh: '戌',
        hour_pillar_zh: '甲戌',
        hour_branch_vi: 'Tuất',
        hour_time_range: '19:00-21:00',
        hour_slot_index: 10,
        civil_day_canchi: giapThinDayCanchi,
        slot_day_canchi: giapThinDayCanchi,
        late_night_day_transition: false,
        cross_day_spillover: false,
        context: {
            policy_id: 'TY_NGO_LUU_CHU_POLICY_V1',
            state: {
                state: 'open',
                slot_class_zh_as_printed: '井',
                phase_annotation_as_printed: '井金',
                points: [openPointIdentity],
                substitution: null,
            },
            disclaimer: {
                id: 'historical_procedural_citation_v1',
                vi: pointOpeningDisclaimerVi,
                en: pointOpeningDisclaimerEn,
            },
            review_state: pendingRowReviewMarker,
            nomenclature_review_state: pendingNomenclatureReviewMarker,
            safety_class: 'historical_procedural_citation',
            time_basis: 'local_civil_hour_branch',
            known_divergence_ids: ['TNLC-DIV-01', 'TNLC-DIV-02', 'TNLC-DIV-03', 'TNLC-DIV-05'],
        },
        provenance: {
            source_id: 'ty-ngo-luu-chu',
            work_evidence: [pointOpeningWorkEvidence],
            table_evidence: pointOpeningTableEvidence,
        },
    },
    point_opening_citation_lines: canonicalOpenCitationLines,
};

export const classicalSurfaceWithClosedPointOpening: ClassicalSurfaceDto = {
    direction_cross_link: classicalSurfaceWithoutImplicitCast.direction_cross_link,
    point_opening: {
        day_stem_zh: '甲',
        hour_branch_zh: '子',
        hour_pillar_zh: '甲子',
        hour_branch_vi: 'Tý',
        hour_time_range: '23:00-01:00',
        hour_slot_index: 0,
        civil_day_canchi: giapThinDayCanchi,
        slot_day_canchi: giapThinDayCanchi,
        late_night_day_transition: false,
        cross_day_spillover: false,
        context: {
            policy_id: 'TY_NGO_LUU_CHU_POLICY_V1',
            state: {
                state: 'closed',
                running_tables: ['gui', 'jia'],
                doctrine_zh:
                    '得時為之開，失時為之闔（論子午流注法）；闔者閉也，陽日遇陰時，陰日遇陽時，則前穴已閉（流注時日）',
                note: 'neither running day-table lists this hour block; the Xu-style tables as printed leave it without an assigned point (閉穴)',
            },
            disclaimer: {
                id: 'historical_procedural_citation_v1',
                vi: canonicalClosedCitationLines[10].slice(
                    canonicalClosedCitationLines[10].indexOf('vi: ') + 4,
                ),
                en: canonicalClosedCitationLines[11].slice(
                    canonicalClosedCitationLines[11].indexOf('en: ') + 4,
                ),
            },
            review_state: pendingRowReviewMarker,
            nomenclature_review_state: pendingNomenclatureReviewMarker,
            safety_class: 'historical_procedural_citation',
            time_basis: 'local_civil_hour_branch',
            known_divergence_ids: ['TNLC-DIV-01', 'TNLC-DIV-02', 'TNLC-DIV-03', 'TNLC-DIV-05'],
        },
        provenance: {
            source_id: 'ty-ngo-luu-chu',
        },
    },
    point_opening_citation_lines: canonicalClosedCitationLines,
};
