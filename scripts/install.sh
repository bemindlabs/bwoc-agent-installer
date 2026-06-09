#!/usr/bin/env bash
# install.sh — BWOC bootstrap installer (macOS + Linux)
#
# Downloads the latest bwoc + bwoc-agent release binaries from GitHub,
# installs them into BWOC_BIN_DIR (default ~/.local/bin), and launches
# bwoc-setup if the release asset is present.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.sh | bash
#
# Environment:
#   BWOC_BIN_DIR  — installation directory (default: ~/.local/bin)

set -euo pipefail

REPO="bemindlabs/BWOC-Framework"
BIN_DIR="${BWOC_BIN_DIR:-$HOME/.local/bin}"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

info()  { printf '\033[1;36m[bwoc-setup]\033[0m %s\n' "$*"; }
ok()    { printf '\033[1;32m[✓]\033[0m %s\n' "$*"; }
warn()  { printf '\033[1;33m[!]\033[0m %s\n' "$*"; }
die()   { printf '\033[1;31m[✗]\033[0m %s\n' "$*" >&2; exit 1; }

# ---------------------------------------------------------------------------
# Detect OS + arch → target triple
# ---------------------------------------------------------------------------

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64|aarch64) TARGET="aarch64-apple-darwin" ;;
      x86_64)        TARGET="x86_64-apple-darwin" ;;
      *) die "ไม่รองรับ architecture: $ARCH บน macOS" ;;
    esac
    SHASUM="shasum -a 256"
    ;;
  Linux)
    case "$ARCH" in
      aarch64|arm64) TARGET="aarch64-unknown-linux-gnu" ;;
      x86_64|amd64)  TARGET="x86_64-unknown-linux-gnu" ;;
      *) die "ไม่รองรับ architecture: $ARCH บน Linux" ;;
    esac
    SHASUM="sha256sum"
    ;;
  *)
    die "ไม่รองรับ OS: $OS  (ใช้ install.ps1 สำหรับ Windows)"
    ;;
esac

info "ระบบ: $OS / $ARCH → target triple: $TARGET"

# ---------------------------------------------------------------------------
# Fetch latest release tag from GitHub API
# ---------------------------------------------------------------------------

info "กำลังตรวจสอบ release ล่าสุด..."
RELEASE_JSON="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest")" \
  || die "ไม่สามารถดึงข้อมูล release ได้ — ตรวจสอบการเชื่อมต่ออินเทอร์เน็ต"

TAG="$(printf '%s' "$RELEASE_JSON" | grep '"tag_name"' | head -1 \
  | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"

[ -n "$TAG" ] || die "ไม่พบ tag_name ใน release JSON"
info "เวอร์ชันล่าสุด: $TAG"

BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"

# ---------------------------------------------------------------------------
# Download helper: download a tarball + optional .sha256 sidecar
# ---------------------------------------------------------------------------

download_and_verify() {
  local NAME="$1"      # e.g. bwoc-v2026.6.7-1-aarch64-apple-darwin
  local DEST_DIR="$2"  # extraction target directory
  local BASE="${3:-$BASE_URL}"  # release base URL (defaults to bwoc's)

  local URL="${BASE}/${NAME}.tar.gz"
  local TMP
  TMP="$(mktemp -d)"
  trap 'rm -rf "$TMP"' RETURN

  info "กำลังดาวน์โหลด ${NAME}.tar.gz ..."
  if ! curl -fsSL -o "${TMP}/${NAME}.tar.gz" "$URL"; then
    return 1
  fi

  # Verify checksum if sidecar exists.
  if curl -fsSL -o "${TMP}/${NAME}.tar.gz.sha256" "${URL}.sha256" 2>/dev/null; then
    info "ตรวจสอบ checksum..."
    (
      cd "$TMP"
      EXPECTED="$(cat "${NAME}.tar.gz.sha256" | awk '{print $1}')"
      ACTUAL="$($SHASUM "${NAME}.tar.gz" | awk '{print $1}')"
      [ "$EXPECTED" = "$ACTUAL" ] || {
        printf 'checksum ไม่ตรงกัน: expected %s, got %s\n' "$EXPECTED" "$ACTUAL" >&2
        exit 1
      }
    )
    ok "checksum ถูกต้อง"
  fi

  tar -xzf "${TMP}/${NAME}.tar.gz" -C "$DEST_DIR"
  return 0
}

# ---------------------------------------------------------------------------
# Install directory
# ---------------------------------------------------------------------------

mkdir -p "$BIN_DIR"

# ---------------------------------------------------------------------------
# Install bwoc
# ---------------------------------------------------------------------------

BWOC_NAME="bwoc-${TAG}-${TARGET}"
BWOC_TMP="$(mktemp -d)"
trap 'rm -rf "$BWOC_TMP"' EXIT

if download_and_verify "$BWOC_NAME" "$BWOC_TMP"; then
  # The tarball may contain bwoc and/or bwoc-agent at the top level.
  for BIN in bwoc bwoc-agent; do
    if [ -f "${BWOC_TMP}/${BIN}" ]; then
      chmod +x "${BWOC_TMP}/${BIN}"
      cp "${BWOC_TMP}/${BIN}" "${BIN_DIR}/${BIN}"
      ok "ติดตั้ง ${BIN} → ${BIN_DIR}/${BIN}"
    fi
    # Also search one level deep (some archives nest inside a dir).
    for NESTED in "${BWOC_TMP}"/*/"${BIN}"; do
      if [ -f "$NESTED" ]; then
        chmod +x "$NESTED"
        cp "$NESTED" "${BIN_DIR}/${BIN}"
        ok "ติดตั้ง ${BIN} → ${BIN_DIR}/${BIN}"
      fi
    done
  done
else
  die "ดาวน์โหลด bwoc ไม่สำเร็จ"
fi

# ---------------------------------------------------------------------------
# PATH warning
# ---------------------------------------------------------------------------

case ":${PATH}:" in
  *":${BIN_DIR}:"*) ;;
  *)
    warn "ไดเรกทอรี ${BIN_DIR} ยังไม่ได้อยู่ใน PATH"
    warn "เพิ่มบรรทัดนี้ลงใน ~/.bashrc หรือ ~/.zshrc:"
    warn "  export PATH=\"${BIN_DIR}:\$PATH\""
    warn "จากนั้น: source ~/.bashrc  (หรือเปิด terminal ใหม่)"
    ;;
esac

# ---------------------------------------------------------------------------
# Try to install bwoc-setup — published from THIS installer repo's own
# releases (decoupled from bwoc, which ships from BWOC-Framework). May not be
# published yet on a brand-new repo.
# ---------------------------------------------------------------------------

SETUP_REPO="bemindlabs/bwoc-agent-installer"
SETUP_TMP="$(mktemp -d)"
SETUP_TAG=""
if SETUP_JSON="$(curl -fsSL "https://api.github.com/repos/${SETUP_REPO}/releases/latest" 2>/dev/null)"; then
  SETUP_TAG="$(printf '%s' "$SETUP_JSON" | grep '"tag_name"' | head -1 \
    | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
fi

if [ -n "$SETUP_TAG" ]; then
  SETUP_BASE="https://github.com/${SETUP_REPO}/releases/download/${SETUP_TAG}"
  SETUP_NAME="bwoc-setup-${SETUP_TAG}-${TARGET}"
  if download_and_verify "$SETUP_NAME" "$SETUP_TMP" "$SETUP_BASE" 2>/dev/null; then
    for F in "${SETUP_TMP}"/bwoc-setup "${SETUP_TMP}"/*/bwoc-setup; do
      if [ -f "$F" ]; then
        chmod +x "$F"
        cp "$F" "${BIN_DIR}/bwoc-setup"
        ok "ติดตั้ง bwoc-setup → ${BIN_DIR}/bwoc-setup"
        rm -rf "$SETUP_TMP"
        info "กำลังเปิด BWOC Setup Wizard..."
        # Reattach stdin to the controlling terminal: under `curl … | bash`
        # the wizard inherits the curl pipe as stdin (EOF), so without this it
        # can't read the keyboard ("Failed to initialize input reader"). When a
        # real terminal is present we hand it /dev/tty; otherwise exec plainly
        # and let the wizard print its own TTY-required message.
        if [ -t 1 ] && [ -r /dev/tty ]; then
          exec "${BIN_DIR}/bwoc-setup" </dev/tty
        else
          exec "${BIN_DIR}/bwoc-setup"
        fi
      fi
    done
  fi
fi
rm -rf "$SETUP_TMP"

# bwoc-setup ยังไม่ได้อยู่ใน release asset — แนะนำวิธี build เอง
warn "--------------------------------------------------------------"
warn "bwoc-setup ยังไม่มีใน release ของ ${SETUP_REPO}"
warn "เมื่อ asset พร้อมแล้ว script นี้จะเปิด wizard ให้อัตโนมัติ"
warn ""
warn "ตอนนี้ build เองได้:"
warn "  git clone https://github.com/bemindlabs/bwoc-agent-installer"
warn "  cd bwoc-agent-installer"
warn "  cargo build --release"
warn "  ./target/release/bwoc-setup"
warn "--------------------------------------------------------------"

ok "ติดตั้ง BWOC เสร็จสมบูรณ์!"
info "ลองรัน: bwoc --version"
