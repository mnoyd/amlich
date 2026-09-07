#!/usr/bin/env python3
"""Regenerate the TNLC finite-core golden matrix (bead amlich-xlag.2.2.7).

Derives `crates/amlich-core/data/ty-ngo-luu-chu/finite-core-golden.json`
independently from the frozen corpus
`crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json` (v1.11
freeze, bead amlich-xlag.2.1): for every one of the 120 day-stem x
hour-branch slots it projects the hour pillar, the cross-day spillover
marker, and the exactly-one open-or-explicit-closed state — open slots
carry the referenced row's slot class, phase annotation, substitution,
table evidence, and the full point identity triples; closed slots carry
the explicit unavailability evidence.

The Rust suite `crates/amlich-core/tests/point_opening_finite_core_golden.rs`
compares the engine resolver output against this file. Any change to the
frozen corpus requires regenerating this golden (re-freeze procedure):

    python3 scripts/gen-tnlc-finite-core-golden.py

and the header pins the corpus fingerprint so the two files can never
drift silently.
"""

import json
import pathlib
import sys

REPO = pathlib.Path(__file__).resolve().parent.parent
CORPUS_PATH = REPO / "crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json"
GOLDEN_PATH = REPO / "crates/amlich-core/data/ty-ngo-luu-chu/finite-core-golden.json"

STEMS = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"]
BRANCHES = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"]

IDENTITY_FIELDS = [
    "point_key",
    "xue_ming_zh",
    "huyet_danh_vi_draft_gate2_pending",
    "standard_code_gloss_draft_gate2_pending",
    "channel_zh",
    "channel_vi",
    "channel_en",
    "role",
]


def fnv1a64(data: bytes) -> str:
    h = 0xCBF29CE484222325
    for byte in data:
        h ^= byte
        h = (h * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{h:016x}"


def project_point(raw_point):
    return {
        "point_key": raw_point["point_key"],
        "xue_ming_zh": raw_point["xue_ming_zh"],
        "huyet_danh_vi": raw_point["huyet_danh_vi_draft_gate2_pending"],
        "standard_code_gloss": raw_point["standard_code_gloss_draft_gate2_pending"],
        "channel_zh": raw_point["channel_zh"],
        "channel_vi": raw_point["channel_vi"],
        "channel_en": raw_point["channel_en"],
        "role": raw_point["role"],
    }


def main() -> int:
    corpus_bytes = CORPUS_PATH.read_bytes()
    corpus = json.loads(corpus_bytes)

    tables = {
        table["table_id"]: {row["row_index"]: row for row in table["rows"]}
        for table in corpus["day_tables"]
    }

    cells = {
        (cell["day_stem_zh"], cell["hour_branch_zh"]): cell for cell in corpus["grid"]
    }
    if len(cells) != 120:
        sys.exit(f"corpus grid must hold 120 unique slots, found {len(cells)}")

    matrix = []
    open_count = closed_count = spillover_count = 0
    open_by_stem = {stem: 0 for stem in STEMS}
    for stem in STEMS:
        for branch in BRANCHES:
            cell = cells[(stem, branch)]
            entry = {
                "day_stem_zh": stem,
                "hour_branch_zh": branch,
                "hour_pillar_zh": cell["hour_pillar_zh"],
            }
            if cell["state"] == "open":
                open_count += 1
                open_by_stem[stem] += 1
                reference = cell["resolves_to"]
                row = tables[reference["table"]][reference["row_index"]]
                spillover = cell["cross_day_spillover"]
                if spillover:
                    spillover_count += 1
                entry["cross_day_spillover"] = spillover
                entry["state"] = {
                    "kind": "open",
                    "resolves_to": {
                        "table": reference["table"],
                        "row_index": reference["row_index"],
                    },
                    "slot_class_zh_as_printed": row["slot_class_zh_as_printed"],
                    "phase_annotation_as_printed": row["phase_annotation_as_printed"],
                    "substitution": row["substitution"],
                    "points": [project_point(point) for point in row["points"]],
                }
            elif cell["state"] == "closed":
                closed_count += 1
                evidence = cell["closed_evidence"]
                entry["cross_day_spillover"] = False
                entry["state"] = {
                    "kind": "closed",
                    "running_tables": evidence["running_tables"],
                    "doctrine_zh": evidence["doctrine_zh"],
                    "note": evidence["note"],
                }
            else:
                sys.exit(f"unknown state {cell['state']!r} at {stem}/{branch}")
            matrix.append(entry)

    golden = {
        "schema_version": "tnlc_finite_core_golden_v1",
        "bead": "amlich-xlag.2.2.7",
        "source_corpus": {
            "path": "crates/amlich-core/data/ty-ngo-luu-chu/najia-open-points.json",
            "fnv1a64": fnv1a64(corpus_bytes),
        },
        "counts": {
            "slots": len(matrix),
            "open": open_count,
            "closed": closed_count,
            "cross_day_spillovers": spillover_count,
            "open_by_day_stem": {stem: open_by_stem[stem] for stem in STEMS},
        },
        "matrix": matrix,
    }

    GOLDEN_PATH.write_text(
        json.dumps(golden, ensure_ascii=False, indent=2) + "\n", encoding="utf-8"
    )
    print(f"wrote {GOLDEN_PATH} ({len(matrix)} slots, {open_count} open, {closed_count} closed)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
