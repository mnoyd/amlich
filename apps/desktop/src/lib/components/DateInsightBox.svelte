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
    category: string;
    is_major: boolean;
  };

  type HolidayInsight = {
    names: LocalizedList;
    origin?: LocalizedText | null;
    significance?: LocalizedText | null;
    traditions?: LocalizedList | null;
    category: string;
    is_major: boolean;
  };

  type CanChiInsight = {
    can: { name: string; element: string; meaning: LocalizedText; nature: LocalizedText };
    chi: {
      name: string;
      animal: LocalizedText;
      element: string;
      meaning: LocalizedText;
      hours: string;
    };
  };

  type DayGuidance = {
    good_for: LocalizedList;
    avoid_for: LocalizedList;
  };

  type TietKhiInsight = {
    name: LocalizedText;
    meaning: LocalizedText;
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
    <h2>{lang === "vi" ? "Insight trong ngày" : "Daily Insight"}</h2>
    <button class="lang-toggle" onclick={() => (lang = lang === "vi" ? "en" : "vi")}> 
      {lang === "vi" ? "EN" : "VI"}
    </button>
  </header>

  {#if !day}
    <p class="muted">{lang === "vi" ? "Chọn một ngày để xem insight." : "Select a day to view insight."}</p>
  {:else if loading}
    <p class="muted">{lang === "vi" ? "Đang tải insight..." : "Loading insight..."}</p>
  {:else if error}
    <p class="error">{error}</p>
  {:else if insight}
    {#if insight.festival}
      <article class="card">
        <h3>{(lang === "vi" ? insight.festival.names.vi : insight.festival.names.en)[0]}</h3>
        {#if insight.festival.origin}
          <p>{text(insight.festival.origin)}</p>
        {/if}
        {#if list(insight.festival.activities).length > 0}
          <ul>
            {#each list(insight.festival.activities).slice(0, 3) as item}
              <li>{item}</li>
            {/each}
          </ul>
        {/if}
      </article>
    {:else if insight.holiday}
      <article class="card">
        <h3>{(lang === "vi" ? insight.holiday.names.vi : insight.holiday.names.en)[0]}</h3>
        {#if insight.holiday.significance}
          <p>{text(insight.holiday.significance)}</p>
        {:else if insight.holiday.origin}
          <p>{text(insight.holiday.origin)}</p>
        {/if}
        {#if list(insight.holiday.traditions).length > 0}
          <ul>
            {#each list(insight.holiday.traditions).slice(0, 3) as item}
              <li>{item}</li>
            {/each}
          </ul>
        {/if}
      </article>
    {/if}

    {#if insight.day_guidance}
      <article class="card">
        <h3>{lang === "vi" ? "Nên làm / Hạn chế" : "Do / Avoid"}</h3>
        <div class="split">
          <div>
            <strong>{lang === "vi" ? "Nên" : "Do"}</strong>
            <ul>
              {#each list(insight.day_guidance.good_for).slice(0, 3) as item}
                <li>{item}</li>
              {/each}
            </ul>
          </div>
          <div>
            <strong>{lang === "vi" ? "Hạn chế" : "Avoid"}</strong>
            <ul>
              {#each list(insight.day_guidance.avoid_for).slice(0, 3) as item}
                <li>{item}</li>
              {/each}
            </ul>
          </div>
        </div>
      </article>
    {/if}

    {#if insight.tiet_khi}
      <article class="card">
        <h3>{text(insight.tiet_khi.name)}</h3>
        <p>{text(insight.tiet_khi.weather)}</p>
        <div class="split">
          <div>
            <strong>{lang === "vi" ? "Nông nghiệp" : "Agriculture"}</strong>
            <ul>
              {#each list(insight.tiet_khi.agriculture).slice(0, 2) as item}
                <li>{item}</li>
              {/each}
            </ul>
          </div>
          <div>
            <strong>{lang === "vi" ? "Sức khỏe" : "Health"}</strong>
            <ul>
              {#each list(insight.tiet_khi.health).slice(0, 2) as item}
                <li>{item}</li>
              {/each}
            </ul>
          </div>
        </div>
      </article>
    {/if}

    {#if insight.canchi}
      <article class="card">
        <h3>{lang === "vi" ? "Can Chi ngày" : "Day's Can Chi"}</h3>
        <p>
          <strong>{insight.canchi.can.name}</strong> · {text(insight.canchi.can.meaning)}
        </p>
        <p>
          <strong>{insight.canchi.chi.name}</strong> ({text(insight.canchi.chi.animal)}) · {text(insight.canchi.chi.meaning)}
        </p>
      </article>
    {/if}
  {/if}
</section>

<style>
  .insight-box {
    background: linear-gradient(180deg, #10151a, #0d1217);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 14px;
    padding: 14px;
    color: #e9eef5;
    display: grid;
    gap: 10px;
    min-height: 260px;
  }

  .insight-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
    font-weight: 700;
  }

  .lang-toggle {
    border: 1px solid rgba(255, 255, 255, 0.2);
    background: transparent;
    color: #d6dee8;
    border-radius: 999px;
    padding: 4px 10px;
    cursor: pointer;
  }

  .card {
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
    padding: 10px;
    background: rgba(255, 255, 255, 0.02);
  }

  h3 {
    margin: 0 0 8px 0;
    font-size: 0.95rem;
  }

  p {
    margin: 0 0 8px 0;
    line-height: 1.45;
    color: #c7d3e0;
  }

  ul {
    margin: 0;
    padding-left: 18px;
    color: #b8c7d6;
  }

  .split {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 12px;
  }

  .muted {
    color: #98a8b8;
  }

  .error {
    color: #ff9f9f;
  }

  @media (max-width: 900px) {
    .split {
      grid-template-columns: 1fr;
    }
  }
</style>
