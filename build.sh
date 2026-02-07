#!/bin/bash

# Rust Tools 打包脚本
# 用于打包 macOS 和其他平台的应用

set -e

# 清除 CI 环境变量（避免 Tauri 的 --ci 参数冲突）
unset CI

echo "🚀 开始打包 Rust Tools..."

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# 清理旧的构建
echo -e "${YELLOW}📦 清理旧的构建文件...${NC}"
npm run clean:build

# 构建前端
echo -e "${YELLOW}🔨 构建前端资源...${NC}"
npm run build

# 检查前端构建是否成功
if [ ! -d "dist" ]; then
    echo -e "${RED}❌ 前端构建失败！${NC}"
    exit 1
fi

echo -e "${GREEN}✅ 前端构建完成${NC}"

# 打包 Tauri 应用
echo -e "${YELLOW}📱 打包 Tauri 应用...${NC}"

# 根据参数决定是否 debug 构建
if [ "$1" == "--debug" ]; then
    echo -e "${YELLOW}🐛 使用 debug 模式打包...${NC}"
    npm run tauri:build:debug
else
    echo -e "${YELLOW}🚀 使用 release 模式打包...${NC}"
    npm run tauri:build
fi

# 检查打包结果
if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ 打包成功！${NC}"
    echo ""
    echo -e "${GREEN}📦 打包产物位置：${NC}"
    
    # macOS
    if [ -d "src-tauri/target/release/bundle/macos" ]; then
        echo -e "  ${GREEN}macOS .app:${NC} src-tauri/target/release/bundle/macos/"
    fi
    if [ -d "src-tauri/target/release/bundle/dmg" ]; then
        echo -e "  ${GREEN}macOS .dmg:${NC} src-tauri/target/release/bundle/dmg/"
    fi
    
    # Windows (如果在 Windows 上打包)
    if [ -d "src-tauri/target/release/bundle/msi" ]; then
        echo -e "  ${GREEN}Windows .msi:${NC} src-tauri/target/release/bundle/msi/"
    fi
    if [ -d "src-tauri/target/release/bundle/nsis" ]; then
        echo -e "  ${GREEN}Windows .exe:${NC} src-tauri/target/release/bundle/nsis/"
    fi
    
    # Linux
    if [ -d "src-tauri/target/release/bundle/deb" ]; then
        echo -e "  ${GREEN}Linux .deb:${NC} src-tauri/target/release/bundle/deb/"
    fi
    if [ -d "src-tauri/target/release/bundle/appimage" ]; then
        echo -e "  ${GREEN}Linux .AppImage:${NC} src-tauri/target/release/bundle/appimage/"
    fi
    
    echo ""
    echo -e "${GREEN}🎉 打包完成！可以在上述目录找到安装包${NC}"
else
    echo -e "${RED}❌ 打包失败！请检查错误信息${NC}"
    exit 1
fi
