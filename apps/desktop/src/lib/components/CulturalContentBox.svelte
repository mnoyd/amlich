<script lang="ts">
  import { buildCulturalContent, type DayForCultural, type Lang } from "$lib/cultural/cultural-engine";

  let { day, language }: { day: DayForCultural; language: Lang } = $props();

  const content = $derived(buildCulturalContent(day, language));
</script>

<section class="culture-panel">
  <div class="culture-header">
    <h2>{content.title}</h2>
  </div>

  <p class="culture-summary">{content.subtitle}</p>

  {#if content.mode === "event"}
    <div class="event-grid">
      {#each content.events as event}
        <article class="event-card">
          <h3 class="event-title">{event.title}</h3>

          <div class="fact-block">
            <h4>{language === "vi" ? "Sự kiện" : "What is this"}</h4>
            <p>{event.whatIsIt}</p>
          </div>

          <div class="fact-block">
            <h4>{language === "vi" ? "Nguồn gốc" : "Origin"}</h4>
            <p>{event.origin}</p>
          </div>

          <div class="list-row">
            <div class="list-col">
              <h4>{language === "vi" ? "Nên làm" : "Do"}</h4>
              <ul>
                {#each event.dos as item}
                  <li>{item}</li>
                {/each}
              </ul>
            </div>

            <div class="list-col">
              <h4>{language === "vi" ? "Không nên" : "Don't"}</h4>
              <ul>
                {#each event.donts as item}
                  <li>{item}</li>
                {/each}
              </ul>
            </div>
          </div>

          <div class="fact-block">
            <h4>{language === "vi" ? "Nếp sống truyền thống" : "Traditional life"}</h4>
            <p>{event.traditionalLife}</p>
          </div>
        </article>
      {/each}
    </div>
  {:else}
    <div class="term-layout">
      <article class="term-card main">
        <h3>{content.termName}</h3>
        <p>{content.termMeaning}</p>
      </article>

      <article class="term-card">
        <h4>{language === "vi" ? "Vì sao có tiết khí" : "Why solar terms exist"}</h4>
        <p>{content.whyItExists}</p>
      </article>

      <article class="term-card">
        <h4>{language === "vi" ? "Bối cảnh mùa" : "Seasonal context"}</h4>
        <p>{content.seasonalContext}</p>
      </article>

      <article class="term-card two-col">
        <div class="list-col">
          <h4>{language === "vi" ? "Nên làm" : "Do"}</h4>
          <ul>
            {#each content.dos as item}
              <li>{item}</li>
            {/each}
          </ul>
        </div>

        <div class="list-col">
          <h4>{language === "vi" ? "Không nên" : "Don't"}</h4>
          <ul>
            {#each content.donts as item}
              <li>{item}</li>
            {/each}
          </ul>
        </div>
      </article>

      <p class="source-note">{content.sourceNote}</p>
    </div>
  {/if}
</section>

<style>
  .culture-panel {
    margin-top: 4px;
    background: var(--surface-white);
    border: 1px solid var(--border-subtle);
    border-radius: 20px;
    padding: 20px;
    box-shadow: var(--shadow-soft);
  }

  .culture-header {
    margin-bottom: 6px;
  }

  .culture-header h2 {
    margin: 0;
    font-size: 1.1rem;
    color: var(--text-primary);
  }

  .culture-summary {
    margin: 0 0 14px;
    color: var(--text-secondary);
    line-height: 1.5;
  }

  .event-grid {
    display: grid;
    gap: 12px;
  }

  .event-card,
  .term-card {
    background: rgba(255, 255, 255, 0.65);
    border: 1px solid var(--border-subtle);
    border-radius: 12px;
    padding: 12px;
  }

  .event-title {
    margin: 0 0 10px;
    color: var(--primary-red);
    font-size: 1rem;
  }

  .fact-block + .fact-block {
    margin-top: 8px;
  }

  .fact-block h4,
  .list-col h4,
  .term-card h4 {
    margin: 0 0 6px;
    font-size: 0.78rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--text-tertiary);
  }

  .fact-block p,
  .term-card p {
    margin: 0;
    color: var(--text-primary);
    line-height: 1.5;
  }

  .list-row,
  .two-col {
    margin-top: 10px;
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 10px;
  }

  .list-col ul {
    margin: 0;
    padding-left: 18px;
    display: grid;
    gap: 6px;
    color: var(--text-primary);
  }

  .term-layout {
    display: grid;
    gap: 12px;
    grid-template-columns: 1.2fr 1fr;
  }

  .term-card.main {
    grid-column: span 2;
    background: linear-gradient(140deg, rgba(42, 110, 100, 0.08), rgba(255, 255, 255, 0.75));
  }

  .term-card.main h3 {
    margin: 0 0 8px;
    color: var(--accent-jade);
  }

  .source-note {
    margin: 0;
    color: var(--text-tertiary);
    font-size: 0.78rem;
    grid-column: span 2;
  }

  @media (max-width: 1024px) {
    .term-layout,
    .list-row,
    .two-col {
      grid-template-columns: 1fr;
    }

    .term-card.main,
    .source-note {
      grid-column: auto;
    }
  }
</style>
