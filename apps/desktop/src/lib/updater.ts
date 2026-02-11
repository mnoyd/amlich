import { check } from '@tauri-apps/plugin-updater';
import { ask, message } from '@tauri-apps/plugin-dialog';
import { openUrl } from '@tauri-apps/plugin-opener';
import { relaunch } from '@tauri-apps/plugin-process';
import { invoke } from '@tauri-apps/api/core';

const RELEASES_URL = 'https://github.com/mnoyd/amlich/releases/latest';

type InstallContext = {
  executablePath?: string;
  isSystemInstall: boolean;
  canSelfUpdate: boolean;
};

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

  if (onUserClick && installContext?.isSystemInstall) {
    await promptOpenReleases(
      'Managed Installation Detected',
      'This copy looks installed by a system package manager (for example pacman/yay). Use your package manager to update, or install the AppImage build for in-app updates.'
    );
    return;
  }

  try {
    const update = await check();

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
