//! Design-token theme for `bwoc-setup`.
//!
//! **Mirrors `bwoc_core::design`** — that module is the upstream single source
//! of truth for the BWOC design system. `bwoc-setup` is intentionally decoupled
//! from `bwoc-core` (it must stay a lean standalone binary), so we copy the
//! *ANSI half* of the palette here. If the framework palette changes, keep
//! this file in sync.
//!
//! Mapping: `bwoc_core::design::color::<TOKEN>.ansi` → `ratatui::style::Color`.
//!   ACCENT       Ansi::Cyan   → Color::Cyan
//!   TITLE        Ansi::Yellow → Color::Yellow
//!   SELECTION_BG Ansi::Blue   → Color::Blue
//!   SELECTION_FG Ansi::White  → Color::White
//!   SUCCESS      Ansi::Green  → Color::Green
//!   WARNING      Ansi::Yellow → Color::Yellow
//!   DANGER       Ansi::Red    → Color::Red
//!   MUTED        Ansi::Gray   → Color::Gray   (never DarkGray — near-invisible on dark terminals)

// Design-token constants are a palette; not every token is consumed by the
// current UI. Suppress dead-code warnings so the full token set can be
// declared (for sync with bwoc_core::design) without forcing use of each.
#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Padding;

// ---------------------------------------------------------------------------
// Colour tokens
// ---------------------------------------------------------------------------

/// Brand/interaction accent: active pane borders, key labels, links.
pub const ACCENT: Color = Color::Cyan;

/// Product title / banner heading.
pub const TITLE: Color = Color::Yellow;

/// Selected-row background. NOT yellow — must not collide with TITLE.
pub const SELECTION_BG: Color = Color::Blue;

/// Selected-row foreground — readable on SELECTION_BG.
pub const SELECTION_FG: Color = Color::White;

/// Positive outcome / healthy / installed.
pub const SUCCESS: Color = Color::Green;

/// Needs attention soon (non-fatal).
pub const WARNING: Color = Color::Yellow;

/// Error / not-installed / destructive.
pub const DANGER: Color = Color::Red;

/// De-emphasised text. Floors at Gray — DarkGray on a dark terminal is
/// near-invisible (design system principle).
pub const MUTED: Color = Color::Gray;

// ---------------------------------------------------------------------------
// Spacing tokens
//
// Terminal UIs take the *concept* of spacing rather than pixel values.
// These map to the "breathing room / Mattaññutā" intent of the design system.
// ---------------------------------------------------------------------------

/// Left/right inner padding inside bordered panes (columns).
pub const PANE_PAD_X: u16 = 2;

/// Top/bottom inner padding inside bordered panes (rows).
pub const PANE_PAD_Y: u16 = 1;

/// Blank-line gap that separates logical blocks inside a pane body.
pub const SECTION_GAP: u16 = 1;

/// Return a `Padding` value suitable for `Block::padding(...)` that applies
/// `PANE_PAD_X` left/right and `PANE_PAD_Y` top/bottom.
pub fn pane_padding() -> Padding {
    Padding::new(PANE_PAD_X, PANE_PAD_X, PANE_PAD_Y, PANE_PAD_Y)
}

// ---------------------------------------------------------------------------
// Status glyphs (mirrors bwoc_core::design::glyph where applicable)
// ---------------------------------------------------------------------------

/// Success / installed / passed.
pub const OK: &str = "✓";

/// Failure / not-installed / error.
pub const BAD: &str = "✗";

/// Informational annotation.
pub const INFO: &str = "ℹ";

/// Navigation / selection pointer.
pub const ARROW: &str = "▶";

// ---------------------------------------------------------------------------
// Convenience style helpers
// ---------------------------------------------------------------------------

/// Selected list row: blue bg, white fg, bold.
pub fn selection_style() -> Style {
    Style::default()
        .bg(SELECTION_BG)
        .fg(SELECTION_FG)
        .add_modifier(Modifier::BOLD)
}

/// Hotkey chip (key label badge): ACCENT bg, black fg, bold — matches
/// bwoc-tui's hotkey label convention.
pub fn key_chip_style() -> Style {
    Style::default()
        .bg(ACCENT)
        .fg(Color::Black)
        .add_modifier(Modifier::BOLD)
}

/// De-emphasised / muted text.
pub fn muted_style() -> Style {
    Style::default().fg(MUTED)
}

/// Title bar: ACCENT bg, black fg, bold.
pub fn title_bar_style() -> Style {
    Style::default()
        .fg(Color::Black)
        .bg(ACCENT)
        .add_modifier(Modifier::BOLD)
}

/// Positive / success annotation.
pub fn success_style() -> Style {
    Style::default().fg(SUCCESS).add_modifier(Modifier::BOLD)
}

/// Error / danger annotation.
pub fn danger_style() -> Style {
    Style::default().fg(DANGER).add_modifier(Modifier::BOLD)
}

/// Warning / needs-attention annotation.
pub fn warning_style() -> Style {
    Style::default().fg(WARNING).add_modifier(Modifier::BOLD)
}
