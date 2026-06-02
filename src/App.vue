<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from "vue";
import Titlebar from "./components/Titlebar.vue";
import ShortcutCard from "./components/ShortcutCard.vue";
import AddShortcutModal from "./components/AddShortcutModal.vue";
import SettingsModal from "./components/SettingsModal.vue";
import AboutModal from "./components/AboutModal.vue";
import { useShortcutStore, type Shortcut } from "./stores/shortcut";
import { useAppStore } from "./stores/app";
import { ShortcutManager } from "./services/shortcutManager";
import { CheckCircle2, CircleCheck, CircleX, Download, Grid, Info, List as ListIcon, Loader2, Maximize2, Minimize2, Palette, Pin, Plus, Radio, Search, Settings, Upload, XCircle } from "lucide-vue-next";
import Sortable from "sortablejs";
import { save, open } from "@tauri-apps/plugin-dialog";
import { writeFile, readTextFile } from "@tauri-apps/plugin-fs";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { getThemePalette } from "./data/themePalettes";
import { isTauriRuntime } from "./utils/browserGuards";

const store = useShortcutStore();
const appStore = useAppStore();
const shortcutManager = ShortcutManager.getInstance();
const shortcutListRef = ref<HTMLElement | null>(null);
let sortable: Sortable | null = null;

const searchQuery = ref("");
const currentFilter = ref("all");
const viewMode = ref<"grid" | "list">("grid");
const selectedIds = ref<Set<string>>(new Set());
const isBatchMode = ref(false);
const isGroupedByColor = ref(false);

const showModal = ref(false);
const showSettings = ref(false);
const showAbout = ref(false);
const editingShortcut = ref<Shortcut | undefined>(undefined);
const triggerFeedback = ref<{
  id: number;
  status: "running" | "success" | "error";
  shortcutId: string;
  name: string;
  keys: string;
  message: string;
} | null>(null);
let feedbackTimer: ReturnType<typeof setTimeout> | null = null;

const filteredShortcuts = computed(() => {
  return store.shortcuts.filter((s) => {
    const matchesSearch = s.name.toLowerCase().includes(searchQuery.value.toLowerCase()) || 
                         s.target.path.toLowerCase().includes(searchQuery.value.toLowerCase());
    const matchesFilter = currentFilter.value === "all" || 
                         (currentFilter.value === "enabled" && s.enabled) || 
                         (currentFilter.value === "disabled" && !s.enabled);
    return matchesSearch && matchesFilter;
  });
});

const groupedShortcuts = computed(() => {
  if (!isGroupedByColor.value) return null;
  const groups: Record<string, Shortcut[]> = {};
  filteredShortcuts.value.forEach(s => {
    const color = s.color || '#3B82F6';
    if (!groups[color]) groups[color] = [];
    groups[color].push(s);
  });
  return groups;
});

const activePalette = computed(() => getThemePalette(store.settings.accentTheme));
const enabledCount = computed(() => store.shortcuts.filter((shortcut) => shortcut.enabled).length);
const disabledCount = computed(() => store.shortcuts.length - enabledCount.value);
const filterOptions = computed(() => [
  { id: "all", label: "全部", count: store.shortcuts.length, icon: Radio },
  { id: "enabled", label: "启用", count: enabledCount.value, icon: CircleCheck },
  { id: "disabled", label: "停用", count: disabledCount.value, icon: CircleX },
] as const);

const appShellStyle = computed(() => {
  const paletteVars = {
    '--accent': activePalette.value.accent,
    '--accent-2': activePalette.value.accent2,
    '--accent-soft': activePalette.value.soft,
    '--accent-border': activePalette.value.border,
    '--accent-text': activePalette.value.text,
  };

  if (appStore.isWorkMode) {
    const transparency = Math.max(0, Math.min(100, store.settings.workOpacity || 100));
    const alpha = Math.min(0.6, Math.max(0, 1 - transparency / 100));
    const surface = store.settings.theme === 'dark'
      ? `rgba(2, 6, 23, ${alpha})`
      : `rgba(248, 250, 252, ${alpha})`;
    return {
      backgroundColor: surface,
      '--window-surface': surface,
      '--window-panel': surface,
      ...paletteVars,
    };
  }
  if (!store.settings.transparentWindow) return paletteVars;
  const transparency = appStore.isWorkMode ? store.settings.workOpacity : store.settings.windowOpacity;
  const alpha = Math.max(0.04, Math.min(1, 1 - (transparency || 0) / 100));
  const surface = store.settings.theme === 'dark'
    ? `rgba(2, 6, 23, ${alpha})`
    : `rgba(248, 250, 252, ${alpha})`;
  return {
    backgroundColor: surface,
    '--window-surface': surface,
    '--window-panel': store.settings.theme === 'dark'
      ? `rgba(15, 23, 42, ${Math.min(0.38, alpha + 0.08)})`
      : `rgba(255, 255, 255, ${Math.min(0.45, alpha + 0.08)})`,
    ...paletteVars,
  };
});

const workGridStyle = computed(() => {
  if (!appStore.isWorkMode) return {};
  const sizeMap = {
    small: '64px',
    medium: '88px',
    large: '116px',
  } as const;
  const size = sizeMap[store.settings.workCardSize || 'medium'];
  return {
    gridTemplateColumns: `repeat(auto-fill, minmax(${size}, ${size}))`,
    gridAutoRows: size,
  };
});

const workCardSizeLabel = computed(() => {
  const size = store.settings.workCardSize || 'medium';
  return size === 'small' ? '小' : size === 'large' ? '大' : '中';
});

const handleAddShortcut = () => {
  editingShortcut.value = undefined;
  showModal.value = true;
};

const handleEdit = (shortcut: Shortcut) => {
  editingShortcut.value = shortcut;
  showModal.value = true;
};

const handleSaveShortcut = async (data: any) => {
  if (editingShortcut.value) {
    await store.updateShortcut(editingShortcut.value.id, data);
  } else {
    await store.addShortcut(data);
  }
  showModal.value = false;
};

const showTriggerFeedback = (feedback: NonNullable<typeof triggerFeedback.value>, duration = 1800) => {
  if (feedbackTimer) {
    clearTimeout(feedbackTimer);
    feedbackTimer = null;
  }
  triggerFeedback.value = feedback;
  if (feedback.status !== "running") {
    feedbackTimer = setTimeout(() => {
      if (triggerFeedback.value?.id === feedback.id) {
        triggerFeedback.value = null;
      }
    }, duration);
  }
};

const handleTrigger = async (shortcut: Shortcut) => {
  const feedbackId = Date.now();
  showTriggerFeedback({
    id: feedbackId,
    status: "running",
    shortcutId: shortcut.id,
    name: shortcut.name,
    keys: shortcut.target.path,
    message: shortcut.trigger.type === "delay" && shortcut.trigger.delay
      ? `等待 ${shortcut.trigger.delay} 秒后发送`
      : "正在发送按键",
  });

  try {
    await shortcutManager.executeShortcut(shortcut);
    if (triggerFeedback.value?.id !== feedbackId) return;
    showTriggerFeedback({
      id: feedbackId,
      status: "success",
      shortcutId: shortcut.id,
      name: shortcut.name,
      keys: shortcut.target.path,
      message: "已发送配置按键",
    });
  } catch (error) {
    if (triggerFeedback.value?.id !== feedbackId) return;
    showTriggerFeedback({
      id: feedbackId,
      status: "error",
      shortcutId: shortcut.id,
      name: shortcut.name,
      keys: shortcut.target.path,
      message: error instanceof Error ? error.message : String(error),
    }, 2600);
  }
};

const handleDelete = (id: string) => {
  store.deleteShortcut(id);
};

const handleToggle = (shortcut: Shortcut) => {
  store.toggleShortcut(shortcut.id);
};

const toggleSelect = (id: string) => {
  if (selectedIds.value.has(id)) {
    selectedIds.value.delete(id);
  } else {
    selectedIds.value.add(id);
  }
};

const handleBatchDelete = async () => {
  for (const id of selectedIds.value) {
    await store.deleteShortcut(id);
  }
  selectedIds.value.clear();
  isBatchMode.value = false;
};

const handleBatchToggle = async (enabled: boolean) => {
  for (const id of selectedIds.value) {
    await store.updateShortcut(id, { enabled });
  }
  isBatchMode.value = false;
};

const handleExport = async () => {
  try {
    const path = await save({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      defaultPath: 'shortcuts-backup.json'
    });
    
    if (path) {
      const data = JSON.stringify({
        shortcuts: store.shortcuts,
        settings: store.settings
      }, null, 2);
      await writeFile(path, new TextEncoder().encode(data));
      console.log('配置已导出到:', path);
    }
  } catch (error) {
    console.error('导出失败:', error);
  }
};

const handleImport = async () => {
  try {
    const path = await open({
      filters: [{ name: 'JSON', extensions: ['json'] }],
      multiple: false
    });
    
    if (path && typeof path === 'string') {
      const content = await readTextFile(path);
      const data = JSON.parse(content);
      if (data.shortcuts) {
        store.shortcuts = data.shortcuts;
        store.settings = data.settings || store.settings;
        await store.saveToLocal();
        console.log('配置已导入');
      }
    }
  } catch (error) {
    console.error('导入失败:', error);
  }
};

// 监听快捷键列表变化，重新初始化定时任务
watch(() => store.shortcuts.map(s => ({ id: s.id, enabled: s.enabled, trigger: s.trigger })), () => {
  shortcutManager.initTasks();
}, { deep: true });

const initSortable = () => {
  if (sortable) {
    sortable.destroy();
    sortable = null;
  }
  // 在分组模式下禁用拖拽排序
  if (isGroupedByColor.value) return;

  if (shortcutListRef.value) {
    sortable = new Sortable(shortcutListRef.value, {
      animation: 150,
      ghostClass: "opacity-50",
      forceFallback: true, // 强制使用自定义拖拽，绕过 HTML5 原生拖拽解决禁止光标问题
      fallbackClass: "shadow-2xl cursor-grabbing",
      fallbackOnBody: true,
      filter: "button, .no-drag", // 防止按钮触发拖拽
      preventOnFilter: false,
      onEnd: (evt) => {
        const { oldIndex, newIndex } = evt;
        if (oldIndex !== undefined && newIndex !== undefined && oldIndex !== newIndex) {
          const movedItem = filteredShortcuts.value[oldIndex];
          const targetItem = filteredShortcuts.value[newIndex];
          
          if (!movedItem || !targetItem) return;

          // 找到在原始仓库中的真实索引，避免在过滤状态下拖拽打乱数据
          const realOldIndex = store.shortcuts.findIndex(s => s.id === movedItem.id);
          const realNewIndex = store.shortcuts.findIndex(s => s.id === targetItem.id);
          
          if (realOldIndex !== -1 && realNewIndex !== -1) {
            const newShortcuts = [...store.shortcuts];
            const [moved] = newShortcuts.splice(realOldIndex, 1);
            newShortcuts.splice(realNewIndex, 0, moved);
            store.updateSort(newShortcuts);
          }
        }
      },
    });
  }
};

watch(() => store.settings.theme, (newTheme) => {
  if (newTheme === 'dark') {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }
});

watch([viewMode, isGroupedByColor], () => {
  nextTick(() => initSortable());
});

onMounted(async () => {
  await store.loadFromLocal();
  await shortcutManager.initTasks();
  nextTick(() => {
    initSortable();
  });

  // 应用主题
  if (store.settings.theme === 'dark') {
    document.documentElement.classList.add('dark');
  } else {
    document.documentElement.classList.remove('dark');
  }

  // 拦截窗口关闭事件，实现最小化到托盘
  if (isTauriRuntime()) {
    const appWindow = getCurrentWindow();
    appWindow.onCloseRequested(async (event) => {
      if (store.settings.minimizeToTray) {
        event.preventDefault();
        await appWindow.hide();
      }
    });
  }
});
</script>

<template>
  <div class="app-window-shell m-px flex h-[calc(100vh-2px)] flex-col overflow-hidden text-slate-800 font-sans transition-colors duration-300 dark:text-slate-200"
       :style="appShellStyle"
       :class="[
         appStore.isWorkMode ? 'rounded-2xl shadow-[inset_0_0_0_1px_rgba(255,255,255,0.12)]' : 'rounded-xl shadow-[0_18px_55px_rgba(2,6,23,0.28),inset_0_0_0_1px_rgba(148,163,184,0.32)] dark:shadow-[0_18px_55px_rgba(0,0,0,0.36),inset_0_0_0_1px_rgba(51,65,85,0.72)]',
         !appStore.isWorkMode && store.settings.transparentWindow ? 'backdrop-blur-2xl' : '',
         !appStore.isWorkMode && !store.settings.transparentWindow ? 'bg-slate-50 dark:bg-slate-950' : ''
       ]">
    <Titlebar v-if="!appStore.isWorkMode" />
    <div class="flex flex-1 overflow-hidden relative">
    <main class="flex-1 flex flex-col min-w-0 relative">
      <div
        v-if="triggerFeedback"
        class="absolute top-4 right-4 z-[70] max-w-[min(360px,calc(100%-2rem))] rounded-lg border px-4 py-3 shadow-xl backdrop-blur-xl transition-all"
        :class="[
          triggerFeedback.status === 'success' ? 'bg-emerald-50/95 dark:bg-emerald-950/80 border-emerald-200 dark:border-emerald-500/30 text-emerald-800 dark:text-emerald-100' : '',
          triggerFeedback.status === 'running' ? 'bg-blue-50/95 dark:bg-blue-950/80 border-blue-200 dark:border-blue-500/30 text-blue-800 dark:text-blue-100' : '',
          triggerFeedback.status === 'error' ? 'bg-red-50/95 dark:bg-red-950/80 border-red-200 dark:border-red-500/30 text-red-800 dark:text-red-100' : ''
        ]"
      >
        <div class="flex items-start gap-3">
          <Loader2 v-if="triggerFeedback.status === 'running'" :size="18" class="mt-0.5 shrink-0 animate-spin" />
          <CheckCircle2 v-else-if="triggerFeedback.status === 'success'" :size="18" class="mt-0.5 shrink-0" />
          <XCircle v-else :size="18" class="mt-0.5 shrink-0" />
          <div class="min-w-0">
            <div class="text-sm font-semibold truncate">{{ triggerFeedback.message }}</div>
            <div class="mt-1 flex items-center gap-2 text-xs opacity-80 min-w-0">
              <span class="truncate">{{ triggerFeedback.name }}</span>
              <span class="font-mono rounded border px-1.5 py-0.5 shrink-0"
                    :class="triggerFeedback.status === 'error' ? 'border-red-300/60 dark:border-red-400/30' : 'border-current/20'">
                {{ triggerFeedback.keys }}
              </span>
            </div>
          </div>
        </div>
      </div>

      <!-- 工作模式悬浮工具 -->
      <div
        v-if="appStore.isWorkMode"
        class="absolute top-2 right-2 z-50 flex items-center gap-1 rounded-full border bg-slate-950/45 p-1 shadow-lg backdrop-blur-xl"
        :style="{ color: 'var(--accent-text)', borderColor: 'var(--accent-border)' }"
      >
        <button
          @click="appStore.toggleAlwaysOnTop"
          class="rounded-full p-1.5 transition-all hover:bg-white/12 active:scale-95"
          :style="appStore.isAlwaysOnTop ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent)' } : undefined"
          title="窗口置顶"
        >
          <Pin :size="14" />
        </button>
        <button
          @click="showSettings = true"
          class="rounded-full p-1.5 transition-all hover:bg-white/12 active:scale-95"
          title="工作模式设置"
        >
          <Settings :size="14" />
        </button>
        <button
          @click="appStore.toggleWorkMode"
          class="rounded-full p-1.5 transition-all hover:bg-white/12 active:scale-95"
          title="退出工作模式"
        >
          <Minimize2 :size="14" />
        </button>
      </div>

      <div
        v-if="appStore.isWorkMode"
        class="absolute left-3 bottom-3 z-40 flex max-w-[calc(100%-1.5rem)] items-center gap-2 rounded-xl border px-3 py-2 text-xs backdrop-blur-xl"
        :style="{ backgroundColor: 'rgba(2, 6, 23, 0.48)', borderColor: 'var(--accent-border)', color: 'var(--accent-text)' }"
      >
        <span class="flex items-center gap-1.5 rounded-lg px-2 py-1" :style="{ backgroundColor: 'var(--accent-soft)' }">
          <span class="h-1.5 w-1.5 rounded-full" :style="{ backgroundColor: 'var(--accent)' }"></span>
          直播模式
        </span>
        <span class="hidden sm:inline opacity-70">快捷键 {{ filteredShortcuts.length }}</span>
        <span class="hidden sm:inline opacity-40">|</span>
        <span class="hidden sm:inline opacity-70">透明 {{ store.settings.workOpacity }}%</span>
        <span class="hidden sm:inline opacity-40">|</span>
        <span class="hidden sm:inline opacity-70">卡片 {{ workCardSizeLabel }}</span>
        <template v-if="triggerFeedback">
          <span class="hidden md:inline opacity-40">|</span>
          <span class="hidden md:inline truncate opacity-80">
            {{ triggerFeedback.status === 'running' ? '发送中' : triggerFeedback.status === 'success' ? '已发送' : '失败' }}：{{ triggerFeedback.name }}
          </span>
        </template>
      </div>

      <!-- 直播控制台 -->
      <header v-if="!appStore.isWorkMode" data-tauri-drag-region class="border-b border-slate-200/60 px-5 py-4 transition-colors duration-300 dark:border-slate-800/70"
              :style="store.settings.transparentWindow ? { backgroundColor: 'var(--window-panel)' } : undefined"
              :class="store.settings.transparentWindow ? 'backdrop-blur-xl' : 'bg-white/70 dark:bg-slate-950/80 backdrop-blur-xl'">
        <div data-tauri-drag-region="false" class="flex flex-col gap-4">
          <div class="flex flex-wrap items-center justify-between gap-3">
            <div class="flex min-w-0 items-center gap-3">
              <div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl border border-white/10 text-slate-950 shadow-lg shadow-slate-950/10"
                   :style="{ background: 'linear-gradient(135deg, var(--accent), var(--accent-2))' }">
                <Radio :size="20" />
              </div>
              <div class="min-w-0">
                <div class="flex items-center gap-2">
                  <span class="h-2 w-2 rounded-full shadow-[0_0_16px_currentColor]" :style="{ color: 'var(--accent)', backgroundColor: 'var(--accent)' }"></span>
                  <span class="text-xs font-bold uppercase tracking-[0.18em] text-slate-500 dark:text-slate-400">Live Console</span>
                </div>
                <h1 class="truncate text-xl font-black leading-tight text-slate-950 dark:text-white">直播快捷键面板</h1>
              </div>
            </div>

            <div class="flex flex-wrap items-center gap-2">
              <button @click="handleImport" class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-slate-200 bg-white/70 text-slate-600 transition-all hover:bg-slate-100 hover:text-slate-950 active:scale-95 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-300 dark:hover:bg-slate-800 dark:hover:text-white" title="导入配置">
                <Upload :size="17" />
              </button>
              <button @click="handleExport" class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-slate-200 bg-white/70 text-slate-600 transition-all hover:bg-slate-100 hover:text-slate-950 active:scale-95 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-300 dark:hover:bg-slate-800 dark:hover:text-white" title="导出配置">
                <Download :size="17" />
              </button>
              <button @click="showAbout = true" class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-slate-200 bg-white/70 text-slate-600 transition-all hover:bg-slate-100 hover:text-slate-950 active:scale-95 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-300 dark:hover:bg-slate-800 dark:hover:text-white" title="关于软件">
                <Info :size="17" />
              </button>
              <button @click="showSettings = true" class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-slate-200 bg-white/70 text-slate-600 transition-all hover:bg-slate-100 hover:text-slate-950 active:scale-95 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-300 dark:hover:bg-slate-800 dark:hover:text-white" title="系统设置">
                <Settings :size="17" />
              </button>
              <button
                @click="handleAddShortcut"
                class="inline-flex h-9 items-center gap-2 rounded-lg px-3 text-sm font-bold text-slate-950 shadow-lg transition-all hover:-translate-y-0.5 active:scale-95"
                :style="{ background: 'linear-gradient(135deg, var(--accent), var(--accent-2))', boxShadow: '0 16px 32px var(--accent-soft)' }"
              >
                <Plus :size="17" />
                添加
              </button>
            </div>
          </div>

          <div class="grid grid-cols-1 gap-3 lg:grid-cols-[minmax(260px,1fr)_auto]">
            <div class="relative">
              <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500" :size="18" />
              <input 
                v-model="searchQuery"
                type="text" 
                placeholder="搜索快捷键或按键..." 
                class="h-11 w-full rounded-xl border border-slate-200 bg-white/70 pl-10 pr-4 text-sm text-slate-900 outline-none transition-all placeholder:text-slate-400 focus:border-transparent focus:ring-2 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-100 dark:placeholder:text-slate-600"
                :style="{ '--tw-ring-color': 'var(--accent)' }"
              />
            </div>

            <div class="flex flex-wrap items-center gap-2">
              <div class="flex rounded-xl border border-slate-200 bg-white/70 p-1 dark:border-slate-800 dark:bg-slate-900/70">
                <button
                  v-for="filter in filterOptions"
                  :key="filter.id"
                  @click="currentFilter = filter.id"
                  class="flex h-9 items-center gap-1.5 rounded-lg px-3 text-xs font-semibold transition-all"
                  :style="currentFilter === filter.id ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent)' } : undefined"
                  :class="currentFilter === filter.id ? '' : 'text-slate-500 hover:text-slate-950 dark:text-slate-400 dark:hover:text-white'"
                  :title="filter.label"
                >
                  <component :is="filter.icon" :size="15" />
                  <span>{{ filter.label }}</span>
                  <span class="rounded-md px-1.5 py-0.5 text-[10px]" :class="currentFilter === filter.id ? 'bg-white/10' : 'bg-slate-100 dark:bg-slate-800'">{{ filter.count }}</span>
                </button>
              </div>

              <div class="flex rounded-xl border border-slate-200 bg-white/70 p-1 dark:border-slate-800 dark:bg-slate-900/70">
                <button 
                  @click="appStore.toggleAlwaysOnTop" 
                  class="h-9 w-9 rounded-lg transition-colors"
                  :style="appStore.isAlwaysOnTop ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent)' } : undefined"
                  :class="appStore.isAlwaysOnTop ? '' : 'text-slate-500 hover:text-slate-950 dark:text-slate-400 dark:hover:text-white'"
                  title="窗口置顶"
                >
                  <Pin :size="17" class="mx-auto" />
                </button>
                <button 
                  @click="appStore.toggleWorkMode" 
                  class="h-9 w-9 rounded-lg text-slate-500 transition-colors hover:text-slate-950 dark:text-slate-400 dark:hover:text-white"
                  title="进入直播模式"
                >
                  <Maximize2 :size="17" class="mx-auto" />
                </button>
              </div>

              <div class="flex rounded-xl border border-slate-200 bg-white/70 p-1 dark:border-slate-800 dark:bg-slate-900/70">
                <button 
                  @click="isGroupedByColor = !isGroupedByColor"
                  class="h-9 w-9 rounded-lg transition-all"
                  :style="isGroupedByColor ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent)' } : undefined"
                  :class="isGroupedByColor ? '' : 'text-slate-500 hover:text-slate-950 dark:text-slate-400 dark:hover:text-white'"
                  title="颜色分组"
                >
                  <Palette :size="17" class="mx-auto" />
                </button>
                <button 
                  @click="viewMode = 'grid'"
                  class="h-9 w-9 rounded-lg transition-all"
                  :class="viewMode === 'grid' ? 'bg-slate-100 text-slate-950 shadow-sm dark:bg-slate-800 dark:text-white' : 'text-slate-500 hover:text-slate-950 dark:text-slate-400 dark:hover:text-white'"
                  title="网格视图"
                >
                  <Grid :size="17" class="mx-auto" />
                </button>
                <button 
                  @click="viewMode = 'list'"
                  class="h-9 w-9 rounded-lg transition-all"
                  :class="viewMode === 'list' ? 'bg-slate-100 text-slate-950 shadow-sm dark:bg-slate-800 dark:text-white' : 'text-slate-500 hover:text-slate-950 dark:text-slate-400 dark:hover:text-white'"
                  title="列表视图"
                >
                  <ListIcon :size="17" class="mx-auto" />
                </button>
              </div>

              <button 
                @click="isBatchMode = !isBatchMode; selectedIds.clear()"
                class="h-11 rounded-xl border px-3 text-sm font-semibold transition-all"
                :class="isBatchMode ? 'border-blue-500 bg-blue-600 text-white shadow-lg shadow-blue-500/20' : 'border-slate-200 bg-white/70 text-slate-700 hover:bg-slate-100 dark:border-slate-800 dark:bg-slate-900/70 dark:text-slate-300 dark:hover:bg-slate-800'"
              >
                {{ isBatchMode ? '取消批量' : '批量操作' }}
              </button>
            </div>
          </div>
        </div>
      </header>

      <!-- 批量操作栏 -->
      <div v-if="isBatchMode && !appStore.isWorkMode" class="flex items-center justify-between border-b border-blue-200 bg-blue-50 px-6 py-3 animate-in slide-in-from-top duration-200 dark:border-blue-500/20 dark:bg-blue-900/20">
        <div class="flex items-center gap-4">
          <span class="text-sm font-medium text-blue-600 dark:text-blue-400">已选择 {{ selectedIds.size }} 项</span>
          <button @click="selectedIds = new Set(filteredShortcuts.map(s => s.id))" class="text-xs text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white">全选</button>
          <button @click="selectedIds.clear()" class="text-xs text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white">取消全选</button>
        </div>
        <div class="flex items-center gap-2">
          <button @click="handleBatchToggle(true)" class="px-3 py-1 bg-white dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 border border-slate-200 dark:border-transparent rounded text-xs text-slate-700 dark:text-slate-200 transition-colors">批量启用</button>
          <button @click="handleBatchToggle(false)" class="px-3 py-1 bg-white dark:bg-slate-800 hover:bg-slate-100 dark:hover:bg-slate-700 border border-slate-200 dark:border-transparent rounded text-xs text-slate-700 dark:text-slate-200 transition-colors">批量禁用</button>
          <button @click="handleBatchDelete" class="px-3 py-1 bg-red-50 dark:bg-red-900/20 hover:bg-red-100 dark:hover:bg-red-900/40 text-red-600 dark:text-red-400 border border-red-200 dark:border-transparent rounded text-xs transition-colors">批量删除</button>
        </div>
      </div>

      <!-- 内容区 -->
      <div class="flex-1 overflow-y-auto" :class="appStore.isWorkMode ? 'p-3 pt-3 pb-16' : 'bg-slate-100/70 p-5 dark:bg-slate-950/70'">
        <div v-if="filteredShortcuts.length === 0" class="h-full flex flex-col items-center justify-center text-slate-500 space-y-4">
          <div class="flex h-20 w-20 items-center justify-center rounded-2xl border border-slate-200 bg-white shadow-sm transition-colors duration-300 dark:border-slate-800 dark:bg-slate-900 dark:shadow-none">
            <Plus :size="38" class="opacity-30 text-slate-400" />
          </div>
          <div class="text-center">
            <p class="text-lg font-medium text-slate-600 dark:text-slate-400">暂无快捷键</p>
            <p class="text-sm text-slate-500 dark:text-slate-500">点击顶部「添加」开始创建直播动作</p>
          </div>
        </div>

        <template v-else-if="isGroupedByColor && groupedShortcuts">
          <div v-for="(group, color) in groupedShortcuts" :key="color" class="mb-8 last:mb-0">
            <div class="flex items-center gap-3 mb-4 px-1">
              <div class="w-4 h-4 rounded-full shadow-sm" :style="{ backgroundColor: color }"></div>
              <h3 class="text-lg font-bold text-slate-700 dark:text-slate-300 transition-colors duration-300">
                <span class="uppercase font-mono text-sm opacity-50 ml-2">{{ color }}</span>
              </h3>
              <div class="flex-1 h-px bg-slate-200 dark:bg-slate-800/50 transition-colors duration-300"></div>
            </div>
            <div class="grid" 
                 :style="workGridStyle"
                 :class="[
                   viewMode === 'grid' && !appStore.isWorkMode ? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4' : '',
                   viewMode === 'grid' && appStore.isWorkMode ? 'justify-start gap-2.5' : '',
                   viewMode === 'list' ? 'grid-cols-1' : ''
                 ]">
              <ShortcutCard 
                v-for="shortcut in group" 
                :key="shortcut.id"
                :shortcut="shortcut"
                :batch-mode="isBatchMode"
                :selected="selectedIds.has(shortcut.id)"
                :work-mode="appStore.isWorkMode"
                :feedback-status="triggerFeedback?.shortcutId === shortcut.id ? triggerFeedback.status : undefined"
                @trigger="handleTrigger"
                @edit="handleEdit"
                @delete="handleDelete"
                @toggle="handleToggle"
                @select="toggleSelect"
              />
            </div>
          </div>
        </template>

        <div 
          v-else
          ref="shortcutListRef"
          class="grid"
          :style="workGridStyle"
          :class="[
            viewMode === 'grid' && !appStore.isWorkMode ? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4' : '',
            viewMode === 'grid' && appStore.isWorkMode ? 'justify-start gap-2.5' : '',
            viewMode === 'list' ? 'grid-cols-1' : ''
          ]"
        >
          <ShortcutCard 
            v-for="shortcut in filteredShortcuts" 
            :key="shortcut.id"
            :shortcut="shortcut"
            :batch-mode="isBatchMode"
            :selected="selectedIds.has(shortcut.id)"
            :work-mode="appStore.isWorkMode"
            :feedback-status="triggerFeedback?.shortcutId === shortcut.id ? triggerFeedback.status : undefined"
            @trigger="handleTrigger"
            @edit="handleEdit"
            @delete="handleDelete"
            @toggle="handleToggle"
            @select="toggleSelect"
          />
        </div>
      </div>
    </main>

    <AddShortcutModal 
      :show="showModal" 
      :edit-shortcut="editingShortcut"
      @close="showModal = false"
      @save="handleSaveShortcut"
    />

    <SettingsModal 
      :show="showSettings" 
      @close="showSettings = false" 
    />

    <AboutModal 
      :show="showAbout" 
      @close="showAbout = false" 
    />
    </div>
  </div>
</template>

<style>
/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #cbd5e1; /* slate-300 */
  border-radius: 4px;
}
.dark ::-webkit-scrollbar-thumb {
  background: #1e293b; /* slate-800 */
}
::-webkit-scrollbar-thumb:hover {
  background: #94a3b8; /* slate-400 */
}
.dark ::-webkit-scrollbar-thumb:hover {
  background: #334155; /* slate-700 */
}
</style>
