<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from "vue";
import Titlebar from "./components/Titlebar.vue";
import Sidebar from "./components/Sidebar.vue";
import ShortcutCard from "./components/ShortcutCard.vue";
import AddShortcutModal from "./components/AddShortcutModal.vue";
import SettingsModal from "./components/SettingsModal.vue";
import AboutModal from "./components/AboutModal.vue";
import { useShortcutStore, type Shortcut } from "./stores/shortcut";
import { useAppStore } from "./stores/app";
import { ShortcutManager } from "./services/shortcutManager";
import { Search, Grid, List as ListIcon, Plus, Pin, Maximize2, Minimize2, Palette, X } from "lucide-vue-next";
import Sortable from "sortablejs";
import { save, open } from "@tauri-apps/plugin-dialog";
import { writeFile, readTextFile } from "@tauri-apps/plugin-fs";
import { getCurrentWindow } from "@tauri-apps/api/window";

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

const handleTrigger = (shortcut: Shortcut) => {
  shortcutManager.executeShortcut(shortcut);
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
  const appWindow = getCurrentWindow();
  appWindow.onCloseRequested(async (event) => {
    if (store.settings.minimizeToTray) {
      event.preventDefault();
      await appWindow.hide();
    }
  });
});
</script>

<template>
  <div class="flex flex-col h-screen text-slate-800 dark:text-slate-200 overflow-hidden font-sans transition-colors duration-300 rounded-xl shadow-2xl border border-slate-200/50 dark:border-slate-700/50"
       :class="store.settings.transparentWindow ? 'bg-white/70 dark:bg-slate-900/70 backdrop-blur-2xl' : 'bg-slate-50 dark:bg-slate-950'">
    <Titlebar />
    <div class="flex flex-1 overflow-hidden relative">
      <Sidebar v-if="!appStore.isWorkMode"
      @add-shortcut="handleAddShortcut" 
      @filter-change="currentFilter = $event" 
      @import="handleImport"
      @export="handleExport"
      @settings="showSettings = true"
      @about="showAbout = true"
    />

    <main class="flex-1 flex flex-col min-w-0 relative">
      <!-- 退出工作模式悬浮按钮 -->
      <button
        v-if="appStore.isWorkMode"
        @click="appStore.toggleWorkMode"
        class="absolute top-2 right-2 z-50 p-2 bg-white/80 dark:bg-slate-800/80 hover:bg-slate-100 dark:hover:bg-slate-700 text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white rounded-full shadow-lg backdrop-blur transition-all active:scale-95 border border-slate-200 dark:border-slate-700"
        title="退出工作模式"
      >
        <Minimize2 :size="16" />
      </button>

      <!-- 顶部状态栏 -->
      <header v-if="!appStore.isWorkMode" data-tauri-drag-region class="h-16 border-b border-slate-200/50 dark:border-slate-800/50 flex items-center justify-between px-6 transition-colors duration-300"
              :class="store.settings.transparentWindow ? 'bg-transparent' : 'bg-white/50 dark:bg-slate-900/50 backdrop-blur-sm'">
        <div data-tauri-drag-region="false" class="flex items-center gap-4 flex-1 max-w-xl">
          <div class="relative w-full">
            <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 dark:text-slate-500" :size="18" />
            <input 
              v-model="searchQuery"
              type="text" 
              placeholder="搜索快捷键或备注..." 
              class="w-full bg-slate-100 dark:bg-slate-800 border-none rounded-lg pl-10 pr-4 py-2 text-sm focus:ring-2 focus:ring-blue-500 transition-all placeholder:text-slate-400 dark:placeholder:text-slate-600 text-slate-900 dark:text-slate-100 outline-none"
            />
          </div>
        </div>

        <div data-tauri-drag-region="false" class="flex items-center gap-3 ml-4">
          <!-- 模式与置顶切换 -->
          <div class="flex items-center gap-1 mr-2 pr-4 border-r border-slate-200 dark:border-slate-700 transition-colors duration-300">
            <button 
              @click="appStore.toggleAlwaysOnTop" 
              class="p-2 rounded-lg transition-colors"
              :class="appStore.isAlwaysOnTop ? 'bg-blue-500/20 text-blue-500 dark:text-blue-400' : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800'"
              title="窗口置顶"
            >
              <Pin :size="18" />
            </button>
            <button 
              @click="appStore.toggleWorkMode" 
              class="p-2 text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 rounded-lg transition-colors"
              title="进入工作模式"
            >
              <Maximize2 :size="18" />
            </button>
          </div>

          <div class="flex bg-slate-100 dark:bg-slate-800 rounded-lg p-1 transition-colors duration-300">
            <button 
              @click="isGroupedByColor = !isGroupedByColor"
              class="p-1.5 rounded-md transition-all mr-1"
              :class="isGroupedByColor ? 'bg-blue-600 text-white shadow-sm' : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-300'"
              title="颜色分组渲染"
            >
              <Palette :size="18" />
            </button>
            <div class="w-px bg-slate-300 dark:bg-slate-700 mx-1 my-1 transition-colors duration-300"></div>
            <button 
              @click="viewMode = 'grid'"
              class="p-1.5 rounded-md transition-all"
              :class="viewMode === 'grid' ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-300'"
            >
              <Grid :size="18" />
            </button>
            <button 
              @click="viewMode = 'list'"
              class="p-1.5 rounded-md transition-all"
              :class="viewMode === 'list' ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-400 hover:text-slate-800 dark:hover:text-slate-300'"
            >
              <ListIcon :size="18" />
            </button>
          </div>
          <button 
            @click="isBatchMode = !isBatchMode; selectedIds.clear()"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
            :class="isBatchMode ? 'bg-blue-600 text-white' : 'bg-white dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 border border-slate-200 dark:border-transparent'"
          >
            {{ isBatchMode ? '取消批量' : '批量操作' }}
          </button>
        </div>
      </header>

      <!-- 批量操作栏 -->
      <div v-if="isBatchMode && !appStore.isWorkMode" class="bg-blue-50 dark:bg-blue-900/20 border-b border-blue-200 dark:border-blue-500/20 px-6 py-3 flex items-center justify-between animate-in slide-in-from-top duration-200">
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
      <div class="flex-1 overflow-y-auto" :class="appStore.isWorkMode ? 'p-2 pt-10' : 'p-6'">
        <div v-if="filteredShortcuts.length === 0" class="h-full flex flex-col items-center justify-center text-slate-500 space-y-4">
          <div class="p-6 bg-white dark:bg-slate-900 rounded-full border border-slate-200 dark:border-slate-800 shadow-sm dark:shadow-none transition-colors duration-300">
            <Plus :size="48" class="opacity-20 text-slate-400" />
          </div>
          <div class="text-center">
            <p class="text-lg font-medium text-slate-600 dark:text-slate-400">暂无快捷键</p>
            <p class="text-sm text-slate-500 dark:text-slate-500">点击左侧「添加快捷键」开始创建</p>
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
            <div class="grid gap-2 sm:gap-3" 
                 :class="[
                   viewMode === 'grid' && !appStore.isWorkMode ? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4' : '',
                   viewMode === 'grid' && appStore.isWorkMode ? 'grid-cols-3 sm:grid-cols-5 md:grid-cols-7 lg:grid-cols-9 xl:grid-cols-11 2xl:grid-cols-14 auto-rows-[48px]' : '',
                   viewMode === 'list' ? 'grid-cols-1' : ''
                 ]">
              <ShortcutCard 
                v-for="shortcut in group" 
                :key="shortcut.id"
                :shortcut="shortcut"
                :batch-mode="isBatchMode"
                :selected="selectedIds.has(shortcut.id)"
                :work-mode="appStore.isWorkMode"
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
          class="grid gap-2 sm:gap-3"
          :class="[
            viewMode === 'grid' && !appStore.isWorkMode ? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4' : '',
            viewMode === 'grid' && appStore.isWorkMode ? 'grid-cols-3 sm:grid-cols-5 md:grid-cols-7 lg:grid-cols-9 xl:grid-cols-11 2xl:grid-cols-14 auto-rows-[48px]' : '',
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
