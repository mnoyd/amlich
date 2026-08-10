# Amlich Domain Context

This context captures the project language for Vietnamese lunar-calendar assessment, advisory, and timing decisions.

## Language

**Day Assessment**:
A suitability verdict for a solar date in the context of an intent and, when available, a personal birth profile. It decides whether the day should be used, approached cautiously, or avoided.
_Avoid_: day ranking, lucky score

**Hour Ranking**:
An ordering of the twelve traditional hour slots within an already-assessed day. It does not change the day assessment or decide whether the day itself is usable.
_Avoid_: hour verdict, hour assessment

**Hour Ranking Vector**:
The semantic dimensions used to order hour slots: Hoàng Đạo quality, intent timing fit, personal hour alignment, and day-hour harmony.
_Avoid_: hour embedding, opaque vector

**Hoàng Đạo Quality**:
The hour-ranking dimension that represents whether a traditional hour slot is auspicious according to the day’s Hoàng Đạo/Hắc Đạo classification.
_Avoid_: lucky hour flag

**Intent Timing Fit**:
The hour-ranking dimension that represents how well an hour slot supports the requested activity, such as travel, contract signing, or opening. It is available only when source-backed hour-specific rules exist for the intent.
_Avoid_: activity bonus

**Personal Hour Alignment**:
The hour-ranking dimension that represents compatibility between the hour slot and the person’s birth-derived facts when those facts are available.
_Avoid_: user bonus

**Day-Hour Harmony**:
The hour-ranking dimension that represents compatibility between the chosen day and the hour slot itself.
_Avoid_: time bonus

**Assessment Factor**:
A source-attributed input considered while forming a Day Assessment, with an explicit role and availability state.
_Avoid_: lucky signal, raw score

**Context Fact**:
An Assessment Factor that establishes calendar or personal context without directly moving an assessment axis or overriding its verdict.
_Avoid_: neutral feature

**Scored Feature**:
An Assessment Factor whose normalized value directly contributes to a declared assessment axis.
_Avoid_: bonus, penalty

**Veto**:
A named, source-attributed constraint that takes precedence over weighted suitability for its declared scope.
_Avoid_: large negative weight, implicit hard rule

**Explanation-only Factor**:
An Assessment Factor retained to explain relevant almanac context but excluded from both weighted aggregation and veto precedence.
_Avoid_: ignored factor, unused data
