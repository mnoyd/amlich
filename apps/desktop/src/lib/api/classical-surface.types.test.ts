/**
 * Compile-time contract checks for the user-facing v1.7 classical surfaces.
 * Rust source of truth: IChingCastSummary, DirectionCrossLinkSummary, and the
 * desktop ClassicalSurfaceDto command projection.
 */

import type {
    ClassicalSurfaceDto,
    DirectionCellDto,
    DirectionCrossLinkSummaryDto,
    IChingCastSummaryDto,
} from './types';

type AssertTrue<T extends true> = T;
type Equals<X, Y> =
    (<T>() => T extends X ? 1 : 2) extends (<T>() => T extends Y ? 1 : 2) ? true : false;

type _SurfaceKeys = AssertTrue<
    Equals<
        keyof ClassicalSurfaceDto,
        'iching_cast' | 'direction_cross_link' | 'traditional_wellness'
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
