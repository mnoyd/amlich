<script lang="ts">
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  export let current: string;

  const variants = [
    { key: 'A', name: 'Dòng kể trong ngày' },
    { key: 'B', name: 'Bàn quan sát' },
    { key: 'C', name: 'Sổ ngày & chứng cứ' },
  ];

  function cycle(offset: number) {
    const currentIndex = variants.findIndex((candidate) => candidate.key === current);
    const nextIndex = (currentIndex + offset + variants.length) % variants.length;
    const url = new URL(window.location.href);
    url.searchParams.set('variant', variants[nextIndex].key);
    goto(`${url.pathname}${url.search}`, {
      replaceState: true,
      keepFocus: true,
      noScroll: true,
    });
  }

  onMount(() => {
    const handleKeydown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      const editing = target?.matches('input, textarea, [contenteditable="true"]');
      if (editing || (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight')) return;
      event.preventDefault();
      cycle(event.key === 'ArrowLeft' ? -1 : 1);
    };

    window.addEventListener('keydown', handleKeydown);
    return () => window.removeEventListener('keydown', handleKeydown);
  });

  $: active = variants.find((candidate) => candidate.key === current) ?? variants[0];
</script>

<nav class="switcher" aria-label="Chọn phương án nguyên mẫu">
  <button type="button" aria-label="Phương án trước" onclick={() => cycle(-1)}>←</button>
  <span><b>{active.key}</b> — {active.name}</span>
  <button type="button" aria-label="Phương án sau" onclick={() => cycle(1)}>→</button>
</nav>

<style>
  .switcher {
    position: fixed;
    z-index: 100;
    bottom: 18px;
    left: 50%;
    display: flex;
    align-items: center;
    gap: 6px;
    min-width: 320px;
    padding: 6px;
    border: 1px solid rgba(255, 255, 255, 0.22);
    border-radius: 999px;
    background: #171a18;
    color: #f8f4e8;
    box-shadow: 0 12px 36px rgba(25, 28, 25, 0.3);
    font: 12px/1.2 ui-monospace, SFMono-Regular, Menlo, monospace;
    transform: translateX(-50%);
  }

  span {
    flex: 1;
    text-align: center;
    letter-spacing: 0.02em;
  }

  button {
    width: 34px;
    height: 34px;
    border: 0;
    border-radius: 50%;
    background: rgba(255, 255, 255, 0.1);
    color: inherit;
    cursor: pointer;
    font-size: 18px;
  }

  button:hover,
  button:focus-visible {
    background: #f1b84b;
    color: #171a18;
    outline: none;
  }
</style>
