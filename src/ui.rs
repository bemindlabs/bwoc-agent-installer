/// ui.rs — ratatui rendering.
///
/// 3-pane layout: left = interactive, right = explanation, bottom = key hints.
/// Title bar shows step N/Total + stage name.

use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap};

use crate::app::{App, InputKind, Stage};

// Colour palette (terminal-theme-friendly named colours).
const C_ACCENT: Color = Color::Cyan;
const C_SUCCESS: Color = Color::Green;
const C_ERROR: Color = Color::Red;
const C_DIM: Color = Color::DarkGray;
const C_SELECTED: Color = Color::Yellow;

pub fn draw(f: &mut Frame, app: &App) {
    let area = f.area();

    // Vertical split: title | body | hints
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // title bar
            Constraint::Min(0),    // body
            Constraint::Length(1), // key hints
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
    let name = app.stage.display_name();
    let text = format!(" BWOC Setup  ขั้นตอน {step}/{total}: {name} ");
    let p = Paragraph::new(Line::from(Span::styled(
        text,
        Style::default()
            .fg(Color::Black)
            .bg(C_ACCENT)
            .add_modifier(Modifier::BOLD),
    )));
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
    let title = format!(" {} ", app.stage.display_name());
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(C_ACCENT));

    match &app.input {
        InputKind::Info => {
            let body = info_body(app);
            let p = Paragraph::new(body)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }

        InputKind::Select { cursor, items } => {
            let list_items: Vec<ListItem> = items
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let style = if i == *cursor {
                        Style::default()
                            .fg(C_SELECTED)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    ListItem::new(Line::from(Span::styled(
                        format!("  {s}"),
                        style,
                    )))
                })
                .collect();
            let mut state = ListState::default();
            state.select(Some(*cursor));
            let list = List::new(list_items)
                .block(block)
                .highlight_style(
                    Style::default()
                        .fg(C_SELECTED)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol("▶ ");
            f.render_stateful_widget(list, area, &mut state);
        }

        InputKind::Text { buffer, placeholder } => {
            let display = if app.in_custom_model {
                // Custom model text input overlays the normal text input.
                format!(
                    "พิมพ์ชื่อ model:\n> {}█\n\n(Enter เพื่อยืนยัน, Esc เพื่อกลับ)",
                    app.custom_model_buffer
                )
            } else {
                let shown = if buffer.is_empty() {
                    format!("▏ (default: {placeholder})")
                } else {
                    format!("> {buffer}█")
                };
                // Show agent-name validation hint if applicable.
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
                    "✓ สำเร็จ",
                    Style::default().fg(C_SUCCESS).add_modifier(Modifier::BOLD),
                ))
            } else {
                Line::from(Span::styled(
                    "✗ เกิดข้อผิดพลาด",
                    Style::default().fg(C_ERROR).add_modifier(Modifier::BOLD),
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
            let opts = ["[ลองใหม่]", "[ออก]"];
            let mut lines: Vec<Line> = vec![
                Line::from(Span::styled(
                    "✗ ไม่พบคำสั่ง bwoc",
                    Style::default().fg(C_ERROR).add_modifier(Modifier::BOLD),
                )),
                Line::from(""),
                Line::from(
                    "bwoc ยังไม่ได้ติดตั้ง หรือยังไม่ได้เพิ่มใน PATH",
                ),
                Line::from(""),
                Line::from("วิธีแก้ไข:"),
                Line::from("  1. รัน scripts/install.sh ก่อน"),
                Line::from("  2. ตรวจสอบว่า PATH มี ~/.local/bin"),
                Line::from("  3. เปิด terminal ใหม่แล้วลองอีกครั้ง"),
                Line::from(""),
            ];
            for (i, opt) in opts.iter().enumerate() {
                let style = if i == *cursor {
                    Style::default()
                        .fg(C_SELECTED)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                lines.push(Line::from(Span::styled(
                    format!("  {opt}"),
                    style,
                )));
            }
            let p = Paragraph::new(lines)
                .block(block)
                .wrap(Wrap { trim: false });
            f.render_widget(p, area);
        }
    }
}

fn draw_right(f: &mut Frame, area: Rect, app: &App) {
    let title = right_pane_title(app);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(C_DIM));

    let text = app.right_pane_text();
    let p = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });
    f.render_widget(p, area);
}

// ---------------------------------------------------------------------------
// Key hints bar
// ---------------------------------------------------------------------------

fn draw_hints(f: &mut Frame, area: Rect, app: &App) {
    let hints = build_hints(app);
    let p = Paragraph::new(Line::from(
        hints
            .into_iter()
            .map(|(key, desc)| {
                vec![
                    Span::styled(
                        format!(" {key}"),
                        Style::default()
                            .fg(Color::Black)
                            .bg(C_DIM)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" {desc} "),
                        Style::default().fg(C_DIM),
                    ),
                ]
            })
            .flatten()
            .collect::<Vec<_>>(),
    ));
    f.render_widget(p, area);
}

fn build_hints(app: &App) -> Vec<(&'static str, &'static str)> {
    let mut hints: Vec<(&'static str, &'static str)> = Vec::new();

    match &app.input {
        InputKind::Select { .. } => {
            hints.push(("↑↓", "เลื่อน"));
            hints.push(("Enter", "เลือก/ถัดไป"));
        }
        InputKind::Text { .. } => {
            hints.push(("พิมพ์", "แก้ไข"));
            hints.push(("Enter", "ยืนยัน"));
            hints.push(("Backspace", "ลบ"));
        }
        InputKind::Info => {
            hints.push(("Enter", "ถัดไป"));
        }
        InputKind::Action { ok, .. } => {
            hints.push(("Enter", "ถัดไป"));
            if !ok {
                hints.push(("r", "ลองใหม่"));
            }
        }
        InputKind::BwocMissing { .. } => {
            hints.push(("↑↓", "เลื่อน"));
            hints.push(("Enter", "เลือก"));
        }
    }

    // Back is always available except at stage 0 and action/Done stages.
    let can_back = app.current_step() > 1
        && !matches!(app.stage, Stage::Done)
        && !matches!(app.input, InputKind::BwocMissing { .. });
    if can_back {
        hints.push(("← / Esc", "ย้อนกลับ"));
    }

    hints.push(("Ctrl-C", "ออก"));
    hints
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn right_pane_title(app: &App) -> String {
    match &app.stage {
        Stage::PickBackend => {
            let b = &crate::catalog::BACKENDS[app.cfg.backend_idx];
            format!(" {} ", b.label)
        }
        Stage::BaseUrl => " ℹ BaseUrl ".to_string(),
        Stage::WorkspacePath => " ℹ Workspace ".to_string(),
        Stage::WorkspaceMode => " ℹ โหมด ".to_string(),
        Stage::WorkspaceRuntime => " ℹ Runtime ".to_string(),
        Stage::WorkspaceLang => " ℹ ภาษา ".to_string(),
        Stage::AgentName => " ℹ ชื่อ Agent ".to_string(),
        Stage::AgentRole => " ℹ หน้าที่ ".to_string(),
        Stage::AgentModel => " ℹ Model ".to_string(),
        Stage::AgentFallback => " ℹ Fallback ".to_string(),
        _ => " ℹ ข้อมูล ".to_string(),
    }
}

/// Build the left-pane body text for info stages.
fn info_body(app: &App) -> String {
    match &app.stage {
        Stage::Welcome => "\
ยินดีต้อนรับสู่ BWOC Setup!\n\n\
BWOC (Buddhist Way of Coding)\n\
คือ framework สำหรับสร้าง AI agent\n\
ที่ทำงานร่วมกับ backend ต่าง ๆ ได้\n\n\
Wizard นี้จะพาคุณตั้งค่าทุกอย่าง\n\
ตั้งแต่ต้นจนจบ ใช้เวลาไม่นาน\n\n\
กด Enter เพื่อเริ่มต้น ⟶"
            .to_string(),

        Stage::AdvancedInfo => "\
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

        Stage::Done => {
            format!(
                "ตั้งค่าเสร็จสมบูรณ์!\n\n\
สิ่งที่สร้างแล้ว:\n\
  Workspace: {}\n\
  Agent: {}\n\
  Backend: {}\n\
  Model: {}\n\n\
คำสั่งเริ่มต้น:\n\
  cd {}\n\
  bwoc list\n\
  bwoc spawn --path \\\n\
    agents/agent-{}\n\
  bwoc chat {}\n\n\
กด Enter หรือ q เพื่อออก",
                app.cfg.workspace_path,
                app.cfg.agent_name,
                app.cfg.backend().label,
                app.cfg.primary_model,
                app.cfg.workspace_path,
                app.cfg.agent_name,
                app.cfg.agent_name,
            )
        }

        _ => String::new(),
    }
}
