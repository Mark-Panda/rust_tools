# GitHub Actions 自动发布配置

## 📋 概述

本项目配置了 GitHub Actions 自动构建和发布流程，支持跨平台打包。

## 🔄 工作流程

### 1. CI 工作流 (`.github/workflows/ci.yml`)

**触发条件：**
- 推送到 `main`/`master`/`develop` 分支
- Pull Request 到这些分支

**功能：**
- ✅ 在 macOS、Linux、Windows 上测试构建
- ✅ 运行前端构建
- ✅ 检查 Rust 代码
- ✅ 运行测试

### 2. Release 工作流 (`.github/workflows/release.yml`)

**触发条件：**
- 推送 `v*` 标签（如 `v0.1.0`）
- 手动触发

**功能：**
- ✅ 创建 GitHub Release（草稿）
- ✅ 为多平台构建应用：
  - macOS (Intel x86_64)
  - macOS (Apple Silicon aarch64)
  - Windows
  - Linux
- ✅ 上传构建产物
- ✅ 发布 Release

## 🚀 如何发布新版本

### 方式一：使用脚本（推荐）

```bash
# 1. 更新版本号
./scripts/version.sh 0.2.0

# 2. 更新 CHANGELOG.md（手动编辑）
vim CHANGELOG.md

# 3. 提交更改
git add -A
git commit -m "chore: bump version to v0.2.0"

# 4. 创建并推送标签
git tag v0.2.0
git push origin main
git push origin v0.2.0
```

### 方式二：手动更新

1. **更新版本号**（确保一致）：
   - `package.json` → `"version": "0.2.0"`
   - `src-tauri/Cargo.toml` → `version = "0.2.0"`
   - `src-tauri/tauri.conf.json` → `"version": "0.2.0"`

2. **更新 CHANGELOG.md**

3. **提交并推送标签**：
   ```bash
   git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json CHANGELOG.md
   git commit -m "chore: bump version to v0.2.0"
   git push origin main
   git tag v0.2.0
   git push origin v0.2.0
   ```

## 📦 构建产物

发布完成后，Release 页面会包含：

- **macOS**:
  - `RustTools_<version>_x86_64.dmg` (Intel)
  - `RustTools_<version>_aarch64.dmg` (Apple Silicon)
  - `RustTools.app.tar.gz`

> 注：当前仅配置 macOS 平台自动构建。如需 Windows/Linux 支持，可在 `.github/workflows/release.yml` 中添加相应平台。

## 🔧 手动触发构建

1. 访问 [GitHub Actions](https://github.com/yourusername/rust-tools/actions)
2. 选择 "Release" workflow
3. 点击 "Run workflow"
4. 选择分支并运行

## ⚙️ 配置要求

### Secrets（无需配置）

- `GITHUB_TOKEN` - 自动提供，用于创建 Release 和上传文件

### 可选配置（macOS 代码签名）

如需对 macOS 应用进行签名和公证：

1. 添加以下 Secrets：
   - `APPLE_CERTIFICATE` - Base64 编码的开发者证书
   - `APPLE_CERTIFICATE_PASSWORD` - 证书密码
   - `APPLE_ID` - Apple ID
   - `APPLE_ID_PASSWORD` - App 专用密码
   - `APPLE_TEAM_ID` - 团队 ID

2. 更新 `release.yml` 中的 `tauri-action` 配置

## 📊 监控构建

- **查看进度**: [Actions 页面](https://github.com/yourusername/rust-tools/actions)
- **构建日志**: 点击具体的 workflow run 查看详细日志
- **失败通知**: GitHub 会自动发送邮件通知

## 🐛 问题排查

### 构建失败

1. 检查 Actions 日志
2. 确认版本号格式正确
3. 验证所有依赖已正确配置

### 无法上传到 Release

- 确保 `GITHUB_TOKEN` 有正确权限
- 检查仓库的 Actions 权限设置

### 某个平台构建失败

- 查看该平台的构建日志
- 可能需要更新依赖或配置

## 📚 相关文档

- [发布指南](RELEASE.md)
- [变更日志](CHANGELOG.md)
- [Tauri GitHub Action](https://github.com/tauri-apps/tauri-action)
- [GitHub Actions 文档](https://docs.github.com/actions)
