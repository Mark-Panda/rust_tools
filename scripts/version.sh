#!/bin/bash

# 版本号同步脚本
# 用于确保 package.json、Cargo.toml 和 tauri.conf.json 中的版本号一致

set -e

# 颜色定义
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

if [ -z "$1" ]; then
    echo -e "${RED}❌ 请提供版本号${NC}"
    echo "用法: ./scripts/version.sh <版本号>"
    echo "示例: ./scripts/version.sh 0.2.0"
    exit 1
fi

NEW_VERSION=$1

# 验证版本号格式（简单的语义化版本检查）
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${RED}❌ 版本号格式无效${NC}"
    echo "版本号必须符合语义化版本格式: X.Y.Z"
    exit 1
fi

echo -e "${YELLOW}🔄 更新版本号到 ${NEW_VERSION}...${NC}"

# 更新 package.json
echo -e "${YELLOW}📝 更新 package.json...${NC}"
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" package.json
rm package.json.bak

# 更新 Cargo.toml
echo -e "${YELLOW}📝 更新 src-tauri/Cargo.toml...${NC}"
sed -i.bak "s/^version = \".*\"/version = \"$NEW_VERSION\"/" src-tauri/Cargo.toml
rm src-tauri/Cargo.toml.bak

# 更新 tauri.conf.json
echo -e "${YELLOW}📝 更新 src-tauri/tauri.conf.json...${NC}"
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$NEW_VERSION\"/" src-tauri/tauri.conf.json
rm src-tauri/tauri.conf.json.bak

# 验证
echo ""
echo -e "${GREEN}✅ 版本号已更新：${NC}"
echo -e "  package.json:        $(grep '"version"' package.json | head -1 | sed 's/.*: "\(.*\)".*/\1/')"
echo -e "  Cargo.toml:          $(grep '^version' src-tauri/Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')"
echo -e "  tauri.conf.json:     $(grep '"version"' src-tauri/tauri.conf.json | sed 's/.*: "\(.*\)".*/\1/')"

echo ""
echo -e "${GREEN}🎉 完成！${NC}"
echo ""
echo -e "${YELLOW}下一步：${NC}"
echo "1. 更新 CHANGELOG.md"
echo "2. 提交更改: git add -A && git commit -m \"chore: bump version to v$NEW_VERSION\""
echo "3. 创建标签: git tag v$NEW_VERSION"
echo "4. 推送标签: git push origin v$NEW_VERSION"
