import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { openUrl } from '@tauri-apps/plugin-opener';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';

const RELEASES_URL = 'https://github.com/mnoyd/amlich/releases/latest';
const LATEST_JSON_URL = `${RELEASES_URL}/download/latest.json`;
const MAC_UPDATER_TARGET_CANDIDATES = [
  'darwin-universal-app',
  'darwin-universal',
  'universal-apple-darwin-app',
  'universal-apple-darwin',
] as const;
const MAC_UPDATER_MIGRATION_PROMPT_KEY = 'amlich.macosUpdaterMigrationPrompt.v1';
const MAC_UPDATER_FIX_VERSION = '0.1.9';

type InstallContext = {
  executablePath?: string;
  isSystemInstall: boolean;
  canSelfUpdate: boolean;
  platform: string;
  arch: string;
  appVersion: string;
};

type LatestJson = {
  platforms?: Record<string, unknown>;
};

function isMacOs(context: InstallContext | null): boolean {
  return context?.platform === 'macos';
}

function isAppleSiliconMac(context: InstallContext | null): boolean {
  return isMacOs(context) && context?.arch === 'aarch64';
}

function parseVersion(version: string): number[] {
  return version
    .split('.')
    .map((part) => Number.parseInt(part, 10))
    .map((n) => (Number.isFinite(n) ? n : 0));
}

function isVersionLessThan(version: string, minVersion: string): boolean {
  const a = parseVersion(version);
  const b = parseVersion(minVersion);
  const len = Math.max(a.length, b.length);
  for (let i = 0; i < len; i += 1) {
    const av = a[i] ?? 0;
    const bv = b[i] ?? 0;
    if (av < bv) return true;
    if (av > bv) return false;
  }
  return false;
}

function shouldShowMacMigrationPrompt(context: InstallContext | null): boolean {
  if (!isAppleSiliconMac(context)) return false;
  if (!context?.appVersion) return false;
  if (!isVersionLessThan(context.appVersion, MAC_UPDATER_FIX_VERSION)) return false;
  try {
    return localStorage.getItem(MAC_UPDATER_MIGRATION_PROMPT_KEY) !== '1';
  } catch {
    return true;
  }
}

function markMacMigrationPromptShown() {
  try {
    localStorage.setItem(MAC_UPDATER_MIGRATION_PROMPT_KEY, '1');
  } catch {
    // Ignore storage failures; this is only for UX de-duplication.
  }
}

async function maybeShowMacMigrationPrompt(context: InstallContext | null, onUserClick: boolean) {
  if (!onUserClick || !shouldShowMacMigrationPrompt(context)) return;

  markMacMigrationPromptShown();
  await promptOpenReleases(
    'One-Time macOS Update Migration',
    'This macOS build uses an older updater path that is no longer supported for universal updates. Please reinstall once from the latest GitHub release (replace the app in /Applications), then future in-app updates should work again.\n\nSafe cleanup: remove AmLich.app from /Applications (or ~/Applications) and clear ~/Library/Caches/com.amlich.calendar. Your preferences and app data can stay.'
  );
}

async function fetchLatestJson(): Promise<LatestJson | null> {
  try {
    const res = await fetch(LATEST_JSON_URL, { cache: 'no-store' });
    if (!res.ok) return null;
    return (await res.json()) as LatestJson;
  } catch {
    return null;
  }
}

export function resolveMacUniversalTarget(
  platforms: Record<string, unknown> | undefined
): string | null {
  if (!platforms) return null;
  for (const key of MAC_UPDATER_TARGET_CANDIDATES) {
    if (key in platforms) return key;
  }
  return null;
}

async function resolveUpdaterTargetForPlatform(context: InstallContext | null): Promise<string | undefined> {
  if (!isMacOs(context)) return undefined;
  const latest = await fetchLatestJson();
  return resolveMacUniversalTarget(latest?.platforms) ?? undefined;
}

async function promptOpenReleases(title: string, detail: string) {
  const open = await ask(`${detail}\n\nOpen GitHub releases page now?`, {
    title,
    kind: 'warning',
    okLabel: 'Open Releases',
    cancelLabel: 'Close',
  });

  if (open) {
    try {
      await openUrl(RELEASES_URL);
    } catch {
      await message(`Unable to open browser automatically.\n\nDownload here:\n${RELEASES_URL}`, {
        title: 'Download Update',
        kind: 'info',
        okLabel: 'OK',
      });
    }
  }
}

async function getInstallContext(): Promise<InstallContext | null> {
  try {
    return await invoke<InstallContext>('get_install_context');
  } catch {
    return null;
  }
}

export async function checkForAppUpdates(onUserClick = false) {
  const installContext = await getInstallContext();

  await maybeShowMacMigrationPrompt(installContext, onUserClick);

  if (onUserClick && installContext?.isSystemInstall) {
    await promptOpenReleases(
      'Managed Installation Detected',
      'This copy looks installed by a system package manager (for example pacman/yay). Use your package manager to update, or install the AppImage build for in-app updates.'
    );
    return;
  }

  try {
    const customTarget = await resolveUpdaterTargetForPlatform(installContext);
    if (isMacOs(installContext) && !customTarget && onUserClick) {
      await promptOpenReleases(
        'Update Metadata Missing',
        'This release does not include a universal mac updater entry in latest.json. Please download the latest universal mac build from GitHub Releases and reinstall once.'
      );
      return;
    }

    const update = customTarget ? await check({ target: customTarget }) : await check();

    if (update?.available) {
      const yes = await ask(
        `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
        {
          title: 'Update Available',
          kind: 'info',
          okLabel: 'Update',
          cancelLabel: 'Cancel',
        }
      );

      if (yes) {
        try {
          await update.downloadAndInstall();
          await relaunch();
        } catch (e) {
          if (onUserClick) {
            await promptOpenReleases(
              'Update Install Failed',
              `Automatic update failed:\n${e}`
            );
          }
          console.error('Update install failed:', e);
        }
      }
    } else if (onUserClick) {
      await message('You are on the latest version.', {
        title: 'No Update Available',
        kind: 'info',
        okLabel: 'OK',
      });
    }
  } catch (e) {
    if (onUserClick) {
      await promptOpenReleases('Update Error', `Failed to check for updates:\n${e}`);
    }
    console.error('Update check failed:', e);
  }
}
