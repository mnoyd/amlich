<script lang="ts">
  import { buildDateInsight } from "$lib/insights/date-insight-engine";
  import type { DayForInsight, Lang, InsightCardExtra } from "$lib/insights/types";

  let { day }: { day: DayForInsight | null } = $props();

  // Language state - Vietnamese is primary
  let lang: Lang = $state("vi");

  const insight = $derived(day ? buildDateInsight(day, lang) : null);

  // Region tab state for regional customs card
  let activeRegion = $state<"north" | "central" | "south">("north");

  // Helper to safely access extra properties
  function getExtra(extra: InsightCardExtra | undefined) {
    return extra ?? {};
  }
</script>

{#if insight}
  <section class="insight-box">
    <header class="insight-header">
      <div class="header-left">
        <h2>
          {#if insight.mode === "festival"}
            {insight.title}
          {:else}
            {lang === "vi" ? "Tìm hiểu về ngày này" : "Learn About This Day"}
          {/if}
        </h2>
        {#if insight.mode === "festival" && insight.subtitle}
          <span class="subtitle">{insight.subtitle}</span>
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
    border-radius: 24px;
    padding: 24px;
    box-shadow: var(--shadow-soft);
  }

  .insight-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    margin-bottom: 20px;
    padding-bottom: 16px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-left {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .insight-header h2 {
    margin: 0;
    font-size: 1.3rem;
    font-weight: 700;
    color: var(--primary-red);
    line-height: 1.3;
  }

  .subtitle {
    font-size: 0.9rem;
    color: var(--text-secondary);
    font-style: italic;
  }

  .lang-toggle {
    padding: 6px 12px;
    background: rgba(0, 0, 0, 0.04);
    border: 1px solid var(--border-subtle);
    border-radius: 8px;
    font-family: var(--font-sans);
    font-size: 0.75rem;
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
    gap: 16px;
  }

  .insight-card {
    background: rgba(255, 255, 255, 0.7);
    border: 1px solid var(--border-subtle);
    border-radius: 16px;
    padding: 16px;
    transition: all 0.2s;
  }

  .insight-card:hover {
    background: rgba(255, 255, 255, 0.9);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
  }

  /* Special card spanning for origin */
  .insight-card.origin {
    grid-column: span 2;
  }

  .card-title {
    margin: 0 0 8px;
    font-size: 0.8rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-jade);
  }

  .card-subtitle {
    margin: 0 0 8px;
    font-size: 1rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .card-text {
    margin: 0;
    font-size: 0.9rem;
    line-height: 1.6;
    color: var(--text-primary);
  }

  .card-list {
    margin: 0;
    padding-left: 20px;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .card-list li {
    font-size: 0.88rem;
    line-height: 1.5;
    color: var(--text-primary);
  }

  .weather-note {
    margin-top: 12px;
    padding: 10px 12px;
    background: rgba(42, 110, 100, 0.06);
    border-radius: 8px;
    font-size: 0.85rem;
    color: var(--text-secondary);
  }

  .weather-label {
    font-weight: 600;
    color: var(--accent-jade);
  }

  .extra-note {
    margin: 10px 0 0;
    padding: 10px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
    font-size: 0.85rem;
    font-style: italic;
    color: var(--text-secondary);
  }

  .guidance-grid,
  .wellness-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 12px;
    margin-top: 12px;
  }

  .guidance-col,
  .wellness-col {
    padding: 10px;
    border-radius: 8px;
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
    margin: 0 0 8px;
    font-size: 0.75rem;
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
    padding-left: 16px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .guidance-col li,
  .wellness-col li {
    font-size: 0.82rem;
    line-height: 1.4;
    color: var(--text-primary);
  }

  /* Proverbs */
  .proverbs-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .proverb {
    margin: 0;
    padding: 12px;
    background: linear-gradient(135deg, rgba(212, 175, 55, 0.08), rgba(255, 255, 255, 0.5));
    border-left: 3px solid var(--accent-gold);
    border-radius: 0 8px 8px 0;
  }

  .proverb-text {
    margin: 0 0 6px;
    font-size: 0.95rem;
    font-style: italic;
    font-weight: 500;
    color: var(--text-primary);
    line-height: 1.5;
  }

  .proverb-meaning {
    font-size: 0.82rem;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  /* Region tabs */
  .region-tabs {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .tab-buttons {
    display: flex;
    gap: 8px;
  }

  .tab-btn {
    flex: 1;
    padding: 8px 12px;
    background: rgba(0, 0, 0, 0.03);
    border: 1px solid transparent;
    border-radius: 8px;
    font-family: var(--font-sans);
    font-size: 0.8rem;
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
    padding: 12px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 8px;
  }

  .tab-content p {
    margin: 0;
    font-size: 0.88rem;
    line-height: 1.6;
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
      padding: 16px;
      border-radius: 16px;
    }

    .insight-grid {
      grid-template-columns: 1fr;
    }

    .insight-card.origin {
      grid-column: auto;
    }

    .insight-header h2 {
      font-size: 1.1rem;
    }

    .guidance-grid,
    .wellness-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
