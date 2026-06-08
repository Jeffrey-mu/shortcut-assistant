<script setup lang="ts">
import { computed, ref, watch, onUnmounted } from 'vue';
import { X, Keyboard, Type, Clock, Palette } from 'lucide-vue-next';
import type { Shortcut, ShortcutTrigger } from '../stores/shortcut';
import { DEFAULT_SHORTCUT_ICON, getShortcutIcon, iconOptions } from '../data/iconOptions';

const props = defineProps<{
  show: boolean;
  editShortcut?: Shortcut;
}>();

const emit = defineEmits(['close', 'save']);

const name = ref('');
const icon = ref(DEFAULT_SHORTCUT_ICON);
const iconSearch = ref('');
const color = ref('#3B82F6');
const targetPath = ref('');
const triggerType = ref<ShortcutTrigger['type']>('instant');
const delaySeconds = ref(3);
const intervalSeconds = ref(5);
const randomMinSeconds = ref(5);
const randomMaxSeconds = ref(30);
const timingValue = ref('');
const isRecording = ref(false);
const isManualMode = ref(false); // 新增：是否为手动输入模式
const recordingMode = ref<'combo' | 'separate'>('combo');
const separateKeys = ref<string[]>([]);
const activeTab = ref<'basic' | 'keys' | 'trigger' | 'style'>('basic');

const tabs = [
  { id: 'basic', label: '基础', icon: Type },
  { id: 'keys', label: '按键', icon: Keyboard },
  { id: 'trigger', label: '触发', icon: Clock },
  { id: 'style', label: '样式', icon: Palette },
] as const;

const resetForm = () => {
  name.value = '';
  icon.value = DEFAULT_SHORTCUT_ICON;
  iconSearch.value = '';
  color.value = '#3B82F6';
  targetPath.value = '';
  triggerType.value = 'instant';
  delaySeconds.value = 3;
  intervalSeconds.value = 5;
  randomMinSeconds.value = 5;
  randomMaxSeconds.value = 30;
  timingValue.value = '';
  separateKeys.value = [];
  recordingMode.value = 'combo';
};

// 监听 editShortcut 变化，实现数据回显
watch(() => props.editShortcut, (newVal) => {
  activeTab.value = 'basic';
  if (newVal) {
    name.value = newVal.name;
    icon.value = newVal.icon || DEFAULT_SHORTCUT_ICON;
    iconSearch.value = '';
    color.value = newVal.color;
    targetPath.value = newVal.target.path;
    triggerType.value = newVal.trigger.type;
    delaySeconds.value = newVal.trigger.delay || 3;
    intervalSeconds.value = newVal.trigger.interval || 5;
    randomMinSeconds.value = newVal.trigger.randomMin || 5;
    randomMaxSeconds.value = newVal.trigger.randomMax || Math.max(randomMinSeconds.value, 30);
    timingValue.value = newVal.trigger.timing || '';
  } else {
    resetForm();
  }
}, { immediate: true });

watch(() => props.show, (newVal) => {
  if (newVal) {
    activeTab.value = 'basic';
  } else {
    stopAllRecording();
  }
});

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

const filteredIconOptions = computed(() => {
  const query = iconSearch.value.trim().toLowerCase();
  if (!query) return iconOptions;
  return iconOptions.filter((option) => {
    return option.label.toLowerCase().includes(query)
      || option.name.toLowerCase().includes(query)
      || option.keywords.toLowerCase().includes(query);
  });
});

// 重新定义录制逻辑
const startRecordingTarget = () => {
  isRecording.value = true;
  if (recordingMode.value === 'separate') {
    separateKeys.value = [];
  }
  targetPath.value = recordingMode.value === 'separate' ? '请分开按下组合键...' : '请按下目标按键...';
  window.addEventListener('keydown', handleKeyDown);
};

const stopAllRecording = () => {
  isRecording.value = false;
  window.removeEventListener('keydown', handleKeyDown);
};

const finishSeparateRecording = () => {
  if (separateKeys.value.length > 0) {
    targetPath.value = separateKeys.value.join('+');
  }
  stopAllRecording();
};

const switchRecordingMode = (mode: 'combo' | 'separate') => {
  recordingMode.value = mode;
  separateKeys.value = [];
  stopAllRecording();
  if (targetPath.value.includes('请按下') || targetPath.value.includes('请分开按下')) {
    targetPath.value = '';
  }
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
  const displayKey = normalizeDisplayKey(key);

  if (recordingMode.value === 'separate') {
    if (!separateKeys.value.includes(displayKey)) {
      separateKeys.value.push(displayKey);
    }
    targetPath.value = separateKeys.value.join('+');
    return;
  }

  if (!['Control', 'Alt', 'Shift', 'Meta'].includes(key)) {
    keys.push(displayKey);
    const result = keys.join('+');
      targetPath.value = result;
      stopAllRecording();
  } else {
    const result = keys.join('+') + '+...';
    targetPath.value = result;
  }
};

const normalizeDisplayKey = (key: string) => {
  if (key === 'Control') return 'Ctrl';
  if (key === 'Alt') return 'Alt';
  if (key === 'Shift') return 'Shift';
  if (key === 'Meta') return 'Command';
  if (key === ' ') return 'Space';
  if (key === 'ArrowUp') return 'Up';
  if (key === 'ArrowDown') return 'Down';
  if (key === 'ArrowLeft') return 'Left';
  if (key === 'ArrowRight') return 'Right';
  if (key === 'Escape') return 'Esc';
  return key.length === 1 ? key.toUpperCase() : key;
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

const isRandomRangeInvalid = computed(() => {
  if (triggerType.value !== 'random') return false;
  return randomMinSeconds.value < 1
    || randomMaxSeconds.value < 1
    || randomMinSeconds.value > randomMaxSeconds.value;
});

const handleSave = () => {
  if (!name.value || isTargetInvalid.value || isRandomRangeInvalid.value) return;
  stopAllRecording();
  
  emit('save', {
    name: name.value,
    icon: icon.value,
    color: color.value,
    enabled: true,
    target: { type: 'keys', path: targetPath.value },
    trigger: { 
      type: triggerType.value,
      delay: triggerType.value === 'delay' ? delaySeconds.value : undefined,
      interval: triggerType.value === 'interval' ? intervalSeconds.value : undefined,
      randomMin: triggerType.value === 'random' ? randomMinSeconds.value : undefined,
      randomMax: triggerType.value === 'random' ? randomMaxSeconds.value : undefined,
      timing: triggerType.value === 'timing' ? timingValue.value : undefined,
    }
  });
  resetForm();
};

const isTargetInvalid = computed(() => {
  return !targetPath.value
    || targetPath.value.includes('...')
    || targetPath.value.includes('请按下')
    || targetPath.value.includes('请分开按下')
    || !targetPath.value.split('+').some((part) => !isModifierKey(part));
});

const isModifierKey = (key: string) => ['Ctrl', 'Control', 'Alt', 'Option', 'Shift', 'Command', 'Cmd', 'Meta', 'Win'].includes(key.trim());

onUnmounted(() => {
  stopAllRecording();
});
</script>

<template>
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center bg-slate-950/55 p-4 backdrop-blur-md transition-colors duration-300 dark:bg-black/76">
    <div class="modal-shell flex max-h-[min(88vh,760px)] w-[min(94vw,600px)] flex-col overflow-hidden rounded-2xl animate-in fade-in zoom-in duration-200 transition-colors duration-300">
      <!-- 头部 -->
      <div class="tool-header shrink-0 px-5 py-4 transition-colors duration-300">
        <div class="flex items-center justify-between gap-3">
          <h2 class="text-lg font-bold text-slate-800 dark:text-white">{{ editShortcut ? '编辑动作' : '添加动作' }}</h2>
          <button @click="$emit('close')" class="rounded-lg p-1 text-slate-500 transition-colors hover:bg-slate-100 hover:text-slate-900 dark:text-slate-400 dark:hover:bg-slate-800 dark:hover:text-white">
            <X :size="20" />
          </button>
        </div>
        <div class="segmented-control mt-4 grid grid-cols-4">
          <button
            v-for="tab in tabs"
            :key="tab.id"
            @click="activeTab = tab.id"
            class="flex min-w-0 items-center justify-center gap-1.5 rounded-lg px-2 py-2 text-xs font-semibold transition-all"
            :style="activeTab === tab.id ? { backgroundColor: 'var(--accent-soft)', color: 'var(--accent-contrast)' } : undefined"
            :class="activeTab === tab.id ? 'shadow-sm' : 'text-slate-500 hover:text-slate-900 dark:text-slate-400 dark:hover:text-white'"
            :title="tab.label"
          >
            <component :is="tab.icon" :size="15" class="shrink-0" />
            <span class="truncate">{{ tab.label }}</span>
          </button>
        </div>
      </div>

      <!-- 内容 -->
      <div class="app-scrollbar min-h-0 flex-1 overflow-y-auto p-5">
        <!-- 基本信息 -->
        <div v-if="activeTab === 'basic'" class="space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-1.5">动作名称</label>
            <input 
              v-model="name"
              type="text" 
              placeholder="例如：微信截屏、播放掌声" 
              class="form-field w-full px-4 py-2 text-slate-900 outline-none placeholder:text-slate-400 dark:text-white dark:placeholder:text-slate-500"
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-1.5">动作图标</label>
            <div class="flex items-center gap-3 mb-3">
              <div 
                class="w-11 h-11 rounded-lg border flex items-center justify-center shrink-0"
                :style="{ color, borderColor: `color-mix(in srgb, ${color} 35%, transparent)`, backgroundColor: `color-mix(in srgb, ${color} 8%, transparent)` }"
              >
                <component :is="getShortcutIcon(icon)" class="w-6 h-6" />
              </div>
              <input
                v-model="iconSearch"
                type="text"
                placeholder="搜索图标..."
                class="form-field flex-1 px-4 py-2 text-slate-900 outline-none placeholder:text-slate-400 dark:text-white dark:placeholder:text-slate-500"
              />
            </div>
            <div class="app-scrollbar grid max-h-36 grid-cols-8 gap-2 overflow-y-auto pr-1">
              <button
                v-for="option in filteredIconOptions"
                :key="option.name"
                @click="icon = option.name"
                class="aspect-square rounded-lg border flex items-center justify-center transition-all hover:scale-105"
                :class="icon === option.name ? 'text-slate-950 shadow-md' : 'bg-white/52 dark:bg-white/[0.055] border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-300 hover:border-[color:var(--accent)]'"
                :style="icon === option.name ? { background: 'linear-gradient(135deg, var(--accent), var(--accent-2))', borderColor: 'var(--accent)' } : undefined"
                :title="option.label"
              >
                <component :is="option.component" class="w-5 h-5" />
              </button>
            </div>
          </div>
        </div>

        <!-- 目标配置 -->
        <div v-else-if="activeTab === 'keys'" class="space-y-4">
          <div class="flex items-center justify-between">
            <label class="block text-sm font-medium text-slate-600 dark:text-slate-400">转发按键内容</label>
            <button 
              @click="toggleManualMode" 
              class="text-xs text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
            >
              {{ isManualMode ? '切换到录制模式' : '无法录制？手动输入' }}
            </button>
          </div>
          
          <div>
            <!-- 录制模式 -->
            <div v-if="!isManualMode" class="space-y-3">
              <div class="grid grid-cols-2 gap-2">
                <button
                  @click="switchRecordingMode('combo')"
                  class="px-3 py-2 rounded-lg border text-xs font-medium transition-all"
                  :class="recordingMode === 'combo' ? 'text-slate-950 shadow-sm' : 'bg-white/50 dark:bg-white/[0.055] border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-400 hover:bg-white/80 dark:hover:border-white/20'"
                  :style="recordingMode === 'combo' ? { background: 'linear-gradient(135deg, var(--accent), var(--accent-2))', borderColor: 'var(--accent)' } : undefined"
                >
                  组合录入
                </button>
                <button
                  @click="switchRecordingMode('separate')"
                  class="px-3 py-2 rounded-lg border text-xs font-medium transition-all"
                  :class="recordingMode === 'separate' ? 'text-slate-950 shadow-sm' : 'bg-white/50 dark:bg-white/[0.055] border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-400 hover:bg-white/80 dark:hover:border-white/20'"
                  :style="recordingMode === 'separate' ? { background: 'linear-gradient(135deg, var(--accent), var(--accent-2))', borderColor: 'var(--accent)' } : undefined"
                >
                  分开录入
                </button>
              </div>
              <div class="flex gap-2">
                <div 
                  @click="startRecordingTarget"
                  class="form-field flex min-w-0 flex-1 cursor-pointer items-center justify-between px-4 py-2 font-mono text-slate-900 transition-all hover:border-[color:var(--accent)] dark:text-white"
                  :class="{ 'ring-2 ring-[color:var(--accent)]/20': isRecording }"
                >
                  <span class="truncate" :class="{'text-slate-400 dark:text-slate-500': isTargetInvalid}">{{ targetPath || (recordingMode === 'separate' ? '点击后分开按键...' : '点击录制目标快捷键...') }}</span>
                  <Keyboard :size="18" class="text-slate-400 dark:text-slate-500 shrink-0" />
                </div>
                <button
                  v-if="recordingMode === 'separate' && isRecording"
                  @click="finishSeparateRecording"
                  class="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-bold transition-all disabled:opacity-50 disabled:pointer-events-none"
                  :disabled="separateKeys.length === 0"
                >
                  完成
                </button>
              </div>
            </div>

            <!-- 手动输入模式 -->
            <div v-else class="space-y-3 p-4 bg-[color:var(--light-panel)] dark:bg-white/[0.045] rounded-xl border border-slate-200 dark:border-white/10 transition-colors duration-300">
              <div class="flex flex-wrap gap-2">
              <button 
                v-for="(_, mod) in manualModifiers" 
                :key="mod"
                @click="manualModifiers[mod as keyof typeof manualModifiers] = !manualModifiers[mod as keyof typeof manualModifiers]; updateManualShortcut()"
                class="px-3 py-1.5 rounded-md text-xs font-bold transition-all border"
                :class="manualModifiers[mod as keyof typeof manualModifiers] ? 'bg-blue-600 border-blue-500 text-white' : 'bg-[color:var(--light-panel-strong)] dark:bg-white/[0.055] border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-400 hover:bg-[color:var(--light-panel-hover)] dark:hover:bg-white/[0.08]'"
              >
                {{ mod }}
              </button>
              </div>
              <div class="flex items-center gap-2">
                <select 
                  v-model="manualKey" 
                  @change="updateManualShortcut"
                  class="flex-1 bg-[color:var(--light-panel-strong)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded-lg px-3 py-2 text-slate-900 dark:text-white text-sm outline-none focus:ring-2 focus:ring-blue-500 transition-colors duration-300"
                >
                  <option value="">选择按键...</option>
                  <option v-for="k in commonKeys" :key="k" :value="k">{{ k }}</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        <!-- 触发方式 -->
        <div v-else-if="activeTab === 'trigger'" class="space-y-4">
          <label class="block text-sm font-medium text-slate-600 dark:text-slate-400">触发方式</label>
          <div class="grid grid-cols-5 gap-2">
            <button 
              v-for="type in ['instant', 'delay', 'interval', 'random', 'timing']" 
              :key="type"
              @click="triggerType = type as any"
              class="px-2 py-2 rounded-lg border text-xs transition-all"
              :class="triggerType === type ? 'bg-blue-600 border-blue-500 text-white' : 'bg-[color:var(--light-panel-strong)] dark:bg-white/[0.055] border-slate-200 dark:border-white/10 text-slate-600 dark:text-slate-400 hover:bg-[color:var(--light-panel-hover)] dark:hover:border-white/20'"
            >
              {{ type === 'instant' ? '立即' : type === 'delay' ? '延时' : type === 'interval' ? '循环' : type === 'random' ? '随机' : '定时' }}
            </button>
          </div>

          <div v-if="triggerType === 'delay'" class="flex items-center gap-3">
            <span class="text-sm text-slate-600 dark:text-slate-400">延时秒数</span>
            <input v-model.number="delaySeconds" type="number" min="1" class="w-20 bg-[color:var(--light-panel)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded px-2 py-1 text-slate-900 dark:text-white outline-none focus:border-blue-500 transition-colors" />
            <span class="text-sm text-slate-500">秒</span>
          </div>

          <div v-if="triggerType === 'interval'" class="flex items-center gap-3">
            <span class="text-sm text-slate-600 dark:text-slate-400">循环间隔</span>
            <input v-model.number="intervalSeconds" type="number" min="1" class="w-20 bg-[color:var(--light-panel)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded px-2 py-1 text-slate-900 dark:text-white outline-none focus:border-blue-500 transition-colors" />
            <span class="text-sm text-slate-500">秒</span>
          </div>

          <div v-if="triggerType === 'random'" class="space-y-2 rounded-xl border border-slate-200 bg-[color:var(--light-panel)] p-3 dark:border-white/10 dark:bg-white/[0.045]">
            <div class="grid grid-cols-2 gap-3">
              <label class="space-y-1">
                <span class="block text-xs font-semibold text-slate-500 dark:text-slate-400">最小间隔</span>
                <div class="flex items-center gap-2">
                  <input v-model.number="randomMinSeconds" type="number" min="1" class="w-full bg-[color:var(--light-panel-strong)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded px-2 py-1 text-slate-900 dark:text-white outline-none focus:border-blue-500 transition-colors" />
                  <span class="text-sm text-slate-500">秒</span>
                </div>
              </label>
              <label class="space-y-1">
                <span class="block text-xs font-semibold text-slate-500 dark:text-slate-400">最大间隔</span>
                <div class="flex items-center gap-2">
                  <input v-model.number="randomMaxSeconds" type="number" min="1" class="w-full bg-[color:var(--light-panel-strong)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded px-2 py-1 text-slate-900 dark:text-white outline-none focus:border-blue-500 transition-colors" />
                  <span class="text-sm text-slate-500">秒</span>
                </div>
              </label>
            </div>
            <p v-if="isRandomRangeInvalid" class="text-xs font-medium text-red-500 dark:text-red-300">最大间隔必须大于或等于最小间隔。</p>
            <p v-else class="text-xs text-slate-500 dark:text-slate-400">进入直播模式后，会在该范围内随机取间隔并自动触发。</p>
          </div>

          <div v-if="triggerType === 'timing'" class="flex items-center gap-3">
            <span class="text-sm text-slate-600 dark:text-slate-400">触发时间</span>
            <input v-model="timingValue" type="time" class="flex-1 bg-[color:var(--light-panel)] dark:bg-white/[0.055] border border-slate-200 dark:border-white/10 rounded px-2 py-1 text-slate-900 dark:text-white outline-none focus:border-blue-500 transition-colors" />
          </div>
        </div>

        <!-- 颜色选择 -->
        <div v-else>
          <label class="block text-sm font-medium text-slate-600 dark:text-slate-400 mb-2">卡片颜色</label>
          <div class="flex flex-wrap gap-2 items-center">
            <button 
              v-for="c in colors" 
              :key="c"
              @click="color = c"
              class="w-8 h-8 rounded-full border-2 transition-all transform hover:scale-110"
              :style="{ backgroundColor: c, borderColor: color === c ? 'currentColor' : 'transparent' }"
              :class="color === c ? 'text-slate-900 dark:text-white' : ''"
              title="选择预设颜色"
            ></button>
            
            <!-- 自定义颜色选择器 -->
            <div class="relative w-8 h-8 rounded-full border-2 transition-all hover:scale-110 overflow-hidden flex items-center justify-center cursor-pointer"
                 :class="!colors.includes(color) ? 'border-slate-900 dark:border-white' : 'border-slate-300 dark:border-slate-600 border-dashed'"
                 :style="{ backgroundColor: !colors.includes(color) ? color : 'transparent' }"
                 title="自定义颜色">
              <input 
                type="color" 
                v-model="color"
                class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
              />
              <span v-if="colors.includes(color)" class="text-slate-400 text-lg leading-none mb-0.5">+</span>
            </div>
          </div>
        </div>
      </div>

      <!-- 底部 -->
      <div class="tool-header flex shrink-0 justify-end gap-3 border-t px-5 py-4 transition-colors duration-300">
        <button 
          @click="$emit('close')"
          class="px-4 py-2 text-slate-600 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-white/[0.075] rounded-lg transition-colors text-sm font-medium"
        >
          取消
        </button>
        <button 
          @click="handleSave"
          class="primary-action disabled:pointer-events-none disabled:opacity-50"
          :style="{ background: 'linear-gradient(135deg, var(--accent), var(--accent-2))' }"
          :disabled="!name || isTargetInvalid || isRandomRangeInvalid"
        >
          保存配置
        </button>
      </div>
    </div>
  </div>
</template>
