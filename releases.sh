# 1. 更新版本号
./scripts/version.sh 0.2.1

# 2. 更新 CHANGELOG.md
vim CHANGELOG.md

# 3. 提交并推送标签
git add -A
git commit -m "chore: bump version to v0.2.1"
git tag v0.2.1
git push origin main
git push origin v0.2.1