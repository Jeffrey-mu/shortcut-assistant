<script setup lang="ts">
import { Minus, Square, X } from 'lucide-vue-next';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { useAppStore } from '../stores/app';

const appWindow = getCurrentWindow();
const appStore = useAppStore();

const handleMinimize = async () => {
  await appWindow.minimize();
};

const handleMaximize = async () => {
  await appWindow.toggleMaximize();
};

const handleClose = async () => {
  await appWindow.close();
};
</script>

<template>
  <div data-tauri-drag-region @dblclick="handleMaximize" class="h-10 flex items-center justify-between px-4 select-none shrink-0 transition-colors duration-300 z-50 border-b border-slate-200/50 dark:border-slate-800/50 cursor-default"
       :class="appStore.isWorkMode ? 'bg-slate-100/50 dark:bg-slate-900/50 backdrop-blur-md' : 'bg-transparent'">
    
    <!-- 左侧标题 -->
    <div data-tauri-drag-region class="flex items-center gap-2 h-full flex-1 pointer-events-none">
      <span class="text-xs font-semibold bg-gradient-to-r from-blue-600 to-purple-600 dark:from-blue-400 dark:to-purple-500 bg-clip-text text-transparent">
        快捷键管理助手
      </span>
    </div>

    <!-- 右侧窗口控制 -->
    <div data-tauri-drag-region="false" class="flex items-center gap-2">
      <button @click.stop="handleMinimize" class="p-1.5 text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-200/50 dark:hover:bg-slate-700/50 rounded transition-colors cursor-pointer">
        <Minus :size="14" />
      </button>
      <button @click.stop="handleMaximize" class="p-1.5 text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-200/50 dark:hover:bg-slate-700/50 rounded transition-colors cursor-pointer">
        <Square :size="12" />
      </button>
      <button @click.stop="handleClose" class="p-1.5 text-slate-500 hover:text-white hover:bg-red-500 rounded transition-colors cursor-pointer">
        <X :size="14" />
      </button>
    </div>
  </div>
</template>
