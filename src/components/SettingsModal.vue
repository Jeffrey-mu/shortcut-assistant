<script setup lang="ts">
import { ref, watch } from 'vue';
import { X, Save, Monitor, ToggleLeft, ToggleRight, Moon, Sun, SlidersHorizontal, Radio, Palette } from 'lucide-vue-next';
import { useShortcutStore, type AppSettings } from '../stores/shortcut';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';
import { themePalettes } from '../data/themePalettes';

const props = defineProps<{
  show: boolean;
  compact?: boolean;
}>();

const emit = defineEmits(['close']);
const store = useShortcutStore();
const activeTab = ref<'general' | 'window' | 'live' | 'appearance'>('general');

const tabs = [
  { id: 'general', label: '常规', icon: Monitor },
  { id: 'window', label: '窗口', icon: SlidersHorizontal },
  { id: 'live', label: '直播', icon: Radio },
  { id: 'appearance', label: '外观', icon: Palette },
] as const;

const localSettings = ref<AppSettings>({
  autoStart: true,
  minimizeToTray: true,
  theme: 'dark',
  transparentWindow: false,
  windowOpacity: 65,
  workOpacity: 86,
  workCardSize: 'medium',
  accentTheme: 'green',
});

// 初始化数据
watch(() => props.show, async (newVal) => {
  if (newVal) {
    activeTab.value = 'general';
    localSettings.value = { ...store.settings };
    try {
      // 从系统实际读取自启状态覆盖
      localSettings.value.autoStart = await isEnabled();
    } catch (e) {
      console.error('获取自启状态失败', e);
    }
  }
});

const handleSave = async () => {
  try {
    if (localSettings.value.autoStart) {
      await enable();
    } else {
      await disable();
    }
  } catch (e) {
    console.error('设置自启失败', e);
  }
  
  store.settings = { ...localSettings.value };
  await store.saveToLocal();
  emit('close');
};
</script>

<template>
  <div
    v-if="show"
    class="fixed inset-0 z-50 flex"
    :class="compact ? 'items-start justify-end p-2' : 'items-center justify-center'"
  >
    <div
      class="absolute inset-0 transition-colors duration-300"
      :class="compact ? 'bg-transparent' : 'bg-slate-900/40 backdrop-blur-sm dark:bg-black/70'"
      @click="emit('close')"
    ></div>
    
    <div
      class="relative flex flex-col overflow-hidden border border-slate-200 bg-[color:var(--light-panel-strong)] shadow-2xl animate-in zoom-in-95 duration-200 transition-colors duration-300 dark:border-white/10 dark:bg-[#090d19] dark:shadow-[0_24px_80px_rgba(0,0,0,0.62)]"
      :class="compact ? 'max-h-[calc(100vh-1rem)] w-[min(92vw,360px)] rounded-xl' : 'max-h-[min(86vh,720px)] w-[min(92vw,520px)] rounded-2xl'"
    >
      <div
        class="shrink-0 border-b border-slate-200 bg-[image:var(--light-header-bg)] transition-colors duration-300 dark:border-white/10 dark:bg-white/[0.035]"
        :class="compact ? 'px-3 py-2.5' : 'px-5 py-4'"
      >
        <div class="flex items-center justify-between gap-3">
          <h2
            class="flex items-center gap-2 font-bold text-slate-800 dark:text-white"
            :class="compact ? 'text-sm' : 'text-lg'"
          >
            <Monitor :size="compact ? 16 : 20" class="text-blue-500 dark:text-blue-400" />
            系统设置
          </h2>
          <button @click="emit('close')" class="rounded-lg p-1 text-slate-500 transition-colors hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-white/[0.075] dark:hover:text-white">
            <X :size="compact ? 17 : 20" />
          </button>
        </div>

        <div
          class="grid grid-cols-4 gap-1 rounded-xl border border-slate-200 bg-[color:var(--light-panel)] p-1 dark:border-white/10 dark:bg-black/24"
          :class="compact ? 'mt-2' : 'mt-4'"
        >
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            class="flex min-w-0 items-center justify-center gap-1.5 rounded-lg px-2 font-semibold transition-all"
            :style="activeTab === tab.id ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent-contrast)' } : undefined"
            :class="[
              compact ? 'py-1.5 text-[11px]' : 'py-2 text-xs',
              activeTab === tab.id ? 'shadow-sm' : 'text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white'
            ]"
            :title="tab.label"
          >
            <component :is="tab.icon" :size="15" class="shrink-0" />
            <span class="truncate">{{ tab.label }}</span>
          </button>
        </div>
      </div>

      <div class="min-h-0 flex-1 overflow-y-auto" :class="compact ? 'p-3' : 'p-5'">
        <div v-if="activeTab === 'general'" class="space-y-3">
          <div class="flex items-center justify-between p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">开机自动启动</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">登录系统时自动在后台运行助手</div>
            </div>
            <button 
              @click="localSettings.autoStart = !localSettings.autoStart"
              class="text-blue-600 dark:text-blue-500 transition-colors"
            >
              <ToggleRight v-if="localSettings.autoStart" :size="32" />
              <ToggleLeft v-else :size="32" class="text-slate-400 dark:text-slate-500" />
            </button>
          </div>

          <div class="flex items-center justify-between p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">关闭时最小化到托盘</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">点击关闭按钮时隐藏到系统托盘而不是退出程序</div>
            </div>
            <button 
              @click="localSettings.minimizeToTray = !localSettings.minimizeToTray"
              class="text-blue-600 dark:text-blue-500 transition-colors"
            >
              <ToggleRight v-if="localSettings.minimizeToTray" :size="32" />
              <ToggleLeft v-else :size="32" class="text-slate-400 dark:text-slate-500" />
            </button>
          </div>
        </div>

        <div v-else-if="activeTab === 'window'" class="space-y-3">
          <div class="flex items-center justify-between p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">开启窗口透明</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">允许窗口背景呈现半透明效果</div>
            </div>
            <button 
              @click="localSettings.transparentWindow = !localSettings.transparentWindow"
              class="text-blue-600 dark:text-blue-500 transition-colors"
            >
              <ToggleRight v-if="localSettings.transparentWindow" :size="32" />
              <ToggleLeft v-else :size="32" class="text-slate-400 dark:text-slate-500" />
            </button>
          </div>

          <div class="p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div class="flex items-center justify-between gap-4">
              <div>
                <div class="text-slate-700 dark:text-slate-200 font-medium">全窗口透明度</div>
                <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">开启窗口透明后应用到整个管理窗口</div>
              </div>
              <span class="text-xs font-mono text-slate-500 dark:text-slate-400 shrink-0">{{ localSettings.windowOpacity }}%</span>
            </div>
            <input
              v-model.number="localSettings.windowOpacity"
              type="range"
              min="0"
              max="95"
              step="1"
              :disabled="!localSettings.transparentWindow"
              :class="!localSettings.transparentWindow ? 'opacity-40' : ''"
              class="mt-3 w-full accent-blue-600"
            />
          </div>
        </div>

        <div v-else-if="activeTab === 'live'" class="space-y-3">
          <div class="p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div class="flex items-center justify-between gap-4">
              <div>
                <div class="text-slate-700 dark:text-slate-200 font-medium">工作模式透明度</div>
                <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">进入工作模式后使用独立透明度</div>
              </div>
              <span class="text-xs font-mono text-slate-500 dark:text-slate-400 shrink-0">{{ localSettings.workOpacity }}%</span>
            </div>
            <input
              v-model.number="localSettings.workOpacity"
              type="range"
              min="0"
              max="96"
              step="1"
              class="mt-3 w-full accent-blue-600"
            />
          </div>

          <div class="flex items-center justify-between p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">工作卡片尺寸</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">控制工作模式按钮密度</div>
            </div>
            <div class="flex bg-[color:var(--light-panel-hover)] dark:bg-black/24 rounded-lg p-1 border border-slate-300 dark:border-white/10 transition-colors duration-300">
              <button
                v-for="size in ['small', 'medium', 'large']"
                :key="size"
                @click="localSettings.workCardSize = size as AppSettings['workCardSize']"
                class="px-2.5 py-1.5 rounded-md text-xs transition-colors"
                :class="localSettings.workCardSize === size ? 'bg-[color:var(--light-panel-strong)] dark:bg-white/[0.10] text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'"
              >
                {{ size === 'small' ? '小' : size === 'medium' ? '中' : '大' }}
              </button>
            </div>
          </div>
        </div>

        <div v-else class="space-y-3">
          <div class="flex items-center justify-between p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">应用主题</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">亮色/暗色模式切换</div>
            </div>
            <div class="flex bg-[color:var(--light-panel-hover)] dark:bg-black/24 rounded-lg p-1 border border-slate-300 dark:border-white/10 transition-colors duration-300">
              <button 
                @click="localSettings.theme = 'light'"
                class="p-1.5 rounded-md transition-colors"
                :class="localSettings.theme === 'light' ? 'bg-[color:var(--light-panel-strong)] dark:bg-white/[0.10] text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'"
              >
                <Sun :size="16" />
              </button>
              <button 
                @click="localSettings.theme = 'dark'"
                class="p-1.5 rounded-md transition-colors"
                :class="localSettings.theme === 'dark' ? 'bg-[color:var(--light-panel-strong)] dark:bg-white/[0.10] text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'"
              >
                <Moon :size="16" />
              </button>
            </div>
          </div>

          <div class="p-3 rounded-xl bg-[color:var(--light-panel)] dark:bg-white/[0.045] border border-slate-200 dark:border-white/10 transition-colors duration-300">
            <div class="mb-3">
              <div class="text-slate-700 dark:text-slate-200 font-medium">配色方案</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">用于工作模式、按钮和状态反馈</div>
            </div>
            <div class="grid grid-cols-5 gap-2">
              <button
                v-for="palette in themePalettes"
                :key="palette.id"
                @click="localSettings.accentTheme = palette.id"
                class="h-14 rounded-lg border transition-all flex flex-col items-center justify-center gap-1"
                :class="localSettings.accentTheme === palette.id ? 'scale-[1.03] shadow-lg' : 'opacity-80 hover:opacity-100'"
                :style="{
                  background: localSettings.theme === 'dark'
                    ? `linear-gradient(135deg, ${palette.soft}, rgba(15, 23, 42, 0.16))`
                    : `linear-gradient(135deg, color-mix(in srgb, ${palette.accent} 13%, #f8fafc), color-mix(in srgb, ${palette.accent2} 9%, #e2e8f0))`,
                  borderColor: localSettings.accentTheme === palette.id ? palette.accent : palette.border,
                  color: localSettings.theme === 'dark' ? palette.text : `color-mix(in srgb, ${palette.accent} 70%, #0f172a)`,
                  boxShadow: localSettings.accentTheme === palette.id ? `0 8px 24px ${palette.soft}` : 'none'
                }"
                :title="palette.name"
              >
                <span class="flex gap-1">
                  <span class="w-3 h-3 rounded-full" :style="{ backgroundColor: palette.accent }"></span>
                  <span class="w-3 h-3 rounded-full" :style="{ backgroundColor: palette.accent2 }"></span>
                </span>
                <span class="text-[10px] font-medium truncate max-w-full px-1">{{ palette.name }}</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      <div
        class="shrink-0 border-t border-slate-200 bg-[image:var(--light-header-bg)] shadow-[0_-12px_28px_rgba(15,23,42,0.06)] transition-colors duration-300 dark:border-white/10 dark:bg-white/[0.035] dark:shadow-[0_-12px_28px_rgba(0,0,0,0.28)] flex justify-end gap-3"
        :class="compact ? 'px-3 py-2.5' : 'px-5 py-4'"
      >
        <button 
          @click="emit('close')"
          class="px-4 py-2 rounded-lg text-sm font-medium text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-white/[0.075] transition-colors"
        >
          取消
        </button>
        <button 
          @click="handleSave"
          class="px-6 py-2 rounded-lg text-sm font-semibold text-slate-950 shadow-lg flex items-center gap-2 transition-all active:scale-95"
          :style="{ background: 'linear-gradient(135deg, var(--accent), var(--accent-2))', boxShadow: '0 10px 24px var(--accent-soft)' }"
        >
          <Save :size="16" />
          保存设置
        </button>
      </div>
    </div>
  </div>
</template>
