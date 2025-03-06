# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 技术选型

 * 基础架构：`tauri`

 * 前端架构：`vite + vue3`

 * UI框架：`Naive UI`

 * 状态管理：`Pinia`

 * 图标库：待定

 * 项目架构

   ```
   desk-flow/
   ├── src-tauri/          # Tauri 主进程代码（不变）
   │   ├── Cargo.toml     
   │   └── src/
   │       ├── main.rs    
   │       └── commands.rs 
   │
   ├── src/               # 前端代码（调整为 JS）
   │   ├── assets/
   │   │   └── icons/     # 存放 Fluent System Icons 的 SVG 文件
   │   ├── components/    
   │   │   ├── widgets/
   │   │   │   ├── ClipboardManager.js  # 改为 .js 文件
   │   │   │   ├── WeatherWidget.js     
   │   │   │   └── FloatingNote.js      
   │   │   └── DesktopGrid.js          
   │   │
   │   ├── stores/        # Pinia 状态管理（JS 写法）
   │   │   ├── desktop.js 
   │   │   └── clipboard.js 
   │   │
   │   ├── hooks/         # 自定义 Hooks（JS 实现）
   │   │   ├── useFileOrganizer.js 
   │   │   └── useWindowsWeather.js 
   │   │
   │   ├── main.js        # Vue 入口（移除 TS 类型）
   │   └── style/         
   │
   ├── index.html         
   ├── vite.config.js     # 移除 TS 插件
   └── tauri.conf.json    
   ```

   

