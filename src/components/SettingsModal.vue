<script setup lang="ts">
import { ref, watch } from 'vue';
import { X, Save, Monitor, ToggleLeft, ToggleRight, Moon, Sun } from 'lucide-vue-next';
import { useShortcutStore, type AppSettings } from '../stores/shortcut';
import { enable, disable, isEnabled } from '@tauri-apps/plugin-autostart';

const props = defineProps<{
  show: boolean;
}>();

const emit = defineEmits(['close']);
const store = useShortcutStore();

const localSettings = ref<AppSettings>({
  autoStart: true,
  minimizeToTray: true,
  theme: 'dark',
});

// 初始化数据
watch(() => props.show, async (newVal) => {
  if (newVal) {
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
  <div v-if="show" class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="absolute inset-0 bg-slate-900/40 dark:bg-slate-950/80 backdrop-blur-sm transition-colors duration-300" @click="emit('close')"></div>
    
    <div class="relative bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden animate-in zoom-in-95 duration-200 transition-colors duration-300">
      <div class="px-6 py-4 border-b border-slate-200 dark:border-slate-800 flex items-center justify-between bg-slate-50 dark:bg-slate-900/50 transition-colors duration-300">
        <h2 class="text-xl font-bold text-slate-800 dark:text-white flex items-center gap-2">
          <Monitor :size="20" class="text-blue-500 dark:text-blue-400" />
          系统设置
        </h2>
        <button @click="emit('close')" class="text-slate-500 dark:text-slate-400 hover:text-slate-900 dark:hover:text-white transition-colors p-1 rounded-lg hover:bg-slate-100 dark:hover:bg-slate-800">
          <X :size="20" />
        </button>
      </div>

      <div class="p-6 space-y-6">
        <!-- 设置项 -->
        <div class="space-y-4">
          <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700/50 transition-colors duration-300">
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

          <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700/50 transition-colors duration-300">
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

          <div class="flex items-center justify-between p-3 rounded-xl bg-slate-50 dark:bg-slate-800/50 border border-slate-200 dark:border-slate-700/50 transition-colors duration-300">
            <div>
              <div class="text-slate-700 dark:text-slate-200 font-medium">应用主题</div>
              <div class="text-slate-500 dark:text-slate-500 text-xs mt-0.5">亮色/暗色模式切换</div>
            </div>
            <div class="flex bg-slate-200 dark:bg-slate-900 rounded-lg p-1 border border-slate-300 dark:border-slate-700 transition-colors duration-300">
              <button 
                @click="localSettings.theme = 'light'"
                class="p-1.5 rounded-md transition-colors"
                :class="localSettings.theme === 'light' ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'"
              >
                <Sun :size="16" />
              </button>
              <button 
                @click="localSettings.theme = 'dark'"
                class="p-1.5 rounded-md transition-colors"
                :class="localSettings.theme === 'dark' ? 'bg-white dark:bg-slate-700 text-slate-900 dark:text-white shadow-sm' : 'text-slate-500 dark:text-slate-500 hover:text-slate-700 dark:hover:text-slate-300'"
              >
                <Moon :size="16" />
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="px-6 py-4 bg-slate-50 dark:bg-slate-800/30 border-t border-slate-200 dark:border-slate-800 flex justify-end gap-3 transition-colors duration-300">
        <button 
          @click="emit('close')"
          class="px-4 py-2 rounded-lg text-sm font-medium text-slate-600 dark:text-slate-300 hover:text-slate-900 dark:hover:text-white hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
        >
          取消
        </button>
        <button 
          @click="handleSave"
          class="px-6 py-2 rounded-lg text-sm font-medium bg-blue-600 hover:bg-blue-700 dark:hover:bg-blue-500 text-white shadow-lg shadow-blue-500/20 flex items-center gap-2 transition-all active:scale-95"
        >
          <Save :size="16" />
          保存设置
        </button>
      </div>
    </div>
  </div>
</template>
