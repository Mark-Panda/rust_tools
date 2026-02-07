# 发布新版本

本指南说明如何发布 RustTools 的新版本。

## 准备工作

1. 确保所有更改已提交并推送到 `main` 分支
2. 更新版本号和 CHANGELOG

## 发布步骤

### 1. 更新版本号

在以下文件中更新版本号：

- `package.json` - `version` 字段
- `src-tauri/Cargo.toml` - `version` 字段
- `src-tauri/tauri.conf.json` - `version` 字段

确保三个文件的版本号一致（如 `0.2.0`）。

### 2. 更新 CHANGELOG

在 `CHANGELOG.md` 中：

1. 将 `[Unreleased]` 下的内容移到新版本标题下
2. 添加发布日期
3. 添加新的空 `[Unreleased]` 部分

示例：

```markdown
## [Unreleased]

## [0.2.0] - 2026-02-15

### 新增
- 新功能描述

### 修复
- Bug 修复描述
```

### 3. 提交更改

```bash
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json CHANGELOG.md
git commit -m "chore: bump version to 0.2.0"
git push origin main
```

### 4. 创建并推送标签

```bash
# 创建标签
git tag v0.2.0

# 推送标签（这会触发自动构建）
git push origin v0.2.0
```

### 5. 自动构建

推送标签后，GitHub Actions 会自动：

1. ✅ 为 macOS (Intel & ARM)、Windows、Linux 构建应用
2. ✅ 创建 GitHub Release（草稿）
3. ✅ 上传构建产物到 Release
4. ✅ 发布 Release

你可以在 GitHub Actions 页面查看构建进度。

### 6. 完善 Release 信息（可选）

构建完成后：

1. 访问 GitHub Releases 页面
2. 编辑自动创建的 Release
3. 添加更详细的发布说明
4. 检查上传的文件是否完整

## 手动触发构建

如果需要手动触发构建（不创建新标签）：

1. 访问 GitHub Actions 页面
2. 选择 "Release" workflow
3. 点击 "Run workflow"
4. 选择分支并运行

## 测试版本发布

在正式发布前，建议：

1. 先推送到测试分支
2. 手动触发 CI workflow 测试构建
3. 确认构建成功后再发布正式版本

## 问题排查

### 构建失败

- 检查 GitHub Actions 日志
- 确认所有平台的依赖都正确安装
- 验证 Rust 和 Node.js 版本兼容性

### 版本号不匹配

确保 `package.json`、`Cargo.toml`、`tauri.conf.json` 中的版本号完全一致。

### 签名问题（macOS）

macOS 构建需要代码签名。如果没有签名证书，构建仍会成功，但用户首次打开时需要允许。
