<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import type { DayForInsight } from "$lib/insights/types";

  type Lang = "vi" | "en";

  type LocalizedText = { vi: string; en: string };
  type LocalizedList = { vi: string[]; en: string[] };

  type FestivalInsight = {
    names: LocalizedList;
    origin?: LocalizedText | null;
    activities?: LocalizedList | null;
  };

  type HolidayInsight = {
    names: LocalizedList;
    origin?: LocalizedText | null;
    significance?: LocalizedText | null;
    traditions?: LocalizedList | null;
  };

  type CanChiInsight = {
    can: { name: string; meaning: LocalizedText };
    chi: { name: string; animal: LocalizedText; meaning: LocalizedText };
  };

  type DayGuidance = { good_for: LocalizedList; avoid_for: LocalizedList };

  type TietKhiInsight = {
    name: LocalizedText;
    weather: LocalizedText;
    agriculture: LocalizedList;
    health: LocalizedList;
  };

  type DayInsightDto = {
    festival?: FestivalInsight | null;
    holiday?: HolidayInsight | null;
    canchi?: CanChiInsight | null;
    day_guidance?: DayGuidance | null;
    tiet_khi?: TietKhiInsight | null;
  };

  let { day }: { day: DayForInsight | null } = $props();

  let lang: Lang = $state("vi");
  let loading = $state(false);
  let error = $state<string | null>(null);
  let insight = $state<DayInsightDto | null>(null);

  function text(v?: LocalizedText | null): string {
    if (!v) return "";
    return lang === "vi" ? v.vi : v.en;
  }

  function list(v?: LocalizedList | null): string[] {
    if (!v) return [];
    return lang === "vi" ? v.vi : v.en;
  }

  $effect(() => {
    if (!day) {
      insight = null;
      error = null;
      return;
    }

    let canceled = false;
    loading = true;
    error = null;

    invoke<DayInsightDto>("get_day_insight", {
      day: day.day,
      month: day.month,
      year: day.year,
    })
      .then((data) => {
        if (!canceled) insight = data;
      })
      .catch((e) => {
        if (!canceled) {
          error = e instanceof Error ? e.message : String(e);
          insight = null;
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
      <article class="insight-card split-card">
        <h3>{lang === "vi" ? "Nên làm / Hạn chế" : "Do / Avoid"}</h3>
        <div class="split">
          <div>
            <h4>{lang === "vi" ? "Nên" : "Do"}</h4>
            <ul>{#each list(insight.day_guidance.good_for).slice(0, 3) as item}<li>{item}</li>{/each}</ul>
          </div>
          <div>
            <h4>{lang === "vi" ? "Hạn chế" : "Avoid"}</h4>
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
  }
</style>
