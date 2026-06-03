<script setup lang="ts">
import { computed, ref, onMounted, onUnmounted } from 'vue';
import { 
  Play, 
  Edit2, 
  Trash2, 
  Clock, 
  Timer, 
  Repeat,
  MoreVertical,
  Check,
  CircleCheck,
  Loader2,
  XCircle
} from 'lucide-vue-next';
import type { Shortcut } from '../stores/shortcut';
import { useShortcutStore } from '../stores/shortcut';
import { getShortcutIcon } from '../data/iconOptions';

const props = defineProps<{
  shortcut: Shortcut;
  selected?: boolean;
  batchMode?: boolean;
  workMode?: boolean;
  animationIndex?: number;
  feedbackStatus?: 'running' | 'success' | 'error';
}>();

defineEmits(['trigger', 'edit', 'delete', 'toggle', 'select']);

const store = useShortcutStore();

const showMenu = ref(false);
const menuRef = ref<HTMLElement | null>(null);
const isDarkTheme = computed(() => store.settings.theme === 'dark');
const readableShortcutColor = computed(() => {
  return isDarkTheme.value
    ? props.shortcut.color
    : `color-mix(in srgb, ${props.shortcut.color} 72%, #0f172a)`;
});
const mainCardStyle = computed(() => {
  const color = props.shortcut.color;
  if (props.workMode) {
    return {
      background: isDarkTheme.value
        ? `linear-gradient(180deg, rgba(18, 24, 38, 0.92), rgba(7, 10, 18, 0.94))`
        : `linear-gradient(180deg, color-mix(in srgb, ${color} 14%, rgba(224, 242, 254, 0.76)), color-mix(in srgb, ${color} 18%, rgba(236, 253, 245, 0.66)))`,
      borderColor: isDarkTheme.value
        ? `color-mix(in srgb, ${color} 36%, rgba(255, 255, 255, 0.16))`
        : `color-mix(in srgb, ${color} 34%, rgba(15, 23, 42, 0.14))`,
    };
  }

  const transparent = store.settings.transparentWindow;
  if (isDarkTheme.value) {
    return {
      backgroundColor: transparent ? 'rgba(5, 9, 18, 0.78)' : 'rgba(7, 10, 18, 0.96)',
      borderColor: props.selected ? `color-mix(in srgb, ${color} 100%, transparent)` : `color-mix(in srgb, ${color} 16%, rgba(148, 163, 184, 0.22))`,
    };
  }

  return {
    backgroundColor: transparent
      ? `color-mix(in srgb, ${color} 5%, rgba(248, 250, 252, 0.72))`
      : `color-mix(in srgb, ${color} 5%, rgba(248, 250, 252, 0.96))`,
    borderColor: props.selected ? `color-mix(in srgb, ${color} 82%, transparent)` : `color-mix(in srgb, ${color} 24%, rgba(203, 213, 225, 0.82))`,
  };
});

const cardShadow = computed(() => {
  const color = props.shortcut.color;
  if (props.feedbackStatus && props.workMode) return isDarkTheme.value ? 'inset 0 1px 0 rgba(255, 255, 255, 0.16)' : 'inset 0 1px 0 rgba(255, 255, 255, 0.92)';
  if (props.feedbackStatus === 'success') return `0 0 0 1px rgba(52, 211, 153, 0.45), 0 12px 32px rgba(52, 211, 153, 0.18)`;
  if (props.feedbackStatus === 'error') return `0 0 0 1px rgba(248, 113, 113, 0.45), 0 12px 32px rgba(248, 113, 113, 0.18)`;
  if (props.feedbackStatus === 'running') return `0 0 0 1px rgba(96, 165, 250, 0.45), 0 12px 32px rgba(96, 165, 250, 0.18)`;
  if (props.workMode) {
    return isDarkTheme.value
      ? `0 10px 24px rgba(0, 0, 0, 0.32), 0 0 0 1px rgba(255,255,255,0.035), inset 0 1px 0 rgba(255, 255, 255, 0.16)`
      : `0 10px 22px rgba(15, 23, 42, 0.10), 0 0 0 1px rgba(255,255,255,0.82), inset 0 1px 0 rgba(255, 255, 255, 0.92)`;
  }
  if (props.selected) return `0 8px 30px ${color}30`;
  return isDarkTheme.value
    ? '0 18px 42px rgba(0, 0, 0, 0.38), inset 0 1px 0 rgba(255,255,255,0.055)'
    : '0 16px 34px rgba(15, 23, 42, 0.08), 0 1px 0 rgba(255,255,255,0.92) inset';
});

const workCardUi = computed(() => {
  const size = store.settings.workCardSize || 'medium';
  return {
    icon: size === 'small' ? 'h-[18px] w-[18px]' : size === 'large' ? 'h-[24px] w-[24px]' : 'h-[21px] w-[21px]',
    iconWrap: size === 'small' ? 'h-[28px] w-[28px]' : size === 'large' ? 'h-[36px] w-[36px]' : 'h-[32px] w-[32px]',
    title: size === 'small' ? 'text-[12px]' : size === 'large' ? 'text-[14px]' : 'text-[13px]',
    padding: size === 'small' ? 'p-1.5' : size === 'large' ? 'p-2.5' : 'p-2',
  };
});

const workCardSizeStyle = computed(() => {
  if (!props.workMode) return {};
  const size = store.settings.workCardSize || 'medium';
  const length = Array.from(props.shortcut.name || '').reduce((total, char) => {
    return total + (/[\u4e00-\u9fff]/.test(char) ? 1.15 : 0.62);
  }, 0);
  const config = {
    small: { min: 68, max: 126, unit: 5.2 },
    medium: { min: 76, max: 148, unit: 5.8 },
    large: { min: 86, max: 172, unit: 6.4 },
  } as const;
  const current = config[size];
  const width = Math.round(Math.min(current.max, Math.max(current.min, current.min + Math.max(0, length - 4) * current.unit)));
  return {
    width: `${width}px`,
    minWidth: `${current.min}px`,
    maxWidth: `${current.max}px`,
    height: 'var(--work-card-height)',
  };
});

const toggleMenu = (e: Event) => {
  e.stopPropagation();
  showMenu.value = !showMenu.value;
};

const closeMenu = () => {
  showMenu.value = false;
};

// 点击外部关闭菜单
const handleClickOutside = (e: MouseEvent) => {
  if (menuRef.value && !menuRef.value.contains(e.target as Node)) {
    closeMenu();
  }
};

onMounted(() => {
  window.addEventListener('click', handleClickOutside);
});

onUnmounted(() => {
  window.removeEventListener('click', handleClickOutside);
});
</script>

<template>
  <div 
    class="border overflow-hidden transition-all duration-300 group relative flex flex-col h-full select-none"
    :class="{ 
      'opacity-50 grayscale-[30%]': !shortcut.enabled && !batchMode,
      'ring-2 ring-offset-2 ring-offset-white dark:ring-offset-slate-950 scale-[1.02] z-10': selected,
      'ring-2 ring-blue-400/50': feedbackStatus === 'running' && !workMode,
      'ring-2 ring-emerald-400/60': feedbackStatus === 'success' && !workMode,
      'ring-2 ring-red-400/60': feedbackStatus === 'error' && !workMode,
      'work-card-enter rounded-xl cursor-pointer backdrop-blur-xl hover:-translate-y-0.5 active:translate-y-0 active:scale-[0.98]': workMode,
      'main-action-card rounded-xl cursor-grab active:cursor-grabbing hover:-translate-y-0.5': !batchMode && !workMode,
      'cursor-pointer hover:border-blue-500': batchMode,
      'no-drag': workMode, // 在工作模式下禁用拖拽
      'backdrop-blur-xl': store.settings.transparentWindow && !workMode
    }"
    :style="{ 
      ...mainCardStyle,
      ...workCardSizeStyle,
      animationDelay: workMode ? `${Math.min(animationIndex || 0, 10) * 28}ms` : undefined,
      boxShadow: cardShadow
    }"
    @click="batchMode ? $emit('select', shortcut.id) : (workMode ? $emit('trigger', shortcut) : null)"
  >
    <!-- 背景渐变发光效果 (工作模式下弱化) -->
    <div 
      class="absolute -top-24 -right-24 w-48 h-48 rounded-full blur-[50px] transition-opacity"
      :class="workMode ? 'opacity-0' : 'opacity-10 dark:opacity-14 group-hover:opacity-20 dark:group-hover:opacity-28'"
      :style="{ backgroundColor: shortcut.color }"
    ></div>
    <div
      v-if="workMode"
      class="pointer-events-none absolute inset-x-1 top-1 h-1/2 rounded-t-lg bg-gradient-to-b from-white/16 to-transparent opacity-80 transition-opacity group-hover:opacity-100"
    ></div>
    <div
      v-if="workMode"
      class="pointer-events-none absolute inset-x-2 top-1 h-px rounded-full opacity-90"
      :style="{ background: `linear-gradient(90deg, transparent, color-mix(in srgb, ${shortcut.color} 72%, white), transparent)` }"
    ></div>
    <div
      v-if="workMode"
      class="pointer-events-none absolute inset-x-0 bottom-0 h-1 rounded-b-xl opacity-90"
      :style="{ background: `linear-gradient(90deg, transparent, ${shortcut.color}, transparent)` }"
    ></div>
    <div
      v-if="workMode"
      class="pointer-events-none absolute inset-0 rounded-xl opacity-70"
      :style="{ boxShadow: `inset 0 0 0 1px color-mix(in srgb, ${shortcut.color} 18%, transparent)` }"
    ></div>
    <div
      v-if="feedbackStatus"
      class="absolute right-1 top-1 z-20 flex items-center justify-center rounded-full border backdrop-blur"
      :class="[
        workMode ? 'h-4 w-4' : 'h-5 w-5',
        feedbackStatus === 'running' ? 'border-blue-300/50 bg-blue-500/20 text-blue-200' : '',
        feedbackStatus === 'success' ? 'border-emerald-300/50 bg-emerald-500/20 text-emerald-200' : '',
        feedbackStatus === 'error' ? 'border-red-300/50 bg-red-500/20 text-red-200' : ''
      ]"
      :title="feedbackStatus === 'running' ? '发送中' : feedbackStatus === 'success' ? '发送成功' : '发送失败'"
    >
      <Loader2 v-if="feedbackStatus === 'running'" :size="workMode ? 10 : 12" class="animate-spin" />
      <Check v-else-if="feedbackStatus === 'success'" :size="workMode ? 10 : 12" />
      <XCircle v-else :size="workMode ? 10 : 12" />
    </div>
    <!-- 复选框 (仅在批量模式显示) -->
    <div v-if="batchMode" class="absolute top-3 left-3 z-10">
      <div 
        class="w-5 h-5 rounded border flex items-center justify-center transition-all bg-white/50 dark:bg-transparent"
        :style="{ 
          backgroundColor: selected ? shortcut.color : undefined,
          borderColor: selected ? shortcut.color : `${shortcut.color}60`
        }"
      >
        <Check v-if="selected" :size="14" class="text-white" />
      </div>
    </div>

    <template v-if="!workMode">
      <div
        class="pointer-events-none absolute inset-0 opacity-60"
        :style="{ background: `radial-gradient(circle at 18% 0%, color-mix(in srgb, ${shortcut.color} 16%, transparent), transparent 42%)` }"
      ></div>
      <div class="relative z-10 flex h-full flex-col p-4">
        <div class="flex items-start gap-3">
          <div
            class="flex h-11 w-11 shrink-0 items-center justify-center rounded-lg border"
            :style="{
              backgroundColor: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 18%, rgba(2, 6, 23, 0.9))` : `color-mix(in srgb, ${shortcut.color} 13%, #f1f5f9)`,
              borderColor: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 24%, rgba(148, 163, 184, 0.2))` : `color-mix(in srgb, ${shortcut.color} 34%, rgba(203, 213, 225, 0.78))`,
              color: readableShortcutColor
            }"
          >
            <component :is="getShortcutIcon(shortcut.icon)" class="h-[22px] w-[22px]" />
          </div>

          <div class="min-w-0 flex-1">
            <div class="flex items-center gap-2">
              <span
                class="h-1.5 w-1.5 shrink-0 rounded-full shadow-[0_0_10px_currentColor]"
                :class="shortcut.enabled ? '' : 'opacity-40 grayscale'"
                :style="{ color: shortcut.enabled ? shortcut.color : undefined, backgroundColor: shortcut.enabled ? shortcut.color : '#94a3b8' }"
              ></span>
              <span class="rounded-md px-1.5 py-0.5 text-[10px] font-medium text-slate-600 dark:bg-white/[0.055] dark:text-slate-300"
                    :style="!isDarkTheme ? { backgroundColor: `color-mix(in srgb, ${shortcut.color} 10%, rgba(248, 250, 252, 0.95))` } : undefined">
                {{ shortcut.enabled ? '已启用' : '已停用' }}
              </span>
            </div>
            <h3 class="mt-2 truncate text-lg font-bold leading-tight text-slate-900 dark:text-slate-100">
              {{ shortcut.name }}
            </h3>
          </div>

          <button
            @click.stop="$emit('trigger', shortcut)"
            class="flex h-10 w-10 shrink-0 items-center justify-center rounded-full border bg-[color:var(--light-panel)] text-slate-600 shadow-sm transition-all duration-200 hover:-translate-y-0.5 hover:bg-[color:var(--light-panel-hover)] hover:text-slate-950 active:translate-y-0 active:scale-95 dark:bg-white/[0.055] dark:text-slate-300 dark:hover:bg-white/[0.09] dark:hover:text-white"
            :style="{
              borderColor: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 18%, rgba(148, 163, 184, 0.24))` : `color-mix(in srgb, ${shortcut.color} 18%, rgba(51, 65, 85, 0.8))`
            }"
            title="立即触发"
          >
            <Play :size="18" fill="currentColor" class="translate-x-[1px]" />
          </button>

          <button
            @click.stop="$emit('edit', shortcut)"
            class="flex h-10 w-8 items-center justify-center rounded-lg text-slate-400 transition-colors hover:bg-slate-100 hover:text-slate-800 active:scale-95 dark:hover:bg-white/[0.075] dark:hover:text-white"
            title="编辑动作"
          >
            <Edit2 :size="16" />
          </button>
          <button
            @click.stop="$emit('delete', shortcut.id)"
            class="flex h-10 w-8 items-center justify-center rounded-lg text-slate-400 transition-colors hover:bg-red-50 hover:text-red-600 active:scale-95 dark:hover:bg-red-500/10 dark:hover:text-red-300"
            title="删除动作"
          >
            <Trash2 :size="16" />
          </button>
        </div>

        <div class="mt-auto flex items-center justify-between gap-3 border-t border-slate-200/80 pt-3 dark:border-white/10">
          <div class="min-w-0">
            <span
              class="inline-flex max-w-full items-center rounded-md border px-2 py-1 font-mono text-[11px] font-semibold"
              :style="{
                backgroundColor: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 12%, rgba(2, 6, 23, 0.82))` : `color-mix(in srgb, ${shortcut.color} 9%, rgba(248, 250, 252, 0.95))`,
                borderColor: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 22%, rgba(148, 163, 184, 0.2))` : `color-mix(in srgb, ${shortcut.color} 28%, rgba(203, 213, 225, 0.9))`,
                color: readableShortcutColor
              }"
              :title="shortcut.target.path"
            >
              <span class="truncate">{{ shortcut.target.path }}</span>
            </span>
          </div>
          <div
            class="relative h-7 w-12 shrink-0 cursor-pointer rounded-full border p-0.5 transition-all"
            :class="!shortcut.enabled ? 'bg-slate-200 dark:bg-slate-800' : ''"
            :style="{
              backgroundColor: shortcut.enabled ? shortcut.color : undefined,
              borderColor: shortcut.enabled ? shortcut.color : (isDarkTheme ? 'rgba(51, 65, 85, 0.9)' : 'rgba(203, 213, 225, 0.95)')
            }"
            @click.stop="$emit('toggle', shortcut)"
          >
            <div
              class="absolute top-0.5 h-5 w-5 rounded-full bg-white shadow transition-transform duration-300"
              :class="{ 'translate-x-5': shortcut.enabled }"
              :style="shortcut.enabled ? { backgroundColor: '#ffffff' } : undefined"
            ></div>
          </div>
        </div>
      </div>
    </template>

    <div v-else class="flex flex-col flex-1 relative z-10 transition-all duration-300" :class="workCardUi.padding">
      <div :class="[workMode ? 'mb-0 flex-col items-center justify-center gap-0' : 'mb-4 items-start flex-row justify-between', 'flex h-full w-full']">
        <div class="flex-1 min-w-0" :class="workMode ? 'flex flex-col items-center justify-center w-full h-full' : 'mr-2'">
          <div 
            class="flex min-w-0 transition-all duration-300"
            :class="workMode ? 'flex-col items-center justify-center gap-1.5 w-full' : 'items-center gap-2 w-full'"
          >
            <div
              class="shrink-0 transition-all duration-200 group-hover:scale-105"
              :class="workMode ? `flex items-center justify-center rounded-lg border ${workCardUi.iconWrap}` : 'flex h-9 w-9 items-center justify-center rounded-lg border bg-slate-50 shadow-sm dark:bg-slate-950/70'"
              :style="workMode ? {
                background: isDarkTheme ? `linear-gradient(180deg, color-mix(in srgb, ${shortcut.color} 24%, rgba(255,255,255,0.08)), rgba(255,255,255,0.045))` : `linear-gradient(180deg, color-mix(in srgb, ${shortcut.color} 12%, #f1f5f9), color-mix(in srgb, ${shortcut.color} 5%, rgba(226,232,240,0.72)))`,
                borderColor: `color-mix(in srgb, ${shortcut.color} ${isDarkTheme ? 34 : 30}%, transparent)`,
                boxShadow: isDarkTheme ? `inset 0 1px 0 rgba(255,255,255,0.12)` : `inset 0 1px 0 rgba(255,255,255,0.86)`
              } : {
                borderColor: `color-mix(in srgb, ${shortcut.color} 26%, transparent)`,
                color: readableShortcutColor,
                boxShadow: `inset 0 1px 0 rgba(255,255,255,0.08), 0 8px 18px ${shortcut.color}12`
              }"
            >
              <component 
                :is="getShortcutIcon(shortcut.icon)" 
                class="shrink-0"
                :class="workMode ? workCardUi.icon : 'h-5 w-5'"
                :style="workMode ? {
                  color: isDarkTheme ? `color-mix(in srgb, ${shortcut.color} 82%, #ffffff)` : `color-mix(in srgb, ${shortcut.color} 72%, #0f172a)`,
                  filter: isDarkTheme ? 'drop-shadow(0 1px 1px rgba(0,0,0,0.5))' : 'drop-shadow(0 1px 0 rgba(255,255,255,0.8))'
                } : { color: readableShortcutColor }"
              />
            </div>
            <h3 
              class="font-bold transition-all duration-300 min-w-0" 
              :class="workMode ? `${workCardUi.title} line-clamp-2 leading-[1.12] text-center w-full break-words` : 'text-base flex-1 truncate leading-tight text-slate-800 dark:text-slate-100'"
              :style="workMode ? {
                color: isDarkTheme ? '#f8fafc' : '#0f172a',
                textShadow: isDarkTheme ? '0 1px 1px rgba(0,0,0,0.62)' : '0 1px 0 rgba(255,255,255,0.72)'
              } : undefined"
              :title="workMode ? shortcut.name : ''"
            >
              {{ shortcut.name }}
            </h3>
          </div>
          <div v-if="!workMode || shortcut.trigger.type !== 'instant'" class="flex items-center justify-center gap-1 w-full" :class="workMode ? 'mt-0' : 'mt-1.5'">
            <span v-if="!workMode" class="ml-11 max-w-[calc(100%-2.75rem)] truncate rounded-md border px-1.5 py-0.5 text-[10px] font-medium transition-all"
              :class="workMode ? 'opacity-60' : ''"
              :style="{ 
                backgroundColor: `color-mix(in srgb, ${shortcut.color} 10%, transparent)`,
                borderColor: `color-mix(in srgb, ${shortcut.color} 22%, transparent)`,
                color: readableShortcutColor
              }"
              :title="shortcut.target.path"
            >
              {{ shortcut.target.path }}
            </span>
            <div v-if="shortcut.trigger.type !== 'instant' && workMode" 
                 class="flex items-center justify-center shrink-0 rounded-full border p-0.5"
                 :style="{
                   color: isDarkTheme ? '#f8fafc' : `color-mix(in srgb, ${shortcut.color} 76%, #0f172a)`,
                   backgroundColor: `color-mix(in srgb, ${shortcut.color} ${isDarkTheme ? 20 : 14}%, transparent)`,
                   borderColor: `color-mix(in srgb, ${shortcut.color} 38%, transparent)`
                 }"
                 :title="shortcut.trigger.type === 'timing' ? '定时' : shortcut.trigger.type === 'delay' ? '延时' : '循环'">
              <Clock v-if="shortcut.trigger.type === 'timing'" :size="8" />
              <Timer v-if="shortcut.trigger.type === 'delay'" :size="8" />
              <Repeat v-if="shortcut.trigger.type === 'interval'" :size="8" />
            </div>
            <span v-else-if="shortcut.trigger.type !== 'instant' && !workMode" class="flex items-center gap-1 text-slate-500 dark:text-slate-400 text-xs shrink-0">
              <Clock v-if="shortcut.trigger.type === 'timing'" :size="12" />
              <Timer v-if="shortcut.trigger.type === 'delay'" :size="12" />
              <Repeat v-if="shortcut.trigger.type === 'interval'" :size="12" />
              {{ shortcut.trigger.type === 'timing' ? '定时' : shortcut.trigger.type === 'delay' ? '延时' : '循环' }}
            </span>
          </div>
        </div>
        
        <div v-if="!workMode" class="flex items-center gap-1.5 shrink-0">
          <button 
            @click.stop="$emit('trigger', shortcut)"
            class="flex h-10 w-10 items-center justify-center rounded-lg border transition-all duration-200 hover:-translate-y-0.5 active:translate-y-0 active:scale-95"
            :class="store.settings.transparentWindow ? 'bg-[color:var(--light-panel)]/35 dark:bg-slate-950/25' : 'bg-[color:var(--light-panel-strong)] dark:bg-slate-950/50'"
            :style="{ 
              borderColor: `color-mix(in srgb, ${shortcut.color} 26%, transparent)`,
              color: readableShortcutColor,
              boxShadow: `0 8px 18px ${shortcut.color}12`
            }"
            title="立即触发"
          >
            <Play :size="17" fill="currentColor" class="translate-x-[1px]" />
          </button>
          <div class="relative" ref="menuRef">
            <button 
              @click.stop="toggleMenu"
              class="flex h-10 w-9 items-center justify-center rounded-lg text-slate-400 transition-all duration-200 active:scale-95"
              :class="[
                showMenu ? 'text-slate-700 dark:text-white shadow-inner' : 'hover:text-slate-700 dark:hover:text-white',
                store.settings.transparentWindow ? (showMenu ? 'bg-black/5 dark:bg-white/10' : 'hover:bg-black/5 dark:hover:bg-white/10') : (showMenu ? 'bg-slate-100 dark:bg-slate-700/50' : 'hover:bg-slate-100 dark:hover:bg-slate-700/50')
              ]"
            >
              <MoreVertical :size="18" />
            </button>
            <!-- 弹出菜单 -->
            <div 
              v-if="showMenu"
              class="absolute right-0 top-full mt-2 backdrop-blur-xl border rounded-xl shadow-xl dark:shadow-2xl z-20 w-36 p-1.5 animate-in fade-in zoom-in-95 duration-200 origin-top-right"
              :class="store.settings.transparentWindow ? 'bg-[color:var(--light-panel)]/80 dark:bg-slate-800/80 border-slate-200/50 dark:border-slate-700/30' : 'bg-[color:var(--light-panel-strong)] dark:bg-slate-800/95 border-slate-200 dark:border-slate-700/50'"
            >
              <button 
                @click.stop="$emit('edit', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-700 dark:text-slate-300 transition-colors rounded-md"
                :class="store.settings.transparentWindow ? 'hover:bg-black/5 dark:hover:bg-white/10 hover:text-slate-900 dark:hover:text-white' : 'hover:bg-slate-100 dark:hover:bg-slate-700/50 hover:text-slate-900 dark:hover:text-white'"
              >
                <Edit2 :size="14" /> 编辑动作
              </button>
              <button 
                @click.stop="$emit('toggle', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-700 dark:text-slate-300 transition-colors rounded-md"
                :class="store.settings.transparentWindow ? 'hover:bg-black/5 dark:hover:bg-white/10 hover:text-slate-900 dark:hover:text-white' : 'hover:bg-slate-100 dark:hover:bg-slate-700/50 hover:text-slate-900 dark:hover:text-white'"
              >
                <CircleCheck :size="14" /> {{ shortcut.enabled ? '禁用动作' : '启用动作' }}
              </button>
              <div class="h-px my-1 mx-2" :class="store.settings.transparentWindow ? 'bg-slate-200/50 dark:bg-slate-700/30' : 'bg-slate-200 dark:bg-slate-700/50'"></div>
              <button 
                @click.stop="$emit('delete', shortcut.id); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-red-500 dark:text-red-400 transition-colors rounded-md"
                :class="store.settings.transparentWindow ? 'hover:bg-red-500/10 dark:hover:bg-red-500/20 hover:text-red-600 dark:hover:text-red-300' : 'hover:bg-red-50 dark:hover:bg-red-500/10 hover:text-red-600 dark:hover:text-red-300'"
              >
                <Trash2 :size="14" /> 删除动作
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="!workMode" class="mt-auto pt-4 border-t flex items-center justify-between transition-colors"
           :class="store.settings.transparentWindow ? 'border-slate-200/40 dark:border-slate-700/40 group-hover:border-slate-300/60 dark:group-hover:border-slate-600/60' : 'border-slate-200 dark:border-slate-700/50 group-hover:border-slate-300 dark:group-hover:border-slate-600/50'">
        <span class="text-[10px] text-slate-400 dark:text-slate-500 font-bold uppercase tracking-widest transition-colors group-hover:text-slate-500 dark:group-hover:text-slate-400">快捷操作</span>
        <div class="flex items-center gap-2">
           <div 
            class="w-10 h-5 rounded-full relative transition-all duration-300 cursor-pointer shadow-inner"
            :class="!shortcut.enabled ? 'bg-slate-300 dark:bg-slate-700' : ''"
            :style="{ 
              backgroundColor: shortcut.enabled ? shortcut.color : undefined,
              boxShadow: shortcut.enabled ? `0 0 10px ${shortcut.color}40 inset` : 'none'
            }"
            @click.stop="$emit('toggle', shortcut)"
          >
            <div 
              class="absolute top-1 left-1 w-3 h-3 bg-white rounded-full transition-transform duration-300 shadow-sm"
              :class="{ 'translate-x-5': shortcut.enabled }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
@keyframes work-card-float-in {
  from {
    opacity: 0;
    transform: translateY(8px) scale(0.94);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.work-card-enter {
  animation: work-card-float-in 260ms cubic-bezier(0.2, 0.85, 0.25, 1.18) both;
}
</style>
