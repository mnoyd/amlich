<script lang="ts">
  import type { GoodHour } from "$lib/insights/types";

  const ZODIAC_HOURS = [
    { name: "Tý", time: "23:00-01:00" },
    { name: "Sửu", time: "01:00-03:00" },
    { name: "Dần", time: "03:00-05:00" },
    { name: "Mão", time: "05:00-07:00" },
    { name: "Thìn", time: "07:00-09:00" },
    { name: "Tỵ", time: "09:00-11:00" },
    { name: "Ngọ", time: "11:00-13:00" },
    { name: "Mùi", time: "13:00-15:00" },
    { name: "Thân", time: "15:00-17:00" },
    { name: "Dậu", time: "17:00-19:00" },
    { name: "Tuất", time: "19:00-21:00" },
    { name: "Hợi", time: "21:00-23:00" },
  ];

  let { goodHours }: { goodHours: GoodHour[] } = $props();

  let hoveredZodiac: (typeof ZODIAC_HOURS)[number] | null = $state(null);
  let currentHandRotation = $state(0);
  let currentTimeStr = $state("");

  $effect(() => {
    const updateHand = () => {
      const now = new Date();
      const hours = now.getHours();
      const minutes = now.getMinutes();
      const totalMinutes = hours * 60 + minutes;
      currentHandRotation = (totalMinutes / 1440) * 360;
      currentTimeStr = now.toLocaleTimeString("vi-VN", {
        hour: "2-digit",
        minute: "2-digit",
      });
    };

    updateHand();
    const interval = setInterval(updateHand, 60000);
    return () => clearInterval(interval);
  });
</script>

<div class="zodiac-clock-container">
  <div class="zodiac-clock">
    <div
      class="clock-hand"
      style="transform: rotate({currentHandRotation}deg);"
    ></div>

    <div class="clock-center">
      {#if hoveredZodiac}
        <div class="center-label">{hoveredZodiac.name}</div>
        <div class="center-time">{hoveredZodiac.time}</div>
      {:else}
        <div class="center-label">Hiện tại</div>
        <div class="center-time">
          {currentTimeStr}
        </div>
      {/if}
    </div>

    {#each ZODIAC_HOURS as zodiac, i}
      {@const isGood = goodHours.some((h) =>
        h.hour_chi.includes(zodiac.name),
      )}
      {@const rotation = i * 30}
      <div
        class="clock-segment-wrapper"
        style="transform: rotate({rotation}deg);"
      >
        <button
          class="clock-segment {isGood ? 'good' : ''}"
          onmouseenter={() => (hoveredZodiac = zodiac)}
          onmouseleave={() => (hoveredZodiac = null)}
          onfocus={() => (hoveredZodiac = zodiac)}
          onblur={() => (hoveredZodiac = null)}
        >
          <div class="segment-shape"></div>
          <span
            class="segment-text"
            style="transform: rotate({-rotation}deg)"
          >
            {zodiac.name}
          </span>
        </button>
      </div>
    {/each}
  </div>
</div>

<style>
  .zodiac-clock-container {
    display: flex;
    justify-content: center;
    padding: 4px 0;
    margin-bottom: 10px;
  }

  .zodiac-clock {
    position: relative;
    width: 220px;
    height: 220px;
    border-radius: 50%;
    border: 1px solid rgba(212, 175, 55, 0.45);
    background: radial-gradient(
      circle,
      #ffffff 0%,
      #fff9f1 62%,
      #f8ede1 100%
    );
    box-shadow:
      inset 0 2px 18px rgba(255, 255, 255, 0.95),
      inset 0 -12px 26px rgba(212, 175, 55, 0.16),
      0 16px 30px rgba(0, 0, 0, 0.12);
  }

  .zodiac-clock::before {
    content: "";
    position: absolute;
    inset: 10px;
    border-radius: 50%;
    border: 1px dashed rgba(160, 126, 52, 0.3);
    pointer-events: none;
  }

  .clock-center {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
    width: 86px;
    height: 86px;
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    border-radius: 50%;
    background: linear-gradient(160deg, #ffffff 0%, #f9f1e6 100%);
    z-index: 10;
    pointer-events: none;
    box-shadow:
      0 8px 16px rgba(0, 0, 0, 0.1),
      inset 0 1px 0 rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(212, 175, 55, 0.4);
  }

  .center-label {
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--primary-red);
    font-family: var(--font-serif);
    line-height: 1;
  }

  .center-time {
    font-size: 0.82rem;
    color: var(--text-secondary);
    font-family: var(--font-sans);
    margin-top: 5px;
    font-weight: 600;
    letter-spacing: 0.01em;
    font-variant-numeric: tabular-nums;
  }

  .clock-hand {
    position: absolute;
    top: 0;
    left: 50%;
    width: 3px;
    height: 50%;
    background: linear-gradient(
      to top,
      rgba(217, 48, 37, 0.92) 55%,
      transparent 100%
    );
    transform-origin: bottom center;
    z-index: 5;
    pointer-events: none;
  }

  .clock-hand::after {
    content: "";
    position: absolute;
    top: 0;
    left: -3.5px;
    width: 10px;
    height: 10px;
    background: var(--primary-red);
    border-radius: 50%;
    box-shadow:
      0 0 8px rgba(217, 48, 37, 0.45),
      0 0 0 2px rgba(255, 255, 255, 0.8);
  }

  .clock-segment-wrapper {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    pointer-events: none;
  }

  .clock-segment {
    position: absolute;
    top: 0;
    left: 50%;
    transform: translateX(-50%);
    width: 50px;
    height: 50%;
    background: transparent;
    border: none;
    cursor: pointer;
    pointer-events: auto;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding-top: 10px;
    transition: all 0.2s;
  }

  .segment-shape {
    width: 7px;
    height: 7px;
    border-radius: 50%;
    background: rgba(83, 70, 50, 0.24);
    margin-bottom: 6px;
    transition: all 0.3s;
  }

  .segment-text {
    font-size: 0.74rem;
    font-weight: 700;
    color: #7c6849;
    transition: all 0.3s;
    margin-top: 2px;
    text-shadow: 0 1px 0 rgba(255, 255, 255, 0.7);
  }

  .clock-segment.good .segment-shape {
    background: var(--accent-jade);
    box-shadow: 0 0 10px rgba(42, 110, 100, 0.42);
    transform: scale(1.45);
  }

  .clock-segment.good .segment-text {
    color: var(--accent-jade);
    font-weight: 700;
  }

  .clock-segment:hover .segment-shape {
    background: var(--primary-red);
    transform: scale(1.25);
  }

  .clock-segment:hover .segment-text {
    color: var(--primary-red);
    font-weight: 700;
  }

  @media (max-width: 640px) {
    .zodiac-clock {
      width: 188px;
      height: 188px;
    }

    .clock-center {
      width: 74px;
      height: 74px;
    }

    .center-label {
      font-size: 1.05rem;
    }

    .center-time {
      font-size: 0.76rem;
    }

    .clock-segment {
      width: 44px;
      padding-top: 8px;
    }

    .segment-text {
      font-size: 0.68rem;
    }
  }
</style>
