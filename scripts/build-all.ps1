# ============================================================
# 加油努力 - Windows 一键安装+编译打包脚本
# 使用方法:
#   1. 右键 PowerShell -> 以管理员身份运行
#   2. cd 到项目目录
#   3. Set-ExecutionPolicy Bypass -Scope Process -Force
#   4. .\scripts\build-all.ps1
# ============================================================

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = Split-Path -Parent $scriptDir
Set-Location $projectRoot

function Write-Step { param([int]$step, [int]$total, [string]$msg)
    Write-Host "`n[$step/$total] $msg" -ForegroundColor Yellow
}
function Write-Ok { param([string]$msg) Write-Host "  [OK] $msg" -ForegroundColor Green }
function Write-Info { param([string]$msg) Write-Host "  $msg" -ForegroundColor Gray }
function Write-Warn { param([string]$msg) Write-Host "  [!] $msg" -ForegroundColor Yellow }
function Write-Err { param([string]$msg) Write-Host "  [X] $msg" -ForegroundColor Red }

Write-Host "`n========================================" -ForegroundColor Cyan
Write-Host "   加油努力 - 安装 & 编译打包" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan

# ============================================================
# 第一阶段: 环境安装
# ============================================================
Write-Host "`n========== 第一阶段: 环境安装 ==========" -ForegroundColor Cyan

$totalSteps = 6
$needRestart = $false

# --- 1. MSVC ---
Write-Step 1 $totalSteps "检查 MSVC 编译器..."
$vsPath = "C:\Program Files\Microsoft Visual Studio"
$hasMsvc = $false
if (Test-Path $vsPath) {
    $clExe = Get-ChildItem -Path $vsPath -Recurse -Filter "cl.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($clExe) { $hasMsvc = $true }
}
if (-not $hasMsvc) {
    Write-Info "下载 Visual Studio Build Tools..."
    $vsInstaller = "$env:TEMP\vs_buildtools.exe"
    Invoke-WebRequest -Uri "https://aka.ms/vs/17/release/vs_BuildTools.exe" -OutFile $vsInstaller
    Write-Info "安装中,请等待...(约5-10分钟)"
    Start-Process -FilePath $vsInstaller -ArgumentList "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --wait" -Wait
    Remove-Item $vsInstaller -Force
    Write-Ok "MSVC 安装完成"
} else {
    Write-Ok "MSVC 已安装,跳过"
}

# --- 2. Rust ---
Write-Step 2 $totalSteps "检查 Rust..."
$rustOk = $false
try { $null = rustc --version 2>&1; if ($LASTEXITCODE -eq 0) { $rustOk = $true } } catch {}
if (-not $rustOk) {
    Write-Info "下载 Rust 安装器..."
    $rustupInit = "$env:TEMP\rustup-init.exe"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile $rustupInit
    Write-Info "安装中..."
    Start-Process -FilePath $rustupInit -ArgumentList "-y --default-toolchain stable" -Wait
    Remove-Item $rustupInit -Force
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    Write-Ok "Rust 安装完成"
    $needRestart = $true
} else {
    Write-Ok "Rust 已安装: $(rustc --version)"
}

# --- 3. Node.js ---
Write-Step 3 $totalSteps "检查 Node.js..."
$nodeOk = $false
try { $null = node --version 2>&1; if ($LASTEXITCODE -eq 0) { $nodeOk = $true } } catch {}
if (-not $nodeOk) {
    Write-Info "下载 Node.js LTS..."
    $nodeMsi = "$env:TEMP\node-install.msi"
    Invoke-WebRequest -Uri "https://nodejs.org/dist/v20.15.0/node-v20.15.0-x64.msi" -OutFile $nodeMsi
    Write-Info "安装中..."
    Start-Process -FilePath "msiexec.exe" -ArgumentList "/i `"$nodeMsi`" /quiet" -Wait
    Remove-Item $nodeMsi -Force
    $env:Path = [System.Environment]::GetEnvironmentVariable("Path","Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path","User")
    Write-Ok "Node.js 安装完成"
    $needRestart = $true
} else {
    Write-Ok "Node.js 已安装: $(node --version)"
}

# --- 4. WiX ---
Write-Step 4 $totalSteps "检查 WiX Toolset..."
$wixOk = $false
if (Test-Path "C:\wix311\candle.exe") {
    $wixOk = $true
} else {
    try { $null = candle -? 2>&1; if ($LASTEXITCODE -eq 0) { $wixOk = $true } } catch {}
}
if (-not $wixOk) {
    Write-Info "下载 WiX Toolset 3.11..."
    $wixZip = "$env:TEMP\wix311.zip"
    Invoke-WebRequest -Uri "https://github.com/wixtoolset/wix3/releases/download/wix3112rtm/wix311-binaries.zip" -OutFile $wixZip
    Expand-Archive -Path $wixZip -DestinationPath "C:\wix311" -Force
    Remove-Item $wixZip -Force
    $currentPath = [System.Environment]::GetEnvironmentVariable("Path","Machine")
    if ($currentPath -notlike "*C:\wix311*") {
        [System.Environment]::SetEnvironmentVariable("Path", $currentPath + ";C:\wix311", "Machine")
    }
    $env:Path += ";C:\wix311"
    Write-Ok "WiX 安装完成"
} else {
    Write-Ok "WiX 已安装,跳过"
}

# --- 5. 重启提示 ---
Write-Step 5 $totalSteps "验证环境..."
$allOk = $true
foreach ($cmd in @("rustc","node","npm")) {
    try {
        $v = & $cmd --version 2>&1
        Write-Ok "$cmd : $v"
    } catch {
        Write-Err "$cmd 不可用"
        $allOk = $false
    }
}
try { $null = candle -? 2>&1; if ($LASTEXITCODE -eq 0) { Write-Ok "WiX : 可用" } } catch { Write-Warn "WiX 不可用,MSI 可能失败" }

if ($needRestart) {
    Write-Host "`n========================================" -ForegroundColor Yellow
    Write-Host "  新安装了 Rust 或 Node.js" -ForegroundColor Yellow
    Write-Host "  请关闭此窗口,重新打开 PowerShell" -ForegroundColor Yellow
    Write-Host "  再次运行: .\scripts\build-all.ps1" -ForegroundColor Yellow
    Write-Host "  脚本会自动跳过已安装的部分" -ForegroundColor Yellow
    Write-Host "========================================`n" -ForegroundColor Yellow
    exit 0
}

if (-not $allOk) {
    Write-Err "环境检查未通过,请手动排查"
    exit 1
}

# ============================================================
# 第二阶段: 编译打包
# ============================================================
Write-Host "`n========== 第二阶段: 编译打包 ==========" -ForegroundColor Cyan

# --- 6. 安装依赖 + 编译 ---
Write-Step 6 $totalSteps "安装依赖并编译..."

Write-Info "安装前端依赖..."
npm install --force
if ($LASTEXITCODE -ne 0) { Write-Err "npm install 失败"; exit 1 }
Write-Ok "依赖安装完成"

# --- 选择打包格式 ---
Write-Host "`n  选择打包格式:" -ForegroundColor Yellow
Write-Host "    1. MSI 安装包 (推荐)" -ForegroundColor White
Write-Host "    2. NSIS exe 安装包" -ForegroundColor White
Write-Host "    3. 全部 (MSI + NSIS)" -ForegroundColor White
Write-Host "    4. 仅编译 exe,不打包" -ForegroundColor White
$choice = Read-Host "`n  请输入选项 (1-4, 默认1)"
if ([string]::IsNullOrWhiteSpace($choice)) { $choice = "1" }

$bundleArg = switch ($choice) {
    "1" { "msi" }
    "2" { "nsis" }
    "3" { "all" }
    "4" { "" }
    default { "msi" }
}

Write-Info "开始编译打包..."
$startTime = Get-Date

if ($bundleArg -eq "") {
    Write-Info "仅编译 Rust release..."
    cargo build --release --manifest-path src-tauri/Cargo.toml
} else {
    Write-Info "执行 tauri build --bundles $bundleArg ..."
    npx tauri build --bundles $bundleArg
}

if ($LASTEXITCODE -ne 0) { Write-Err "编译失败!"; exit 1 }

$elapsed = ((Get-Date) - $startTime).TotalSeconds
Write-Ok "编译完成,耗时 $([math]::Round($elapsed,1)) 秒"

# --- 显示产物 ---
Write-Host "`n========================================" -ForegroundColor Green
Write-Host "  打包完成!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green

$bundleDir = "src-tauri\target\release\bundle"
if (Test-Path $bundleDir) {
    Write-Host "`n  产物文件:" -ForegroundColor Cyan
    $files = Get-ChildItem -Path $bundleDir -Recurse -File -ErrorAction SilentlyContinue
    foreach ($f in $files) {
        $sizeMB = [math]::Round($f.Length / 1MB, 1)
        $sizeKB = [math]::Round($f.Length / 1KB, 1)
        $sizeStr = if ($f.Length -gt 1MB) { "$sizeMB MB" } else { "$sizeKB KB" }
        Write-Host "    $($f.Name)" -ForegroundColor White
        Write-Host "      $sizeStr  -  $($f.FullName)" -ForegroundColor Gray
    }
} else {
    Write-Host "`n  产物: src-tauri\target\release\stock-widget.exe" -ForegroundColor Cyan
}

Write-Host "`n完成!`n" -ForegroundColor Green
