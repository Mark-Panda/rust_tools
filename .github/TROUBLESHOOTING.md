# GitHub Actions 常见问题

## Windows 构建失败

### 问题：Command failed with exit code 1

**可能原因：**

1. **CI 环境变量冲突**
   - GitHub Actions 默认设置 `CI=true`
   - 某些 Tauri 版本可能错误解析此变量

   **解决方案：** 在 workflow 中明确设置：
   ```yaml
   - uses: tauri-apps/tauri-action@v0
     env:
       GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
       CI: '' # 清除 CI 变量
   ```

2. **npm scripts 使用 `&&` 在 Windows 上失败**
   - PowerShell 和 cmd 对 `&&` 支持不同

   **解决方案：** 
   - 让 Tauri 的 `beforeBuildCommand` 处理前端构建
   - `package.json` 中的 `tauri:build` 不要包含 `npm run build &&`

3. **路径分隔符问题**
   - Windows 使用 `\`，Unix 使用 `/`

   **解决方案：** 使用跨平台工具或在脚本中处理

### 问题：WebView2 安装失败

**解决方案：** GitHub Actions 的 Windows runner 已预装 WebView2，通常不会出现此问题。如果出现，可以在 workflow 中添加：

```yaml
- name: Install WebView2 (Windows only)
  if: matrix.platform == 'windows-latest'
  run: |
    choco install webview2-runtime -y
```

## macOS 构建失败

### 问题：代码签名错误

**原因：** macOS 应用需要开发者签名才能正常分发

**解决方案：**
1. **无签名（开发/测试）**：用户首次打开需要在"安全性与隐私"中允许
2. **添加签名**：配置以下 secrets 并更新 workflow：
   - `APPLE_CERTIFICATE`
   - `APPLE_CERTIFICATE_PASSWORD`
   - `APPLE_ID`
   - `APPLE_ID_PASSWORD`
   - `APPLE_TEAM_ID`

### 问题：Target not found

**原因：** 缺少交叉编译目标

**解决方案：** 确保 Rust toolchain 安装了目标：
```yaml
- name: Install Rust stable
  uses: dtolnay/rust-toolchain@stable
  with:
    targets: aarch64-apple-darwin,x86_64-apple-darwin
```

## Linux 构建失败

### 问题：缺少系统依赖

**原因：** Linux 需要 GTK、WebKit 等库

**解决方案：** 确保安装了所有依赖：
```yaml
- name: Install dependencies
  run: |
    sudo apt-get update
    sudo apt-get install -y \
      libwebkit2gtk-4.0-dev \
      libwebkit2gtk-4.1-dev \
      libappindicator3-dev \
      librsvg2-dev \
      patchelf
```

## 通用问题

### 问题：npm install 失败

**解决方案：**
- 确保 `package.json` 格式正确
- 检查依赖版本兼容性
- 清理缓存后重试

### 问题：构建超时

**解决方案：**
- 添加 Rust 缓存加速编译：
  ```yaml
  - name: Rust cache
    uses: swatinem/rust-cache@v2
    with:
      workspaces: './src-tauri -> target'
  ```

### 问题：Release 创建失败

**原因：** 权限不足

**解决方案：** 确保 workflow 有正确权限：
```yaml
jobs:
  create-release:
    permissions:
      contents: write
```

## 调试技巧

### 1. 查看详细日志

在 GitHub Actions 页面点击失败的 job，展开每个步骤查看详细输出。

### 2. 本地复现

尝试在对应平台本地运行相同命令：
```bash
npm install
npm run build
cd src-tauri
cargo build --release
```

### 3. 手动触发测试

在 Actions 页面手动触发 workflow，选择 debug 模式。

### 4. 分步调试

在 workflow 中添加临时步骤输出环境信息：
```yaml
- name: Debug info
  run: |
    node --version
    npm --version
    rustc --version
    cargo --version
```

## 性能优化

### 1. 缓存依赖

```yaml
- name: Cache Node modules
  uses: actions/cache@v3
  with:
    path: node_modules
    key: ${{ runner.os }}-node-${{ hashFiles('package-lock.json') }}

- name: Rust cache
  uses: swatinem/rust-cache@v2
```

### 2. 并行构建

`fail-fast: false` 确保其他平台继续构建，即使某个平台失败。

### 3. 按需触发

只在标签推送时触发 Release，避免每次提交都构建。
