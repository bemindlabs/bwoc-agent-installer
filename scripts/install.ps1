# install.ps1 — BWOC bootstrap installer (Windows)
#
# Downloads the latest bwoc + bwoc-agent release binaries from GitHub,
# installs them into $env:LOCALAPPDATA\Programs\bwoc, and launches
# bwoc-setup if the release asset is present.
#
# Usage (PowerShell):
#   irm https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.ps1 | iex
#
# Environment:
#   BWOC_BIN_DIR  — override installation directory

$ErrorActionPreference = 'Stop'

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------

$REPO     = 'bemindlabs/BWOC-Framework'
$BIN_DIR  = if ($env:BWOC_BIN_DIR) { $env:BWOC_BIN_DIR } `
            else { Join-Path $env:LOCALAPPDATA 'Programs\bwoc' }

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

function Info  { param($msg) Write-Host "[bwoc-setup] $msg" -ForegroundColor Cyan }
function Ok    { param($msg) Write-Host "[OK] $msg"         -ForegroundColor Green }
function Warn  { param($msg) Write-Host "[!]  $msg"         -ForegroundColor Yellow }
function Die   { param($msg) Write-Host "[!]  $msg"         -ForegroundColor Red; exit 1 }

# ---------------------------------------------------------------------------
# Detect architecture → target triple
# ---------------------------------------------------------------------------

$arch = $env:PROCESSOR_ARCHITECTURE
$TARGET = switch ($arch) {
    'ARM64' { 'aarch64-pc-windows-msvc' }
    'AMD64' { 'x86_64-pc-windows-msvc'  }
    default { Die "ไม่รองรับ architecture: $arch" }
}

Info "ระบบ: Windows / $arch → target triple: $TARGET"

# ---------------------------------------------------------------------------
# Fetch latest release tag
# ---------------------------------------------------------------------------

Info "กำลังตรวจสอบ release ล่าสุด..."

try {
    $releaseInfo = Invoke-RestMethod "https://api.github.com/repos/$REPO/releases/latest"
} catch {
    Die "ไม่สามารถดึงข้อมูล release ได้: $_"
}

$TAG = $releaseInfo.tag_name
if (-not $TAG) { Die "ไม่พบ tag_name ใน release JSON" }
Info "เวอร์ชันล่าสุด: $TAG"

$BASE_URL = "https://github.com/$REPO/releases/download/$TAG"

# ---------------------------------------------------------------------------
# Install directory
# ---------------------------------------------------------------------------

if (-not (Test-Path $BIN_DIR)) {
    New-Item -ItemType Directory -Path $BIN_DIR -Force | Out-Null
}

# ---------------------------------------------------------------------------
# Download + extract helper
# ---------------------------------------------------------------------------

function Download-AndExtract {
    param(
        [string]$Name,
        [string]$DestDir,
        [string]$Base = $BASE_URL
    )

    $url  = "$Base/$Name.zip"
    $tmp  = Join-Path ([System.IO.Path]::GetTempPath()) ([System.Guid]::NewGuid())
    New-Item -ItemType Directory -Path $tmp | Out-Null

    $zipPath = Join-Path $tmp "$Name.zip"
    Info "กำลังดาวน์โหลด $Name.zip ..."

    try {
        Invoke-WebRequest -Uri $url -OutFile $zipPath -UseBasicParsing
    } catch {
        Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
        return $false
    }

    Expand-Archive -Path $zipPath -DestinationPath $tmp -Force
    Remove-Item $zipPath

    # Copy all .exe files found in the extracted tree to DestDir.
    Get-ChildItem -Path $tmp -Recurse -Filter '*.exe' | ForEach-Object {
        Copy-Item $_.FullName -Destination (Join-Path $DestDir $_.Name) -Force
    }

    Remove-Item $tmp -Recurse -Force
    return $true
}

# ---------------------------------------------------------------------------
# Install bwoc + bwoc-agent
# ---------------------------------------------------------------------------

$bwocName = "bwoc-$TAG-$TARGET"
$bwocTmp  = Join-Path $BIN_DIR '_tmp_bwoc'
New-Item -ItemType Directory -Path $bwocTmp -Force | Out-Null

if (Download-AndExtract -Name $bwocName -DestDir $bwocTmp) {
    foreach ($bin in @('bwoc.exe', 'bwoc-agent.exe')) {
        $src = Join-Path $bwocTmp $bin
        if (Test-Path $src) {
            Copy-Item $src -Destination (Join-Path $BIN_DIR $bin) -Force
            Ok "ติดตั้ง $bin → $BIN_DIR\$bin"
        }
    }
} else {
    Die "ดาวน์โหลด bwoc ไม่สำเร็จ"
}
Remove-Item $bwocTmp -Recurse -Force -ErrorAction SilentlyContinue

# ---------------------------------------------------------------------------
# Add BIN_DIR to user PATH if missing
# ---------------------------------------------------------------------------

$userPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
if ($userPath -notlike "*$BIN_DIR*") {
    $newPath = "$BIN_DIR;$userPath"
    [Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
    Warn "เพิ่ม $BIN_DIR เข้า PATH แล้ว"
    Warn "เปิด PowerShell ใหม่เพื่อให้ PATH มีผล"
    $env:PATH = "$BIN_DIR;$env:PATH"
}

# ---------------------------------------------------------------------------
# Try to install bwoc-setup (may not be published yet)
# ---------------------------------------------------------------------------

# bwoc-setup ships from THIS installer repo's own releases (decoupled from
# bwoc, which comes from BWOC-Framework).
$SETUP_REPO = 'bemindlabs/bwoc-agent-installer'
$setupTmp   = Join-Path $BIN_DIR '_tmp_setup'
New-Item -ItemType Directory -Path $setupTmp -Force | Out-Null

$setupTag = $null
try {
    $setupTag = (Invoke-RestMethod "https://api.github.com/repos/$SETUP_REPO/releases/latest").tag_name
} catch { $setupTag = $null }

$installed = $false
if ($setupTag) {
    $setupBase = "https://github.com/$SETUP_REPO/releases/download/$setupTag"
    $setupName = "bwoc-setup-$setupTag-$TARGET"
    $installed = Download-AndExtract -Name $setupName -DestDir $setupTmp -Base $setupBase
}
$setupExe  = Join-Path $setupTmp 'bwoc-setup.exe'

if ($installed -and (Test-Path $setupExe)) {
    Copy-Item $setupExe -Destination (Join-Path $BIN_DIR 'bwoc-setup.exe') -Force
    Ok "ติดตั้ง bwoc-setup.exe → $BIN_DIR\bwoc-setup.exe"
    Remove-Item $setupTmp -Recurse -Force -ErrorAction SilentlyContinue
    Info "กำลังเปิด BWOC Setup Wizard..."
    & (Join-Path $BIN_DIR 'bwoc-setup.exe')
} else {
    Remove-Item $setupTmp -Recurse -Force -ErrorAction SilentlyContinue
    Warn '--------------------------------------------------------------'
    Warn "bwoc-setup ยังไม่มีใน release ของ $SETUP_REPO"
    Warn 'เมื่อ asset พร้อมแล้ว script นี้จะเปิด wizard ให้อัตโนมัติ'
    Warn ''
    Warn 'ตอนนี้ build เองได้:'
    Warn '  git clone https://github.com/bemindlabs/bwoc-agent-installer'
    Warn '  cd bwoc-agent-installer'
    Warn '  cargo build --release'
    Warn '  .\target\release\bwoc-setup.exe'
    Warn '--------------------------------------------------------------'
}

Ok 'ติดตั้ง BWOC เสร็จสมบูรณ์!'
Info 'ลองรัน: bwoc --version'
