/// bwoc-setup — guided first-run installer wizard for the BWOC framework.
///
/// Terminal setup/teardown mirrors bwoc-tui exactly:
///   - enable_raw_mode + EnterAlternateScreen on entry
///   - panic hook restores the terminal before the default panic handler fires
///   - disable_raw_mode + LeaveAlternateScreen on every exit path
///
/// The draw loop polls crossterm key events on a 50ms timeout and redraws on
/// any state change.  All `bwoc` CLI invocations happen via `exec::bwoc` (no
/// interactive children, output always captured).
///
/// Hidden flag: `bwoc-setup --version` prints the package version and exits
/// without entering the TUI (used by CI smoke tests).

mod app;
mod catalog;
mod exec;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use app::{App, InputKind, Stage};

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() {
    // Hidden --version flag: print version and exit cleanly (no TUI).
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("bwoc-setup {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    // Install panic hook: restore terminal before the default handler prints
    // the panic message so the user's shell is left in a usable state.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore_terminal();
        default_hook(info);
    }));

    let exit_code = run();
    std::process::exit(exit_code);
}

// ---------------------------------------------------------------------------
// Run: setup → event loop → teardown
// ---------------------------------------------------------------------------

fn run() -> i32 {
    use std::io::IsTerminal;
    if !io::stdout().is_terminal() {
        eprintln!(
            "bwoc-setup: stdout is not a TTY. \
             Run in an interactive terminal or use --version for a smoke test."
        );
        return 2;
    }
    // stdin must be a TTY too, or the key-event reader can't read the keyboard.
    // This commonly happens under `curl … | bash`, where the wizard inherits
    // the curl pipe as stdin. The installer reattaches /dev/tty for us; if you
    // hit this, re-run the install command or launch `bwoc-setup` directly.
    if !io::stdin().is_terminal() {
        eprintln!(
            "bwoc-setup: stdin is not a TTY — keyboard input is unavailable.\n\
             If you launched via `curl … | bash`, just run `bwoc-setup` directly \
             in your terminal."
        );
        return 2;
    }

    let mut term = match setup_terminal() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("bwoc-setup: failed to enter alternate screen: {e}");
            return 1;
        }
    };

    let mut app = App::new();
    let result = event_loop(&mut term, &mut app);

    if let Err(e) = restore_terminal() {
        eprintln!("bwoc-setup: warning — failed to restore terminal: {e}");
    }

    match result {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("bwoc-setup: {e}");
            1
        }
    }
}

// ---------------------------------------------------------------------------
// Terminal helpers (matching bwoc-tui idiom)
// ---------------------------------------------------------------------------

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

fn restore_terminal() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// Event loop: poll → handle → redraw
// ---------------------------------------------------------------------------

fn event_loop(
    term: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        term.draw(|f| ui::draw(f, app))?;

        if app.quit {
            return Ok(());
        }

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if handle_key(app, key) {
                    return Ok(());
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Key dispatch
// ---------------------------------------------------------------------------

/// Returns `true` when the user has requested an immediate quit (Ctrl-C).
fn handle_key(app: &mut App, key: KeyEvent) -> bool {
    let KeyEvent {
        code, modifiers, ..
    } = key;

    // Ctrl-C always quits immediately.
    if let (KeyCode::Char('c'), KeyModifiers::CONTROL) = (code, modifiers) {
        return true;
    }

    match &app.stage {
        // BwocMissing has its own [ลองใหม่] / [ออก] menu.
        s if matches!(
            app.input,
            InputKind::BwocMissing { .. }
        ) && s == &Stage::CheckBwoc =>
        {
            return handle_bwoc_missing(app, code);
        }

        Stage::Done => match code {
            KeyCode::Enter | KeyCode::Char('q') | KeyCode::Esc => {
                app.quit = true;
            }
            _ => {}
        },

        _ => match code {
            KeyCode::Up => app.handle_up(),
            KeyCode::Down => app.handle_down(),

            KeyCode::Enter => {
                // Validate agent name before advancing.
                if app.stage == Stage::AgentName && app.agent_name_error().is_some() {
                    // Stay on the field — error hint is rendered by ui.
                    return false;
                }
                app.next();
            }

            KeyCode::Backspace => app.handle_backspace(),

            KeyCode::Esc | KeyCode::Left => app.back(),

            // 'r' retries action stages that failed.
            KeyCode::Char('r') => {
                if let InputKind::Action { ok: false, .. } = &app.input {
                    app.retry();
                }
            }

            // 'q' quits when not in a text input (same rule as bwoc-tui).
            KeyCode::Char('q') => {
                if !matches!(app.input, InputKind::Text { .. }) {
                    app.quit = true;
                } else {
                    app.handle_char('q');
                }
            }

            KeyCode::Char(c) => app.handle_char(c),

            _ => {}
        },
    }

    false
}

/// Handle keys in the BwocMissing screen.  Returns `true` to quit.
fn handle_bwoc_missing(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Up => app.handle_up(),
        KeyCode::Down => app.handle_down(),
        KeyCode::Enter => {
            if let InputKind::BwocMissing { cursor } = &app.input {
                match cursor {
                    0 => app.retry(), // ลองใหม่
                    _ => return true, // ออก
                }
            }
        }
        _ => {}
    }
    false
}
