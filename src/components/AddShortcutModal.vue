<script setup lang="ts">
import { ref, watch, onUnmounted } from 'vue';
import { X, Keyboard } from 'lucide-vue-next';
import type { Shortcut } from '../stores/shortcut';

const props = defineProps<{
  show: boolean;
  editShortcut?: Shortcut;
}>();

const emit = defineEmits(['close', 'save']);

const name = ref('');
const color = ref('#3B82F6');
const targetPath = ref('');
const triggerType = ref<'instant' | 'timing' | 'delay' | 'interval'>('instant');
const delaySeconds = ref(3);
const intervalSeconds = ref(5);
const timingValue = ref('');

const resetForm = () => {
  name.value = '';
  color.value = '#3B82F6';
  targetPath.value = '';
  triggerType.value = 'instant';
};

// 监听 editShortcut 变化，实现数据回显
watch(() => props.editShortcut, (newVal) => {
  if (newVal) {
    name.value = newVal.name;
    color.value = newVal.color;
    targetPath.value = newVal.target.path;
    triggerType.value = newVal.trigger.type;
    delaySeconds.value = newVal.trigger.delay || 3;
    intervalSeconds.value = newVal.trigger.interval || 5;
    timingValue.value = newVal.trigger.timing || '';
  } else {
    resetForm();
  }
}, { immediate: true });

const isRecording = ref(false);
const isManualMode = ref(false); // 新增：是否为手动输入模式
const manualModifiers = ref({
  Ctrl: false,
  Alt: false,
  Shift: false,
  Win: false
});
const manualKey = ref('');

const commonKeys = [
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
  'F1', 'F2', 'F3', 'F4', 'F5', 'F6', 'F7', 'F8', 'F9', 'F10', 'F11', 'F12',
  'Enter', 'Space', 'Esc', 'Tab', 'Backspace', 'Delete', 'Insert', 'Home', 'End', 'PageUp', 'PageDown', 'Up', 'Down', 'Left', 'Right'
];

const colors = [
  '#3B82F6', '#EF4444', '#10B981', '#F59E0B', 
  '#8B5CF6', '#EC4899', '#6366F1', '#14B8A6'
];

// 重新定义录制逻辑
const startRecordingTarget = () => {
  isRecording.value = true;
  targetPath.value = '请按下目标按键...';
  window.addEventListener('keydown', handleKeyDown);
};

const stopAllRecording = () => {
  isRecording.value = false;
  window.removeEventListener('keydown', handleKeyDown);
};

const handleKeyDown = (e: KeyboardEvent) => {
  e.preventDefault();
  e.stopPropagation();
  
  const keys: string[] = [];
  if (e.ctrlKey) keys.push('Ctrl');
  if (e.altKey) keys.push('Alt');
  if (e.shiftKey) keys.push('Shift');
  if (e.metaKey) keys.push('Command');

  const key = e.key;
  if (!['Control', 'Alt', 'Shift', 'Meta'].includes(key)) {
    let displayKey = key.toUpperCase();
    if (key === ' ') displayKey = 'Space';
    if (key === 'ArrowUp') displayKey = 'Up';
    if (key === 'ArrowDown') displayKey = 'Down';
    if (key === 'ArrowLeft') displayKey = 'Left';
    if (key === 'ArrowRight') displayKey = 'Right';
    
    keys.push(displayKey);
    const result = keys.join('+');
    targetPath.value = result;
    stopAllRecording();
  } else {
    const result = keys.join('+') + '+...';
    targetPath.value = result;
  }
};

const toggleManualMode = () => {
  isManualMode.value = !isManualMode.value;
  if (isManualMode.value) {
    stopAllRecording();
    // 尝试解析现有目标路径到手动模式
    if (targetPath.value && !targetPath.value.includes('...')) {
      const parts = targetPath.value.split('+');
      manualModifiers.value.Ctrl = parts.includes('Ctrl');
      manualModifiers.value.Alt = parts.includes('Alt');
      manualModifiers.value.Shift = parts.includes('Shift');
      manualModifiers.value.Win = parts.includes('Command') || parts.includes('Win');
      manualKey.value = parts[parts.length - 1];
    }
  }
};

const updateManualShortcut = () => {
  const keys: string[] = [];
  if (manualModifiers.value.Ctrl) keys.push('Ctrl');
  if (manualModifiers.value.Alt) keys.push('Alt');
  if (manualModifiers.value.Shift) keys.push('Shift');
  if (manualModifiers.value.Win) keys.push('Command');
  
  if (manualKey.value) {
    keys.push(manualKey.value);
    targetPath.value = keys.join('+');
  } else if (keys.length > 0) {
    targetPath.value = keys.join('+') + '+...';
  } else {
    targetPath.value = '';
  }
};

const handleSave = () => {
  if (!name.value || !targetPath.value || targetPath.value.includes('...')) return;
  
  emit('save', {
    name: name.value,
    color: color.value,
    enabled: true,
    target: { type: 'keys', path: targetPath.value },
    trigger: { 
      type: triggerType.value,
      delay: triggerType.value === 'delay' ? delaySeconds.value : undefined,
      interval: triggerType.value === 'interval' ? intervalSeconds.value : undefined,
      timing: triggerType.value === 'timing' ? timingValue.value : undefined,
    }
  });
  resetForm();
};

onUnmounted(() => {
  stopAllRecording();
});
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm">
    <div class="bg-slate-900 border border-slate-700 rounded-2xl shadow-2xl w-full max-w-lg overflow-hidden animate-in fade-in zoom-in duration-200">
      <!-- 头部 -->
      <div class="flex items-center justify-between px-6 py-4 border-b border-slate-800">
        <h2 class="text-xl font-bold text-white">{{ editShortcut ? '编辑动作' : '添加动作' }}</h2>
        <button @click="$emit('close')" class="text-slate-400 hover:text-white transition-colors">
          <X :size="20" />
        </button>
      </div>

      <!-- 内容 -->
      <div class="p-6 space-y-6">
        <!-- 基本信息 -->
        <div class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-400 mb-1.5">动作名称</label>
            <input 
              v-model="name"
              type="text" 
              placeholder="例如：微信截屏、播放掌声" 
              class="w-full bg-slate-800 border border-slate-700 rounded-lg px-4 py-2 text-white focus:ring-2 focus:ring-blue-500 focus:border-transparent outline-none transition-all"
            />
          </div>
        </div>

        <!-- 目标配置 -->
        <div class="space-y-4">
          <div class="flex items-center justify-between">
            <label class="block text-sm font-medium text-slate-400">转发按键内容</label>
            <button 
              @click="toggleManualMode" 
              class="text-xs text-blue-400 hover:text-blue-300 transition-colors"
            >
              {{ isManualMode ? '切换到录制模式' : '无法录制？手动输入' }}
            </button>
          </div>
          
          <div>
            <!-- 录制模式 -->
            <div v-if="!isManualMode" class="flex gap-2">
              <div 
                @click="startRecordingTarget"
                class="flex-1 bg-slate-800 border border-slate-700 rounded-lg px-4 py-2 text-white font-mono cursor-pointer hover:border-blue-500 transition-all flex items-center justify-between"
                :class="{ 'border-blue-500 ring-2 ring-blue-500/20': isRecording }"
              >
                <span>{{ targetPath || '点击录制目标快捷键...' }}</span>
                <Keyboard :size="18" class="text-slate-500" />
              </div>
            </div>

            <!-- 手动输入模式 -->
            <div v-else class="space-y-3 p-4 bg-slate-800/50 rounded-xl border border-slate-700/50">
              <div class="flex flex-wrap gap-2">
              <button 
                v-for="(_, mod) in manualModifiers" 
                :key="mod"
                @click="manualModifiers[mod as keyof typeof manualModifiers] = !manualModifiers[mod as keyof typeof manualModifiers]; updateManualShortcut()"
                class="px-3 py-1.5 rounded-md text-xs font-bold transition-all border"
                :class="manualModifiers[mod as keyof typeof manualModifiers] ? 'bg-blue-600 border-blue-500 text-white' : 'bg-slate-800 border-slate-700 text-slate-400'"
              >
                {{ mod }}
              </button>
              </div>
              <div class="flex items-center gap-2">
                <select 
                  v-model="manualKey" 
                  @change="updateManualShortcut"
                  class="flex-1 bg-slate-800 border border-slate-700 rounded-lg px-3 py-2 text-white text-sm outline-none focus:ring-2 focus:ring-blue-500"
                >
                  <option value="">选择按键...</option>
                  <option v-for="k in commonKeys" :key="k" :value="k">{{ k }}</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <!-- 触发方式 -->
        <div class="space-y-4">
          <label class="block text-sm font-medium text-slate-400">触发方式</label>
          <div class="grid grid-cols-4 gap-2">
            <button 
              v-for="type in ['instant', 'delay', 'interval', 'timing']" 
              :key="type"
              @click="triggerType = type as any"
              class="px-2 py-2 rounded-lg border text-xs transition-all"
              :class="triggerType === type ? 'bg-blue-600 border-blue-500 text-white' : 'bg-slate-800 border-slate-700 text-slate-400 hover:border-slate-600'"
            >
              {{ type === 'instant' ? '立即' : type === 'delay' ? '延时' : type === 'interval' ? '循环' : '定时' }}
            </button>
          </div>

          <div v-if="triggerType === 'delay'" class="flex items-center gap-3">
            <span class="text-sm text-slate-400">延时秒数</span>
            <input v-model.number="delaySeconds" type="number" min="1" class="w-20 bg-slate-800 border border-slate-700 rounded px-2 py-1 text-white outline-none focus:border-blue-500" />
            <span class="text-sm text-slate-500">秒</span>
          </div>

          <div v-if="triggerType === 'interval'" class="flex items-center gap-3">
            <span class="text-sm text-slate-400">循环间隔</span>
            <input v-model.number="intervalSeconds" type="number" min="1" class="w-20 bg-slate-800 border border-slate-700 rounded px-2 py-1 text-white outline-none focus:border-blue-500" />
            <span class="text-sm text-slate-500">秒</span>
          </div>

          <div v-if="triggerType === 'timing'" class="flex items-center gap-3">
            <span class="text-sm text-slate-400">触发时间</span>
            <input v-model="timingValue" type="time" class="flex-1 bg-slate-800 border border-slate-700 rounded px-2 py-1 text-white outline-none focus:border-blue-500" />
          </div>
        </div>

        <!-- 颜色选择 -->
        <div>
          <label class="block text-sm font-medium text-slate-400 mb-2">卡片颜色</label>
          <div class="flex flex-wrap gap-2">
            <button 
              v-for="c in colors" 
              :key="c"
              @click="color = c"
              class="w-8 h-8 rounded-full border-2 transition-all transform hover:scale-110"
              :style="{ backgroundColor: c, borderColor: color === c ? 'white' : 'transparent' }"
            ></button>
          </div>
        </div>
      </div>

      <!-- 底部 -->
      <div class="px-6 py-4 bg-slate-800/50 border-t border-slate-800 flex justify-end gap-3">
        <button 
          @click="$emit('close')"
          class="px-4 py-2 text-slate-400 hover:text-white transition-colors text-sm font-medium"
        >
          取消
        </button>
        <button 
          @click="handleSave"
          class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold transition-all shadow-lg shadow-blue-600/20 active:scale-95 disabled:opacity-50 disabled:pointer-events-none"
          :disabled="!name || !targetPath || targetPath.includes('...')"
        >
          保存配置
        </button>
      </div>
    </div>
  </div>
</template>
