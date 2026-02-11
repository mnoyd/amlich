<script lang="ts">
  import { buildDateInsight } from "$lib/insights/date-insight-engine";
  import type { DayForInsight, Lang, InsightCardExtra } from "$lib/insights/types";

  let { day }: { day: DayForInsight | null } = $props();

  // Language state - Vietnamese is primary
  let lang: Lang = $state("vi");

  const insight = $derived(day ? buildDateInsight(day, lang) : null);

  // Derived helpers for template rendering — avoids Svelte template type narrowing issues
  const isSpecialDay = $derived(insight != null && insight.mode !== "normal");
  const isFestival = $derived(insight != null && insight.mode === ("festival" as string));
  const specialDayTitle = $derived(
    insight != null && insight.mode !== "normal" ? insight.title : ""
  );
  const specialDaySubtitle = $derived(
    insight != null && insight.mode !== "normal" ? insight.subtitle : ""
  );

  // National holiday category helpers
  const isNationalHoliday = $derived(insight != null && insight.mode === ("national-holiday" as string));
  const holidayCategory = $derived(
    isNationalHoliday && insight != null ? (insight as any).category as string | null : null
  );

  const categoryLabels: Record<string, { vi: string; en: string }> = {
    "public-holiday": { vi: "Ngày lễ", en: "Public Holiday" },
    "commemorative": { vi: "Ngày kỷ niệm", en: "Commemorative" },
    "professional": { vi: "Ngày truyền thống", en: "Professional Day" },
    "social": { vi: "Ngày xã hội", en: "Social Day" },
    "international": { vi: "Quốc tế", en: "International" },
  };

  const categoryLabel = $derived(
    holidayCategory ? categoryLabels[holidayCategory]?.[lang] ?? "" : ""
  );

  // Region tab state for regional customs card
  let activeRegion = $state<"north" | "central" | "south">("north");

  // Helper to safely access extra properties
  function getExtra(extra: InsightCardExtra | undefined) {
    return extra ?? {};
  }
</script>

{#if insight}
  <section
    class="insight-box"
    class:is-festival={isFestival}
    class:national-holiday={isNationalHoliday}
    class:cat-public-holiday={holidayCategory === "public-holiday"}
    class:cat-commemorative={holidayCategory === "commemorative"}
    class:cat-professional={holidayCategory === "professional"}
    class:cat-social={holidayCategory === "social"}
    class:cat-international={holidayCategory === "international"}
  >
    <header class="insight-header">
      <div class="header-left">
        {#if isFestival}
          <span class="category-badge festival-badge">{lang === "vi" ? "Lễ truyền thống" : "Traditional Festival"}</span>
        {/if}
        {#if isNationalHoliday && categoryLabel}
          <span class="category-badge">{categoryLabel}</span>
        {/if}
        <h2>
          {#if isSpecialDay}
            {specialDayTitle}
          {:else}
            {lang === "vi" ? "Tìm hiểu về ngày này" : "Learn About This Day"}
          {/if}
        </h2>
        {#if isSpecialDay && specialDaySubtitle}
          <span class="subtitle">{specialDaySubtitle}</span>
        {:else if insight.mode === "normal"}
          <span class="subtitle">
            {lang === "vi" ? "Tiết khí" : "Solar Term"}: {insight.termName} • {insight.canchiDay}
          </span>
        {/if}
      </div>
      <button
        class="lang-toggle"
        onclick={() => (lang = lang === "vi" ? "en" : "vi")}
        aria-label="Toggle language"
      >
        {lang === "vi" ? "EN" : "VI"}
      </button>
    </header>

    <div class="insight-grid">
      {#each insight.cards as card (card.id)}
        {@const extra = getExtra(card.extra)}
        <article class="insight-card {card.id}">
          <h3 class="card-title">{card.title}</h3>
          {#if card.subtitle}
            <p class="card-subtitle">{card.subtitle}</p>
          {/if}

          {#if card.type === "text"}
            <p class="card-text">{card.content}</p>
            {#if extra.weather}
              <div class="weather-note">
                <span class="weather-label">{lang === "vi" ? "Thời tiết" : "Weather"}:</span>
                <span>{extra.weather}</span>
              </div>
            {/if}

          {:else if card.type === "list"}
            {#if Array.isArray(card.content) && card.content.length > 0}
              <ul class="card-list">
                {#each card.content as item}
                  <li>{item}</li>
                {/each}
              </ul>
            {/if}
            
            {#if extra.canNature}
              <p class="extra-note">{extra.canNature}</p>
            {/if}

            {#if extra.goodFor || extra.avoidFor}
              <div class="guidance-grid">
                {#if extra.goodFor}
                  <div class="guidance-col good">
                    <h4>{extra.goodFor.title}</h4>
                    <ul>
                      {#each extra.goodFor.items as item}
                        <li>{item}</li>
                      {/each}
                    </ul>
                  </div>
                {/if}
                {#if extra.avoidFor}
                  <div class="guidance-col avoid">
                    <h4>{extra.avoidFor.title}</h4>
                    <ul>
                      {#each extra.avoidFor.items as item}
                        <li>{item}</li>
                      {/each}
                    </ul>
                  </div>
                {/if}
              </div>
            {/if}

            {#if extra.agriculture || extra.health}
              <div class="wellness-grid">
                {#if extra.agriculture}
                  <div class="wellness-col">
                    <h4>{extra.agriculture.title}</h4>
                    <ul>
                      {#each extra.agriculture.items as item}
                        <li>{item}</li>
                      {/each}
                    </ul>
                  </div>
                {/if}
                {#if extra.health}
                  <div class="wellness-col">
                    <h4>{extra.health.title}</h4>
                    <ul>
                      {#each extra.health.items as item}
                        <li>{item}</li>
                      {/each}
                    </ul>
                  </div>
                {/if}
              </div>
            {/if}

          {:else if card.type === "proverb"}
            {#if extra.proverbs}
              <div class="proverbs-list">
                {#each extra.proverbs as proverb}
                  <blockquote class="proverb">
                    <p class="proverb-text">"{proverb.text}"</p>
                    <footer class="proverb-meaning">{proverb.meaning}</footer>
                  </blockquote>
                {/each}
              </div>
            {/if}

          {:else if card.type === "region-tabs"}
            {#if extra.north && extra.central && extra.south}
              <div class="region-tabs">
                <div class="tab-buttons">
                  <button
                    class="tab-btn"
                    class:active={activeRegion === "north"}
                    onclick={() => (activeRegion = "north")}
                  >
                    {extra.north.title}
                  </button>
                  <button
                    class="tab-btn"
                    class:active={activeRegion === "central"}
                    onclick={() => (activeRegion = "central")}
                  >
                    {extra.central.title}
                  </button>
                  <button
                    class="tab-btn"
                    class:active={activeRegion === "south"}
                    onclick={() => (activeRegion = "south")}
                  >
                    {extra.south.title}
                  </button>
                </div>
                <div class="tab-content">
                  {#if activeRegion === "north"}
                    <p>{extra.north.content}</p>
                  {:else if activeRegion === "central"}
                    <p>{extra.central.content}</p>
                  {:else}
                    <p>{extra.south.content}</p>
                  {/if}
                </div>
              </div>
            {/if}
          {/if}
        </article>
      {/each}
    </div>
  </section>
{/if}

<style>
  .insight-box {
    background: var(--surface-white);
    border: 1px solid var(--border-subtle);
    border-radius: 14px;
    padding: 14px;
    box-shadow: var(--shadow-soft);
  }

  /* National holiday category-specific styling */
  .insight-box.national-holiday {
    border-left: 4px solid var(--cat-color, var(--accent-jade));
  }

  /* Traditional festival styling */
  .insight-box.is-festival {
    border-left: 4px solid var(--cat-festival, #C62828);
  }

  .is-festival .insight-header h2 {
    color: var(--cat-festival, #C62828);
  }

  .is-festival .card-title {
    color: var(--cat-festival, #C62828);
  }

  .festival-badge {
    background: var(--cat-festival, #C62828) !important;
  }

  .insight-box.cat-public-holiday {
    --cat-color: #C62828;
    --cat-bg: rgba(198, 40, 40, 0.06);
    --cat-badge-bg: #C62828;
  }

  .insight-box.cat-commemorative {
    --cat-color: #1565C0;
    --cat-bg: rgba(21, 101, 192, 0.06);
    --cat-badge-bg: #1565C0;
  }

  .insight-box.cat-professional {
    --cat-color: #2E7D32;
    --cat-bg: rgba(46, 125, 50, 0.06);
    --cat-badge-bg: #2E7D32;
  }

  .insight-box.cat-social {
    --cat-color: #E65100;
    --cat-bg: rgba(230, 81, 0, 0.06);
    --cat-badge-bg: #E65100;
  }

  .insight-box.cat-international {
    --cat-color: #6A1B9A;
    --cat-bg: rgba(106, 27, 154, 0.06);
    --cat-badge-bg: #6A1B9A;
  }

  .national-holiday .insight-header h2 {
    color: var(--cat-color, var(--primary-red));
  }

  .national-holiday .card-title {
    color: var(--cat-color, var(--accent-jade));
  }

  .category-badge {
    display: inline-block;
    font-family: var(--font-sans);
    font-size: 0.65rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    padding: 3px 8px;
    border-radius: 4px;
    background: var(--cat-badge-bg, var(--accent-jade));
    color: white;
    width: fit-content;
  }

  .insight-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 12px;
    padding-bottom: 10px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .insight-header h2 {
    margin: 0;
    font-size: 1.15rem;
    font-weight: 700;
    color: var(--primary-red);
    line-height: 1.25;
  }

  .subtitle {
    font-size: 0.82rem;
    color: var(--text-secondary);
    font-style: italic;
  }

  .lang-toggle {
    padding: 4px 10px;
    background: rgba(0, 0, 0, 0.04);
    border: 1px solid var(--border-subtle);
    border-radius: 6px;
    font-family: var(--font-sans);
    font-size: 0.7rem;
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .lang-toggle:hover {
    background: rgba(0, 0, 0, 0.08);
    color: var(--text-primary);
  }

  .insight-grid {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
    gap: 10px;
  }

  .insight-card {
    background: rgba(255, 255, 255, 0.7);
    border: 1px solid var(--border-subtle);
    border-radius: 10px;
    padding: 10px 12px;
    transition: all 0.2s;
  }

  .insight-card:hover {
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 3px 8px rgba(0, 0, 0, 0.05);
  }

  /* Special card spanning for origin */
  .insight-card.origin {
    grid-column: span 2;
  }

  .card-title {
    margin: 0 0 5px;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-jade);
  }

  .card-subtitle {
    margin: 0 0 5px;
    font-size: 0.92rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .card-text {
    margin: 0;
    font-size: 0.84rem;
    line-height: 1.5;
    color: var(--text-primary);
  }

  .card-list {
    margin: 0;
    padding-left: 16px;
    display: flex;
    flex-direction: column;
    gap: 3px;
  }

  .card-list li {
    font-size: 0.82rem;
    line-height: 1.45;
    color: var(--text-primary);
  }

  .weather-note {
    margin-top: 8px;
    padding: 6px 10px;
    background: rgba(42, 110, 100, 0.06);
    border-radius: 6px;
    font-size: 0.8rem;
    color: var(--text-secondary);
  }

  .weather-label {
    font-weight: 600;
    color: var(--accent-jade);
  }

  .extra-note {
    margin: 6px 0 0;
    padding: 6px 8px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 6px;
    font-size: 0.8rem;
    font-style: italic;
    color: var(--text-secondary);
  }

  .guidance-grid,
  .wellness-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
    margin-top: 8px;
  }

  .guidance-col,
  .wellness-col {
    padding: 6px 8px;
    border-radius: 6px;
  }

  .guidance-col.good {
    background: rgba(42, 110, 100, 0.06);
  }

  .guidance-col.avoid {
    background: rgba(217, 48, 37, 0.06);
  }

  .wellness-col {
    background: rgba(212, 175, 55, 0.08);
  }

  .guidance-col h4,
  .wellness-col h4 {
    margin: 0 0 4px;
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--text-tertiary);
  }

  .guidance-col.good h4 {
    color: var(--accent-jade);
  }

  .guidance-col.avoid h4 {
    color: var(--primary-red);
  }

  .guidance-col ul,
  .wellness-col ul {
    margin: 0;
    padding-left: 14px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .guidance-col li,
  .wellness-col li {
    font-size: 0.78rem;
    line-height: 1.35;
    color: var(--text-primary);
  }

  /* Proverbs */
  .proverbs-list {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .proverb {
    margin: 0;
    padding: 8px 10px;
    background: linear-gradient(135deg, rgba(212, 175, 55, 0.08), rgba(255, 255, 255, 0.5));
    border-left: 3px solid var(--accent-gold);
    border-radius: 0 6px 6px 0;
  }

  .proverb-text {
    margin: 0 0 4px;
    font-size: 0.88rem;
    font-style: italic;
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1.4;
  }

  .proverb-meaning {
    font-size: 0.78rem;
    color: var(--text-secondary);
    line-height: 1.35;
  }

  /* Region tabs */
  .region-tabs {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .tab-buttons {
    display: flex;
    gap: 6px;
  }

  .tab-btn {
    flex: 1;
    padding: 5px 10px;
    background: rgba(0, 0, 0, 0.03);
    border: 1px solid transparent;
    border-radius: 6px;
    font-family: var(--font-sans);
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.2s;
  }

  .tab-btn:hover {
    background: rgba(0, 0, 0, 0.06);
    color: var(--text-secondary);
  }

  .tab-btn.active {
    background: var(--accent-jade);
    color: white;
    border-color: var(--accent-jade);
  }

  .tab-content {
    padding: 8px 10px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 6px;
  }

  .tab-content p {
    margin: 0;
    font-size: 0.82rem;
    line-height: 1.5;
    color: var(--text-primary);
  }

  /* Responsive */
  @media (max-width: 1200px) {
    .insight-grid {
      grid-template-columns: repeat(2, 1fr);
    }

    .insight-card.origin {
      grid-column: span 2;
    }
  }

  @media (max-width: 768px) {
    .insight-box {
      padding: 10px;
      border-radius: 10px;
    }

    .insight-grid {
      grid-template-columns: 1fr;
    }

    .insight-card.origin {
      grid-column: auto;
    }

    .insight-header h2 {
      font-size: 1rem;
    }

    .guidance-grid,
    .wellness-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
