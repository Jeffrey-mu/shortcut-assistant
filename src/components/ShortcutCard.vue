<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { 
  Play, 
  Edit2, 
  Trash2, 
  Clock, 
  Timer, 
  Repeat,
  MoreVertical,
  Check,
  CircleCheck
} from 'lucide-vue-next';
import type { Shortcut } from '../stores/shortcut';

defineProps<{
  shortcut: Shortcut;
  selected?: boolean;
  batchMode?: boolean;
  workMode?: boolean;
}>();

defineEmits(['trigger', 'edit', 'delete', 'toggle', 'select']);

const showMenu = ref(false);
const menuRef = ref<HTMLElement | null>(null);

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
    class="rounded-xl border overflow-hidden transition-all duration-300 group relative shadow-sm hover:shadow-md dark:shadow-lg flex flex-col h-full select-none bg-white dark:bg-slate-900 border-slate-200 dark:border-slate-800"
    :class="{ 
      'opacity-50 grayscale-[30%]': !shortcut.enabled && !batchMode,
      'ring-2 ring-offset-2 ring-offset-white dark:ring-offset-slate-950 scale-[1.02] z-10': selected,
      'cursor-pointer hover:shadow-xl dark:hover:shadow-2xl hover:-translate-y-1 active:scale-[0.97] active:shadow-md': workMode,
      'cursor-grab active:cursor-grabbing hover:shadow-lg dark:hover:shadow-xl hover:-translate-y-0.5': !batchMode && !workMode,
      'cursor-pointer hover:border-blue-500': batchMode,
      'no-drag': workMode // 在工作模式下禁用拖拽
    }"
    :style="{ 
      backgroundColor: `color-mix(in srgb, ${shortcut.color} 5%, transparent)`, 
      borderColor: selected ? shortcut.color : undefined,
      boxShadow: selected ? `0 8px 30px ${shortcut.color}30` : 'none'
    }"
    @click="batchMode ? $emit('select', shortcut.id) : (workMode ? $emit('trigger', shortcut) : null)"
  >
    <!-- 背景渐变发光效果 -->
    <div 
      class="absolute -top-24 -right-24 w-48 h-48 rounded-full blur-[50px] opacity-10 dark:opacity-20 transition-opacity group-hover:opacity-20 dark:group-hover:opacity-40"
      :style="{ backgroundColor: shortcut.color }"
    ></div>
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

    <!-- 顶部装饰色条 -->
    <div 
      class="h-1 w-full opacity-80" 
      :style="{ backgroundColor: shortcut.color }"
    ></div>

    <div class="p-4 flex flex-col flex-1 relative z-10">
      <div :class="[workMode ? 'mb-0 items-center' : 'mb-3 items-start', 'flex justify-between']">
        <div class="flex-1 min-w-0 mr-2">
          <h3 
            class="font-bold truncate transition-all duration-300" 
            :class="workMode ? 'text-xl' : 'text-lg'"
            :style="{ color: shortcut.color }"
          >
            {{ shortcut.name }}
          </h3>
          <div class="flex items-center gap-2 mt-1.5">
            <span class="px-2 py-0.5 text-xs font-mono rounded-md border backdrop-blur-md shadow-sm transition-all"
              :style="{ 
                backgroundColor: `color-mix(in srgb, ${shortcut.color} 10%, transparent)`,
                borderColor: `color-mix(in srgb, ${shortcut.color} 20%, transparent)`,
                color: `color-mix(in srgb, ${shortcut.color} 80%, black)`
              }"
            >
              {{ shortcut.target.path }}
            </span>
            <span v-if="shortcut.trigger.type !== 'instant'" class="flex items-center gap-1 text-slate-500 dark:text-slate-400 text-xs">
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
            class="p-2.5 rounded-xl transition-all duration-300 hover:scale-110 hover:-translate-y-0.5 active:scale-95 shadow-sm hover:shadow-md bg-white/50 dark:bg-transparent"
            :style="{ 
              backgroundColor: `color-mix(in srgb, ${shortcut.color} 15%, transparent)`,
              color: shortcut.color,
              boxShadow: `0 4px 12px ${shortcut.color}15`
            }"
            title="立即触发"
          >
            <Play :size="18" fill="currentColor" class="translate-x-[1px]" />
          </button>
          <div class="relative" ref="menuRef">
            <button 
              @click.stop="toggleMenu"
              class="p-2.5 text-slate-400 hover:text-slate-700 dark:hover:text-white transition-all duration-200 rounded-xl hover:bg-slate-100 dark:hover:bg-slate-700/50 active:scale-95"
              :class="{ 'bg-slate-100 dark:bg-slate-700/50 text-slate-700 dark:text-white shadow-inner': showMenu }"
            >
              <MoreVertical :size="18" />
            </button>
            <!-- 弹出菜单 -->
            <div 
              v-if="showMenu"
              class="absolute right-0 top-full mt-2 bg-white/95 dark:bg-slate-800/95 backdrop-blur-xl border border-slate-200 dark:border-slate-700/50 rounded-xl shadow-xl dark:shadow-2xl z-20 w-36 p-1.5 animate-in fade-in zoom-in-95 duration-200 origin-top-right"
            >
              <button 
                @click.stop="$emit('edit', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700/50 hover:text-slate-900 dark:hover:text-white transition-colors rounded-md"
              >
                <Edit2 :size="14" /> 编辑动作
              </button>
              <button 
                @click.stop="$emit('toggle', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700/50 hover:text-slate-900 dark:hover:text-white transition-colors rounded-md"
              >
                <CircleCheck :size="14" /> {{ shortcut.enabled ? '禁用动作' : '启用动作' }}
              </button>
              <div class="h-px bg-slate-200 dark:bg-slate-700/50 my-1 mx-2"></div>
              <button 
                @click.stop="$emit('delete', shortcut.id); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-red-500 dark:text-red-400 hover:bg-red-50 dark:hover:bg-red-500/10 hover:text-red-600 dark:hover:text-red-300 transition-colors rounded-md"
              >
                <Trash2 :size="14" /> 删除动作
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="!workMode" class="mt-auto pt-4 border-t border-slate-200 dark:border-slate-700/50 flex items-center justify-between transition-colors group-hover:border-slate-300 dark:group-hover:border-slate-600/50">
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
</style>
