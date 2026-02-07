# Rust Tools - 工具包

基于 Tauri 2 的跨平台桌面工具集合，包含多种实用开发工具。

## ✨ 功能

### XMind 转 Markdown
- 支持拖拽上传 `.xmind` 文件
- 一键转换为 Markdown 格式
- 实时预览转换结果
- 下载/另存为 `.md` 文件
- 支持多级标题（1-6 级）、备注（引用块）
- 仅支持含 `content.json` 的 XMind 新版格式（XMind 8 / XMind Zen）

## 🛠 技术栈

- **前端**: Vite + 原生 JavaScript + Marked.js
- **后端**: Tauri 2 + Rust
- **UI**: 简洁现代风格
- **核心库**: zip、serde_json（XMind 解析）

## 📦 开发

### 前置要求

#### 所有平台
- Node.js 18+
- Rust 1.70+
- npm 或 yarn

#### macOS
- [Xcode Command Line Tools](https://developer.apple.com/xcode/)
  ```bash
  xcode-select --install
  ```

#### Windows
- [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)（Win10/11 通常已预装）
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)（MSVC 工具链）

#### Linux
- webkit2gtk、libgtk 等依赖，参见 [Tauri 文档](https://v2.tauri.app/start/install/)

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

如遇到 `__TAURI_INTERNALS__` 为 undefined 或选择文件失败，可使用先构建再运行的模式：

```bash
npm run tauri:dev
```

### 构建前端

```bash
npm run build
```

## 🚀 打包

### 快速打包（推荐）

```bash
# Release 模式打包（体积小、性能好）
./scripts/build.sh

# Debug 模式打包（更快，但体积更大）
./scripts/build.sh --debug
```

### 手动打包

```bash
# 1. 构建前端
npm run build

# 2. 打包 Tauri 应用
npm run tauri:build
```

### 打包产物位置

- **macOS**: 
  - `.app`: `src-tauri/target/release/bundle/macos/`
  - `.dmg`: `src-tauri/target/release/bundle/dmg/`
- **Windows**:
  - `.msi`: `src-tauri/target/release/bundle/msi/`
  - `.exe`: `src-tauri/target/release/bundle/nsis/`
- **Linux**:
  - `.deb`: `src-tauri/target/release/bundle/deb/`
  - `.AppImage`: `src-tauri/target/release/bundle/appimage/`

## 📝 可用脚本

```bash
npm run dev                  # 启动 Vite 开发服务器
npm run build                # 构建前端资源
npm run tauri:dev            # 启动 Tauri 开发模式（先构建）
npm run tauri:build          # 打包应用（release）
npm run tauri:build:debug    # 打包应用（debug）
npm run clean                # 清理所有构建文件
npm run clean:build          # 清理构建输出
```

## 🐛 常见问题

### macOS 安装后打开无窗口
- 检查「系统设置 → 隐私与安全性」是否有拦截提示
- 尝试重新打包或在终端运行 `.app/Contents/MacOS/rust-tools` 查看日志

### 打包后白屏/功能无反应
- 确保先执行 `npm run build` 再打包
- 使用 `./scripts/build.sh` 自动化打包流程

### 转换失败：未找到 content.json
- 该 XMind 文件为旧版格式（content.xml）
- 请在 XMind 中「另存为」或「导出」为新版 .xmind 格式

## 📄 许可证

MIT License

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！
