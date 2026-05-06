# Shortcut Assistant (快捷键管理助手)

基于 **Tauri 2.0** + **Vue 3** + **Tailwind CSS v4** 构建的现代化桌面快捷键管理与自动化触发工具。

这款工具的核心设计理念是：**统一管理并驱动其他软件的快捷键**。通过将繁杂的快捷键（如游戏连招、软件动作、OBS场景切换等）收拢到本助手中，以可视化的卡片进行管理，并通过鼠标点击或自动化策略（定时、循环、延时）来精准、极速地触发目标软件。

---

## 🌟 核心特性

- **🚀 极速无缝触发**：采用底层 Windows API（`SendInput`）结合物理扫描码（ScanCode）模拟真实键盘按键，不切焦点、不抢前台，0 延迟闪电触发目标软件组合键。
- **💻 沉浸式工作模式**：一键隐藏所有侧边栏与菜单，化身为纯粹的悬浮按键面板。支持窗口置顶（Always on Top），边看边点，效率倍增。
- **⏱️ 多样化触发引擎**：
  - **即时触发**：点按卡片立即执行动作。
  - **延时触发**：点击后倒计时执行，适合需要提前准备操作环境的场景。
  - **循环触发**：设定时间间隔，后台自动重复执行。
  - **定时触发**：每天到达指定时间自动执行。
- **🎨 现代化交互与 UI**：
  - 全新的 Tailwind v4 设计，支持暗色模式，带有磨砂玻璃与细腻动画。
  - 卡片支持自由拖拽排序，色彩自定义。
  - 支持多选批量启用、禁用、删除。
- **📦 配置随心迁移**：支持将所有快捷键配置一键导出为 JSON，并在其他设备一键导入。
- **⚙️ 系统级深度集成**：支持开机自动启动、关闭时最小化到系统托盘后台运行。

---

## 🛠️ 技术栈

- **桌面端底座**: [Tauri 2.0](https://v2.tauri.app/)
- **系统层交互**: Rust + `windows-rs` 驱动级键盘模拟与系统托盘管理
- **前端框架**: [Vue 3](https://vuejs.org/) (Composition API) + [Vite](https://vitejs.dev/)
- **状态管理**: [Pinia](https://pinia.vuejs.org/)
- **UI 与样式**: [Tailwind CSS v4](https://tailwindcss.com/) + [Lucide Icons](https://lucide.dev/)
- **拖拽交互**: [SortableJS](https://sortablejs.github.io/Sortable/)
- **持久化与插件**: `@tauri-apps/plugin-fs`、`plugin-dialog`、`plugin-autostart` 等

---

## 🚀 快速开始

### 环境依赖
1. [Node.js](https://nodejs.org/) (v18+)
2. [Rust](https://www.rust-lang.org/tools/install) (1.70+)
3. Windows 桌面开发环境 (C++ Build Tools)

### 安装与运行

1. 安装前端依赖
```bash
npm install
```

2. 启动开发环境
```bash
npm run tauri dev
```

3. 构建生产安装包
```bash
npm run tauri build
```
构建成功后，安装包（`.msi` 或 `.exe`）将生成在 `src-tauri/target/release/bundle/` 目录下。

---

## 💡 使用指南

1. **添加动作**：点击左侧的“添加快捷键”，设置一个易记的名称和颜色，点击输入框直接在键盘上按下你想要录制的组合键（如 `Ctrl + Alt + A`）。
2. **工作模式与置顶**：在顶部搜索栏右侧，点击大头针图标 📌 将窗口置顶；点击全屏图标 🔲 进入工作模式，此时所有卡片都会变成快捷触发按钮。
3. **系统设置**：点击左下角系统设置，可以开启开机自启或关闭时最小化到托盘功能，让助手静默在后台为你工作。

---

## 📝 License

MIT License
