# Changelog

所有重要的变更都会记录在这个文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.0.0/)，
并且本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [0.1.0] - 2026-02-07

### 新增
- 🎉 初始版本发布
- ✨ XMind 转 Markdown 功能
  - 支持拖拽上传 .xmind 文件
  - 实时预览转换结果
  - 下载/另存为 .md 文件
- 🔄 同时支持新旧版本 XMind
  - XMind Zen / XMind 2020+ (content.json)
  - XMind 8 及更早版本 (content.xml)
- 🎨 简洁现代的用户界面
  - 侧边栏工具包导航
  - 清爽的浅色主题
- 📦 跨平台支持
  - macOS (Intel & Apple Silicon)
  - Windows
  - Linux
- 🛠 工具包架构
  - 可扩展的侧边栏菜单
  - 为未来更多工具预留接口

### 技术栈
- Tauri 2
- Vite + 原生 JavaScript
- Rust (zip, serde_json, quick-xml)
- Marked.js (Markdown 渲染)

[Unreleased]: https://github.com/yourusername/rust-tools/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/rust-tools/releases/tag/v0.1.0
