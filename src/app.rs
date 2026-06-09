/// app.rs — wizard state machine.
///
/// `App` holds the current `Stage` and all collected answers.  `next()` /
/// `back()` advance or retreat the stage sequence.  Action stages (RunInit,
/// RunNew, Verify) shell out to `bwoc` and store the result for rendering.
/// The UI module is responsible only for drawing; all logic lives here.

use crate::catalog::{self, Backend, BACKENDS};
use crate::exec;

// ---------------------------------------------------------------------------
// Answers collected across the wizard
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct WizardConfig {
    pub backend_idx: usize,           // index into BACKENDS
    pub base_url: String,             // only for openai-compatible
    pub workspace_path: String,
    pub workspace_single_agent: bool, // false = fleet (default)
    pub workspace_no_runtime: bool,   // false = runtime (default)
    pub lang: String,                 // "th" or "en"
    pub agent_name: String,
    pub agent_role: String,
    pub primary_model: String,
    pub fallback_model: String,       // empty = none
    pub advanced: bool,
}

impl WizardConfig {
    pub fn backend(&self) -> &Backend {
        &BACKENDS[self.backend_idx]
    }
}

// ---------------------------------------------------------------------------
// Per-stage UI variants
// ---------------------------------------------------------------------------

/// Which kind of interactive widget the current stage uses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputKind {
    /// Info-only screen — press Enter to continue.
    Info,
    /// A selection list: `cursor` points to the current item.
    Select { cursor: usize, items: Vec<String> },
    /// Single-line text buffer.
    Text { buffer: String, placeholder: String },
    /// A completed (or failed) action: show output.
    Action { ok: bool, output: String },
    /// bwoc-not-found error screen with retry/quit options.
    BwocMissing { cursor: usize },
}

// ---------------------------------------------------------------------------
// Stages
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Stage {
    Welcome,
    CheckBwoc,
    PickBackend,
    BaseUrl,
    WorkspacePath,
    WorkspaceMode,
    WorkspaceRuntime,
    WorkspaceLang,
    RunInit,
    AgentName,
    AgentRole,
    AgentModel,
    AgentFallback,
    RunNew,
    AdvancedPrompt,
    AdvancedInfo,
    Verify,
    Done,
}

impl Stage {
    /// Human-readable Thai name shown in the title bar.
    pub fn display_name(&self) -> &'static str {
        match self {
            Stage::Welcome => "ยินดีต้อนรับ",
            Stage::CheckBwoc => "ตรวจสอบ bwoc",
            Stage::PickBackend => "เลือก Backend",
            Stage::BaseUrl => "ที่อยู่ API (baseUrl)",
            Stage::WorkspacePath => "ที่อยู่ Workspace",
            Stage::WorkspaceMode => "โหมด Workspace",
            Stage::WorkspaceRuntime => "Runtime",
            Stage::WorkspaceLang => "ภาษา",
            Stage::RunInit => "สร้าง Workspace",
            Stage::AgentName => "ชื่อ Agent",
            Stage::AgentRole => "หน้าที่ Agent",
            Stage::AgentModel => "เลือก Model",
            Stage::AgentFallback => "Fallback Model",
            Stage::RunNew => "สร้าง Agent",
            Stage::AdvancedPrompt => "Feature เสริม?",
            Stage::AdvancedInfo => "Feature เสริม",
            Stage::Verify => "ตรวจสอบ",
            Stage::Done => "เสร็จสิ้น!",
        }
    }

    /// Sequence index (0-based), accounting for optional stages.
    pub fn index(&self, cfg: &WizardConfig) -> usize {
        Stage::sequence(self, cfg)
            .iter()
            .position(|s| s == self)
            .unwrap_or(0)
    }

    /// Full ordered sequence, conditioned on wizard choices so far.
    pub fn sequence(_self: &Stage, cfg: &WizardConfig) -> Vec<Stage> {
        let is_oai = BACKENDS[cfg.backend_idx].id == "openai-compatible";
        let has_advanced = cfg.advanced;

        let mut seq = vec![
            Stage::Welcome,
            Stage::CheckBwoc,
            Stage::PickBackend,
        ];
        if is_oai {
            seq.push(Stage::BaseUrl);
        }
        seq.extend([
            Stage::WorkspacePath,
            Stage::WorkspaceMode,
            Stage::WorkspaceRuntime,
            Stage::WorkspaceLang,
            Stage::RunInit,
            Stage::AgentName,
            Stage::AgentRole,
            Stage::AgentModel,
            Stage::AgentFallback,
            Stage::RunNew,
            Stage::AdvancedPrompt,
        ]);
        if has_advanced {
            seq.push(Stage::AdvancedInfo);
        }
        seq.extend([Stage::Verify, Stage::Done]);
        seq
    }
}

// ---------------------------------------------------------------------------
// App
// ---------------------------------------------------------------------------

pub struct App {
    pub stage: Stage,
    pub input: InputKind,
    pub cfg: WizardConfig,
    pub bwoc_version: String, // filled on CheckBwoc
    pub quit: bool,
    /// Custom model text buffer (AgentModel / AgentFallback "พิมพ์เอง").
    pub custom_model_buffer: String,
    pub in_custom_model: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            stage: Stage::Welcome,
            input: InputKind::Info,
            cfg: WizardConfig {
                lang: "th".to_string(),
                workspace_path: default_workspace(),
                agent_name: "alpha".to_string(),
                agent_role: "ผู้ช่วยทั่วไป".to_string(),
                primary_model: String::new(),
                fallback_model: String::new(),
                ..Default::default()
            },
            bwoc_version: String::new(),
            quit: false,
            custom_model_buffer: String::new(),
            in_custom_model: false,
        };
        app.enter_stage();
        app
    }

    // -----------------------------------------------------------------------
    // Navigation
    // -----------------------------------------------------------------------

    pub fn next(&mut self) {
        // If we're in a text field for custom model, confirm it first.
        if self.in_custom_model {
            let model = self.custom_model_buffer.trim().to_string();
            if !model.is_empty() {
                match self.stage {
                    Stage::AgentModel => self.cfg.primary_model = model,
                    Stage::AgentFallback => self.cfg.fallback_model = model,
                    _ => {}
                }
            }
            self.in_custom_model = false;
            self.custom_model_buffer.clear();
            self.advance_stage();
            return;
        }

        // Commit current input into cfg before advancing.
        self.commit_input();
        self.advance_stage();
    }

    pub fn back(&mut self) {
        if self.in_custom_model {
            self.in_custom_model = false;
            self.custom_model_buffer.clear();
            self.enter_stage(); // rebuild normal stage input
            return;
        }
        let seq = Stage::sequence(&self.stage, &self.cfg);
        if let Some(pos) = seq.iter().position(|s| s == &self.stage) {
            if pos > 0 {
                self.stage = seq[pos - 1].clone();
                self.enter_stage();
            }
        }
    }

    /// Retry action stages (RunInit, RunNew, Verify).
    pub fn retry(&mut self) {
        self.enter_stage();
    }

    // -----------------------------------------------------------------------
    // Key routing for the current stage
    // -----------------------------------------------------------------------

    pub fn handle_up(&mut self) {
        if self.in_custom_model {
            return;
        }
        match &mut self.input {
            InputKind::Select { cursor, items } => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
                let _ = items; // borrow satisfied
            }
            InputKind::BwocMissing { cursor } => {
                if *cursor > 0 {
                    *cursor -= 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_down(&mut self) {
        if self.in_custom_model {
            return;
        }
        match &mut self.input {
            InputKind::Select { cursor, items } => {
                let max = items.len().saturating_sub(1);
                if *cursor < max {
                    *cursor += 1;
                }
            }
            InputKind::BwocMissing { cursor } => {
                if *cursor < 1 {
                    *cursor += 1;
                }
            }
            _ => {}
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if self.in_custom_model {
            self.custom_model_buffer.push(c);
            return;
        }
        if let InputKind::Text { buffer, .. } = &mut self.input {
            buffer.push(c);
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.in_custom_model {
            self.custom_model_buffer.pop();
            return;
        }
        if let InputKind::Text { buffer, .. } = &mut self.input {
            buffer.pop();
        }
    }

    // -----------------------------------------------------------------------
    // Internal helpers
    // -----------------------------------------------------------------------

    fn advance_stage(&mut self) {
        let seq = Stage::sequence(&self.stage, &self.cfg);
        if let Some(pos) = seq.iter().position(|s| s == &self.stage) {
            if pos + 1 < seq.len() {
                self.stage = seq[pos + 1].clone();
                self.enter_stage();
            } else {
                // Already at Done.
                self.quit = true;
            }
        }
    }

    /// Build the `InputKind` for the newly-entered stage and run any
    /// side-effects (action stages shell out here).
    fn enter_stage(&mut self) {
        match &self.stage {
            Stage::Welcome => {
                self.input = InputKind::Info;
            }

            Stage::CheckBwoc => {
                let res = exec::bwoc(&["--version"]);
                if res.ok {
                    self.bwoc_version = res.stdout.clone();
                    self.input = InputKind::Action {
                        ok: true,
                        output: format!(
                            "พบ bwoc แล้ว! เวอร์ชัน: {}\n\nกด Enter เพื่อเริ่มต้น",
                            res.stdout
                        ),
                    };
                } else {
                    self.input = InputKind::BwocMissing { cursor: 0 };
                }
            }

            Stage::PickBackend => {
                let items: Vec<String> = BACKENDS
                    .iter()
                    .map(|b| {
                        let present = if b.binary.is_empty() {
                            // openai-compatible — no single binary
                            "  (ไม่ต้องการ CLI)".to_string()
                        } else if exec::binary_present(b.binary) {
                            "  ✓ ติดตั้งแล้ว".to_string()
                        } else {
                            "  ✗ ยังไม่ได้ติดตั้ง".to_string()
                        };
                        format!("{}{}", b.label, present)
                    })
                    .collect();
                self.input = InputKind::Select {
                    cursor: self.cfg.backend_idx,
                    items,
                };
            }

            Stage::BaseUrl => {
                self.input = InputKind::Text {
                    buffer: if self.cfg.base_url.is_empty() {
                        String::new()
                    } else {
                        self.cfg.base_url.clone()
                    },
                    placeholder: "http://localhost:11434/v1".to_string(),
                };
            }

            Stage::WorkspacePath => {
                self.input = InputKind::Text {
                    buffer: self.cfg.workspace_path.clone(),
                    placeholder: default_workspace(),
                };
            }

            Stage::WorkspaceMode => {
                self.input = InputKind::Select {
                    cursor: if self.cfg.workspace_single_agent { 1 } else { 0 },
                    items: vec![
                        "Fleet (ทีม) — หลาย agent ทำงานร่วมกัน".to_string(),
                        "Single-agent — agent เดี่ยว".to_string(),
                    ],
                };
            }

            Stage::WorkspaceRuntime => {
                self.input = InputKind::Select {
                    cursor: if self.cfg.workspace_no_runtime { 1 } else { 0 },
                    items: vec![
                        "รันได้จริง (มี runtime) — แนะนำ".to_string(),
                        "อ่านอย่างเดียว (--no-runtime)".to_string(),
                    ],
                };
            }

            Stage::WorkspaceLang => {
                self.input = InputKind::Select {
                    cursor: if self.cfg.lang == "en" { 1 } else { 0 },
                    items: vec![
                        "ไทย (th) — แนะนำ".to_string(),
                        "อังกฤษ (en)".to_string(),
                    ],
                };
            }

            Stage::RunInit => {
                let output = self.run_init();
                self.input = InputKind::Action {
                    ok: output.0,
                    output: output.1,
                };
            }

            Stage::AgentName => {
                self.input = InputKind::Text {
                    buffer: self.cfg.agent_name.clone(),
                    placeholder: "alpha".to_string(),
                };
            }

            Stage::AgentRole => {
                self.input = InputKind::Text {
                    buffer: self.cfg.agent_role.clone(),
                    placeholder: "ผู้ช่วยทั่วไป".to_string(),
                };
            }

            Stage::AgentModel => {
                let mut items = model_items_for(self.cfg.backend_idx);
                items.push("พิมพ์เอง (custom)".to_string());
                let cursor = if self.cfg.primary_model.is_empty() {
                    0
                } else {
                    items
                        .iter()
                        .position(|m| m == &self.cfg.primary_model)
                        .unwrap_or(0)
                };
                self.input = InputKind::Select { cursor, items };
            }

            Stage::AgentFallback => {
                let mut items = vec!["ไม่มี (none)".to_string()];
                items.extend(model_items_for(self.cfg.backend_idx));
                items.push("พิมพ์เอง (custom)".to_string());
                let cursor = 0; // default: none
                self.input = InputKind::Select { cursor, items };
            }

            Stage::RunNew => {
                let output = self.run_new();
                self.input = InputKind::Action {
                    ok: output.0,
                    output: output.1,
                };
            }

            Stage::AdvancedPrompt => {
                self.input = InputKind::Select {
                    cursor: if self.cfg.advanced { 0 } else { 1 },
                    items: vec![
                        "ใช่ อยากรู้ feature เสริม".to_string(),
                        "ไม่ ข้ามไปเลย".to_string(),
                    ],
                };
            }

            Stage::AdvancedInfo => {
                self.input = InputKind::Info;
            }

            Stage::Verify => {
                let output = self.run_verify();
                self.input = InputKind::Action {
                    ok: output.0,
                    output: output.1,
                };
            }

            Stage::Done => {
                self.input = InputKind::Info;
            }
        }
    }

    /// Persist the current widget value into `cfg`.
    fn commit_input(&mut self) {
        match self.stage.clone() {
            Stage::PickBackend => {
                if let InputKind::Select { cursor, .. } = &self.input {
                    self.cfg.backend_idx = *cursor;
                }
            }

            Stage::BaseUrl => {
                if let InputKind::Text { buffer, placeholder } = &self.input {
                    self.cfg.base_url = if buffer.trim().is_empty() {
                        placeholder.clone()
                    } else {
                        buffer.trim().to_string()
                    };
                }
            }

            Stage::WorkspacePath => {
                if let InputKind::Text { buffer, placeholder } = &self.input {
                    self.cfg.workspace_path = if buffer.trim().is_empty() {
                        placeholder.clone()
                    } else {
                        buffer.trim().to_string()
                    };
                }
            }

            Stage::WorkspaceMode => {
                if let InputKind::Select { cursor, .. } = &self.input {
                    self.cfg.workspace_single_agent = *cursor == 1;
                }
            }

            Stage::WorkspaceRuntime => {
                if let InputKind::Select { cursor, .. } = &self.input {
                    self.cfg.workspace_no_runtime = *cursor == 1;
                }
            }

            Stage::WorkspaceLang => {
                if let InputKind::Select { cursor, .. } = &self.input {
                    self.cfg.lang = if *cursor == 1 {
                        "en".to_string()
                    } else {
                        "th".to_string()
                    };
                }
            }

            Stage::AgentName => {
                if let InputKind::Text { buffer, placeholder } = &self.input {
                    let name = buffer.trim();
                    self.cfg.agent_name = if name.is_empty() {
                        placeholder.clone()
                    } else {
                        name.to_string()
                    };
                }
            }

            Stage::AgentRole => {
                if let InputKind::Text { buffer, placeholder } = &self.input {
                    let role = buffer.trim();
                    self.cfg.agent_role = if role.is_empty() {
                        placeholder.clone()
                    } else {
                        role.to_string()
                    };
                }
            }

            Stage::AgentModel => {
                if let InputKind::Select { cursor, items } = &self.input {
                    let chosen = &items[*cursor];
                    if chosen == "พิมพ์เอง (custom)" {
                        // Switch to custom text input instead of advancing.
                        self.in_custom_model = true;
                        self.custom_model_buffer.clear();
                        return;
                    }
                    self.cfg.primary_model = chosen.clone();
                }
            }

            Stage::AgentFallback => {
                if let InputKind::Select { cursor, items } = &self.input {
                    let chosen = &items[*cursor];
                    if chosen == "พิมพ์เอง (custom)" {
                        self.in_custom_model = true;
                        self.custom_model_buffer.clear();
                        return;
                    }
                    self.cfg.fallback_model = if chosen == "ไม่มี (none)" {
                        String::new()
                    } else {
                        chosen.clone()
                    };
                }
            }

            Stage::AdvancedPrompt => {
                if let InputKind::Select { cursor, .. } = &self.input {
                    self.cfg.advanced = *cursor == 0;
                }
            }

            // Action and info stages: nothing to commit.
            _ => {}
        }
    }

    // -----------------------------------------------------------------------
    // Action stage executors
    // -----------------------------------------------------------------------

    fn run_init(&self) -> (bool, String) {
        // Ensure directory exists.
        if let Err(e) = std::fs::create_dir_all(&self.cfg.workspace_path) {
            return (
                false,
                format!(
                    "ไม่สามารถสร้างโฟลเดอร์ {} ได้: {}",
                    self.cfg.workspace_path, e
                ),
            );
        }

        let mut args = vec!["init", &self.cfg.workspace_path];
        if self.cfg.workspace_single_agent {
            args.push("--single-agent");
        }
        if self.cfg.workspace_no_runtime {
            args.push("--no-runtime");
        }
        args.push("--lang");
        args.push(&self.cfg.lang);

        let res = exec::bwoc(&args);
        if res.ok {
            (
                true,
                format!(
                    "✓ สร้าง workspace สำเร็จ!\n\nที่อยู่: {}\n\n{}",
                    self.cfg.workspace_path,
                    res.combined()
                ),
            )
        } else {
            (
                false,
                format!(
                    "✗ เกิดข้อผิดพลาดขณะสร้าง workspace:\n\n{}",
                    res.combined()
                ),
            )
        }
    }

    fn run_new(&self) -> (bool, String) {
        let backend = self.cfg.backend();
        let mut args: Vec<String> = vec![
            "new".to_string(),
            self.cfg.agent_name.clone(),
            "--workspace".to_string(),
            self.cfg.workspace_path.clone(),
            "--backend".to_string(),
            backend.id.to_string(),
            "--role".to_string(),
            self.cfg.agent_role.clone(),
            "--primary-model".to_string(),
            self.cfg.primary_model.clone(),
            "--lang".to_string(),
            self.cfg.lang.clone(),
        ];

        if !self.cfg.fallback_model.is_empty() {
            args.push("--fallback-model".to_string());
            args.push(self.cfg.fallback_model.clone());
        }

        if backend.id == "openai-compatible" && !self.cfg.base_url.is_empty() {
            // `bwoc new` exposes the OpenAI-compatible baseUrl via `--endpoint`
            // (recorded as `baseUrl` in config.manifest.json) — there is no
            // `--base-url` flag.
            args.push("--endpoint".to_string());
            args.push(self.cfg.base_url.clone());
        }

        let arg_refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let res = exec::bwoc(&arg_refs);

        if res.ok {
            (
                true,
                format!(
                    "✓ สร้าง agent '{}' สำเร็จ!\n\nBackend: {}\nModel: {}\n\n{}",
                    self.cfg.agent_name,
                    backend.label,
                    self.cfg.primary_model,
                    res.combined()
                ),
            )
        } else {
            (
                false,
                format!("✗ เกิดข้อผิดพลาดขณะสร้าง agent:\n\n{}", res.combined()),
            )
        }
    }

    fn run_verify(&self) -> (bool, String) {
        let mut lines: Vec<String> = Vec::new();
        let mut all_ok = true;

        // bwoc doctor
        let r = exec::bwoc(&["doctor"]);
        let mark = if r.ok { "✓" } else { "✗" };
        lines.push(format!("{mark} bwoc doctor"));
        if !r.stdout.is_empty() {
            for l in r.stdout.lines() {
                lines.push(format!("  {l}"));
            }
        }
        if !r.ok {
            all_ok = false;
        }

        // bwoc check <agent path>
        let agent_path = format!(
            "{}/agents/agent-{}",
            self.cfg.workspace_path, self.cfg.agent_name
        );
        let r2 = exec::bwoc(&["check", &agent_path]);
        let mark2 = if r2.ok { "✓" } else { "✗" };
        lines.push(format!("{mark2} bwoc check agent-{}", self.cfg.agent_name));
        if !r2.stdout.is_empty() {
            for l in r2.stdout.lines() {
                lines.push(format!("  {l}"));
            }
        }
        if !r2.ok {
            all_ok = false;
        }

        // bwoc list --workspace <path>
        let r3 = exec::bwoc(&["list", "--workspace", &self.cfg.workspace_path]);
        if r3.ok {
            lines.push("✓ bwoc list".to_string());
            for l in r3.stdout.lines() {
                lines.push(format!("  {l}"));
            }
        } else {
            // Fallback without --workspace flag.
            let r4 = exec::bwoc(&["list"]);
            let mark4 = if r4.ok { "✓" } else { "✗" };
            lines.push(format!("{mark4} bwoc list"));
            if !r4.ok {
                all_ok = false;
            }
        }

        (all_ok, lines.join("\n"))
    }

    // -----------------------------------------------------------------------
    // Stage total count (for step indicator)
    // -----------------------------------------------------------------------

    pub fn total_stages(&self) -> usize {
        Stage::sequence(&self.stage, &self.cfg).len()
    }

    pub fn current_step(&self) -> usize {
        self.stage.index(&self.cfg) + 1
    }

    // -----------------------------------------------------------------------
    // Right-pane explanation for current stage
    // -----------------------------------------------------------------------

    pub fn right_pane_text(&self) -> String {
        match &self.stage {
            Stage::Welcome => "\
BWOC (Buddhist Way of Coding) เป็น framework\n\
สำหรับสร้างและจัดการ AI coding agent\n\n\
Wizard นี้จะช่วยคุณตั้งค่าทุกอย่างทีละขั้นตอน:\n\
  • เลือก AI backend ที่ต้องการ\n\
  • สร้าง workspace (โฟลเดอร์หลัก)\n\
  • สร้าง agent ตัวแรก\n\
  • ตรวจสอบว่าทุกอย่างพร้อมใช้งาน\n\n\
กด Enter เพื่อเริ่มต้น"
                .to_string(),

            Stage::CheckBwoc => "\
กำลังตรวจสอบว่า bwoc CLI\n\
ติดตั้งและพร้อมใช้งานหรือยัง\n\n\
ถ้า bwoc ยังไม่ได้ติดตั้ง\n\
ให้รัน install.sh ก่อนแล้วลองใหม่\n\n\
ดู README.md สำหรับวิธีติดตั้ง"
                .to_string(),

            Stage::PickBackend => {
                let b = self.cfg.backend();
                b.description.to_string()
            }

            Stage::BaseUrl => catalog::HELP_BASE_URL.body.to_string(),

            Stage::WorkspacePath => catalog::HELP_WORKSPACE_PATH.body.to_string(),

            Stage::WorkspaceMode => catalog::HELP_WORKSPACE_MODE.body.to_string(),

            Stage::WorkspaceRuntime => catalog::HELP_WORKSPACE_RUNTIME.body.to_string(),

            Stage::WorkspaceLang => catalog::HELP_WORKSPACE_LANG.body.to_string(),

            Stage::RunInit => "\
กำลังรัน: bwoc init\n\n\
สร้าง workspace ตามการตั้งค่าของคุณ\n\
กระบวนการนี้ใช้เวลาไม่กี่วินาที\n\n\
ถ้าเกิดข้อผิดพลาด กด ← เพื่อกลับไป\n\
แก้ไขการตั้งค่าแล้วลองอีกครั้ง"
                .to_string(),

            Stage::AgentName => catalog::HELP_AGENT_NAME.body.to_string(),

            Stage::AgentRole => catalog::HELP_AGENT_ROLE.body.to_string(),

            Stage::AgentModel => catalog::HELP_AGENT_MODEL.body.to_string(),

            Stage::AgentFallback => catalog::HELP_AGENT_FALLBACK.body.to_string(),

            Stage::RunNew => "\
กำลังรัน: bwoc new\n\n\
สร้าง agent ตามการตั้งค่าของคุณ\n\
รวมถึง persona, mindset, และ config\n\n\
ถ้าเกิดข้อผิดพลาด กด ← เพื่อกลับไป\n\
แก้ไขแล้วลองอีกครั้ง"
                .to_string(),

            Stage::AdvancedPrompt => "\
BWOC มี feature เสริมที่น่าสนใจ:\n\n\
Teams — สร้างกลุ่ม agent ที่ทำงาน\n\
ร่วมกันและแชร์ task list\n\n\
Skills — เพิ่มความสามารถพิเศษให้ agent\n\
เช่น ค้นหาเว็บ, อ่าน PDF\n\n\
Plugins — เชื่อมต่อกับระบบนอก\n\
เช่น Jira, Figma, Slack\n\n\
ทุกอย่างสามารถเพิ่มได้ทีหลัง"
                .to_string(),

            Stage::AdvancedInfo => "\
สรุป feature เสริมของ BWOC:\n\n\
TEAMS\n\
  bwoc team list\n\
  bwoc team create <ชื่อ>\n\
  กลุ่ม agent ที่แชร์ task list\n\n\
SKILLS\n\
  bwoc skill list\n\
  bwoc skill add <agent> <skill>\n\
  ความสามารถเสริมที่ติดให้ agent\n\n\
PLUGINS\n\
  bwoc plugin list\n\
  bwoc plugin install <ชื่อ>\n\
  เชื่อมต่อ Jira/Figma/Slack/ฯลฯ\n\n\
ไม่ต้องทำตอนนี้ก็ได้!"
                .to_string(),

            Stage::Verify => "\
กำลังตรวจสอบทุกอย่าง:\n\n\
• bwoc doctor — สุขภาพโดยรวม\n\
• bwoc check — ตรวจสอบ agent\n\
• bwoc list — แสดง agent ที่มี\n\n\
✓ = ผ่าน\n\
✗ = มีปัญหา (ดู error ด้านซ้าย)"
                .to_string(),

            Stage::Done => {
                format!(
                    "ยินดีด้วย! ตั้งค่าเสร็จสมบูรณ์\n\n\
Workspace: {}\nAgent: {}\nBackend: {}\nModel: {}\n\n\
ขั้นตอนต่อไป:\n\
  cd {}\n\
  bwoc list\n\
  bwoc spawn --path agents/agent-{}\n\
  bwoc chat {}\n\n\
กด Enter หรือ q เพื่อออก",
                    self.cfg.workspace_path,
                    self.cfg.agent_name,
                    self.cfg.backend().label,
                    self.cfg.primary_model,
                    self.cfg.workspace_path,
                    self.cfg.agent_name,
                    self.cfg.agent_name,
                )
            }
        }
    }

    // -----------------------------------------------------------------------
    // Agent-name validation
    // -----------------------------------------------------------------------

    /// Returns Some(error message) if current agent name buffer is invalid.
    pub fn agent_name_error(&self) -> Option<String> {
        if self.stage != Stage::AgentName {
            return None;
        }
        if let InputKind::Text { buffer, .. } = &self.input {
            let name = buffer.trim();
            if name.is_empty() {
                return None; // will fall back to placeholder
            }
            if !name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
            {
                return Some(
                    "ชื่อต้องเป็น lowercase, ตัวเลข, และ - เท่านั้น".to_string(),
                );
            }
        }
        None
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn default_workspace() -> String {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());
    format!("{}/bwoc-workspace", home)
}

fn model_items_for(backend_idx: usize) -> Vec<String> {
    BACKENDS[backend_idx]
        .models
        .iter()
        .map(|m| m.to_string())
        .collect()
}
