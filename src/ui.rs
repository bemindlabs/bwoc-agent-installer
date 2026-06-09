/// ui.rs — ratatui rendering.
///
/// 3-pane layout: left = interactive, right = explanation, bottom = key hints.
/// Title bar shows step N/Total + stage name.
///
/// All user-visible strings consult `app.lang` for bilingual output.
/// F2 toggles the language live at any stage.
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::app::{App, InputKind, Stage};
use crate::exec;
use crate::i18n::{Lang, t};
use crate::theme;

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // Vertical split: title | body | hints
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title bar
            Constraint::Min(0),    // body
            Constraint::Length(2), // key hints (2 rows so wrapping doesn't truncate)
        ])
        .split(area);

    draw_title(f, rows[0], app);
    draw_body(f, rows[1], app);
    draw_hints(f, rows[2], app);
}

// ---------------------------------------------------------------------------
// Title bar
// ---------------------------------------------------------------------------

fn draw_title(f: &mut Frame, area: Rect, app: &App) {
    let step = app.current_step();
    let total = app.total_stages();
    let name = app.stage.display_name(app.lang);
    let step_label = t(app.lang, "Step", "ขั้นตอน");
    let text = format!(" BWOC Setup  {step_label} {step}/{total}: {name} ");
    let p = Paragraph::new(Line::from(Span::styled(text, theme::title_bar_style())));
    f.render_widget(p, area);
}

// ---------------------------------------------------------------------------
// Body: left pane + right pane
// ---------------------------------------------------------------------------

fn draw_body(f: &mut Frame, area: Rect, app: &App) {
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    draw_left(f, cols[0], app);
    draw_right(f, cols[1], app);
}

fn draw_left(f: &mut Frame, area: Rect, app: &App) {
    let lang = app.lang;
    let title = format!(" {} ", app.stage.display_name(lang));
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(theme::ACCENT))
        .padding(theme::pane_padding());

    match &app.input {
        InputKind::Info => {
            let body = info_body(app);
            let p = Paragraph::new(body).block(block).wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }

        InputKind::Select { cursor, items } => {
            // PickBackend uses styled lines (✓ green / ✗ red); other selects
            // use plain strings.
            let is_pick_backend = app.stage == Stage::PickBackend;
            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let _ = i; // cursor highlight is done via highlight_style
                    if is_pick_backend {
                        ListItem::new(backend_line(s))
                    } else {
                        ListItem::new(Line::from(s.as_str()))
                    }
                })
                .collect();
            let mut state = ListState::default();
            state.select(Some(*cursor));
            let list = List::new(list_items)
                .block(block)
                .highlight_style(theme::selection_style())
                .highlight_symbol("▶ ");
            f.render_stateful_widget(list, area, &mut state);
        }

        InputKind::Text {
            buffer,
            placeholder,
        } => {
            let display = if app.in_custom_model {
                format!(
                    "{}:\n> {}█\n\n({})",
                    t(lang, "Type model name", "พิมพ์ชื่อ model"),
                    app.custom_model_buffer,
                    t(
                        lang,
                        "Enter to confirm, Esc to cancel",
                        "Enter เพื่อยืนยัน, Esc เพื่อกลับ"
                    ),
                )
            } else {
                let shown = if buffer.is_empty() {
                    format!("▏ ({}: {placeholder})", t(lang, "default", "ค่าเริ่มต้น"))
                } else {
                    format!("> {buffer}█")
                };
                let hint = app
                    .agent_name_error()
                    .map(|e| format!("\n\n⚠ {e}"))
                    .unwrap_or_default();
                format!("{shown}{hint}")
            };
            let p = Paragraph::new(display)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }

        InputKind::Action { ok, output } => {
            let status_line = if *ok {
                Line::from(Span::styled(
                    t(lang, "✓ Success", "✓ สำเร็จ"),
                    theme::success_style(),
                ))
            } else {
                Line::from(Span::styled(
                    t(lang, "✗ Error", "✗ เกิดข้อผิดพลาด"),
                    theme::danger_style(),
                ))
            };
            let mut lines: Vec<Line> = vec![status_line, Line::from("")];
            for l in output.lines() {
                lines.push(Line::from(l.to_string()));
            }
            let p = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }

        InputKind::BwocMissing { cursor } => {
            let retry_label = t(lang, "[Retry]", "[ลองใหม่]");
            let quit_label = t(lang, "[Quit]", "[ออก]");
            let opts = [retry_label, quit_label];
            let mut lines: Vec<Line> = vec![
                Line::from(Span::styled(
                    t(lang, "✗ bwoc command not found", "✗ ไม่พบคำสั่ง bwoc"),
                    theme::danger_style(),
                )),
                Line::from(""),
                Line::from(t(
                    lang,
                    "bwoc is not installed or not on PATH.",
                    "bwoc ยังไม่ได้ติดตั้ง หรือยังไม่ได้เพิ่มใน PATH",
                )),
                Line::from(""),
                Line::from(t(lang, "How to fix:", "วิธีแก้ไข:")),
                Line::from(t(
                    lang,
                    "  1. Run scripts/install.sh first",
                    "  1. รัน scripts/install.sh ก่อน",
                )),
                Line::from(t(
                    lang,
                    "  2. Verify ~/.local/bin is on PATH",
                    "  2. ตรวจสอบว่า PATH มี ~/.local/bin",
                )),
                Line::from(t(
                    lang,
                    "  3. Open a new terminal and retry",
                    "  3. เปิด terminal ใหม่แล้วลองอีกครั้ง",
                )),
                Line::from(""),
            ];
            for (i, opt) in opts.iter().enumerate() {
                let style = if i == *cursor {
                    theme::selection_style()
                } else {
                    Style::default()
                };
                lines.push(Line::from(Span::styled(opt.to_string(), style)));
            }
            let p = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }
    }
}

/// Parse a backend list-item string (e.g. "Claude (Anthropic)  ✓ installed")
/// into a styled `Line`: backend label in default colour, then the
/// ✓/✗ suffix in SUCCESS (green) or DANGER (red).
fn backend_line(s: &str) -> Line<'static> {
    // The strings were built in app.rs as `format!("{}{}", b.label, present)`
    // where `present` starts with two spaces and a ✓ or ✗.
    // We locate the first ✓ or ✗ to split label from status.
    let owned = s.to_string();
    if let Some(pos) = owned.find(['✓', '✗']) {
        let label = owned[..pos].trim_end().to_string();
        let status_str = owned[pos..].to_string();
        let status_style = if status_str.starts_with('✓') {
            Style::default().fg(theme::SUCCESS)
        } else {
            Style::default().fg(theme::DANGER)
        };
        Line::from(vec![
            Span::raw(label),
            Span::raw("  "),
            Span::styled(status_str, status_style),
        ])
    } else {
        // openai-compatible "(no CLI required)" or unrecognised — plain text
        Line::from(owned)
    }
}

fn draw_right(f: &mut Frame, area: Rect, app: &App) {
    let title = right_pane_title(app);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(theme::MUTED))
        .padding(theme::pane_padding());

    // For PickBackend, optionally append a warning if the highlighted backend's
    // CLI is not installed.
    let base_text = app.right_pane_text();
    let extra = pick_backend_warning(app);
    let full_text = if extra.is_empty() {
        base_text
    } else {
        format!("{base_text}\n\n{extra}")
    };

    let p = Paragraph::new(full_text)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);
}

/// Return a warning string when the highlighted PickBackend entry's CLI is
/// absent (and the backend is not openai-compatible).  Empty otherwise.
fn pick_backend_warning(app: &App) -> String {
    if app.stage != Stage::PickBackend {
        return String::new();
    }
    if let InputKind::Select { cursor, .. } = &app.input {
        use crate::catalog::BACKENDS;
        let b = &BACKENDS[*cursor];
        if b.binary.is_empty() {
            // openai-compatible — no CLI required
            return String::new();
        }
        if !exec::binary_present(b.binary) {
            return t(
                app.lang,
                "⚠ This backend's CLI isn't installed yet — install & log in before the agent can run.",
                "⚠ ยังไม่ได้ติดตั้ง CLI ของ backend นี้ — ต้องติดตั้งและล็อกอินก่อน agent ถึงจะรันได้",
            ).to_string();
        }
    }
    String::new()
}

// ---------------------------------------------------------------------------
// Key hints bar
// ---------------------------------------------------------------------------

fn draw_hints(f: &mut Frame, area: Rect, app: &App) {
    let hints = build_hints(app);
    // Build spans: [key-chip] [space desc space] ...
    let spans: Vec<Span> = hints
        .into_iter()
        .flat_map(|(key, desc)| {
            vec![
                Span::styled(format!(" {key} "), theme::key_chip_style()),
                Span::styled(format!(" {desc}  "), theme::muted_style()),
            ]
        })
        .collect();
    // Wrap so hints don't truncate on narrow terminals.
    let p = Paragraph::new(Line::from(spans)).wrap(Wrap { trim: true });
    f.render_widget(p, area);
}

fn build_hints(app: &App) -> Vec<(&'static str, &'static str)> {
    let lang = app.lang;
    let mut hints: Vec<(&'static str, &'static str)> = Vec::new();

    match &app.input {
        InputKind::Select { .. } => {
            hints.push(("↑↓", t(lang, "move", "เลื่อน")));
            hints.push(("Enter", t(lang, "select/next", "เลือก/ถัดไป")));
        }
        InputKind::Text { .. } => {
            hints.push(("type", t(lang, "edit", "แก้ไข")));
            hints.push(("Enter", t(lang, "confirm", "ยืนยัน")));
            hints.push(("Backspace", t(lang, "delete", "ลบ")));
        }
        InputKind::Info => {
            hints.push(("Enter", t(lang, "next", "ถัดไป")));
        }
        InputKind::Action { ok, .. } => {
            hints.push(("Enter", t(lang, "next", "ถัดไป")));
            if !ok {
                hints.push(("r", t(lang, "retry", "ลองใหม่")));
            }
        }
        InputKind::BwocMissing { .. } => {
            hints.push(("↑↓", t(lang, "move", "เลื่อน")));
            hints.push(("Enter", t(lang, "select", "เลือก")));
        }
    }

    let can_back = app.current_step() > 1
        && !matches!(app.stage, Stage::Done)
        && !matches!(app.input, InputKind::BwocMissing { .. });
    if can_back {
        hints.push(("← / Esc", t(lang, "back", "ย้อนกลับ")));
    }

    hints.push(("F2", "language / ภาษา"));
    hints.push(("Ctrl-C", t(lang, "quit", "ออก")));
    hints
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn right_pane_title(app: &App) -> String {
    let lang = app.lang;
    match &app.stage {
        Stage::LangSelect => " Language / ภาษา ".to_string(),
        Stage::PickBackend => {
            let b = &crate::catalog::BACKENDS[app.cfg.backend_idx];
            format!(" {} ", b.label)
        }
        Stage::BaseUrl => format!(" {} {} ", theme::INFO, t(lang, "BaseUrl", "BaseUrl")),
        Stage::WorkspacePath => format!(" {} {} ", theme::INFO, t(lang, "Workspace", "Workspace")),
        Stage::WorkspaceMode => format!(" {} {} ", theme::INFO, t(lang, "Mode", "โหมด")),
        Stage::WorkspaceRuntime => format!(" {} Runtime ", theme::INFO),
        Stage::WorkspaceLang => {
            format!(" {} {} ", theme::INFO, t(lang, "CLI Language", "ภาษา CLI"))
        }
        Stage::AgentName => format!(" {} {} ", theme::INFO, t(lang, "Agent Name", "ชื่อ Agent")),
        Stage::AgentRole => format!(" {} {} ", theme::INFO, t(lang, "Role", "หน้าที่")),
        Stage::AgentModel => format!(" {} Model ", theme::INFO),
        Stage::AgentFallback => format!(" {} Fallback ", theme::INFO),
        _ => format!(" {} {} ", theme::INFO, t(lang, "Info", "ข้อมูล")),
    }
}

/// Build the left-pane body text for Info stages.
fn info_body(app: &App) -> String {
    let lang = app.lang;
    match &app.stage {
        Stage::Welcome => match lang {
            Lang::En => "\
Welcome to BWOC Setup!\n\n\
BWOC (Buddhist Way of Coding)\n\
is a framework for building AI agents\n\
that work with a variety of backends.\n\n\
This wizard walks you through setup\n\
from start to finish — won't take long.\n\n\
Press Enter to begin ⟶"
                .to_string(),
            Lang::Th => "\
ยินดีต้อนรับสู่ BWOC Setup!\n\n\
BWOC (Buddhist Way of Coding)\n\
คือ framework สำหรับสร้าง AI agent\n\
ที่ทำงานร่วมกับ backend ต่าง ๆ ได้\n\n\
Wizard นี้จะพาคุณตั้งค่าทุกอย่าง\n\
ตั้งแต่ต้นจนจบ ใช้เวลาไม่นาน\n\n\
กด Enter เพื่อเริ่มต้น ⟶"
                .to_string(),
        },

        Stage::AdvancedInfo => match lang {
            Lang::En => "\
BWOC extra features:\n\n\
TEAMS (bwoc team)\n\
  Create agent groups that share tasks\n\
  and collaborate together.\n\n\
SKILLS (bwoc skill)\n\
  Add special capabilities to an agent,\n\
  e.g. web search, read PDF files.\n\n\
PLUGINS (bwoc plugin)\n\
  Connect to Jira, Figma, Slack,\n\
  and other external systems.\n\n\
Everything can be added later — no rush.\n\n\
Press Enter to continue ⟶"
                .to_string(),
            Lang::Th => "\
Feature เสริมของ BWOC:\n\n\
TEAMS (bwoc team)\n\
  สร้างกลุ่ม agent ที่แชร์ task\n\
  และทำงานร่วมกันได้\n\n\
SKILLS (bwoc skill)\n\
  เพิ่มความสามารถพิเศษให้ agent\n\
  เช่น ค้นหาเว็บ, อ่านไฟล์ PDF\n\n\
PLUGINS (bwoc plugin)\n\
  เชื่อมต่อกับ Jira, Figma, Slack\n\
  และระบบภายนอกอื่น ๆ\n\n\
ทุกอย่างเพิ่มได้ทีหลัง ไม่ต้องทำตอนนี้\n\n\
กด Enter เพื่อไปขั้นตอนต่อไป ⟶"
                .to_string(),
        },

        Stage::Done => {
            let (ws, agent, backend, model) = (
                &app.cfg.workspace_path,
                &app.cfg.agent_name,
                app.cfg.backend().label,
                &app.cfg.primary_model,
            );
            match lang {
                Lang::En => format!(
                    "Setup complete!\n\n\
What was created:\n\
  Workspace: {ws}\n\
  Agent:     {agent}\n\
  Backend:   {backend}\n\
  Model:     {model}\n\n\
Getting started:\n\
  cd {ws}\n\
  bwoc list\n\
  bwoc spawn --path agents/agent-{agent}\n\
  bwoc chat {agent}\n\n\
Press Enter or q to exit"
                ),
                Lang::Th => format!(
                    "ตั้งค่าเสร็จสมบูรณ์!\n\n\
สิ่งที่สร้างแล้ว:\n\
  Workspace: {ws}\n\
  Agent:     {agent}\n\
  Backend:   {backend}\n\
  Model:     {model}\n\n\
คำสั่งเริ่มต้น:\n\
  cd {ws}\n\
  bwoc list\n\
  bwoc spawn --path agents/agent-{agent}\n\
  bwoc chat {agent}\n\n\
กด Enter หรือ q เพื่อออก"
                ),
            }
        }

        _ => String::new(),
    }
}
