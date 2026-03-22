
<p align="center">
  <img src="./assets/logo.png" alt="cnlint logo" width="120"/>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

<p align="center" style="font-weight: normal; font-size: 18px">
  一个用于检测和清理源代码中文注释的命令行(CLI)工具。
</p>

<p align="center">
  <a href="README.zh.md">简体中文</a> | <a href="README.md">English</a>
</p>

---


## ✨ 简介

**cnlint** 是一款轻量级的命令行工具，帮助团队：

- 检测源代码中的中文注释
- 安全地清理或移除它们
- 维护纯英文代码库一致性

适用于:
- 国际团队
- 开源项目
- 代码标准执行（CI/CD）
---

## 🚀 功能

- 🔍 扫描中文注释
- 🧹 清理行和块注释
- 🧪 Dry Run 运行模式（安全预览）
- 📦 JSON输出（可集成）
- ✅ CI友好的检查模式
- ⚡ 快速且无依赖的二进制文件

---

## 📦 安装

### 方法 1: 通过 Release 安装
点击 → https://github.com/blakeandersondev/cnlint/releases

**下载与您的平台匹配的产品**

#### 🍎 macOS
```aiignore
# rename (optional but recommended)
mv cnlint-macos cnlint

# make executable
chmod +x cnlint

# remove security warning
xattr -d com.apple.quarantine cnlint
```

#### 🐧 Linux
```aiignore
# rename (optional)
mv cnlint-linux cnlint

# make executable
chmod +x cnlint
```

#### 🪟Windows
```aiignore
# rename (optional)
ren cnlint-windows.exe cnlint.exe
```

### 方法2: 使用源代码

```aiignore
git clone https://github.com/your-username/cnlint.git
cd cnlint
cargo install --path .
# or
cargo build --release
```

## 🛠 Usage

| 子命令          | 功能                                                                     | 应用场景                        | 退出码                            |
|--------------|------------------------------------------------------------------------|-----------------------------|--------------------------------|
| check <path> | 检查是否有中文评论，不要输出详细信息 | CI/CD<br />pre-commit hook  | 1 = Exists<br />0 = Not Exists |
| scan <path>  | 扫描并列出所有包含中文注释的位置                | 人工检查<br />问题定位 |                                |
| clean <path> | 删除中文注释（仅预览/仅处理特定类型）          | 代码批量清理                      |                                |

### 常见选项 ( Support scan || clean )

```
--json          # 输出JSON格式的结果（便于脚本解析）
--dry-run       # 模拟执行，实际上不修改任何文件(只在子命令 clean 下使用)
--line          # 仅处理单行注释 (// ...)
--block         # 仅处理块注释 (/* ... */)
--trim-empty    # 自动清理删除注释产生的多余的空白行
```

### 快速示例

```
# Check
cnlint check /path

# Scan and output Chinese comments in JSON format
cnlint scan /path --json

# Preview the content to be deleted (dry run)
cnlint clean /path --dry-run

# Formal cleanup and deletion of empty lines at the original comment positions
cnlint clean /path --trim-empty

# Only clean single-line comments
cnlint clean /path --line --trim-empty

```



## 🤝 贡献

### 欢迎提交PR和issue！
