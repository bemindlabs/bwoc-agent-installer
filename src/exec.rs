/// exec.rs — shell-out helpers.
///
/// All bwoc invocations go through `bwoc()`.  The wizard never spawns an
/// interactive child that grabs the TTY — full flags are always passed so bwoc
/// runs non-interactively and output is captured.
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ExecResult {
    pub ok: bool,
    pub stdout: String,
    pub stderr: String,
}

impl ExecResult {
    /// Merged output useful for display (stdout first, then stderr if non-empty).
    pub fn combined(&self) -> String {
        let mut out = self.stdout.clone();
        if !self.stderr.is_empty() {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&self.stderr);
        }
        out
    }
}

/// Run `bwoc <args>` and capture the result.  Never interactive.
pub fn bwoc(args: &[&str]) -> ExecResult {
    match Command::new("bwoc").args(args).output() {
        Ok(output) => ExecResult {
            ok: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_string(),
        },
        Err(e) => ExecResult {
            ok: false,
            stdout: String::new(),
            stderr: format!("ไม่สามารถรัน bwoc ได้: {e}"),
        },
    }
}

/// Check whether a named binary exists on PATH using a pure-Rust PATH scan
/// (no subprocess — works cross-platform without shell).
pub fn binary_present(name: &str) -> bool {
    if name.is_empty() {
        // openai-compatible has no single binary; treat as "present" (N/A).
        return true;
    }

    let path_var = std::env::var("PATH").unwrap_or_default();
    let separator = if cfg!(windows) { ';' } else { ':' };

    for dir in path_var.split(separator) {
        let mut candidate = std::path::PathBuf::from(dir);
        candidate.push(name);

        // On Windows also probe <name>.exe / <name>.cmd.
        if cfg!(windows) {
            let mut exe = candidate.clone();
            exe.set_extension("exe");
            if exe.is_file() {
                return true;
            }
            let mut cmd = candidate.clone();
            cmd.set_extension("cmd");
            if cmd.is_file() {
                return true;
            }
        } else if candidate.is_file() {
            return true;
        }
    }
    false
}
