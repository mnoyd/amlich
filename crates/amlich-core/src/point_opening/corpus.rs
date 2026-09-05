//! Validated loader for the frozen Xu-style Najia point-opening corpus.
//!
//! The JSON is embedded and parsed once. Loading is deliberately stricter than
//! serde shape checking: dimensions, keys, row references, identities,
//! provenance, review markers, safety classes, and divergence ids must all be
//! internally consistent before a record can be queried. The query performs no
//! calendar calculation and never fills a missing or closed slot.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use crate::sources::SOURCE_TY_NGO_LUU_CHU;
use crate::traditional_wellness::divergence::ExternalReviewState;

use super::divergence::tnlc_divergence_by_id;
use super::policy::{policy_contract, SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION};
use super::state::{PointOpeningContext, PointOpeningIdentity, PointOpeningSlotState};

const CORPUS_JSON: &str = include_str!("../../data/ty-ngo-luu-chu/najia-open-points.json");
const SCHEMA_VERSION: &str = "najia_open_points_v1";
const DAY_STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const HOUR_BRANCHES: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];

/// A validated frozen grid record. `context.state` is copied from the exact
/// referenced row or explicit closed evidence; it is never computed.
#[derive(Debug, Clone, PartialEq)]
pub struct FrozenPointOpeningRecord {
    pub day_stem_zh: String,
    pub hour_branch_zh: String,
    pub hour_pillar_zh: String,
    pub cross_day_spillover: bool,
    pub context: PointOpeningContext,
}

/// Deterministic failure returned while validating corpus content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusValidationError(String);

impl CorpusValidationError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for CorpusValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for CorpusValidationError {}

#[derive(Debug)]
struct ValidatedCorpus {
    records: Vec<FrozenPointOpeningRecord>,
}

static CORPUS: OnceLock<ValidatedCorpus> = OnceLock::new();

/// Return all 120 validated day-stem × hour-branch records.
///
/// # Panics
/// Panics if the compile-embedded corpus violates its frozen contract. Such a
/// failure is a repository defect and is exercised by focused mutation tests.
pub fn all_frozen_point_opening_records() -> &'static [FrozenPointOpeningRecord] {
    &CORPUS
        .get_or_init(|| {
            validate_corpus_json(CORPUS_JSON)
                .unwrap_or_else(|error| panic!("invalid frozen TNLC corpus: {error}"))
        })
        .records
}

/// Look up one already-frozen record by Chinese day stem and hour branch.
/// Unknown keys return `None`; no interpolation or fallback is attempted.
pub fn frozen_point_opening_record(
    day_stem_zh: &str,
    hour_branch_zh: &str,
) -> Option<&'static FrozenPointOpeningRecord> {
    all_frozen_point_opening_records()
        .iter()
        .find(|record| record.day_stem_zh == day_stem_zh && record.hour_branch_zh == hour_branch_zh)
}

#[derive(Debug, Deserialize)]
struct RawCorpus {
    metadata: Metadata,
    counts: Counts,
    day_tables: Vec<DayTable>,
    grid: Vec<GridCell>,
    point_nomenclature_registry: Vec<RegistryPoint>,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    schema_version: String,
    primary_source: PrimarySource,
}

#[derive(Debug, Deserialize)]
struct PrimarySource {
    source_id: String,
    transcription_uri: String,
    edition_or_facsimile_uri: String,
    translation_kind: String,
}

#[derive(Debug, Deserialize)]
struct Counts {
    day_tables: usize,
    table_rows: usize,
    grid_cells: usize,
    open_cells: usize,
    closed_cells: usize,
    registry_points: usize,
    open_cells_by_day_stem: HashMap<String, usize>,
}

#[derive(Debug, Deserialize)]
struct DayTable {
    table_id: String,
    day_stem_zh: String,
    rows: Vec<TableRow>,
}

#[derive(Debug, Deserialize)]
struct TableRow {
    row_index: usize,
    slot_class_zh_as_printed: String,
    phase_annotation_as_printed: String,
    points: Vec<RowPoint>,
    substitution: Option<String>,
    resolved_cell: ResolvedCell,
    sources: Vec<SourceEvidence>,
    reviewer: String,
    safety_class: String,
    known_divergence_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RowPoint {
    point_key: String,
    xue_ming_zh: String,
    huyet_danh_vi_draft_gate2_pending: String,
    standard_code_gloss_draft_gate2_pending: String,
    channel_zh: String,
    channel_vi: String,
    channel_en: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct ResolvedCell {
    day_stem_zh: String,
    hour_branch_zh: String,
    cross_day_spillover: bool,
}

#[derive(Debug, Deserialize)]
struct SourceEvidence {
    source_id: String,
    work_title: String,
    volume_or_chapter: String,
    passage_key: String,
    edition_or_facsimile_uri: String,
    transcription_uri: String,
    cross_reference_uri: String,
    translation_kind: String,
}

#[derive(Debug, Deserialize)]
struct GridCell {
    day_stem_zh: String,
    hour_branch_zh: String,
    hour_pillar_zh: String,
    state: String,
    resolves_to: Option<RowReference>,
    cross_day_spillover: Option<bool>,
    closed_evidence: Option<ClosedEvidence>,
    reviewer: String,
    safety_class: String,
    known_divergence_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RowReference {
    table: String,
    row_index: usize,
}

#[derive(Debug, Deserialize)]
struct ClosedEvidence {
    running_tables: Vec<String>,
    doctrine_zh: String,
    note: String,
}

#[derive(Debug, Deserialize)]
struct RegistryPoint {
    point_key: String,
    xue_ming_zh_as_printed: String,
    huyet_danh_vi_draft_gate2_pending: String,
    standard_code_gloss_draft_gate2_pending: String,
    channel_zh: String,
    channel_vi: String,
    channel_en: String,
    nomenclature_reviewer: String,
    known_divergence_ids: Vec<String>,
}

fn validate_corpus_json(json: &str) -> Result<ValidatedCorpus, CorpusValidationError> {
    let raw: RawCorpus = serde_json::from_str(json)
        .map_err(|error| CorpusValidationError::new(format!("JSON parse failed: {error}")))?;
    validate_metadata(&raw.metadata)?;
    validate_declared_counts(&raw)?;

    let registry = validate_registry(&raw.point_nomenclature_registry)?;
    let nomenclature_review_state =
        common_nomenclature_review_state(&raw.point_nomenclature_registry)?;
    let rows = validate_tables(&raw.day_tables, &registry)?;
    validate_grid(&raw, &rows, nomenclature_review_state)
}

fn validate_metadata(metadata: &Metadata) -> Result<(), CorpusValidationError> {
    ensure(
        metadata.schema_version == SCHEMA_VERSION,
        format!(
            "metadata.schema_version must be {SCHEMA_VERSION:?}, found {:?}",
            metadata.schema_version
        ),
    )?;
    let contract = policy_contract();
    ensure(
        metadata.primary_source.source_id == contract.source_id,
        "metadata.primary_source.source_id violates the policy contract",
    )?;
    for (field, value) in [
        (
            "transcription_uri",
            metadata.primary_source.transcription_uri.as_str(),
        ),
        (
            "edition_or_facsimile_uri",
            metadata.primary_source.edition_or_facsimile_uri.as_str(),
        ),
        (
            "translation_kind",
            metadata.primary_source.translation_kind.as_str(),
        ),
    ] {
        ensure(
            !value.trim().is_empty(),
            format!("metadata primary source {field} is empty"),
        )?;
    }
    Ok(())
}

fn validate_declared_counts(raw: &RawCorpus) -> Result<(), CorpusValidationError> {
    let row_count: usize = raw.day_tables.iter().map(|table| table.rows.len()).sum();
    let open_count = raw.grid.iter().filter(|cell| cell.state == "open").count();
    let closed_count = raw
        .grid
        .iter()
        .filter(|cell| cell.state == "closed")
        .count();
    let actual = [
        (
            "day_tables",
            raw.counts.day_tables,
            raw.day_tables.len(),
            10,
        ),
        ("table_rows", raw.counts.table_rows, row_count, 60),
        ("grid_cells", raw.counts.grid_cells, raw.grid.len(), 120),
        ("open_cells", raw.counts.open_cells, open_count, 60),
        ("closed_cells", raw.counts.closed_cells, closed_count, 60),
        (
            "registry_points",
            raw.counts.registry_points,
            raw.point_nomenclature_registry.len(),
            66,
        ),
    ];
    for (name, declared, found, expected) in actual {
        ensure(
            declared == found && found == expected,
            format!(
                "count {name} must be declared={expected} and actual={expected}, found declared={declared}, actual={found}"
            ),
        )?;
    }
    Ok(())
}

fn validate_registry(
    points: &[RegistryPoint],
) -> Result<HashMap<&str, &RegistryPoint>, CorpusValidationError> {
    let mut registry = HashMap::new();
    for point in points {
        ensure(
            !point.point_key.is_empty(),
            "registry contains an empty point_key",
        )?;
        ensure(
            registry.insert(point.point_key.as_str(), point).is_none(),
            format!("duplicate registry point_key {:?}", point.point_key),
        )?;
        validate_review_marker(
            &point.nomenclature_reviewer,
            &format!("registry point {:?}", point.point_key),
        )?;
        validate_divergences(
            &point.known_divergence_ids,
            &format!("registry point {:?}", point.point_key),
        )?;
        ensure(
            point.known_divergence_ids == ["TNLC-DIV-04"],
            format!(
                "registry point {:?} must carry only TNLC-DIV-04",
                point.point_key
            ),
        )?;
        for (name, value) in point_identity_fields(point) {
            ensure(
                !value.trim().is_empty(),
                format!("registry point {:?} has empty {name}", point.point_key),
            )?;
        }
    }
    Ok(registry)
}

fn common_nomenclature_review_state(
    points: &[RegistryPoint],
) -> Result<ExternalReviewState, CorpusValidationError> {
    let first = points
        .first()
        .ok_or_else(|| CorpusValidationError::new("nomenclature registry is empty"))?;
    ensure(
        points
            .iter()
            .all(|point| point.nomenclature_reviewer == first.nomenclature_reviewer),
        "nomenclature registry contains inconsistent review states",
    )?;
    parse_review_marker(&first.nomenclature_reviewer, "nomenclature registry")
}

fn validate_tables<'a>(
    tables: &'a [DayTable],
    registry: &HashMap<&str, &RegistryPoint>,
) -> Result<HashMap<(&'a str, usize), &'a TableRow>, CorpusValidationError> {
    let mut table_ids = HashSet::new();
    let mut stems = HashSet::new();
    let mut rows = HashMap::new();
    let mut used_points = HashSet::new();
    for table in tables {
        ensure(
            table_ids.insert(table.table_id.as_str()),
            format!("duplicate table_id {:?}", table.table_id),
        )?;
        ensure(
            DAY_STEMS.contains(&table.day_stem_zh.as_str())
                && stems.insert(table.day_stem_zh.as_str()),
            format!(
                "duplicate or unknown day table stem {:?}",
                table.day_stem_zh
            ),
        )?;
        ensure(
            table.rows.len() == 6,
            format!(
                "table {:?} must contain 6 rows, found {}",
                table.table_id,
                table.rows.len()
            ),
        )?;
        for row in &table.rows {
            ensure(
                (1..=6).contains(&row.row_index)
                    && rows
                        .insert((table.table_id.as_str(), row.row_index), row)
                        .is_none(),
                format!(
                    "table {:?} has duplicate or invalid row_index {}",
                    table.table_id, row.row_index
                ),
            )?;
            ensure(
                !row.points.is_empty(),
                format!(
                    "table {:?} row {} has no points",
                    table.table_id, row.row_index
                ),
            )?;
            ensure(
                !row.slot_class_zh_as_printed.is_empty(),
                format!(
                    "table {:?} row {} has empty slot class",
                    table.table_id, row.row_index
                ),
            )?;
            ensure(
                !row.phase_annotation_as_printed.is_empty(),
                format!(
                    "table {:?} row {} has empty phase annotation",
                    table.table_id, row.row_index
                ),
            )?;
            validate_review_marker(&row.reviewer, &row_label(table, row))?;
            validate_safety(&row.safety_class, &row_label(table, row))?;
            validate_divergences(&row.known_divergence_ids, &row_label(table, row))?;
            validate_sources(&row.sources, &row_label(table, row))?;
            for point in &row.points {
                ensure(
                    !point.role.trim().is_empty(),
                    format!(
                        "table {:?} row {} point {:?} has empty role",
                        table.table_id, row.row_index, point.point_key
                    ),
                )?;
                let registered = registry.get(point.point_key.as_str()).ok_or_else(|| {
                    CorpusValidationError::new(format!(
                        "table {:?} row {} references missing registry point {:?}",
                        table.table_id, row.row_index, point.point_key
                    ))
                })?;
                ensure(
                    row_point_matches_registry(point, registered),
                    format!(
                        "table {:?} row {} identity for {:?} differs from registry",
                        table.table_id, row.row_index, point.point_key
                    ),
                )?;
                used_points.insert(point.point_key.as_str());
            }
        }
    }
    ensure(
        stems.len() == DAY_STEMS.len(),
        "day tables do not cover all ten stems",
    )?;
    ensure(
        used_points.len() == registry.len(),
        format!(
            "nomenclature registry has unreferenced points: used {}, registered {}",
            used_points.len(),
            registry.len()
        ),
    )?;
    Ok(rows)
}

fn validate_grid(
    raw: &RawCorpus,
    rows: &HashMap<(&str, usize), &TableRow>,
    nomenclature_review_state: ExternalReviewState,
) -> Result<ValidatedCorpus, CorpusValidationError> {
    let table_ids: HashSet<&str> = rows.keys().map(|(table_id, _)| *table_id).collect();
    let mut keys = HashSet::new();
    let mut open_by_stem: HashMap<&str, usize> = HashMap::new();
    let mut records = Vec::with_capacity(120);
    for cell in &raw.grid {
        ensure(
            DAY_STEMS.contains(&cell.day_stem_zh.as_str())
                && HOUR_BRANCHES.contains(&cell.hour_branch_zh.as_str()),
            format!(
                "grid contains unknown slot {}/{}",
                cell.day_stem_zh, cell.hour_branch_zh
            ),
        )?;
        ensure(
            keys.insert((cell.day_stem_zh.as_str(), cell.hour_branch_zh.as_str())),
            format!(
                "duplicate grid slot {}/{}",
                cell.day_stem_zh, cell.hour_branch_zh
            ),
        )?;
        let review_state = parse_review_marker(&cell.reviewer, &cell_label(cell))?;
        validate_safety(&cell.safety_class, &cell_label(cell))?;
        validate_divergences(&cell.known_divergence_ids, &cell_label(cell))?;

        let (state, spillover) = match cell.state.as_str() {
            "open" => {
                *open_by_stem.entry(&cell.day_stem_zh).or_default() += 1;
                let reference = cell.resolves_to.as_ref().ok_or_else(|| {
                    CorpusValidationError::new(format!(
                        "{} is open without resolves_to",
                        cell_label(cell)
                    ))
                })?;
                ensure(
                    cell.closed_evidence.is_none(),
                    format!("{} is open but carries closed_evidence", cell_label(cell)),
                )?;
                let row = rows
                    .get(&(reference.table.as_str(), reference.row_index))
                    .ok_or_else(|| {
                        CorpusValidationError::new(format!(
                            "{} references missing row {:?}/{}",
                            cell_label(cell),
                            reference.table,
                            reference.row_index
                        ))
                    })?;
                let spillover = cell.cross_day_spillover.ok_or_else(|| {
                    CorpusValidationError::new(format!(
                        "{} is open without cross_day_spillover",
                        cell_label(cell)
                    ))
                })?;
                ensure(
                    row.resolved_cell.day_stem_zh == cell.day_stem_zh
                        && row.resolved_cell.hour_branch_zh == cell.hour_branch_zh
                        && row.resolved_cell.cross_day_spillover == spillover,
                    format!(
                        "{} disagrees with referenced row resolved_cell",
                        cell_label(cell)
                    ),
                )?;
                ensure(
                    row.reviewer == cell.reviewer,
                    format!(
                        "{} review state differs from referenced row",
                        cell_label(cell)
                    ),
                )?;
                ensure(
                    row.safety_class == cell.safety_class,
                    format!(
                        "{} safety class differs from referenced row",
                        cell_label(cell)
                    ),
                )?;
                ensure(
                    row.known_divergence_ids == cell.known_divergence_ids,
                    format!(
                        "{} divergences differ from referenced row",
                        cell_label(cell)
                    ),
                )?;
                (
                    PointOpeningSlotState::Open {
                        slot_class_zh_as_printed: row.slot_class_zh_as_printed.clone(),
                        phase_annotation_as_printed: row.phase_annotation_as_printed.clone(),
                        points: row.points.iter().map(to_identity).collect(),
                        substitution: row.substitution.clone(),
                    },
                    spillover,
                )
            }
            "closed" => {
                ensure(
                    cell.resolves_to.is_none(),
                    format!("{} is closed but carries resolves_to", cell_label(cell)),
                )?;
                ensure(
                    cell.cross_day_spillover.is_none(),
                    format!(
                        "{} is closed but carries cross_day_spillover",
                        cell_label(cell)
                    ),
                )?;
                let evidence = cell.closed_evidence.as_ref().ok_or_else(|| {
                    CorpusValidationError::new(format!(
                        "{} is closed without closed_evidence",
                        cell_label(cell)
                    ))
                })?;
                ensure(
                    evidence.running_tables.len() == 2,
                    format!(
                        "{} closed evidence must name two running tables",
                        cell_label(cell)
                    ),
                )?;
                ensure(
                    evidence
                        .running_tables
                        .iter()
                        .all(|table_id| table_ids.contains(table_id.as_str())),
                    format!("{} names an unknown running table", cell_label(cell)),
                )?;
                ensure(
                    !evidence.doctrine_zh.trim().is_empty() && !evidence.note.trim().is_empty(),
                    format!("{} has incomplete closed evidence", cell_label(cell)),
                )?;
                (
                    PointOpeningSlotState::Closed {
                        running_tables: evidence.running_tables.clone(),
                        doctrine_zh: evidence.doctrine_zh.clone(),
                        note: evidence.note.clone(),
                    },
                    false,
                )
            }
            other => {
                return Err(CorpusValidationError::new(format!(
                    "{} has unknown state {other:?}",
                    cell_label(cell)
                )))
            }
        };
        records.push(FrozenPointOpeningRecord {
            day_stem_zh: cell.day_stem_zh.clone(),
            hour_branch_zh: cell.hour_branch_zh.clone(),
            hour_pillar_zh: cell.hour_pillar_zh.clone(),
            cross_day_spillover: spillover,
            context: PointOpeningContext::new(
                state,
                review_state,
                nomenclature_review_state.clone(),
                cell.known_divergence_ids.clone(),
            ),
        });
    }
    ensure(
        keys.len() == DAY_STEMS.len() * HOUR_BRANCHES.len(),
        "grid does not cover all 120 unique slots",
    )?;
    for stem in DAY_STEMS {
        let actual = open_by_stem.get(stem).copied().unwrap_or(0);
        let declared = raw.counts.open_cells_by_day_stem.get(stem).copied();
        ensure(
            declared == Some(actual),
            format!(
                "open_cells_by_day_stem mismatch for {stem}: declared {declared:?}, actual {actual}"
            ),
        )?;
    }
    ensure(
        raw.counts.open_cells_by_day_stem.len() == DAY_STEMS.len(),
        "open_cells_by_day_stem contains unexpected extra or missing stems",
    )?;
    Ok(ValidatedCorpus { records })
}

fn validate_sources(sources: &[SourceEvidence], label: &str) -> Result<(), CorpusValidationError> {
    ensure(
        !sources.is_empty(),
        format!("{label} has no source evidence"),
    )?;
    let contract = policy_contract();
    for source in sources {
        ensure(
            source.source_id == SOURCE_TY_NGO_LUU_CHU && source.source_id == contract.source_id,
            format!("{label} has invalid source_id {:?}", source.source_id),
        )?;
        ensure(
            source.source_id != contract.never_cross_cites,
            format!("{label} cross-cites the forbidden Tier-0 source"),
        )?;
        for (name, value) in [
            ("work_title", source.work_title.as_str()),
            ("volume_or_chapter", source.volume_or_chapter.as_str()),
            ("passage_key", source.passage_key.as_str()),
            (
                "edition_or_facsimile_uri",
                source.edition_or_facsimile_uri.as_str(),
            ),
            ("transcription_uri", source.transcription_uri.as_str()),
            ("cross_reference_uri", source.cross_reference_uri.as_str()),
            ("translation_kind", source.translation_kind.as_str()),
        ] {
            ensure(
                !value.trim().is_empty(),
                format!("{label} source evidence has empty {name}"),
            )?;
        }
    }
    Ok(())
}

fn validate_review_marker(marker: &str, label: &str) -> Result<(), CorpusValidationError> {
    parse_review_marker(marker, label).map(|_| ())
}

fn parse_review_marker(
    marker: &str,
    label: &str,
) -> Result<ExternalReviewState, CorpusValidationError> {
    ExternalReviewState::from_marker(marker).ok_or_else(|| {
        CorpusValidationError::new(format!("{label} has invalid review marker {marker:?}"))
    })
}

fn validate_safety(value: &str, label: &str) -> Result<(), CorpusValidationError> {
    ensure(
        value == SAFETY_CLASS_HISTORICAL_PROCEDURAL_CITATION,
        format!("{label} has invalid safety_class {value:?}"),
    )
}

fn validate_divergences(ids: &[String], label: &str) -> Result<(), CorpusValidationError> {
    ensure(
        !ids.is_empty(),
        format!("{label} has no divergence references"),
    )?;
    let mut seen = HashSet::new();
    for id in ids {
        ensure(
            tnlc_divergence_by_id(id).is_some(),
            format!("{label} has unknown divergence id {id:?}"),
        )?;
        ensure(
            seen.insert(id),
            format!("{label} repeats divergence id {id:?}"),
        )?;
    }
    Ok(())
}

fn row_point_matches_registry(point: &RowPoint, registered: &RegistryPoint) -> bool {
    point.xue_ming_zh == registered.xue_ming_zh_as_printed
        && point.huyet_danh_vi_draft_gate2_pending == registered.huyet_danh_vi_draft_gate2_pending
        && point.standard_code_gloss_draft_gate2_pending
            == registered.standard_code_gloss_draft_gate2_pending
        && point.channel_zh == registered.channel_zh
        && point.channel_vi == registered.channel_vi
        && point.channel_en == registered.channel_en
}

fn point_identity_fields(point: &RegistryPoint) -> [(&'static str, &str); 7] {
    [
        ("point_key", &point.point_key),
        ("xue_ming_zh_as_printed", &point.xue_ming_zh_as_printed),
        ("huyet_danh_vi", &point.huyet_danh_vi_draft_gate2_pending),
        (
            "standard_code_gloss",
            &point.standard_code_gloss_draft_gate2_pending,
        ),
        ("channel_zh", &point.channel_zh),
        ("channel_vi", &point.channel_vi),
        ("channel_en", &point.channel_en),
    ]
}

fn to_identity(point: &RowPoint) -> PointOpeningIdentity {
    PointOpeningIdentity {
        point_key: point.point_key.clone(),
        xue_ming_zh: point.xue_ming_zh.clone(),
        huyet_danh_vi: point.huyet_danh_vi_draft_gate2_pending.clone(),
        standard_code_gloss: point.standard_code_gloss_draft_gate2_pending.clone(),
        channel_zh: point.channel_zh.clone(),
        channel_vi: point.channel_vi.clone(),
        channel_en: point.channel_en.clone(),
        role: point.role.clone(),
    }
}

fn row_label(table: &DayTable, row: &TableRow) -> String {
    format!("table {:?} row {}", table.table_id, row.row_index)
}

fn cell_label(cell: &GridCell) -> String {
    format!("grid cell {}/{}", cell.day_stem_zh, cell.hour_branch_zh)
}

fn ensure(condition: bool, message: impl Into<String>) -> Result<(), CorpusValidationError> {
    if condition {
        Ok(())
    } else {
        Err(CorpusValidationError::new(message))
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    fn mutate(mutator: impl FnOnce(&mut Value)) -> CorpusValidationError {
        let mut value: Value = serde_json::from_str(CORPUS_JSON).unwrap();
        mutator(&mut value);
        validate_corpus_json(&serde_json::to_string(&value).unwrap()).unwrap_err()
    }

    #[test]
    fn complete_frozen_corpus_loads() {
        let corpus = validate_corpus_json(CORPUS_JSON).unwrap();
        assert_eq!(corpus.records.len(), 120);
    }

    #[test]
    fn query_returns_exact_open_record() {
        let record = frozen_point_opening_record("癸", "子").unwrap();
        assert!(record.cross_day_spillover);
        assert_eq!(record.hour_pillar_zh, "壬子");
        let PointOpeningSlotState::Open {
            points,
            substitution,
            ..
        } = &record.context.state
        else {
            panic!("癸/子 must be frozen open")
        };
        assert_eq!(points[0].xue_ming_zh, "關沖");
        assert_eq!(substitution.as_deref(), Some("qi_na_san_jiao"));
    }

    #[test]
    fn query_preserves_explicit_closed_record_and_never_falls_back() {
        let record = frozen_point_opening_record("甲", "子").unwrap();
        assert!(!record.cross_day_spillover);
        assert!(matches!(
            record.context.state,
            PointOpeningSlotState::Closed { .. }
        ));
        assert!(frozen_point_opening_record("甲", "not-a-branch").is_none());
    }

    #[test]
    fn invalid_dimensions_fail_specifically() {
        let error = mutate(|value| {
            value["grid"].as_array_mut().unwrap().pop();
        });
        assert!(error.to_string().contains("count grid_cells"), "{error}");
    }

    #[test]
    fn duplicate_slots_fail_specifically() {
        let error = mutate(|value| {
            let duplicate = value["grid"][0].clone();
            value["grid"][2] = duplicate;
        });
        assert!(error.to_string().contains("duplicate grid slot"), "{error}");
    }

    #[test]
    fn missing_identity_fails_specifically() {
        let error = mutate(|value| {
            value["day_tables"][0]["rows"][0]["points"][0]["point_key"] =
                Value::String("absent-point".into());
        });
        assert!(
            error.to_string().contains("missing registry point"),
            "{error}"
        );
    }

    #[test]
    fn malformed_source_evidence_fails_specifically() {
        let error = mutate(|value| {
            value["day_tables"][0]["rows"][0]["sources"][0]["transcription_uri"] =
                Value::String(String::new());
        });
        assert!(
            error.to_string().contains("empty transcription_uri"),
            "{error}"
        );
    }

    #[test]
    fn bad_review_safety_and_divergence_fail_specifically() {
        let review = mutate(|value| {
            value["grid"][0]["reviewer"] = Value::String("pending-ish".into());
        });
        assert!(
            review.to_string().contains("invalid review marker"),
            "{review}"
        );

        let safety = mutate(|value| {
            value["grid"][0]["safety_class"] = Value::String("clinical".into());
        });
        assert!(
            safety.to_string().contains("invalid safety_class"),
            "{safety}"
        );

        let divergence = mutate(|value| {
            value["grid"][0]["known_divergence_ids"][0] = Value::String("TNLC-DIV-99".into());
        });
        assert!(
            divergence.to_string().contains("unknown divergence id"),
            "{divergence}"
        );
    }

    #[test]
    fn broken_row_reference_fails_specifically() {
        let error = mutate(|value| {
            let open = value["grid"]
                .as_array_mut()
                .unwrap()
                .iter_mut()
                .find(|cell| cell["state"] == "open")
                .unwrap();
            open["resolves_to"]["row_index"] = Value::from(99);
        });
        assert!(
            error.to_string().contains("references missing row"),
            "{error}"
        );
    }
}
