import { defineStore } from 'pinia';
import { v4 as uuidv4 } from 'uuid';
import { writeTextFile, readTextFile, mkdir, exists } from '@tauri-apps/plugin-fs';
import { appDataDir, join } from '@tauri-apps/api/path';

export interface ShortcutTarget {
  type: 'keys'; // 仅保留转发按键类型
  path: string;
}

export interface ShortcutTrigger {
  type: 'instant' | 'timing' | 'delay' | 'interval';
  timing?: string;
  delay?: number;
  interval?: number;
  loopCount?: number;
}

export interface Shortcut {
  id: string;
  name: string;
  color: string;
  enabled: boolean;
  target: ShortcutTarget;
  trigger: ShortcutTrigger;
  sort: number;
  createTime: string;
}

export interface AppSettings {
  autoStart: boolean;
  minimizeToTray: boolean;
  theme: 'light' | 'dark';
  transparentWindow: boolean;
}

const CONFIG_FILE = 'config.json';

export const useShortcutStore = defineStore('shortcut', {
  state: () => ({
    shortcuts: [] as Shortcut[],
    settings: {
      autoStart: true,
      minimizeToTray: true,
      theme: 'dark',
      transparentWindow: false,
    } as AppSettings,
    isLoaded: false,
  }),
  actions: {
    async addShortcut(shortcut: Omit<Shortcut, 'id' | 'createTime' | 'sort'>) {
      const newShortcut: Shortcut = {
        ...shortcut,
        id: uuidv4(),
        createTime: new Date().toISOString(),
        sort: this.shortcuts.length,
      };
      this.shortcuts.push(newShortcut);
      await this.saveToLocal();
    },
    async updateShortcut(id: string, updates: Partial<Shortcut>) {
      const index = this.shortcuts.findIndex(s => s.id === id);
      if (index !== -1) {
        this.shortcuts[index] = { ...this.shortcuts[index], ...updates };
        await this.saveToLocal();
      }
    },
    async deleteShortcut(id: string) {
      this.shortcuts = this.shortcuts.filter(s => s.id !== id);
      await this.saveToLocal();
    },
    async toggleShortcut(id: string) {
      const shortcut = this.shortcuts.find(s => s.id === id);
      if (shortcut) {
        shortcut.enabled = !shortcut.enabled;
        await this.saveToLocal();
      }
    },
    async updateSort(newShortcuts: Shortcut[]) {
      this.shortcuts = newShortcuts.map((s, index) => ({ ...s, sort: index }));
      await this.saveToLocal();
    },
    async saveToLocal() {
      try {
        const dataDir = await appDataDir();
        const configPath = await join(dataDir, CONFIG_FILE);
        
        // 确保目录存在
        if (!(await exists(dataDir))) {
          await mkdir(dataDir, { recursive: true });
        }

        const data = JSON.stringify({
          shortcuts: this.shortcuts,
          settings: this.settings
        }, null, 2);

        await writeTextFile(configPath, data);
        console.log('配置已保存到:', configPath);
      } catch (error) {
        console.error('保存配置失败:', error);
      }
    },
    async loadFromLocal() {
      if (this.isLoaded) return;
      
      try {
        const dataDir = await appDataDir();
        const configPath = await join(dataDir, CONFIG_FILE);

        if (await exists(configPath)) {
          const content = await readTextFile(configPath);
          const data = JSON.parse(content);
          this.shortcuts = data.shortcuts || [];
          this.settings = data.settings || this.settings;
          console.log('配置已加载');
        }
      } catch (error) {
        console.error('加载配置失败:', error);
      } finally {
        this.isLoaded = true;
      }
    }
  },
});
