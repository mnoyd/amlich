<script lang="ts">
  import { relaunch } from "@tauri-apps/plugin-process";
  import { check } from "@tauri-apps/plugin-updater";
  import { openUrl } from "@tauri-apps/plugin-opener";
  import { settings } from "$lib/stores/settings.svelte";

  let { open: isOpen = $bindable(false) }: { open: boolean } = $props();

  // Tab state
  let activeTab = $state<"general" | "account" | "help">("general");

  // Update state
  let isCheckingUpdate = $state(false);
  let updateStatus = $state<string | null>(null);
  let updateStatusKind = $state<"info" | "success" | "error">("info");

  // App version (from tauri.conf.json)
  const appVersion = "0.1.2";

  async function checkForUpdates() {
    if (isCheckingUpdate) return;
    isCheckingUpdate = true;
    updateStatusKind = "info";
    updateStatus = settings.language === "vi"
      ? "Đang kiểm tra bản cập nhật..."
      : "Checking for updates...";

    try {
      const update = await check();
      if (!update) {
        updateStatusKind = "success";
        updateStatus = settings.language === "vi"
          ? "Bạn đang dùng phiên bản mới nhất."
          : "You're using the latest version.";
        return;
      }

      updateStatus = settings.language === "vi"
        ? "Đang tải và cài đặt bản cập nhật..."
        : "Downloading and installing update...";
      await update.downloadAndInstall();
      updateStatus = settings.language === "vi"
        ? "Đã cài đặt. Đang khởi động lại..."
        : "Installed. Restarting app...";
      await relaunch();
    } catch (err) {
      updateStatusKind = "error";
      updateStatus = err instanceof Error ? err.message : String(err);
    } finally {
      isCheckingUpdate = false;
    }
  }

  function closePanel() {
    isOpen = false;
  }

  function handleKeydown(e: KeyboardEvent) {
    if (e.key === "Escape" && isOpen) {
      closePanel();
    }
  }

  async function openLink(url: string) {
    try {
      await openUrl(url);
    } catch (e) {
      console.error("Failed to open link:", e);
    }
  }

  const labels = $derived({
    title: settings.language === "vi" ? "Cài đặt" : "Settings",
    tabs: {
      general: settings.language === "vi" ? "Chung" : "General",
      account: settings.language === "vi" ? "Tài khoản" : "Account",
      help: settings.language === "vi" ? "Trợ giúp" : "Help",
    },
    general: {
      language: settings.language === "vi" ? "Ngôn ngữ" : "Language",
      update: settings.language === "vi" ? "Cập nhật" : "Updates",
      version: settings.language === "vi" ? "Phiên bản hiện tại" : "Current version",
      checkUpdate: settings.language === "vi" ? "Kiểm tra cập nhật" : "Check for updates",
      autoCheck: settings.language === "vi" ? "Tự động kiểm tra khi khởi động" : "Auto-check on startup",
    },
    account: {
      title: settings.language === "vi" ? "Đồng bộ & Tài khoản" : "Sync & Account",
      comingSoon: settings.language === "vi" ? "Tính năng đang phát triển" : "Feature in development",
      description: settings.language === "vi"
        ? "Sắp ra mắt các tính năng mới"
        : "New features coming soon",
      features: [
        {
          icon: "calendar",
          title: settings.language === "vi" ? "Ngày giỗ" : "Anniversaries",
          desc: settings.language === "vi"
            ? "Lưu ngày giỗ ông bà, tổ tiên"
            : "Save ancestor anniversaries",
        },
        {
          icon: "sync",
          title: settings.language === "vi" ? "Đồng bộ" : "Sync",
          desc: settings.language === "vi"
            ? "Google Calendar, Zalo"
            : "Google Calendar, Zalo",
        },
        {
          icon: "star",
          title: settings.language === "vi" ? "Sự kiện riêng" : "Custom Events",
          desc: settings.language === "vi"
            ? "Thêm ngày đặc biệt của bạn"
            : "Add your special dates",
        },
      ],
    },
    help: {
      shortcuts: settings.language === "vi" ? "Phím tắt" : "Keyboard Shortcuts",
      about: settings.language === "vi" ? "Về ứng dụng" : "About",
      reportBug: settings.language === "vi" ? "Báo lỗi" : "Report a bug",
      prevMonth: settings.language === "vi" ? "Tháng trước" : "Previous month",
      nextMonth: settings.language === "vi" ? "Tháng sau" : "Next month",
      today: settings.language === "vi" ? "Về hôm nay" : "Go to today",
      closePanel: settings.language === "vi" ? "Đóng cài đặt" : "Close settings",
      openSettings: settings.language === "vi" ? "Mở cài đặt" : "Open settings",
    },
  });

  const shortcuts = [
    { key: "←", action: () => labels.help.prevMonth },
    { key: "→", action: () => labels.help.nextMonth },
    { key: "T", action: () => labels.help.today },
    { key: "Esc", action: () => labels.help.closePanel },
    { key: "?", action: () => labels.help.openSettings },
  ];
</script>

<svelte:window onkeydown={handleKeydown} />

{#if isOpen}
  <!-- Backdrop -->
  <button
    class="settings-backdrop"
    onclick={closePanel}
    onkeydown={(e) => e.key === "Enter" && closePanel()}
    tabindex="-1"
    aria-label="Close settings"
  ></button>
{/if}

<!-- Panel -->
<div class="settings-panel" class:open={isOpen} role="dialog" aria-modal="true" aria-label={labels.title}>
  <header class="settings-header">
    <div class="header-title">
      <svg
        class="header-icon"
        xmlns="http://www.w3.org/2000/svg"
        width="22"
        height="22"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" />
        <circle cx="12" cy="12" r="3" />
      </svg>
      <h2>{labels.title}</h2>
    </div>
    <button class="close-btn" onclick={closePanel} aria-label="Close">
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="18"
        height="18"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <path d="M18 6 6 18" />
        <path d="m6 6 12 12" />
      </svg>
    </button>
  </header>

  <nav class="settings-tabs">
    <button
      class="tab-btn"
      class:active={activeTab === "general"}
      onclick={() => (activeTab = "general")}
    >
      <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="3" />
        <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" />
      </svg>
      <span>{labels.tabs.general}</span>
    </button>
    <button
      class="tab-btn"
      class:active={activeTab === "account"}
      onclick={() => (activeTab = "account")}
    >
      <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2" />
        <circle cx="12" cy="7" r="4" />
      </svg>
      <span>{labels.tabs.account}</span>
    </button>
    <button
      class="tab-btn"
      class:active={activeTab === "help"}
      onclick={() => (activeTab = "help")}
    >
      <svg class="tab-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
        <circle cx="12" cy="12" r="10" />
        <path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" />
        <path d="M12 17h.01" />
      </svg>
      <span>{labels.tabs.help}</span>
    </button>
  </nav>

  <div class="settings-content">
    {#if activeTab === "general"}
      <div class="tab-content">
        <!-- Language Section -->
        <section class="settings-section">
          <div class="section-header">
            <svg class="section-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <circle cx="12" cy="12" r="10" />
              <path d="M2 12h20" />
              <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
            </svg>
            <h3 class="section-title">{labels.general.language}</h3>
          </div>
          <div class="language-switcher">
            <button
              class="lang-btn"
              class:active={settings.language === "vi"}
              onclick={() => (settings.language = "vi")}
            >
              <span class="lang-flag">🇻🇳</span>
              <span class="lang-name">Tiếng Việt</span>
            </button>
            <button
              class="lang-btn"
              class:active={settings.language === "en"}
              onclick={() => (settings.language = "en")}
            >
              <span class="lang-flag">🇺🇸</span>
              <span class="lang-name">English</span>
            </button>
          </div>
        </section>

        <!-- Update Section -->
        <section class="settings-section">
          <div class="section-header">
            <svg class="section-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
              <polyline points="7 10 12 15 17 10" />
              <line x1="12" x2="12" y1="15" y2="3" />
            </svg>
            <h3 class="section-title">{labels.general.update}</h3>
          </div>
          
          <div class="version-display">
            <span class="version-label">{labels.general.version}</span>
            <span class="version-badge">{appVersion}</span>
          </div>

          {#if updateStatus}
            <div class="update-status {updateStatusKind}">
              {#if updateStatusKind === "info"}
                <svg class="status-icon spinning" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M21 12a9 9 0 1 1-6.219-8.56" />
                </svg>
              {:else if updateStatusKind === "success"}
                <svg class="status-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <path d="M22 11.08V12a10 10 0 1 1-5.93-9.14" />
                  <polyline points="22 4 12 14.01 9 11.01" />
                </svg>
              {:else}
                <svg class="status-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                  <circle cx="12" cy="12" r="10" />
                  <line x1="12" x2="12" y1="8" y2="12" />
                  <line x1="12" x2="12.01" y1="16" y2="16" />
                </svg>
              {/if}
              <span>{updateStatus}</span>
            </div>
          {/if}

          <button
            class="update-btn"
            onclick={checkForUpdates}
            disabled={isCheckingUpdate}
          >
            {#if isCheckingUpdate}
              <span class="spinner-small"></span>
            {:else}
              <svg class="btn-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <polyline points="23 4 23 10 17 10" />
                <path d="M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
              </svg>
            {/if}
            {labels.general.checkUpdate}
          </button>

          <label class="toggle-row">
            <div class="toggle-info">
              <span class="toggle-label">{labels.general.autoCheck}</span>
            </div>
            <div class="toggle-wrapper">
              <input
                type="checkbox"
                class="toggle-input"
                checked={settings.autoCheckUpdates}
                onchange={(e) => (settings.autoCheckUpdates = e.currentTarget.checked)}
              />
              <span class="toggle-track">
                <span class="toggle-thumb"></span>
              </span>
            </div>
          </label>
        </section>
      </div>

    {:else if activeTab === "account"}
      <div class="tab-content">
        <!-- Coming Soon with Feature Preview -->
        <div class="account-preview">
          <div class="preview-header">
            <div class="preview-icon-wrapper">
              <svg class="preview-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" />
                <path d="m9 12 2 2 4-4" />
              </svg>
            </div>
            <div class="preview-text">
              <h3>{labels.account.comingSoon}</h3>
              <p>{labels.account.description}</p>
            </div>
          </div>

          <div class="feature-cards">
            {#each labels.account.features as feature}
              <div class="feature-card">
                <div class="feature-icon">
                  {#if feature.icon === "calendar"}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <rect width="18" height="18" x="3" y="4" rx="2" ry="2" />
                      <line x1="16" x2="16" y1="2" y2="6" />
                      <line x1="8" x2="8" y1="2" y2="6" />
                      <line x1="3" x2="21" y1="10" y2="10" />
                      <path d="m9 16 2 2 4-4" />
                    </svg>
                  {:else if feature.icon === "sync"}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <path d="M21 12a9 9 0 0 0-9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" />
                      <path d="M3 3v5h5" />
                      <path d="M3 12a9 9 0 0 0 9 9 9.75 9.75 0 0 0 6.74-2.74L21 16" />
                      <path d="M16 16h5v5" />
                    </svg>
                  {:else}
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                      <polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" />
                    </svg>
                  {/if}
                </div>
                <div class="feature-content">
                  <span class="feature-title">{feature.title}</span>
                  <span class="feature-desc">{feature.desc}</span>
                </div>
              </div>
            {/each}
          </div>
        </div>
      </div>

    {:else if activeTab === "help"}
      <div class="tab-content">
        <!-- Shortcuts Section -->
        <section class="settings-section">
          <div class="section-header">
            <svg class="section-icon" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <rect width="20" height="8" x="2" y="14" rx="2" />
              <path d="M6 18h.01" />
              <path d="M10 18h.01" />
              <path d="M14 18h.01" />
              <path d="M18 18h.01" />
              <path d="M6 10h.01" />
              <path d="M10 10h.01" />
              <path d="M14 10h.01" />
              <path d="M18 10h.01" />
              <rect width="20" height="8" x="2" y="6" rx="2" />
            </svg>
            <h3 class="section-title">{labels.help.shortcuts}</h3>
          </div>
          <div class="shortcuts-grid">
            {#each shortcuts as shortcut}
              <div class="shortcut-item">
                <kbd class="shortcut-key">{shortcut.key}</kbd>
                <span class="shortcut-action">{shortcut.action()}</span>
              </div>
            {/each}
          </div>
        </section>

        <!-- About Section -->
        <section class="settings-section about-section">
          <div class="about-header">
            <img class="about-logo" src="/amlich-logo.svg" alt="Âm Lịch logo" />
            <div class="about-info">
              <span class="about-name">Âm Lịch</span>
              <span class="about-tagline">
                {settings.language === "vi"
                  ? "Lịch Âm Việt Nam"
                  : "Vietnamese Lunar Calendar"}
              </span>
            </div>
            <span class="about-version-badge">v{appVersion}</span>
          </div>
          <p class="about-desc">
            {settings.language === "vi"
              ? "Ứng dụng lịch âm Việt Nam với Can Chi, Tiết Khí, giờ hoàng đạo và thông tin ngày lễ."
              : "Vietnamese Lunar Calendar with Can Chi, Solar Terms, auspicious hours and holiday information."}
          </p>
          <div class="about-footer">
            <span class="about-copyright">© 2024 Âm Lịch</span>
          </div>
        </section>

        <!-- Links Section -->
        <div class="link-buttons">
          <button
            class="link-btn github"
            onclick={() => openLink("https://github.com/mnoyd/amlich")}
          >
            <svg viewBox="0 0 24 24" fill="currentColor">
              <path
                d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
              />
            </svg>
            <span>GitHub</span>
          </button>
          <button
            class="link-btn report"
            onclick={() => openLink("https://github.com/mnoyd/amlich/issues/new")}
          >
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z" />
              <path d="M12 9v4" />
              <path d="M12 17h.01" />
            </svg>
            <span>{labels.help.reportBug}</span>
          </button>
        </div>
      </div>
    {/if}
  </div>
</div>

<style>
  /* Backdrop */
  .settings-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(44, 36, 27, 0.25);
    backdrop-filter: blur(4px);
    -webkit-backdrop-filter: blur(4px);
    z-index: 99;
    border: none;
    cursor: default;
    animation: fadeIn 0.2s ease-out;
  }

  @keyframes fadeIn {
    from { opacity: 0; }
    to { opacity: 1; }
  }

  /* Panel */
  .settings-panel {
    position: fixed;
    top: 24px;
    right: 24px;
    bottom: 24px;
    width: 380px;
    max-width: calc(100vw - 48px);
    background: linear-gradient(180deg, #FFFEFB 0%, #FDF8F3 100%);
    border: 1px solid rgba(212, 175, 55, 0.25);
    border-radius: 24px;
    box-shadow: 
      0 24px 48px rgba(44, 36, 27, 0.12),
      0 0 0 1px rgba(255, 255, 255, 0.8) inset;
    z-index: 100;
    display: flex;
    flex-direction: column;
    transform: translateX(calc(100% + 48px));
    transition: transform 0.35s cubic-bezier(0.32, 0.72, 0, 1);
    overflow: hidden;
  }

  .settings-panel.open {
    transform: translateX(0);
  }

  /* Header */
  .settings-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px 20px 16px;
    border-bottom: 1px solid rgba(212, 175, 55, 0.15);
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.9) 0%, rgba(255, 255, 255, 0.5) 100%);
  }

  .header-title {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .header-icon {
    color: var(--accent-jade);
  }

  .settings-header h2 {
    margin: 0;
    font-size: 1.35rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: -0.01em;
  }

  .close-btn {
    width: 32px;
    height: 32px;
    border: none;
    background: rgba(0, 0, 0, 0.04);
    border-radius: 8px;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-tertiary);
    transition: all 0.15s ease;
  }

  .close-btn:hover {
    background: rgba(217, 48, 37, 0.1);
    color: var(--primary-red);
  }

  .close-btn:focus-visible {
    outline: 2px solid var(--accent-jade);
    outline-offset: 2px;
  }

  /* Tabs */
  .settings-tabs {
    display: flex;
    gap: 6px;
    padding: 12px 16px;
    background: rgba(0, 0, 0, 0.02);
    border-bottom: 1px solid rgba(0, 0, 0, 0.04);
  }

  .tab-btn {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
    padding: 10px 8px;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 12px;
    font-family: var(--font-sans);
    font-size: 0.82rem;
    font-weight: 600;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .tab-icon {
    width: 20px;
    height: 20px;
    stroke-width: 1.75;
  }

  .tab-btn:hover {
    background: rgba(255, 255, 255, 0.8);
    color: var(--text-secondary);
  }

  .tab-btn:focus-visible {
    outline: 2px solid var(--accent-jade);
    outline-offset: 2px;
  }

  .tab-btn.active {
    background: var(--accent-jade);
    color: white;
    border-color: var(--accent-jade);
    box-shadow: 0 4px 12px rgba(42, 110, 100, 0.3);
  }

  .tab-btn.active .tab-icon {
    stroke-width: 2;
  }

  /* Content */
  .settings-content {
    flex: 1;
    overflow-y: auto;
    overflow-x: hidden;
  }

  .tab-content {
    padding: 16px;
    display: flex;
    flex-direction: column;
    gap: 14px;
    animation: slideUp 0.25s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  /* Section */
  .settings-section {
    background: rgba(255, 255, 255, 0.85);
    border: 1px solid rgba(212, 175, 55, 0.18);
    border-radius: 16px;
    padding: 16px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.02);
  }

  .section-header {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 14px;
    padding-bottom: 10px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.05);
  }

  .section-icon {
    width: 18px;
    height: 18px;
    color: var(--accent-jade);
    stroke-width: 2;
  }

  .section-title {
    margin: 0;
    font-size: 0.92rem;
    font-weight: 700;
    color: var(--text-primary);
    letter-spacing: 0.01em;
  }

  /* Language Switcher */
  .language-switcher {
    display: flex;
    gap: 8px;
  }

  .lang-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px;
    background: rgba(0, 0, 0, 0.02);
    border: 2px solid transparent;
    border-radius: 12px;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .lang-btn:hover {
    background: rgba(0, 0, 0, 0.04);
  }

  .lang-btn:focus-visible {
    outline: 2px solid var(--accent-jade);
    outline-offset: 2px;
  }

  .lang-btn.active {
    background: rgba(42, 110, 100, 0.08);
    border-color: var(--accent-jade);
  }

  .lang-flag {
    font-size: 1.25rem;
  }

  .lang-name {
    font-family: var(--font-sans);
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  /* Version Display */
  .version-display {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 14px;
    padding: 10px 12px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 10px;
  }

  .version-label {
    font-size: 0.92rem;
    color: var(--text-secondary);
  }

  .version-badge {
    font-family: var(--font-sans);
    font-size: 0.9rem;
    font-weight: 700;
    color: var(--accent-jade);
    background: rgba(42, 110, 100, 0.1);
    padding: 4px 10px;
    border-radius: 6px;
  }

  /* Update Status */
  .update-status {
    display: flex;
    align-items: center;
    gap: 10px;
    margin-bottom: 14px;
    padding: 12px 14px;
    border-radius: 12px;
    font-family: var(--font-sans);
    font-size: 0.92rem;
    font-weight: 500;
    animation: slideUp 0.2s ease-out;
  }

  .status-icon {
    width: 18px;
    height: 18px;
    flex-shrink: 0;
  }

  .status-icon.spinning {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .update-status.info {
    background: linear-gradient(135deg, rgba(212, 175, 55, 0.12) 0%, rgba(212, 175, 55, 0.08) 100%);
    color: #8a6d1c;
    border: 1px solid rgba(212, 175, 55, 0.2);
  }

  .update-status.success {
    background: linear-gradient(135deg, rgba(42, 110, 100, 0.12) 0%, rgba(42, 110, 100, 0.08) 100%);
    color: var(--accent-jade);
    border: 1px solid rgba(42, 110, 100, 0.2);
  }

  .update-status.error {
    background: linear-gradient(135deg, rgba(217, 48, 37, 0.1) 0%, rgba(217, 48, 37, 0.06) 100%);
    color: var(--primary-red);
    border: 1px solid rgba(217, 48, 37, 0.2);
  }

  /* Update Button */
  .update-btn {
    width: 100%;
    padding: 14px 18px;
    background: linear-gradient(135deg, var(--accent-jade) 0%, #1f5f55 100%);
    border: none;
    border-radius: 12px;
    font-family: var(--font-sans);
    font-size: 0.95rem;
    font-weight: 600;
    color: white;
    cursor: pointer;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-bottom: 14px;
    box-shadow: 0 4px 12px rgba(42, 110, 100, 0.25);
  }

  .update-btn:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 6px 20px rgba(42, 110, 100, 0.35);
  }

  .update-btn:active:not(:disabled) {
    transform: translateY(0);
  }

  .update-btn:focus-visible {
    outline: 2px solid var(--accent-jade);
    outline-offset: 2px;
  }

  .update-btn:disabled {
    opacity: 0.7;
    cursor: not-allowed;
  }

  .btn-icon {
    width: 18px;
    height: 18px;
  }

  .spinner-small {
    width: 18px;
    height: 18px;
    border: 2px solid rgba(255, 255, 255, 0.3);
    border-top-color: white;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  /* Toggle */
  .toggle-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    cursor: pointer;
    padding: 12px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 12px;
    transition: background 0.15s ease;
  }

  .toggle-row:hover {
    background: rgba(0, 0, 0, 0.04);
  }

  .toggle-info {
    flex: 1;
    min-width: 0;
  }

  .toggle-label {
    font-size: 0.95rem;
    color: var(--text-primary);
    font-weight: 500;
  }

  .toggle-wrapper {
    position: relative;
    flex-shrink: 0;
  }

  .toggle-input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
  }

  .toggle-track {
    display: block;
    width: 48px;
    height: 28px;
    background: rgba(0, 0, 0, 0.12);
    border-radius: 14px;
    transition: background 0.2s ease;
    position: relative;
  }

  .toggle-thumb {
    position: absolute;
    top: 3px;
    left: 3px;
    width: 22px;
    height: 22px;
    background: white;
    border-radius: 50%;
    box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
    transition: transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
  }

  .toggle-input:checked + .toggle-track {
    background: var(--accent-jade);
  }

  .toggle-input:checked + .toggle-track .toggle-thumb {
    transform: translateX(20px);
  }

  .toggle-input:focus-visible + .toggle-track {
    box-shadow: 0 0 0 3px rgba(42, 110, 100, 0.3);
  }

  /* Account Preview */
  .account-preview {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .preview-header {
    display: flex;
    gap: 14px;
    padding: 20px;
    background: linear-gradient(135deg, rgba(212, 175, 55, 0.08) 0%, rgba(255, 255, 255, 0.9) 100%);
    border: 1px solid rgba(212, 175, 55, 0.2);
    border-radius: 16px;
  }

  .preview-icon-wrapper {
    width: 48px;
    height: 48px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: linear-gradient(135deg, var(--accent-gold) 0%, #c4a030 100%);
    border-radius: 12px;
    flex-shrink: 0;
  }

  .preview-icon {
    width: 26px;
    height: 26px;
    color: white;
  }

  .preview-text h3 {
    margin: 0 0 4px;
    font-size: 1.1rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .preview-text p {
    margin: 0;
    font-size: 0.92rem;
    color: var(--text-secondary);
    line-height: 1.4;
  }

  .feature-cards {
    display: flex;
    flex-direction: column;
    gap: 10px;
  }

  .feature-card {
    display: flex;
    gap: 12px;
    padding: 14px;
    background: rgba(255, 255, 255, 0.85);
    border: 1px solid rgba(212, 175, 55, 0.15);
    border-radius: 14px;
    transition: all 0.2s ease;
  }

  .feature-card:hover {
    background: white;
    border-color: rgba(212, 175, 55, 0.3);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.04);
  }

  .feature-icon {
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(42, 110, 100, 0.08);
    border-radius: 10px;
    flex-shrink: 0;
  }

  .feature-icon svg {
    width: 18px;
    height: 18px;
    color: var(--accent-jade);
  }

  .feature-content {
    display: flex;
    flex-direction: column;
    gap: 2px;
    min-width: 0;
  }

  .feature-title {
    font-size: 0.95rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .feature-desc {
    font-size: 0.85rem;
    color: var(--text-tertiary);
  }

  /* Shortcuts Grid */
  .shortcuts-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 8px;
  }

  .shortcut-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    background: rgba(0, 0, 0, 0.02);
    border-radius: 10px;
  }

  .shortcut-key {
    min-width: 32px;
    padding: 5px 8px;
    background: white;
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 6px;
    font-family: var(--font-sans);
    font-size: 0.85rem;
    font-weight: 700;
    color: var(--text-primary);
    text-align: center;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.05);
  }

  .shortcut-action {
    font-size: 0.88rem;
    color: var(--text-secondary);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* About Section */
  .about-section {
    text-align: center;
  }

  .about-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 10px;
    margin-bottom: 14px;
    position: relative;
  }

  .about-logo {
    width: 56px;
    height: 56px;
    border-radius: 14px;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .about-info {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .about-name {
    font-size: 1.2rem;
    font-weight: 700;
    color: var(--text-primary);
  }

  .about-tagline {
    font-size: 0.88rem;
    color: var(--text-tertiary);
  }

  .about-version-badge {
    position: absolute;
    top: 0;
    right: 0;
    font-family: var(--font-sans);
    font-size: 0.78rem;
    font-weight: 600;
    color: var(--accent-jade);
    background: rgba(42, 110, 100, 0.1);
    padding: 3px 8px;
    border-radius: 6px;
  }

  .about-desc {
    margin: 0 0 14px;
    font-size: 0.95rem;
    color: var(--text-secondary);
    line-height: 1.55;
  }

  .about-footer {
    padding-top: 12px;
    border-top: 1px solid rgba(0, 0, 0, 0.05);
  }

  .about-copyright {
    font-size: 0.82rem;
    color: var(--text-tertiary);
  }

  /* Link Buttons */
  .link-buttons {
    display: flex;
    gap: 10px;
  }

  .link-btn {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px 16px;
    background: rgba(255, 255, 255, 0.9);
    border: 1px solid rgba(0, 0, 0, 0.08);
    border-radius: 12px;
    font-family: var(--font-sans);
    font-size: 0.92rem;
    font-weight: 600;
    color: var(--text-secondary);
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .link-btn svg {
    width: 18px;
    height: 18px;
  }

  .link-btn:hover {
    background: white;
    border-color: rgba(0, 0, 0, 0.12);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
    transform: translateY(-1px);
  }

  .link-btn:focus-visible {
    outline: 2px solid var(--accent-jade);
    outline-offset: 2px;
  }

  .link-btn.github:hover {
    color: #333;
  }

  .link-btn.report:hover {
    color: var(--primary-red);
    border-color: rgba(217, 48, 37, 0.2);
  }

  /* Responsive */
  @media (max-width: 480px) {
    .settings-panel {
      top: 0;
      right: 0;
      bottom: 0;
      width: 100%;
      max-width: 100%;
      border-radius: 0;
    }

    .shortcuts-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
