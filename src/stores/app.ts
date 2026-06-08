import { defineStore } from 'pinia';
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { LogicalSize, PhysicalPosition, PhysicalSize } from '@tauri-apps/api/dpi';
import { isTauriRuntime } from '../utils/browserGuards';

type SavedWindowState = {
  size: PhysicalSize;
  position: PhysicalPosition;
  maximized: boolean;
  fullscreen: boolean;
  alwaysOnTop: boolean;
};

export const useAppStore = defineStore('app', () => {
  const isWorkMode = ref(false);
  const isAlwaysOnTop = ref(false);
  const savedWindowState = ref<SavedWindowState | null>(null);

  const toggleAlwaysOnTop = async () => {
    isAlwaysOnTop.value = !isAlwaysOnTop.value;
    try {
      await getCurrentWindow().setAlwaysOnTop(isAlwaysOnTop.value);
    } catch (e) {
      console.error('Failed to set always on top:', e);
    }
  };

  const enterWorkMode = async () => {
    if (isWorkMode.value) return;
    isWorkMode.value = true;

    if (!isTauriRuntime()) return;

    const appWindow = getCurrentWindow();
    try {
      savedWindowState.value = {
        size: await appWindow.outerSize(),
        position: await appWindow.outerPosition(),
        maximized: await appWindow.isMaximized(),
        fullscreen: await appWindow.isFullscreen(),
        alwaysOnTop: isAlwaysOnTop.value,
      };

      if (savedWindowState.value.fullscreen) {
        await appWindow.setFullscreen(false);
      }
      if (savedWindowState.value.maximized) {
        await appWindow.unmaximize();
      }

      await appWindow.setSize(new LogicalSize(420, 220));
      await appWindow.setAlwaysOnTop(true);
      isAlwaysOnTop.value = true;
    } catch (e) {
      console.error('Failed to enter work mode:', e);
    }
  };

  const exitWorkMode = async () => {
    if (!isWorkMode.value) return;
    isWorkMode.value = false;

    if (!isTauriRuntime()) return;

    const appWindow = getCurrentWindow();
    const previous = savedWindowState.value;
    savedWindowState.value = null;

    if (!previous) return;

    try {
      await appWindow.setFullscreen(false);
      if (previous.maximized) {
        await appWindow.maximize();
      } else {
        await appWindow.setSize(previous.size);
        await appWindow.setPosition(previous.position);
      }
      if (previous.fullscreen) {
        await appWindow.setFullscreen(true);
      }
      await appWindow.setAlwaysOnTop(previous.alwaysOnTop);
      isAlwaysOnTop.value = previous.alwaysOnTop;
    } catch (e) {
      console.error('Failed to exit work mode:', e);
    }
  };

  const toggleWorkMode = async () => {
    if (isWorkMode.value) {
      await exitWorkMode();
    } else {
      await enterWorkMode();
    }
  };

  return { isWorkMode, isAlwaysOnTop, toggleAlwaysOnTop, enterWorkMode, exitWorkMode, toggleWorkMode };
});
