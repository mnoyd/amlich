<script lang="ts">
  import { fetchPersonalDayMatrixReport, fetchPersonalDayReport } from "$lib/api/invoke";
  import type {
    DomainDayBoostEntryDto,
    InitiationOpeningDecisionExportDto,
    PersonalDayMatrixReportDto,
    PersonalDayReportDto,
    PersonalHourEntryDto,
    UnavailableSectionDto,
  } from "$lib/api/types";
  import type { DayForInsight } from "$lib/insights/types";

  let { day }: { day: DayForInsight | null } = $props();

  let birthYear = $state("");
  let birthMonth = $state("");
  let birthDay = $state("");
  let birthHour = $state("");
  let birthMinute = $state("");
  let gender = $state("");

  let loadingReport = $state(false);
  let loadingMatrix = $state(false);
  let reportError = $state<string | null>(null);
  let matrixError = $state<string | null>(null);
  let report = $state<PersonalDayReportDto | null>(null);
  let matrix = $state<PersonalDayMatrixReportDto | null>(null);

  let loadToken = 0;

  function parseIntOrNull(value: string): number | null {
    if (!value.trim()) return null;
    const parsed = Number.parseInt(value, 10);
    return Number.isFinite(parsed) ? parsed : null;
  }

  const parsedBirthYear = $derived(parseIntOrNull(birthYear));
  const parsedBirthMonth = $derived(parseIntOrNull(birthMonth));
  const parsedBirthDay = $derived(parseIntOrNull(birthDay));
  const parsedBirthHour = $derived(parseIntOrNull(birthHour));
  const parsedBirthMinute = $derived(parseIntOrNull(birthMinute));
  const normalizedGender = $derived(gender || null);

  const hasBirthDate = $derived(
    parsedBirthYear !== null && parsedBirthMonth !== null && parsedBirthDay !== null,
  );
  const hasBirthTime = $derived(parsedBirthHour !== null && parsedBirthMinute !== null);
  const canLoadMatrix = $derived(hasBirthDate && hasBirthTime);

  function formatAxis(axis: string): string {
    return axis.replaceAll("_", " ");
  }

  function formatUnavailable(unavailable: UnavailableSectionDto[]): string {
    if (!unavailable.length) return "";
    return unavailable
      .map((section) => `${section.section}: ${section.reason}`)
      .join(" | ");
  }

  function positiveDomains(entries: DomainDayBoostEntryDto[] = []): DomainDayBoostEntryDto[] {
    return [...entries]
      .sort((left, right) => right.boosted_score - left.boosted_score)
      .slice(0, 3);
  }

  function topHours(hours: PersonalHourEntryDto[] = []): PersonalHourEntryDto[] {
    return [...hours].sort((left, right) => right.score - left.score).slice(0, 4);
  }

  function helpNotes(decision: InitiationOpeningDecisionExportDto): string[] {
    return decision.strongest_supports.map((note) => note.summary_vi);
  }

  function watchNotes(decision: InitiationOpeningDecisionExportDto): string[] {
    return [
      ...decision.strongest_resistances,
      ...decision.override_factors,
      ...decision.conflict_notes,
    ].map((note) => note.summary_vi);
  }

  function nextMoves(decision: InitiationOpeningDecisionExportDto): string[] {
    return [...decision.suggested_hours, ...decision.suggested_directions];
  }

  function explanationMeta(decision: InitiationOpeningDecisionExportDto): string[] {
    return [
      `mức ${decision.recommendation_bucket}`,
      `độ tin cậy ${decision.confidence}`,
      decision.context_is_clear ? "bối cảnh khá rõ" : "bối cảnh còn pha trộn",
    ];
  }

  function evidenceSummary(report: PersonalDayReportDto): string {
    if (!report.graph) return "Không có lớp bằng chứng sâu hơn cho ngày này.";
    return `${report.graph.nodes.length} nút lý do · ${report.graph.edges.length} liên kết giải thích`;
  }

  $effect(() => {
    if (!day) {
      report = null;
      matrix = null;
      reportError = null;
      matrixError = null;
      return;
    }

    const token = ++loadToken;
    loadingReport = true;
    reportError = null;

    fetchPersonalDayReport(
      day.day,
      day.month,
      day.year,
      parsedBirthYear ?? undefined,
      parsedBirthMonth ?? undefined,
      parsedBirthDay ?? undefined,
      normalizedGender ?? undefined,
    )
      .then((value) => {
        if (token !== loadToken) return;
        report = value;
      })
      .catch((error) => {
        if (token !== loadToken) return;
        report = null;
        reportError = error instanceof Error ? error.message : String(error);
      })
      .finally(() => {
        if (token !== loadToken) return;
        loadingReport = false;
      });

    if (!canLoadMatrix) {
      matrix = null;
      matrixError = null;
      loadingMatrix = false;
      return;
    }

    const birthYearValue = parsedBirthYear;
    const birthMonthValue = parsedBirthMonth;
    const birthDayValue = parsedBirthDay;
    const birthHourValue = parsedBirthHour;
    const birthMinuteValue = parsedBirthMinute;
    if (
      birthYearValue === null ||
      birthMonthValue === null ||
      birthDayValue === null ||
      birthHourValue === null ||
      birthMinuteValue === null
    ) {
      matrix = null;
      matrixError = null;
      loadingMatrix = false;
      return;
    }

    loadingMatrix = true;
    matrixError = null;
    fetchPersonalDayMatrixReport(
      day.day,
      day.month,
      day.year,
      birthYearValue,
      birthMonthValue,
      birthDayValue,
      birthHourValue,
      birthMinuteValue,
      normalizedGender ?? undefined,
    )
      .then((value) => {
        if (token !== loadToken) return;
        matrix = value;
      })
      .catch((error) => {
        if (token !== loadToken) return;
        matrix = null;
        matrixError = error instanceof Error ? error.message : String(error);
      })
      .finally(() => {
        if (token !== loadToken) return;
        loadingMatrix = false;
      });
  });
</script>

<section class="personal-panel">
  <header class="panel-header">
    <div>
      <h2>Hồ sơ cá nhân & cách đọc ngày này</h2>
      <p class="panel-copy">
        Nhập ngày sinh để xem kết luận, điểm nâng đỡ và điều cần lưu ý; thêm giờ sinh để mở ma trận cá nhân.
      </p>
    </div>
  </header>

  <div class="profile-grid">
    <label>
      <span>Năm sinh</span>
      <input bind:value={birthYear} inputmode="numeric" placeholder="1990" />
    </label>
    <label>
      <span>Tháng</span>
      <input bind:value={birthMonth} inputmode="numeric" placeholder="8" />
    </label>
    <label>
      <span>Ngày</span>
      <input bind:value={birthDay} inputmode="numeric" placeholder="15" />
    </label>
    <label>
      <span>Giờ</span>
      <input bind:value={birthHour} inputmode="numeric" placeholder="9" />
    </label>
    <label>
      <span>Phút</span>
      <input bind:value={birthMinute} inputmode="numeric" placeholder="30" />
    </label>
    <label>
      <span>Giới tính</span>
      <select bind:value={gender}>
        <option value="">Không chọn</option>
        <option value="male">Nam</option>
        <option value="female">Nữ</option>
      </select>
    </label>
  </div>

  {#if !day}
    <p class="muted">Chọn một ngày để xem khuyến nghị cá nhân.</p>
  {:else}
    <div class="section-stack">
      <article class="card">
        <div class="card-head">
          <div>
            <h3>Giải thích cho ngày này</h3>
            <p class="meta-line">
              {#if loadingReport}
                Đang tải phần giải thích...
              {:else if report}
                hồ sơ {report.computed_metrics.tier} · đầy đủ {report.computed_metrics.profile_completeness}/4
              {:else}
                Chưa có dữ liệu giải thích
              {/if}
            </p>
          </div>
          {#if report?.decision_export}
            <div class="status-chip">{report.decision_export.recommendation_bucket}</div>
          {/if}
        </div>

        {#if reportError}
          <p class="error">{reportError}</p>
        {:else if report?.decision_export}
          {@const decision = report.decision_export}
          <p class="summary">{decision.primary_conclusion}</p>

          <div class="pill-row">
            {#each explanationMeta(decision) as detail}
              <span class="pill">{detail}</span>
            {/each}
          </div>

          <div class="columns explanation-columns">
            <section>
              <h4>Điểm đang nâng đỡ</h4>
              <ul>
                {#each helpNotes(decision) as note}
                  <li>{note}</li>
                {/each}
              </ul>
            </section>
            <section>
              <h4>Điểm cần giữ chừng mực</h4>
              <ul>
                {#each watchNotes(decision) as note}
                  <li>{note}</li>
                {/each}
              </ul>
            </section>
          </div>

          {#if nextMoves(decision).length}
            <section>
              <h4>Nếu vẫn tiến hành</h4>
              <ul>
                {#each nextMoves(decision) as step}
                  <li>{step}</li>
                {/each}
              </ul>
            </section>
          {/if}

          <section class="details-block">
            <h4>Chi tiết & bằng chứng</h4>
            <p class="card-note">{evidenceSummary(report)}</p>
            {#if decision.axis_scores.length}
              <div class="axis-grid">
                {#each decision.axis_scores as axis}
                  <div class="axis-row">
                    <span>{formatAxis(axis.axis)}</span>
                    <strong>{axis.score.toFixed(2)}</strong>
                  </div>
                {/each}
              </div>
            {/if}
          </section>

          {#if report.analysis.unavailable_sections.length}
            <p class="card-note">{formatUnavailable(report.analysis.unavailable_sections)}</p>
          {/if}
        {:else if report}
          <p class="summary">{report.summary}</p>
          <p class="card-note">Chưa có lớp giải thích cá nhân cho dữ liệu hiện tại.</p>
        {/if}
      </article>

      <article class="card">
        <div class="card-head">
          <div>
            <h3>Personal-day matrix</h3>
            <p class="meta-line">
              {#if !canLoadMatrix}
                Cần đủ ngày và giờ sinh để tính ma trận.
              {:else if loadingMatrix}
                Đang tải ma trận...
              {:else if matrix}
                tier: {matrix.tier}
              {:else}
                Chưa có dữ liệu ma trận
              {/if}
            </p>
          </div>
        </div>

        {#if matrixError}
          <p class="error">{matrixError}</p>
        {:else if matrix}
          <section>
            <h4>Day-person</h4>
            <p class="summary">{matrix.day_person.day_canchi} · nhật chủ {matrix.day_person.day_master}</p>
            <ul>
              {#each matrix.day_person.pillars as pillar}
                <li>{pillar.pillar}: {pillar.pillar_canchi} · {pillar.thap_than} · {pillar.element_interaction}</li>
              {/each}
            </ul>
          </section>

          <section>
            <h4>Element resonance</h4>
            <div class="axis-grid">
              {#each matrix.element_resonance.entries as entry}
                <div class="axis-row">
                  <span>{entry.element}</span>
                  <strong>{entry.effective_resonance.toFixed(2)}</strong>
                </div>
              {/each}
            </div>
          </section>

          {#if matrix.personal_hours}
            <section>
              <h4>Personal hours</h4>
              <ul>
                {#each topHours(matrix.personal_hours.hours) as hour}
                  <li>{hour.chi} {hour.time_range} · {hour.score}/100 · {hour.star_name}</li>
                {/each}
              </ul>
            </section>
          {/if}

          {#if matrix.direction_merge}
            <section>
              <h4>Direction merge</h4>
              <ul>
                {#each [...matrix.direction_merge.entries].sort((left, right) => right.net_score - left.net_score).slice(0, 4) as direction}
                  <li>{direction.direction} · score {direction.net_score} · {direction.signals.join(", ")}</li>
                {/each}
              </ul>
            </section>
          {/if}

          {#if matrix.domain_day_boost}
            <section>
              <h4>Domain day boost</h4>
              <ul>
                {#each positiveDomains(matrix.domain_day_boost.entries) as domain}
                  <li>{domain.domain} · {domain.boosted_score.toFixed(1)} (base {domain.base_score.toFixed(1)})</li>
                {/each}
              </ul>
            </section>
          {/if}

          {#if matrix.unavailable_sections.length}
            <p class="card-note">{formatUnavailable(matrix.unavailable_sections)}</p>
          {/if}
        {/if}
      </article>
    </div>
  {/if}
</section>

<style>
  .personal-panel {
    display: grid;
    gap: 14px;
    padding: 16px;
    border-radius: 20px;
    border: 1px solid rgba(86, 62, 44, 0.16);
    background:
      linear-gradient(180deg, rgba(255, 250, 240, 0.96), rgba(250, 241, 228, 0.92)),
      radial-gradient(circle at top right, rgba(177, 124, 62, 0.16), transparent 40%);
  }

  .panel-header h2,
  .card h3,
  .card h4 {
    margin: 0;
  }

  .panel-copy,
  .meta-line,
  .card-note,
  .muted {
    margin: 4px 0 0;
    color: #7a6551;
  }

  .profile-grid {
    display: grid;
    grid-template-columns: repeat(6, minmax(0, 1fr));
    gap: 10px;
  }

  label {
    display: grid;
    gap: 6px;
    font-size: 0.88rem;
    color: #5a4736;
  }

  input,
  select {
    border-radius: 12px;
    border: 1px solid rgba(116, 85, 52, 0.2);
    padding: 10px 12px;
    background: rgba(255, 255, 255, 0.85);
    color: #2f241b;
  }

  .section-stack {
    display: grid;
    gap: 12px;
  }

  .card {
    display: grid;
    gap: 12px;
    padding: 14px;
    border-radius: 16px;
    background: rgba(255, 255, 255, 0.68);
    border: 1px solid rgba(139, 99, 58, 0.14);
  }

  .card-head {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 12px;
  }

  .status-chip,
  .pill {
    display: inline-flex;
    align-items: center;
    border-radius: 999px;
    padding: 6px 10px;
    background: rgba(125, 80, 34, 0.1);
    color: #7d5022;
    font-size: 0.78rem;
    text-transform: lowercase;
  }

  .pill-row,
  .axis-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }

  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 16px;
  }

  .explanation-columns section,
  .details-block {
    padding: 12px;
    border-radius: 14px;
    background: rgba(255, 251, 245, 0.76);
    border: 1px solid rgba(139, 99, 58, 0.1);
  }

  .summary {
    margin: 0;
    color: #312417;
    font-weight: 600;
  }

  ul {
    margin: 0;
    padding-left: 18px;
  }

  li {
    margin: 6px 0;
    color: #4b3a2d;
  }

  .axis-row {
    min-width: 160px;
    display: flex;
    justify-content: space-between;
    gap: 12px;
    padding: 8px 10px;
    border-radius: 10px;
    background: rgba(255, 255, 255, 0.72);
  }

  .error {
    margin: 0;
    color: #a22828;
  }

  @media (max-width: 1100px) {
    .profile-grid {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }

    .columns {
      grid-template-columns: 1fr;
    }
  }

  @media (max-width: 720px) {
    .profile-grid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
