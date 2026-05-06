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
  CircleCheck,
  AlertTriangle,
  XCircle
} from 'lucide-vue-next';
import type { Shortcut } from '../stores/shortcut';

const props = defineProps<{
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
    class="rounded-xl border overflow-hidden transition-all group relative shadow-lg flex flex-col h-full select-none"
    :class="{ 
      'opacity-60': !shortcut.enabled && !batchMode,
      'ring-2 ring-offset-2 ring-offset-slate-950': selected,
      'cursor-pointer hover:shadow-xl active:scale-[0.98]': workMode,
      'cursor-grab active:cursor-grabbing': !batchMode && !workMode,
      'cursor-pointer': batchMode,
      'no-drag': workMode // 在工作模式下禁用拖拽
    }"
    :style="{ 
      backgroundColor: `${shortcut.color}10`, 
      borderColor: selected ? shortcut.color : `${shortcut.color}40`,
      boxShadow: selected ? `0 0 20px ${shortcut.color}20` : 'none'
    }"
    @click="batchMode ? $emit('select', shortcut.id) : (workMode ? $emit('trigger', shortcut) : null)"
  >
    <!-- 复选框 (仅在批量模式显示) -->
    <div v-if="batchMode" class="absolute top-3 left-3 z-10">
      <div 
        class="w-5 h-5 rounded border flex items-center justify-center transition-all"
        :style="{ 
          backgroundColor: selected ? shortcut.color : 'transparent',
          borderColor: selected ? shortcut.color : `${shortcut.color}60`
        }"
      >
        <Check v-if="selected" :size="14" class="text-white" />
      </div>
    </div>

    <!-- 顶部装饰色条 -->
    <div 
      class="h-1 w-full" 
      :style="{ backgroundColor: shortcut.color }"
    ></div>

    <div class="p-4 flex flex-col flex-1">
      <div :class="[workMode ? 'mb-0' : 'mb-3', 'flex justify-between items-start']">
        <div class="flex-1 min-w-0 mr-2">
          <h3 class="text-lg font-bold text-white truncate" :style="{ color: shortcut.color }">
            {{ shortcut.name }}
          </h3>
          <div class="flex items-center gap-2 mt-1">
            <span class="px-2 py-0.5 text-xs font-mono rounded border backdrop-blur-sm"
              :style="{ 
                backgroundColor: `${shortcut.color}20`,
                borderColor: `${shortcut.color}40`,
                color: 'white'
              }"
            >
              {{ shortcut.target.path }}
            </span>
            <span v-if="shortcut.trigger.type !== 'instant'" class="flex items-center gap-1 text-slate-400 text-xs">
              <Clock v-if="shortcut.trigger.type === 'timing'" :size="12" />
              <Timer v-if="shortcut.trigger.type === 'delay'" :size="12" />
              <Repeat v-if="shortcut.trigger.type === 'interval'" :size="12" />
              {{ shortcut.trigger.type === 'timing' ? '定时' : shortcut.trigger.type === 'delay' ? '延时' : '循环' }}
            </span>
          </div>
        </div>
        
        <div v-if="!workMode" class="flex items-center gap-1 shrink-0">
          <button 
            @click.stop="$emit('trigger', shortcut)"
            class="p-2 rounded-lg transition-all hover:scale-110 active:scale-95"
            :style="{ 
              backgroundColor: `${shortcut.color}20`,
              color: shortcut.color
            }"
            title="立即触发"
          >
            <Play :size="18" fill="currentColor" />
          </button>
          <div class="relative" ref="menuRef">
            <button 
              @click.stop="toggleMenu"
              class="p-2 text-slate-400 hover:text-white transition-colors rounded-lg"
              :class="{ 'bg-slate-700/50 text-white': showMenu }"
            >
              <MoreVertical :size="18" />
            </button>
            <!-- 弹出菜单 -->
            <div 
              v-if="showMenu"
              class="absolute right-0 top-full mt-1 bg-slate-900 border border-slate-700 rounded-lg shadow-2xl z-20 w-32 py-1 animate-in fade-in zoom-in duration-100 origin-top-right"
            >
              <button 
                @click.stop="$emit('edit', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-300 hover:bg-slate-800 hover:text-white transition-colors"
              >
                <Edit2 :size="14" /> 编辑动作
              </button>
              <button 
                @click.stop="$emit('toggle', shortcut); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-slate-300 hover:bg-slate-800 hover:text-white transition-colors"
              >
                <CircleCheck :size="14" /> {{ shortcut.enabled ? '禁用动作' : '启用动作' }}
              </button>
              <div class="h-px bg-slate-700 my-1"></div>
              <button 
                @click.stop="$emit('delete', shortcut.id); closeMenu()" 
                class="w-full flex items-center gap-2 px-3 py-2 text-sm text-red-400 hover:bg-red-500/10 hover:text-red-300 transition-colors"
              >
                <Trash2 :size="14" /> 删除动作
              </button>
            </div>
          </div>
        </div>
      </div>

      <div v-if="!workMode" class="mt-auto pt-3 border-t border-white/5 flex items-center justify-between">
        <span class="text-[10px] text-slate-500 font-mono uppercase tracking-wider">转发动作</span>
        <div class="flex items-center gap-2">
           <div 
            class="w-8 h-4 rounded-full relative transition-colors cursor-pointer"
            :style="{ backgroundColor: shortcut.enabled ? shortcut.color : '#334155' }"
            @click.stop="$emit('toggle', shortcut)"
          >
            <div 
              class="absolute top-0.5 left-0.5 w-3 h-3 bg-white rounded-full transition-transform"
              :class="{ 'translate-x-4': shortcut.enabled }"
            ></div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
</style>
