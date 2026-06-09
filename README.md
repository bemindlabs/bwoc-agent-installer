<div align="center">

# 🧙 bwoc-agent-installer

**Friendly installer + guided TUI setup wizard for the BWOC framework — built for non-coders.**

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](Cargo.toml)
[![UI](https://img.shields.io/badge/UI-EN%20%7C%20TH-blue.svg)](#-the-wizard)
[![Status](https://img.shields.io/badge/status-1.0.0%20stable-brightgreen.svg)](#-status)
[![CI](https://github.com/bemindlabs/bwoc-agent-installer/actions/workflows/ci.yml/badge.svg)](https://github.com/bemindlabs/bwoc-agent-installer/actions/workflows/ci.yml)

</div>

Installing [BWOC](https://github.com/bemindlabs/BWOC-Framework) normally means a Rust toolchain, `cargo install`, and knowing what a backend, workspace, and agent *are*. This project removes all of that: **one command** downloads a prebuilt binary (no Rust), then a bilingual **TUI wizard** (English by default, Thai with one keypress) walks a first-timer through every setup choice with a plain-language explanation beside each option.

## 📦 Install

**macOS / Linux**

```bash
curl -fsSL https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.sh | bash
```

**Windows (PowerShell)**

```powershell
irm https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.ps1 | iex
```

The bootstrap detects your OS + architecture, downloads the latest `bwoc` + `bwoc-agent` binaries from [BWOC-Framework releases](https://github.com/bemindlabs/BWOC-Framework/releases) (checksum-verified), installs them to `~/.local/bin` (macOS/Linux) or `%LOCALAPPDATA%\Programs\bwoc` (Windows), puts them on `PATH`, then launches the wizard. **No Rust required.**

## 🧙 The wizard

`bwoc-setup` is a three-pane terminal wizard — **options** on the left, a **plain-language explanation** of the focused choice on the right, **key hints** at the bottom. It shells out to the `bwoc` CLI under the hood, so it stays correct against whatever `bwoc` version is installed. It steps through:

- **Backend** — Claude · Antigravity · Codex · Kimi · Copilot · Ollama · OpenAI-compatible, each explained, with a ✓/✗ probe for whether its CLI is already installed.
- **Workspace** — folder path, single-agent vs fleet, runtime vs inspection-only, and CLI language.
- **First agent** — name, role, primary model (from the backend's catalog), optional fallback.
- **Advanced (opt-in)** — what teams, skills, and plugins are, for later.
- **Verify** — runs `bwoc doctor`, `bwoc check`, and `bwoc list`, then prints the exact next commands.

The wizard opens in **English** and switches to **Thai** (or back) at any time with **F2** — pick once on the first screen, or flip mid-flow. Written for someone who has never touched a terminal. Preset the language with `bwoc-setup --lang th` if you prefer.

## 🗂️ Layout

```
src/
  main.rs      terminal setup/teardown, panic hook, event loop
  app.rs       Stage state machine, WizardConfig, next()/back(), bwoc orchestration
  catalog.rs   7 backends — Thai descriptions, CLI binary names, model catalogs
  exec.rs      bwoc() shell-out + binary_present() PATH scan (cross-platform)
  ui.rs        ratatui three-pane rendering
scripts/
  install.sh   POSIX bootstrap (macOS + Linux)
  install.ps1  PowerShell bootstrap (Windows)
```

Standalone — depends only on `ratatui` + `crossterm`, never on a `bwoc-*` crate.

## 🔨 Build from source

Requires Rust 1.85+.

```bash
git clone https://github.com/bemindlabs/bwoc-agent-installer
cd bwoc-agent-installer
cargo build --release        # → target/release/bwoc-setup
```

## 📊 Status

**v1.0.0 — stable.** End-to-end verified: the public one-liner downloads `bwoc` from BWOC-Framework releases + `bwoc-setup` from this repo's own releases (checksum-verified), installs (re-installing in place over an existing `bwoc`), and launches the wizard. The wizard guides backend → workspace → first agent → verify, passing the verification-gate commands `bwoc new` needs and writing `baseUrl` into the manifest for OpenAI-compatible backends. Bilingual (English default · **F2** toggles Thai), three-pane TUI whose palette mirrors the `bwoc_core::design` token system (Cyan accent · Blue/White selection · Gray muted — see `src/theme.rs`). CI runs fmt + clippy (`-D warnings`) + build/test on Linux/macOS/Windows. Note: the TUI needs a real standalone terminal (it cannot run inside an embedded/IDE console).

## 📄 License

MIT.
