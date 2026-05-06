<script setup lang="ts">
import { ref, computed, onMounted, watch, nextTick } from "vue";
import Sidebar from "./components/Sidebar.vue";
import ShortcutCard from "./components/ShortcutCard.vue";
import AddShortcutModal from "./components/AddShortcutModal.vue";
import SettingsModal from "./components/SettingsModal.vue";
import AboutModal from "./components/AboutModal.vue";
import { useShortcutStore, type Shortcut } from "./stores/shortcut";
import { useAppStore } from "./stores/app";
import { ShortcutManager } from "./services/shortcutManager";
import { Search, Grid, List as ListIcon, Plus, Pin, Maximize2, Minimize2 } from "lucide-vue-next";
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
  if (shortcutListRef.value) {
    if (sortable) {
      sortable.destroy();
    }
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

watch(viewMode, () => {
  if (sortable) {
    sortable.destroy();
    sortable = null;
  }
  nextTick(() => initSortable());
});

onMounted(async () => {
  await store.loadFromLocal();
  await shortcutManager.initTasks();
  nextTick(() => {
    initSortable();
  });

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
  <div class="flex h-screen bg-slate-950 text-slate-200 overflow-hidden font-sans">
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
        class="absolute top-4 right-4 z-50 p-3 bg-slate-800/80 hover:bg-slate-700 text-slate-300 hover:text-white rounded-full shadow-lg backdrop-blur transition-all active:scale-95 border border-slate-700"
        title="退出工作模式"
      >
        <Minimize2 :size="20" />
      </button>

      <!-- 顶部状态栏 -->
      <header v-if="!appStore.isWorkMode" class="h-16 border-b border-slate-800 flex items-center justify-between px-6 bg-slate-900/50 backdrop-blur-sm">
        <div class="flex items-center gap-4 flex-1 max-w-xl">
          <div class="relative w-full">
            <Search class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500" :size="18" />
            <input 
              v-model="searchQuery"
              type="text" 
              placeholder="搜索快捷键或备注..." 
              class="w-full bg-slate-800 border-none rounded-lg pl-10 pr-4 py-2 text-sm focus:ring-2 focus:ring-blue-500 transition-all placeholder:text-slate-600"
            />
          </div>
        </div>

        <div class="flex items-center gap-3 ml-4">
          <!-- 模式与置顶切换 -->
          <div class="flex items-center gap-1 mr-2 pr-4 border-r border-slate-700">
            <button 
              @click="appStore.toggleAlwaysOnTop" 
              class="p-2 rounded-lg transition-colors"
              :class="appStore.isAlwaysOnTop ? 'bg-blue-500/20 text-blue-400' : 'text-slate-400 hover:text-slate-200 hover:bg-slate-800'"
              title="窗口置顶"
            >
              <Pin :size="18" />
            </button>
            <button 
              @click="appStore.toggleWorkMode" 
              class="p-2 text-slate-400 hover:text-slate-200 hover:bg-slate-800 rounded-lg transition-colors"
              title="进入工作模式"
            >
              <Maximize2 :size="18" />
            </button>
          </div>

          <div class="flex bg-slate-800 rounded-lg p-1">
            <button 
              @click="viewMode = 'grid'"
              class="p-1.5 rounded-md transition-all"
              :class="viewMode === 'grid' ? 'bg-slate-700 text-white shadow-sm' : 'text-slate-500 hover:text-slate-300'"
            >
              <Grid :size="18" />
            </button>
            <button 
              @click="viewMode = 'list'"
              class="p-1.5 rounded-md transition-all"
              :class="viewMode === 'list' ? 'bg-slate-700 text-white shadow-sm' : 'text-slate-500 hover:text-slate-300'"
            >
              <ListIcon :size="18" />
            </button>
          </div>
          <button 
            @click="isBatchMode = !isBatchMode; selectedIds.clear()"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
            :class="isBatchMode ? 'bg-blue-600 text-white' : 'bg-slate-800 text-slate-300 hover:bg-slate-700'"
          >
            {{ isBatchMode ? '取消批量' : '批量操作' }}
          </button>
        </div>
      </header>

      <!-- 批量操作栏 -->
      <div v-if="isBatchMode && !appStore.isWorkMode" class="bg-blue-600/10 border-b border-blue-500/20 px-6 py-3 flex items-center justify-between animate-in slide-in-from-top duration-200">
        <div class="flex items-center gap-4">
          <span class="text-sm font-medium text-blue-400">已选择 {{ selectedIds.size }} 项</span>
          <button @click="selectedIds = new Set(filteredShortcuts.map(s => s.id))" class="text-xs text-slate-400 hover:text-white">全选</button>
          <button @click="selectedIds.clear()" class="text-xs text-slate-400 hover:text-white">取消全选</button>
        </div>
        <div class="flex items-center gap-2">
          <button @click="handleBatchToggle(true)" class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-xs">批量启用</button>
          <button @click="handleBatchToggle(false)" class="px-3 py-1 bg-slate-800 hover:bg-slate-700 rounded text-xs">批量禁用</button>
          <button @click="handleBatchDelete" class="px-3 py-1 bg-red-600/20 hover:bg-red-600 text-red-400 hover:text-white rounded text-xs transition-colors">批量删除</button>
        </div>
      </div>

      <!-- 内容区 -->
      <div class="flex-1 overflow-y-auto" :class="appStore.isWorkMode ? 'p-3' : 'p-6'">
        <div v-if="filteredShortcuts.length === 0" class="h-full flex flex-col items-center justify-center text-slate-500 space-y-4">
          <div class="p-6 bg-slate-900 rounded-full border border-slate-800">
            <Plus :size="48" class="opacity-20" />
          </div>
          <div class="text-center">
            <p class="text-lg font-medium text-slate-400">暂无快捷键</p>
            <p class="text-sm">点击左侧「添加快捷键」开始创建</p>
          </div>
        </div>

        <div 
          v-show="filteredShortcuts.length > 0"
          ref="shortcutListRef"
          class="grid gap-4"
          :class="viewMode === 'grid' ? 'grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4' : 'grid-cols-1'"
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
  background: #1e293b;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #334155;
}
</style>
