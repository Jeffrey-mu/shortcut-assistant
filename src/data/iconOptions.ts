import type { Component } from 'vue';

import KeyboardIcon from 'virtual:icons/tabler/keyboard';
import CameraIcon from 'virtual:icons/tabler/camera';
import ClipboardIcon from 'virtual:icons/tabler/clipboard';
import MusicIcon from 'virtual:icons/tabler/music';
import PlayerPlayIcon from 'virtual:icons/tabler/player-play';
import PlayerSkipForwardIcon from 'virtual:icons/tabler/player-skip-forward';
import ScissorsIcon from 'virtual:icons/tabler/scissors';
import PhotoIcon from 'virtual:icons/tabler/photo';
import TerminalIcon from 'virtual:icons/tabler/terminal-2';
import AppWindowIcon from 'virtual:icons/tabler/app-window';
import BrowserIcon from 'virtual:icons/tabler/browser';
import MessageIcon from 'virtual:icons/tabler/message';
import BellIcon from 'virtual:icons/tabler/bell';
import MailIcon from 'virtual:icons/tabler/mail';
import CopyIcon from 'virtual:icons/tabler/copy';
import DownloadIcon from 'virtual:icons/tabler/download';
import UploadIcon from 'virtual:icons/tabler/upload';
import SettingsIcon from 'virtual:icons/tabler/settings';
import SearchIcon from 'virtual:icons/tabler/search';
import VolumeIcon from 'virtual:icons/tabler/volume';
import MicrophoneIcon from 'virtual:icons/tabler/microphone';
import VideoIcon from 'virtual:icons/tabler/video';
import ScreenShareIcon from 'virtual:icons/tabler/screen-share';
import DeviceDesktopIcon from 'virtual:icons/tabler/device-desktop';
import DeviceLaptopIcon from 'virtual:icons/tabler/device-laptop';
import BrandWechatIcon from 'virtual:icons/tabler/brand-wechat';
import BrandAppleIcon from 'virtual:icons/tabler/brand-apple';
import CommandIcon from 'virtual:icons/tabler/command';
import CalendarIcon from 'virtual:icons/tabler/calendar';
import ClockIcon from 'virtual:icons/tabler/clock';
import DatabaseIcon from 'virtual:icons/tabler/database';
import FolderIcon from 'virtual:icons/tabler/folder';
import FileIcon from 'virtual:icons/tabler/file';
import BoltIcon from 'virtual:icons/tabler/bolt';
import StarIcon from 'virtual:icons/tabler/star';
import HeartIcon from 'virtual:icons/tabler/heart';
import BookmarkIcon from 'virtual:icons/tabler/bookmark';
import LockIcon from 'virtual:icons/tabler/lock';
import KeyIcon from 'virtual:icons/tabler/key';
import WifiIcon from 'virtual:icons/tabler/wifi';
import CloudIcon from 'virtual:icons/tabler/cloud';
import TrashIcon from 'virtual:icons/tabler/trash';
import EditIcon from 'virtual:icons/tabler/edit';
import PlusIcon from 'virtual:icons/tabler/plus';
import MinusIcon from 'virtual:icons/tabler/minus';
import RefreshIcon from 'virtual:icons/tabler/refresh';
import ArrowUpIcon from 'virtual:icons/tabler/arrow-up';
import ArrowDownIcon from 'virtual:icons/tabler/arrow-down';
import HomeIcon from 'virtual:icons/tabler/home';
import RobotIcon from 'virtual:icons/tabler/robot';

export interface IconOption {
  name: string;
  label: string;
  keywords: string;
  component: Component;
}

export const DEFAULT_SHORTCUT_ICON = 'keyboard';

export const iconOptions: IconOption[] = [
  { name: 'keyboard', label: '键盘', keywords: 'keyboard hotkey shortcut keys', component: KeyboardIcon },
  { name: 'camera', label: '截图', keywords: 'camera screenshot capture', component: CameraIcon },
  { name: 'scissors', label: '裁剪', keywords: 'scissors cut snip', component: ScissorsIcon },
  { name: 'clipboard', label: '剪贴板', keywords: 'clipboard paste board', component: ClipboardIcon },
  { name: 'copy', label: '复制', keywords: 'copy duplicate', component: CopyIcon },
  { name: 'music', label: '音乐', keywords: 'music audio song', component: MusicIcon },
  { name: 'player-skip-forward', label: '切歌', keywords: 'next skip song track music', component: PlayerSkipForwardIcon },
  { name: 'player-play', label: '播放', keywords: 'play media start', component: PlayerPlayIcon },
  { name: 'volume', label: '音量', keywords: 'volume sound audio', component: VolumeIcon },
  { name: 'microphone', label: '麦克风', keywords: 'microphone mic voice', component: MicrophoneIcon },
  { name: 'video', label: '视频', keywords: 'video record movie', component: VideoIcon },
  { name: 'screen-share', label: '屏幕', keywords: 'screen share display', component: ScreenShareIcon },
  { name: 'photo', label: '图片', keywords: 'photo image picture', component: PhotoIcon },
  { name: 'terminal-2', label: '终端', keywords: 'terminal shell command', component: TerminalIcon },
  { name: 'app-window', label: '窗口', keywords: 'window app panel', component: AppWindowIcon },
  { name: 'browser', label: '浏览器', keywords: 'browser web chrome', component: BrowserIcon },
  { name: 'device-desktop', label: '桌面', keywords: 'desktop computer monitor', component: DeviceDesktopIcon },
  { name: 'device-laptop', label: '电脑', keywords: 'laptop mac computer', component: DeviceLaptopIcon },
  { name: 'brand-apple', label: 'Mac', keywords: 'apple mac command', component: BrandAppleIcon },
  { name: 'command', label: 'Command', keywords: 'command mac meta', component: CommandIcon },
  { name: 'message', label: '消息', keywords: 'message chat talk', component: MessageIcon },
  { name: 'brand-wechat', label: '微信', keywords: 'wechat weixin chat', component: BrandWechatIcon },
  { name: 'bell', label: '提醒', keywords: 'bell alert notify', component: BellIcon },
  { name: 'mail', label: '邮件', keywords: 'mail email message', component: MailIcon },
  { name: 'download', label: '下载', keywords: 'download save', component: DownloadIcon },
  { name: 'upload', label: '上传', keywords: 'upload send', component: UploadIcon },
  { name: 'settings', label: '设置', keywords: 'settings config gear', component: SettingsIcon },
  { name: 'search', label: '搜索', keywords: 'search find', component: SearchIcon },
  { name: 'calendar', label: '日历', keywords: 'calendar date time', component: CalendarIcon },
  { name: 'clock', label: '时间', keywords: 'clock timer time', component: ClockIcon },
  { name: 'database', label: '数据', keywords: 'database data store', component: DatabaseIcon },
  { name: 'folder', label: '文件夹', keywords: 'folder directory', component: FolderIcon },
  { name: 'file', label: '文件', keywords: 'file document', component: FileIcon },
  { name: 'bolt', label: '闪电', keywords: 'bolt power fast', component: BoltIcon },
  { name: 'star', label: '星标', keywords: 'star favorite', component: StarIcon },
  { name: 'heart', label: '喜欢', keywords: 'heart like love', component: HeartIcon },
  { name: 'bookmark', label: '书签', keywords: 'bookmark mark', component: BookmarkIcon },
  { name: 'lock', label: '锁定', keywords: 'lock secure', component: LockIcon },
  { name: 'key', label: '钥匙', keywords: 'key password', component: KeyIcon },
  { name: 'wifi', label: '网络', keywords: 'wifi network', component: WifiIcon },
  { name: 'cloud', label: '云端', keywords: 'cloud sync', component: CloudIcon },
  { name: 'trash', label: '删除', keywords: 'trash delete remove', component: TrashIcon },
  { name: 'edit', label: '编辑', keywords: 'edit write pencil', component: EditIcon },
  { name: 'plus', label: '加号', keywords: 'plus add', component: PlusIcon },
  { name: 'minus', label: '减号', keywords: 'minus reduce', component: MinusIcon },
  { name: 'refresh', label: '刷新', keywords: 'refresh reload sync', component: RefreshIcon },
  { name: 'arrow-up', label: '上移', keywords: 'arrow up move', component: ArrowUpIcon },
  { name: 'arrow-down', label: '下移', keywords: 'arrow down move', component: ArrowDownIcon },
  { name: 'home', label: '首页', keywords: 'home house', component: HomeIcon },
  { name: 'robot', label: '自动化', keywords: 'robot bot automation ai', component: RobotIcon },
];

export const iconComponentMap = iconOptions.reduce<Record<string, Component>>((map, option) => {
  map[option.name] = option.component;
  return map;
}, {});

export const getShortcutIcon = (name?: string) => {
  return iconComponentMap[name || DEFAULT_SHORTCUT_ICON] || iconComponentMap[DEFAULT_SHORTCUT_ICON];
};
