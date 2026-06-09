# bwoc-agent-installer

**TH**: ตัวติดตั้งและ Setup Wizard สำหรับ [BWOC Framework](https://github.com/bemindlabs/BWOC-Framework) — ออกแบบมาสำหรับมือใหม่ที่ไม่รู้จัก coding

**EN**: A friendly guided installer + first-run setup wizard for the BWOC Framework, aimed at non-developer end-users.

---

## ติดตั้ง / Install

### macOS / Linux

```bash
curl -fsSL https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.sh | bash
```

### Windows (PowerShell)

```powershell
irm https://raw.githubusercontent.com/bemindlabs/bwoc-agent-installer/main/scripts/install.ps1 | iex
```

Script จะ:
1. ตรวจจับ OS + architecture
2. ดาวน์โหลด `bwoc` + `bwoc-agent` binary ล่าสุดจาก GitHub Releases
3. ติดตั้งเข้า `~/.local/bin` (macOS/Linux) หรือ `%LOCALAPPDATA%\Programs\bwoc` (Windows)
4. เปิด `bwoc-setup` wizard อัตโนมัติ (ถ้า release asset พร้อม)

---

## Wizard ทำอะไรบ้าง?

`bwoc-setup` พาคุณผ่านทุกขั้นตอน:

- **เลือก Backend** — Claude, Antigravity, Codex, Kimi, Copilot, Ollama, หรือ OpenAI-compatible
- **สร้าง Workspace** — โฟลเดอร์หลักที่เก็บ agent ทั้งหมด (mode, runtime, ภาษา)
- **สร้าง Agent ตัวแรก** — ชื่อ, หน้าที่, model หลัก, fallback model
- **ตรวจสอบ** — `bwoc doctor` + `bwoc check` + `bwoc list`
- **สรุปขั้นตอนต่อไป** — คำสั่งที่ใช้งานได้ทันที

ทุกข้อความในหน้าจอเป็นภาษาไทย อธิบายแต่ละตัวเลือกอย่างละเอียด

---

## Build จาก Source

ต้องการ Rust 1.85+

```bash
git clone https://github.com/bemindlabs/bwoc-agent-installer
cd bwoc-agent-installer
cargo build --release
./target/release/bwoc-setup
```

Binary อยู่ที่ `target/release/bwoc-setup`

---

## Layout

```
src/
  main.rs      — terminal setup/teardown, panic hook, event loop
  app.rs       — App state machine: Stage enum, WizardConfig, next()/back()
  catalog.rs   — 7 backends + Thai descriptions + model lists
  exec.rs      — bwoc() shell-out, binary_present() PATH scan
  ui.rs        — ratatui 3-pane rendering
scripts/
  install.sh   — POSIX bootstrap (macOS + Linux)
  install.ps1  — PowerShell bootstrap (Windows)
```

---

## TODO

- Publish `bwoc-setup-<tag>-<target>.tar.gz` / `.zip` release assets so `install.sh` / `install.ps1` can launch the wizard automatically after installing `bwoc`.
- The placeholder repo URL in the install scripts (`bemindlabs/bwoc-agent-installer`) should be updated once the repo is published.
