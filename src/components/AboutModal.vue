<script setup lang="ts">
import { X, Info, ExternalLink } from 'lucide-vue-next';
import { open } from '@tauri-apps/plugin-shell';

defineProps<{
  show: boolean;
}>();

const emit = defineEmits(['close']);

const openLink = async (url: string) => {
  try {
    await open(url);
  } catch (e) {
    console.error('Failed to open link:', e);
  }
};
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="absolute inset-0 bg-slate-900/40 dark:bg-slate-950/80 backdrop-blur-sm transition-colors duration-300" @click="emit('close')"></div>
    
    <div class="relative bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden animate-in zoom-in-95 duration-200 transition-colors duration-300">
      <div class="px-6 py-4 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between bg-slate-50 dark:bg-slate-900/50 transition-colors duration-300">
        <h2 class="text-xl font-bold text-slate-800 dark:text-white flex items-center gap-2">
          <Info :size="20" class="text-blue-500 dark:text-blue-400" />
          关于软件
        </h2>
        <button @click="emit('close')" class="text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition-colors p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800">
          <X :size="20" />
        </button>
      </div>

      <div class="p-8 flex flex-col items-center text-center">
        <div class="w-20 h-20 bg-gradient-to-tr from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-lg shadow-blue-500/20 mb-6 rotate-3">
          <span class="text-3xl font-bold text-white italic">-S-</span>
        </div>
        
        <h3 class="text-2xl font-bold text-slate-800 dark:text-white mb-2">快捷键管理助手</h3>
        <p class="text-slate-500 dark:text-slate-400 text-sm mb-6">版本 v1.0.0 (Beta)</p>
        
        <p class="text-slate-600 dark:text-slate-300 text-sm leading-relaxed mb-8 max-w-xs">
          一款现代化的桌面快捷键统一管理工具。支持记录、集中管理并以多种模式（定时、延时、循环）将虚拟按键发送到目标窗口，大幅提升日常工作与操作效率。
        </p>

        <div class="flex flex-col w-full gap-3">
          <button 
            @click="openLink('https://tauri.app/')"
            class="flex items-center justify-center gap-2 w-full py-2.5 px-4 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-300 rounded-xl transition-colors text-sm"
          >
            <ExternalLink :size="16" />
            基于 Tauri 2.0 构建
          </button>
        </div>
      </div>

      <div class="px-6 py-4 bg-slate-50 dark:bg-slate-800/30 border-t border-slate-200 dark:border-slate-800 text-center transition-colors duration-300">
        <p class="text-xs text-slate-400 dark:text-slate-500">&copy; 2026 Jeffrey. All rights reserved.</p>
      </div>
    </div>
  </div>
</template>
