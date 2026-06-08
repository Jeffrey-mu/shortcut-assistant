<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { Minimize2, Minus, Square, X } from 'lucide-vue-next';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useAppStore } from '../stores/app';
import { isTauriRuntime } from '../utils/browserGuards';

const appStore = useAppStore();
const isMac = /Mac|iPhone|iPad|iPod/.test(navigator.platform);
const isWindowExpanded = ref(false);

const syncExpandedState = async () => {
  if (!isTauriRuntime()) return;
  try {
    const appWindow = getCurrentWindow();
    isWindowExpanded.value = (await appWindow.isMaximized()) || (await appWindow.isFullscreen());
  } catch (error) {
    console.error('Failed to sync window expanded state:', error);
  }
};

const handleMinimize = async () => {
  if (!isTauriRuntime()) return;
  await getCurrentWindow().minimize();
};

const handleMaximize = async () => {
  if (!isTauriRuntime()) {
    isWindowExpanded.value = !isWindowExpanded.value;
    return;
  }
  try {
    const appWindow = getCurrentWindow();
    if (isWindowExpanded.value) {
      if (await appWindow.isFullscreen()) {
        await appWindow.setFullscreen(false);
      }
      await appWindow.unmaximize();
      isWindowExpanded.value = false;
    } else {
      await appWindow.maximize();
      isWindowExpanded.value = true;
    }
  } catch (error) {
    console.error('Failed to toggle window fullscreen/maximize:', error);
  }
};

const handleClose = async () => {
  if (!isTauriRuntime()) return;
  await getCurrentWindow().close();
};

onMounted(async () => {
  await syncExpandedState();
});
</script>

<template>
  <div data-tauri-drag-region @dblclick="handleMaximize" class="flex items-center justify-between select-none shrink-0 transition-colors duration-300 z-50 cursor-default backdrop-blur-xl"
       :style="{ backgroundColor: 'var(--window-surface, transparent)' }"
       :class="[
         appStore.isWorkMode ? 'h-8 bg-transparent border-b border-white/5' : 'h-10 bg-transparent border-b border-slate-200/50 dark:border-slate-800/50',
         isMac ? 'px-3' : 'px-4'
       ]">
    
    <!-- macOS 窗口控制 -->
    <div v-if="isMac" data-tauri-drag-region="false" class="group/window-controls flex w-24 items-center gap-2">
      <button
        @click.stop="handleClose"
        class="mac-traffic-light bg-[#ff5f57]"
        title="关闭"
      >
        <X :size="8" class="opacity-0 transition-opacity group-hover/window-controls:opacity-70" />
      </button>
      <button
        @click.stop="handleMinimize"
        class="mac-traffic-light bg-[#ffbd2e]"
        title="最小化"
      >
        <Minus :size="8" class="opacity-0 transition-opacity group-hover/window-controls:opacity-70" />
      </button>
      <button
        @click.stop="handleMaximize"
        class="mac-traffic-light bg-[#28c840]"
        :title="isWindowExpanded ? '还原' : '全屏'"
      >
        <span class="h-1.5 w-1.5 rounded-[1px] border border-current opacity-0 transition-opacity group-hover/window-controls:opacity-60"></span>
      </button>
    </div>

    <!-- 标题 -->
    <div data-tauri-drag-region class="flex h-full flex-1 items-center gap-2 pointer-events-none"
         :class="isMac ? 'justify-center' : 'justify-start'">
      <span class="text-xs font-semibold bg-gradient-to-r from-blue-600 to-purple-600 dark:from-blue-400 dark:to-purple-500 bg-clip-text text-transparent"
            :class="appStore.isWorkMode ? 'opacity-70' : ''">
        快捷键管理助手
      </span>
    </div>

    <div v-if="isMac" class="w-24"></div>

    <!-- 非 macOS 窗口控制 -->
    <div v-else data-tauri-drag-region="false" class="flex items-center" :class="appStore.isWorkMode ? 'gap-1 opacity-70 hover:opacity-100' : 'gap-2'">
      <button @click.stop="handleMinimize" class="p-1.5 text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-200/50 dark:hover:bg-slate-700/50 rounded transition-colors cursor-pointer">
        <Minus :size="14" />
      </button>
      <button
        @click.stop="handleMaximize"
        class="p-1.5 text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-200/50 dark:hover:bg-slate-700/50 rounded transition-colors cursor-pointer"
        :title="isWindowExpanded ? '还原' : '全屏'"
      >
        <Minimize2 v-if="isWindowExpanded" :size="14" />
        <Square v-else :size="12" />
      </button>
      <button @click.stop="handleClose" class="p-1.5 text-slate-500 hover:text-white hover:bg-red-500 rounded transition-colors cursor-pointer">
        <X :size="14" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.mac-traffic-light {
  display: inline-flex;
  height: 12px;
  width: 12px;
  align-items: center;
  justify-content: center;
  border-radius: 999px;
  color: rgba(31, 41, 55, 0.82);
  box-shadow:
    inset 0 0 0 0.5px rgba(0, 0, 0, 0.18),
    0 1px 1px rgba(0, 0, 0, 0.12);
  transition: filter 160ms ease, transform 160ms ease;
}

.mac-traffic-light:hover {
  filter: brightness(1.05);
}

.mac-traffic-light:active {
  transform: scale(0.92);
}
</style>
