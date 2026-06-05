import { useShortcutStore, type Shortcut } from '../stores/shortcut';
import { invoke } from '@tauri-apps/api/core';

export interface ShortcutTriggerFeedback {
  status: 'running' | 'success' | 'error';
  shortcut: Shortcut;
  message: string;
}

export class ShortcutManager {
  private static instance: ShortcutManager;
  private store = useShortcutStore();
  private timers: Map<string, any> = new Map();
  private keySendQueue: Promise<void> = Promise.resolve();
  private feedbackListener?: (feedback: ShortcutTriggerFeedback) => void;

  private constructor() {}

  public static getInstance(): ShortcutManager {
    if (!ShortcutManager.instance) {
      ShortcutManager.instance = new ShortcutManager();
    }
    return ShortcutManager.instance;
  }

  public setFeedbackListener(listener?: (feedback: ShortcutTriggerFeedback) => void) {
    this.feedbackListener = listener;
  }

  /**
   * 初始化所有任务（如定时任务）
   */
  public async initTasks(options: { liveMode?: boolean } = {}) {
    this.clearTimers();
    if (!options.liveMode) return;
    
    for (const shortcut of this.store.shortcuts) {
      if (shortcut.enabled) {
        // 如果是定时任务，启动定时检查
        if (shortcut.trigger.type === 'timing') {
          this.setupTimingTrigger(shortcut);
        } else if (shortcut.trigger.type === 'interval') {
          this.setupIntervalTrigger(shortcut);
        } else if (shortcut.trigger.type === 'random') {
          this.setupRandomTrigger(shortcut);
        }
      }
    }
  }

  private clearTimers() {
    this.timers.forEach(timer => {
      if (typeof timer === 'number' || timer) {
        clearInterval(timer);
        clearTimeout(timer);
      }
    });
    this.timers.clear();
  }

  private setupTimingTrigger(shortcut: Shortcut) {
    if (!shortcut.trigger.timing) return;

    const checkTiming = () => {
      const now = new Date();
      const timeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}`;
      if (timeStr === shortcut.trigger.timing) {
        this.executeScheduledShortcut(shortcut, '定时触发发送中', '定时触发已发送');
      }
    };

    const timer = setInterval(checkTiming, 60000); // 每分钟检查一次
    this.timers.set(`timing-${shortcut.id}`, timer);
  }

  private setupIntervalTrigger(shortcut: Shortcut) {
    if (!shortcut.trigger.interval) return;

    const timer = setInterval(() => {
      this.executeScheduledShortcut(shortcut, '循环触发发送中', '循环触发已发送');
    }, shortcut.trigger.interval * 1000);
    this.timers.set(`interval-${shortcut.id}`, timer);
  }

  private setupRandomTrigger(shortcut: Shortcut) {
    const min = Math.max(1, Math.floor(shortcut.trigger.randomMin || 1));
    const max = Math.max(min, Math.floor(shortcut.trigger.randomMax || min));

    const scheduleNext = () => {
      const delaySeconds = this.getRandomSeconds(min, max);
      const timer = setTimeout(async () => {
        if (!this.timers.has(`random-${shortcut.id}`)) return;
        await this.executeScheduledShortcut(shortcut, '随机触发发送中', '随机触发已发送');
        if (this.timers.has(`random-${shortcut.id}`)) {
          scheduleNext();
        }
      }, delaySeconds * 1000);
      this.timers.set(`random-${shortcut.id}`, timer);
    };

    scheduleNext();
  }

  private getRandomSeconds(min: number, max: number) {
    return Math.floor(Math.random() * (max - min + 1)) + min;
  }

  private async executeScheduledShortcut(shortcut: Shortcut, runningMessage: string, successMessage: string) {
    this.emitFeedback({ status: 'running', shortcut, message: runningMessage });
    try {
      await this.doExecute(shortcut);
      this.emitFeedback({ status: 'success', shortcut, message: successMessage });
    } catch (error) {
      console.error(`${runningMessage}: ${shortcut.name}`, error);
      this.emitFeedback({
        status: 'error',
        shortcut,
        message: error instanceof Error ? error.message : String(error),
      });
    }
  }

  private emitFeedback(feedback: ShortcutTriggerFeedback) {
    this.feedbackListener?.(feedback);
  }

  /**
   * 执行快捷键操作 (通常由手动点击触发)
   */
  public async executeShortcut(shortcut: Shortcut): Promise<void> {
    if (!shortcut.enabled) {
      throw new Error('动作已禁用');
    }
    
    console.log(`手动触发动作: ${shortcut.name}, 模式: ${shortcut.trigger.type}`);
    
    if (shortcut.trigger.type === 'delay' && shortcut.trigger.delay) {
      await new Promise(resolve => setTimeout(resolve, shortcut.trigger.delay! * 1000));
      await this.doExecute(shortcut);
    } else {
      await this.doExecute(shortcut);
    }
  }

  /**
   * 实际执行按键转发逻辑
   */
  private doExecute(shortcut: Shortcut): Promise<void> {
    console.log(`正在执行动作: ${shortcut.name}`);
    if (shortcut.target.type === 'keys') {
      const keys = shortcut.target.path;
      this.keySendQueue = this.keySendQueue
        .then(() => this.simulateKeys(keys))
        .catch((err) => {
          console.error('队列执行出现异常:', err);
          throw err;
        });
      return this.keySendQueue;
    }
    return Promise.resolve();
  }

  private async simulateKeys(keys: string) {
    console.log(`模拟按键: ${keys}`);
    await invoke('simulate_keys', { keys });
  }
}
