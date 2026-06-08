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
    <div class="absolute inset-0 bg-slate-950/55 backdrop-blur-md transition-colors duration-300 dark:bg-black/76" @click="emit('close')"></div>
    
    <div class="modal-shell relative w-full max-w-md overflow-hidden rounded-2xl animate-in zoom-in-95 duration-200 transition-colors duration-300">
      <div class="tool-header flex items-center justify-between px-6 py-4 transition-colors duration-300">
        <h2 class="text-xl font-bold text-slate-800 dark:text-white flex items-center gap-2">
          <Info :size="20" class="text-blue-500 dark:text-blue-400" />
          关于软件
        </h2>
        <button @click="emit('close')" class="text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition-colors p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-white/[0.075]">
          <X :size="20" />
        </button>
      </div>

      <div class="flex flex-col items-center p-8 text-center">
        <div class="mb-6 flex h-24 w-24 rotate-3 items-center justify-center rounded-2xl border border-white/40 text-slate-950 shadow-[0_22px_48px_color-mix(in_srgb,var(--accent)_24%,transparent)] dark:border-white/10"
             :style="{ background: 'linear-gradient(135deg, var(--accent), var(--accent-2))' }">
          <span class="text-3xl font-black italic tracking-[-0.08em]">SA</span>
        </div>
        
        <h3 class="text-2xl font-bold text-slate-800 dark:text-white mb-2">快捷键管理助手</h3>
        <p class="text-slate-500 dark:text-slate-400 text-sm mb-6">版本 v1.0.0 (Beta)</p>
        
        <p class="text-slate-600 dark:text-slate-300 text-sm leading-relaxed mb-8 max-w-xs">
          一款现代化的桌面快捷键统一管理工具。支持记录、集中管理并以多种模式（定时、延时、循环）将虚拟按键发送到目标窗口，大幅提升日常工作与操作效率。
        </p>

        <div class="flex flex-col w-full gap-3">
          <button 
            @click="openLink('https://tauri.app/')"
            class="flex items-center justify-center gap-2 w-full py-2.5 px-4 bg-[color:var(--light-panel)] dark:bg-white/[0.055] hover:bg-[color:var(--light-panel-hover)] dark:hover:bg-white/[0.085] text-slate-700 dark:text-slate-300 rounded-xl transition-colors text-sm"
          >
            <ExternalLink :size="16" />
            基于 Tauri 2.0 构建
          </button>
        </div>
      </div>

      <div class="tool-header border-t px-6 py-4 text-center transition-colors duration-300">
        <p class="text-xs text-slate-400 dark:text-slate-500">&copy; 2026 Jeffrey. All rights reserved.</p>
      </div>
    </div>
  </div>
</template>
