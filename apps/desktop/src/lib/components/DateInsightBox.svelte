<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type {
    BilingualList,
    BilingualText,
    DailyRecommendationsDto,
    DayForInsight,
    DayInfoDto,
    DayInsightDto,
    Lang,
    RecommendationBucketDto,
    RecommendationEvidenceSourceDto,
    RecommendationReasonDto,
    SynthesizedRecommendationDto,
  } from "$lib/insights/types";

  let { day }: { day: DayForInsight | null } = $props();

  let lang: Lang = $state("vi");
  let loading = $state(false);
  let error = $state<string | null>(null);
  let insight = $state<DayInsightDto | null>(null);
  let dayInfo = $state<DayInfoDto | null>(null);

  const bucketConfig: {
    bucket: RecommendationBucketDto;
    label: { vi: string; en: string };
    toneClass: string;
  }[] = [
    { bucket: "nen", label: { vi: "Nên", en: "Nên · Recommended" }, toneClass: "nen" },
    { bucket: "co_the", label: { vi: "Có thể", en: "Có thể · Consider" }, toneClass: "co-the" },
    { bucket: "tranh", label: { vi: "Tránh", en: "Tránh · Avoid" }, toneClass: "tranh" },
    { bucket: "ky_manh", label: { vi: "Kỵ mạnh", en: "Kỵ mạnh · Hard stop" }, toneClass: "ky-manh" },
  ];

  function text(v?: BilingualText | null): string {
    if (!v) return "";
    return lang === "vi" ? v.vi : v.en;
  }

  function list(v?: BilingualList | null): string[] {
    if (!v) return [];
    return lang === "vi" ? v.vi : v.en;
  }

  function activitiesForBucket(
    recommendations: DailyRecommendationsDto,
    bucket: RecommendationBucketDto,
  ): SynthesizedRecommendationDto[] {
    return recommendations.activities.filter((activity) => activity.bucket === bucket);
  }

  function strongestReason(activity: SynthesizedRecommendationDto): RecommendationReasonDto | null {
    return activity.reasons[0] ?? null;
  }

  function reasonSummary(reason: RecommendationReasonDto): string {
    return lang === "vi" ? reason.summary_vi : reason.summary_en;
  }

  function severityLabel(reason: RecommendationReasonDto): string {
    switch (reason.severity) {
      case "override":
        return "override";
      case "primary":
        return "primary";
      default:
        return "support";
    }
  }

  function sourceLabel(source: RecommendationEvidenceSourceDto): string {
    switch (source) {
      case "day_guidance":
        return "guidance";
      case "truc":
        return "trực";
      case "stars":
        return "sao";
      case "day_deity":
        return "thần sát";
      case "taboo":
        return "kiêng kỵ";
      case "xung_hop":
        return "xung-hợp";
      case "tiet_khi":
        return "tiết khí";
      case "gio_hoang_dao":
        return "giờ tốt";
      case "travel":
        return "xuất hành";
      default:
        return lang === "vi" ? "mở rộng" : "extension";
    }
  }

  $effect(() => {
    if (!day) {
      insight = null;
      dayInfo = null;
      error = null;
      return;
    }

    let canceled = false;
    loading = true;
    error = null;

    Promise.all([
      invoke<DayInsightDto>("get_day_insight", {
        day: day.day,
        month: day.month,
        year: day.year,
      }),
      invoke<DayInfoDto>("get_day_info", {
        day: day.day,
        month: day.month,
        year: day.year,
      }),
    ])
      .then(([insightData, dayInfoData]) => {
        if (!canceled) {
          insight = insightData;
          dayInfo = dayInfoData;
        }
      })
      .catch((e) => {
        if (!canceled) {
          error = e instanceof Error ? e.message : String(e);
          insight = null;
          dayInfo = null;
        }
      })
      .finally(() => {
        if (!canceled) loading = false;
      });

    return () => {
      canceled = true;
    };
  });
</script>

<section class="insight-box">
  <header class="insight-header">
    <h2>{lang === "vi" ? "Tìm hiểu về ngày này" : "Learn About This Day"}</h2>
    <button class="lang-toggle" onclick={() => (lang = lang === "vi" ? "en" : "vi")}> 
      {lang === "vi" ? "EN" : "VI"}
    </button>
  </header>

  {#if !day}
    <p class="muted">{lang === "vi" ? "Chọn một ngày để xem insight." : "Select a day to view insight."}</p>
  {:else if loading}
    <p class="muted">{lang === "vi" ? "Đang tải..." : "Loading..."}</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else if insight}
    {#if dayInfo}
      <article class="insight-card recommendation-card">
        <div class="card-header">
          <div>
            <h3>{lang === "vi" ? "Khuyến nghị trong ngày" : "Daily Recommendations"}</h3>
            <p class="meta-line">
              {dayInfo.daily_recommendations.ruleset_id}
              · {dayInfo.daily_recommendations.ruleset_version}
              · {dayInfo.daily_recommendations.profile}
            </p>
          </div>
          <p class="summary-chip">
            {lang === "vi"
              ? dayInfo.daily_recommendations.summary_vi
              : dayInfo.daily_recommendations.summary_en}
          </p>
        </div>

        <div class="bucket-grid">
          {#each bucketConfig as section}
            {@const bucketItems = activitiesForBucket(dayInfo.daily_recommendations, section.bucket)}
            {#if bucketItems.length > 0}
              <section class={`bucket-panel ${section.toneClass}`}>
                <h4>{lang === "vi" ? section.label.vi : section.label.en} ({bucketItems.length})</h4>
                <ul class="recommendation-list">
                  {#each bucketItems.slice(0, 4) as activity, index}
                    {@const reason = strongestReason(activity)}
                    <li class="recommendation-row">
                      <span class="marker">{index === 0 ? "★" : "•"}</span>
                      <div>
                        <div class="recommendation-label">
                          {lang === "vi" ? activity.label.vi : activity.label.en}
                        </div>
                        {#if reason}
                          <div class="reason-stack">
                            <span class="reason-chip">
                              [{severityLabel(reason)} • {sourceLabel(reason.evidence.source)}]
                            </span>
                            <span class="reason-copy">{reasonSummary(reason)}</span>
                          </div>
                        {/if}
                      </div>
                    </li>
                  {/each}
                </ul>
              </section>
            {/if}
          {/each}
        </div>
      </article>
    {/if}

    {#if insight.festival}
      <article class="insight-card">
        <h3>{(lang === "vi" ? insight.festival.names.vi : insight.festival.names.en)[0]}</h3>
        {#if insight.festival.origin}<p>{text(insight.festival.origin)}</p>{/if}
        {#if list(insight.festival.activities).length > 0}
          <ul>{#each list(insight.festival.activities).slice(0, 3) as item}<li>{item}</li>{/each}</ul>
        {/if}
      </article>
    {:else if insight.holiday}
      <article class="insight-card">
        <h3>{(lang === "vi" ? insight.holiday.names.vi : insight.holiday.names.en)[0]}</h3>
        <p>{text(insight.holiday.significance) || text(insight.holiday.origin)}</p>
        {#if list(insight.holiday.traditions).length > 0}
          <ul>{#each list(insight.holiday.traditions).slice(0, 3) as item}<li>{item}</li>{/each}</ul>
        {/if}
      </article>
    {/if}

    {#if insight.day_guidance}
      <article class="insight-card split-card guidance-reference">
        <h3>{lang === "vi" ? "Tham khảo day guidance" : "Day Guidance Reference"}</h3>
        <p class="card-note">
          {lang === "vi"
            ? "Mục này chỉ mang tính thông tin và không dùng để xếp khuyến nghị mặc định."
            : "This section is informational only and does not drive the default recommendation buckets."}
        </p>
        <div class="split">
          <div>
            <h4>{lang === "vi" ? "Gợi ý tham khảo" : "Reference positives"}</h4>
            <ul>{#each list(insight.day_guidance.good_for).slice(0, 3) as item}<li>{item}</li>{/each}</ul>
          </div>
          <div>
            <h4>{lang === "vi" ? "Điểm cần lưu ý" : "Reference cautions"}</h4>
            <ul>{#each list(insight.day_guidance.avoid_for).slice(0, 3) as item}<li>{item}</li>{/each}</ul>
          </div>
        </div>
      </article>
    {/if}

    {#if insight.tiet_khi}
      <article class="insight-card split-card">
        <h3>{text(insight.tiet_khi.name)}</h3>
        <p>{text(insight.tiet_khi.weather)}</p>
        <div class="split">
          <div>
            <h4>{lang === "vi" ? "Nông nghiệp" : "Agriculture"}</h4>
            <ul>{#each list(insight.tiet_khi.agriculture).slice(0, 2) as item}<li>{item}</li>{/each}</ul>
          </div>
          <div>
            <h4>{lang === "vi" ? "Sức khỏe" : "Health"}</h4>
            <ul>{#each list(insight.tiet_khi.health).slice(0, 2) as item}<li>{item}</li>{/each}</ul>
          </div>
        </div>
      </article>
    {/if}

    {#if insight.canchi}
      <article class="insight-card">
        <h3>{lang === "vi" ? "Can Chi ngày" : "Day's Can Chi"}</h3>
        <p><strong>{insight.canchi.can.name}</strong> • {text(insight.canchi.can.meaning)}</p>
        <p><strong>{insight.canchi.chi.name}</strong> ({text(insight.canchi.chi.animal)}) • {text(insight.canchi.chi.meaning)}</p>
      </article>
    {/if}
  {/if}
</section>

<style>
  .insight-box {
    border-radius: 16px;
    border: 1px solid rgba(212, 175, 55, 0.35);
    background:
      radial-gradient(140% 120% at 0% 0%, rgba(212, 175, 55, 0.08), transparent 55%),
      linear-gradient(180deg, #fffefb 0%, #f7f1e7 100%);
    padding: 14px;
    display: grid;
    gap: 10px;
    color: #3f352a;
  }

  .insight-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    border-bottom: 1px solid rgba(212, 175, 55, 0.3);
    padding-bottom: 8px;
  }

  .insight-header h2 {
    margin: 0;
    font-size: 1.18rem;
    color: #7f1d1d;
  }

  .lang-toggle {
    border: 1px solid rgba(126, 79, 22, 0.35);
    background: #fffaf0;
    color: #5f4a2c;
    border-radius: 999px;
    padding: 3px 10px;
    font-weight: 700;
    cursor: pointer;
  }

  .insight-card {
    border-radius: 12px;
    border: 1px solid rgba(126, 79, 22, 0.2);
    background: rgba(255, 255, 255, 0.7);
    padding: 10px 12px;
  }

  .insight-card h3 {
    margin: 0 0 6px;
    color: #4e2a14;
    font-size: 1.02rem;
  }

  .insight-card h4 {
    margin: 0 0 4px;
    color: #7f1d1d;
    font-size: 0.92rem;
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    align-items: flex-start;
    margin-bottom: 10px;
  }

  .meta-line,
  .card-note {
    margin: 0;
    font-size: 0.8rem;
    color: #7c6a59;
  }

  .summary-chip {
    margin: 0;
    padding: 6px 10px;
    border-radius: 999px;
    background: rgba(127, 29, 29, 0.08);
    color: #7f1d1d;
    font-size: 0.83rem;
    line-height: 1.3;
  }

  .bucket-grid {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 10px;
  }

  .bucket-panel {
    border-radius: 10px;
    border: 1px solid rgba(126, 79, 22, 0.16);
    padding: 10px;
    background: rgba(255, 255, 255, 0.76);
  }

  .bucket-panel h4 {
    margin-bottom: 8px;
  }

  .bucket-panel.nen {
    border-color: rgba(22, 101, 52, 0.24);
    background: rgba(240, 255, 244, 0.9);
  }

  .bucket-panel.co-the {
    border-color: rgba(180, 83, 9, 0.24);
    background: rgba(255, 251, 235, 0.92);
  }

  .bucket-panel.tranh {
    border-color: rgba(185, 28, 28, 0.2);
    background: rgba(254, 242, 242, 0.92);
  }

  .bucket-panel.ky-manh {
    border-color: rgba(127, 29, 29, 0.3);
    background: rgba(254, 226, 226, 0.95);
  }

  .recommendation-list {
    list-style: none;
    padding: 0;
    display: grid;
    gap: 8px;
  }

  .recommendation-row {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 8px;
    align-items: flex-start;
  }

  .marker {
    color: #7f1d1d;
    font-weight: 700;
    line-height: 1.4;
  }

  .recommendation-label {
    font-weight: 700;
    color: #4e2a14;
    font-size: 0.92rem;
  }

  .reason-stack {
    display: grid;
    gap: 3px;
    margin-top: 3px;
  }

  .reason-chip {
    width: fit-content;
    font-size: 0.74rem;
    color: #7c2d12;
    background: rgba(255, 247, 237, 0.9);
    border: 1px solid rgba(194, 65, 12, 0.12);
    border-radius: 999px;
    padding: 2px 7px;
  }

  .reason-copy {
    color: #5c4b3d;
    font-size: 0.82rem;
    line-height: 1.32;
  }

  .guidance-reference {
    border-style: dashed;
  }

  p {
    margin: 0 0 8px;
    color: #4d3f32;
    line-height: 1.38;
    font-size: 0.93rem;
  }

  ul {
    margin: 0;
    padding-left: 18px;
  }

  li {
    margin-bottom: 2px;
    color: #504437;
    font-size: 0.9rem;
  }

  .split {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .muted { color: #7c6a59; }
  .error { color: #b3261e; }

  @media (max-width: 900px) {
    .split { grid-template-columns: 1fr; }
    .bucket-grid { grid-template-columns: 1fr; }
    .card-header { flex-direction: column; }
  }
</style>
