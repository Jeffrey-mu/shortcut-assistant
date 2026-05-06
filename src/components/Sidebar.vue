<script setup lang="ts">
import { 
  Plus, 
  List, 
  CircleCheck, 
  CircleX, 
  Download, 
  Upload, 
  Settings, 
  Info 
} from 'lucide-vue-next';

defineEmits(['add-shortcut', 'filter-change', 'import', 'export', 'settings', 'about']);

const filters = [
  { id: 'all', label: '全部', icon: List },
  { id: 'enabled', label: '已启用', icon: CircleCheck },
  { id: 'disabled', label: '已禁用', icon: CircleX },
];
</script>

<template>
  <aside class="w-64 bg-slate-900 text-white flex flex-col h-screen border-r border-slate-700">
    <div class="p-6">
      <h1 class="text-xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
        快捷键管理助手
      </h1>
    </div>

    <div class="px-4 mb-6">
      <button 
        @click="$emit('add-shortcut')"
        class="w-full flex items-center justify-center gap-2 bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors font-medium"
      >
        <Plus :size="18" />
        添加快捷键
      </button>
    </div>

    <nav class="flex-1 px-2 space-y-1">
      <button 
        v-for="filter in filters" 
        :key="filter.id"
        @click="$emit('filter-change', filter.id)"
        class="w-full flex items-center gap-3 px-3 py-2 text-slate-300 hover:bg-slate-800 hover:text-white rounded-md transition-colors"
      >
        <component :is="filter.icon" :size="18" />
        {{ filter.label }}
      </button>
    </nav>

    <div class="p-4 border-t border-slate-800 space-y-1">
      <button 
        @click="$emit('import')"
        class="w-full flex items-center gap-3 px-3 py-2 text-slate-400 hover:bg-slate-800 hover:text-white rounded-md transition-colors text-sm"
      >
        <Upload :size="16" />
        导入配置
      </button>
      <button 
        @click="$emit('export')"
        class="w-full flex items-center gap-3 px-3 py-2 text-slate-400 hover:bg-slate-800 hover:text-white rounded-md transition-colors text-sm"
      >
        <Download :size="16" />
        导出配置
      </button>
      <button 
        @click="$emit('settings')"
        class="w-full flex items-center gap-3 px-3 py-2 text-slate-400 hover:bg-slate-800 hover:text-white rounded-md transition-colors text-sm"
      >
        <Settings :size="16" />
        系统设置
      </button>
      <button 
        @click="$emit('about')"
        class="w-full flex items-center gap-3 px-3 py-2 text-slate-400 hover:bg-slate-800 hover:text-white rounded-md transition-colors text-sm"
      >
        <Info :size="16" />
        关于软件
      </button>
    </div>
  </aside>
</template>
