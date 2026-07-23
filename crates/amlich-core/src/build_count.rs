//! Per-thread build counters. The personal-day and hour-selection request
//! paths must build the snapshot, the canonical assessment, and the
//! per-request personal facts at most once per request. These counters
//! let regression tests assert that — see `amlich-9z7i` and the
//! "no endpoint-local independent verdict" acceptance criterion in
//! `REPAIR-PLAN.md`.

use std::cell::Cell;

#[derive(Default, Debug, Clone, Copy)]
pub struct BuildCounters {
    pub snapshot_builds: u32,
    pub canonical_assessments: u32,
    pub bazi_charts: u32,
    pub element_distributions: u32,
    pub kua_computations: u32,
    pub day_person_matrices: u32,
    pub personal_hour_matrices: u32,
    pub direction_merge_matrices: u32,
}

thread_local! {
    static COUNTERS: Cell<BuildCounters> = const { Cell::new(BuildCounters {
        snapshot_builds: 0,
        canonical_assessments: 0,
        bazi_charts: 0,
        element_distributions: 0,
        kua_computations: 0,
        day_person_matrices: 0,
        personal_hour_matrices: 0,
        direction_merge_matrices: 0,
    }) };
}

pub fn snapshot_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.snapshot_builds += 1;
        c.set(v);
    });
}

pub fn canonical_assessment_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.canonical_assessments += 1;
        c.set(v);
    });
}

pub fn bazi_chart_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.bazi_charts += 1;
        c.set(v);
    });
}

pub fn element_distribution_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.element_distributions += 1;
        c.set(v);
    });
}

pub fn kua_computed() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.kua_computations += 1;
        c.set(v);
    });
}

pub fn day_person_matrix_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.day_person_matrices += 1;
        c.set(v);
    });
}

pub fn personal_hour_matrix_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.personal_hour_matrices += 1;
        c.set(v);
    });
}

pub fn direction_merge_matrix_built() {
    COUNTERS.with(|c| {
        let mut v = c.get();
        v.direction_merge_matrices += 1;
        c.set(v);
    });
}

pub fn reset() {
    COUNTERS.with(|c| c.set(BuildCounters::default()));
}

pub fn snapshot() -> BuildCounters {
    COUNTERS.with(|c| c.get())
}
