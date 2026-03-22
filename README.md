
<p align="center">
  <img src="./assets/logo.png" alt="cnlint logo" width="120"/>
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
</p>

<p align="center" style="font-weight: normal; font-size: 18px">
  A CLI tool to detect and clean Chinese comments in source code.
</p>

<p align="center">
  <a href="#README.md">English</a> | <a href="README.zh.md">简体中文</a>
</p>

---


## ✨ Introduction

**cnlint** is a lightweight CLI tool that helps teams:

- Detect Chinese comments in source code
- Clean or remove them safely
- Maintain consistent English-only codebases

Perfect for:
- International teams
- Open source projects
- Code standard enforcement (CI/CD)

---

## 🚀 Features

- 🔍 Scan Chinese comments
- 🧹 Clean line and block comments
- 🧪 Dry-run mode (safe preview)
- 📦 JSON output (for integration)
- ✅ CI-friendly check mode
- ⚡ Fast and zero-dependency binary

---

## 📦 Installation

### From GitHub Releases (pre-built binaries)
Go to → https://github.com/your-username/cnlint/releases

Download the binary matching your platform (e.g. cnlint-x86_64-unknown-linux-gnu.tar.gz or .zip)

Then:

```aiignore
# Example for Linux/macOS
tar -xzf cnlint-*.tar.gz
sudo mv cnlint /usr/local/bin/
# or move to any directory in your $PATH
```

### From source
```aiignore
git clone https://github.com/your-username/cnlint.git
cd cnlint
cargo install --path .
# or
cargo build --release
```

## 🛠 Usage

| Subcommand | Function | Applicable scenarios | Exit CODE                      |
|-----|------|-----|--------------------------------|
| check <path> | Check if there are any Chinese comments present, do not output details | CI/CD<br />pre-commit hook | 1 = Exists<br />0 = Not Exists |
| scan <path> | Scan and list all positions containing Chinese comments | Manual inspection<br />Problem stabilization |                                |
| clean <path> | Remove Chinese comments (preview only/process specific types) | Batch code cleaning |                                |

### Common options( Support scan || clean )

```
--json          # Output JSON format results (for easy script parsing)
--dry-run       # Simulate execution, without actually modifying any files (for clean use)
--line          # Process only single-line comments (// ...)
--block         # Process only block comments (/* ... */)
--trim-empty    # Automatically clean up extra blank lines after removing comments
```

### Quick Example

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



## 🤝 Contributing

### PRs and issues are welcome!
