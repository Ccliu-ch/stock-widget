# Windows MSI 打包指南

## 环境要求

### 1. 安装 Rust 工具链
```powershell
# 下载并安装 rustup
# 访问 https://rustup.rs 下载 rustup-init.exe 并运行
# 或使用 winget:
winget install Rustlang.Rustup

# 安装完成后验证
rustc --version
cargo --version
```

### 2. 安装 Node.js (>= 18)
```powershell
winget install OpenJS.NodeJS.LTS

# 验证
node --version
npm --version
```

### 3. 安装 WiX Toolset 3.x (MSI 打包必需)
```powershell
# 下载 WiX Toolset 3.11
# https://wixtoolset.org/releases/v3.11.2/stable
# 安装后需要将 bin 目录加入系统 PATH

# 或使用 winget:
winget install WixToolset.WiX

# 验证 (需要重启终端使 PATH 生效)
candle --help
light --help
```

### 4. 安装 Visual Studio C++ 构建工具
```powershell
# Tauri 需要 MSVC 编译器
winget install Microsoft.VisualStudio.2022.BuildTools

# 或手动下载: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# 安装时勾选 "使用 C++ 的桌面开发" 工作负载
```

## 打包步骤

### 方式一: 一键打包 (推荐)

```powershell
# 进入项目目录
cd stock-widget

# 安装前端依赖 (如未安装)
npm install --force

# 执行 Tauri 构建 (自动编译前端 + Rust + 打包)
npx tauri build
```

构建产物位于: `src-tauri/target/release/bundle/msi/Stock Widget_1.0.0_x64_en-US.msi`

### 方式二: 仅打包 MSI

```powershell
# 指定只生成 MSI 安装包
npx tauri build --bundles msi
```

### 方式三: 仅打包 NSIS exe 安装包

```powershell
npx tauri build --bundles nsis
```

### 方式四: 分步构建

```powershell
# 1. 构建前端
npm run build

# 2. 构建 Rust release 二进制
cargo build --release --manifest-path src-tauri/Cargo.toml

# 3. 打包 MSI
npx tauri build --no-bundle
npx tauri build --bundles msi
```

## 构建配置说明

`src-tauri/tauri.conf.json` 中的 MSI 相关配置:

```json
{
  "bundle": {
    "active": true,
    "targets": "all",
    "windows": {
      "wix": {
        "language": "zh-CN"
      }
    }
  }
}
```

### 自定义 WiX 配置 (可选)

如需更细粒度控制,可在 `tauri.conf.json` 中扩展:

```json
{
  "bundle": {
    "windows": {
      "wix": {
        "language": "zh-CN",
        "license": "../LICENSE.rtf",
        "template": "./installer/custom.wxs",
        "dialogImagePath": "./installer/dialog.bmp",
        "bannerPath": "./installer/banner.bmp"
      }
    }
  }
}
```

## 常见问题

### Q: 提示 "WiX Toolset not found"
确保 WiX 安装后重启终端,且 `candle.exe` 在 PATH 中。

### Q: 构建时提示缺少 MSVC
安装 Visual Studio Build Tools 并勾选 C++ 桌面开发。

### Q: 如何修改安装包产品名
在 `tauri.conf.json` 的 `productName` 字段修改。

### Q: 如何修改安装包版本号
在 `tauri.conf.json` 的 `version` 字段和 `Cargo.toml` 的 `version` 字段同时修改。

### Q: 如何设置应用图标
替换 `src-tauri/icons/` 下的图标文件,或使用 Tauri CLI 生成:
```powershell
npx tauri icon path/to/your-icon.png
```

## CI/CD 自动打包 (GitHub Actions)

在 `.github/workflows/build-windows.yml` 中:

```yaml
name: Build Windows MSI

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install WiX
        run: |
          curl -L https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip -o wix.zip
          7z x wix.zip -o"C:\Program Files (x86)\WiX Toolset v3.11\bin"
          echo "C:\Program Files (x86)\WiX Toolset v3.11\bin" >> $GITHUB_PATH

      - name: Install dependencies
        run: npm install --force

      - name: Build MSI
        run: npx tauri build --bundles msi

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: stock-widget-msi
          path: src-tauri/target/release/bundle/msi/*.msi
```
