import { defineStore } from 'pinia';
import { ref } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';

export const useAppStore = defineStore('app', () => {
  const isWorkMode = ref(false);
  const isAlwaysOnTop = ref(false);

  const toggleAlwaysOnTop = async () => {
    isAlwaysOnTop.value = !isAlwaysOnTop.value;
    try {
      await getCurrentWindow().setAlwaysOnTop(isAlwaysOnTop.value);
    } catch (e) {
      console.error('Failed to set always on top:', e);
    }
  };

  const toggleWorkMode = () => {
    isWorkMode.value = !isWorkMode.value;
  };

  return { isWorkMode, isAlwaysOnTop, toggleAlwaysOnTop, toggleWorkMode };
});
