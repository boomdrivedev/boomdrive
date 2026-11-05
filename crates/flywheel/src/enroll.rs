//! `flywheel enroll` (design Decision 4; record 0006; core.31, core.39):
//! the partition `<label>` of the configuration's control plane enrolled
//! once, with the one-time token read from standard input, never from an
//! argument or the environment, and never shown. It reads the
//! configuration as `run` reads it, reaches the implementation's
//! enrolment through `planes::enrol` alone (core.2), takes no lock, and
//! writes no configuration and nothing in the data directory (core.35).

use crate::config::{self, Env};
use crate::planes::{self, Enrolled, Refusal};
use serde_json::json;
use std::ffi::OsString;
use std::io::{BufRead, IsTerminal, Write};
use std::path::Path;
use std::process::ExitCode;

/// Enrols `label`, answering in words or, with `json`, as one JSON
/// object, and exiting 0 enrolled, 1 refused by the control plane or
/// unreachable, and 2 for a host that must be fixed (core.39).
pub fn main(file: Option<&Path>, json: bool, label: &str) -> ExitCode {
    let answer = enroll(
        file,
        label,
        |name| std::env::var_os(name),
        || read_token(label),
    );
    match answer {
        Ok(enrolled) if json => {
            println!(
                "{}",
                json!({"enrolled": {"partition": label, "host": enrolled.host}})
            );
            ExitCode::SUCCESS
        }
        Ok(enrolled) => {
            println!("{label}: enrolled as host {}", enrolled.host);
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            if json {
                println!(
                    "{}",
                    json!({"refused": {"refusal": {"reason": refusal.words}}})
                );
            } else {
                eprintln!("flywheel: {}", refusal.words);
            }
            ExitCode::from(if refusal.host_to_fix { 2 } else { 1 })
        }
    }
}

/// Enrols the partition `label` of the control plane the configuration
/// `file` names, else the default one, in the environment `var` gives,
/// with the token `token` gives, or words saying why it could not be
/// read. A configuration that cannot be read, or names no control plane,
/// is refused as the host's to fix, before the token is asked for.
pub fn enroll(
    file: Option<&Path>,
    label: &str,
    var: impl Fn(&str) -> Option<OsString>,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Enrolled, Refusal> {
    let env = Env::read(var).map_err(host)?;
    let file = config::config_path(file, &env);
    let config = config::read(&file, &env.home).map_err(host)?;
    let Some(table) = config.control_plane else {
        return Err(host(format!(
            "{}: names no `[control_plane]`, so no partition of one to enrol",
            file.display()
        )));
    };
    planes::enrol(
        &table.implementation,
        &table.implementation_table,
        &file,
        label,
        token,
    )
}

/// The one-time token: asked for on standard error and read without echo
/// where standard input is a terminal, and otherwise standard input's
/// first line, its line ending left off.
fn read_token(label: &str) -> Result<String, String> {
    let unread = |e: std::io::Error| format!("the enrolment token could not be read: {e}");
    if std::io::stdin().is_terminal() {
        eprint!("{label}'s enrolment token: ");
        std::io::stderr().flush().map_err(unread)?;
        return rpassword::read_password().map_err(unread);
    }
    let mut line = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut line)
        .map_err(unread)?;
    let token = line.strip_suffix('\n').unwrap_or(&line);
    Ok(token.strip_suffix('\r').unwrap_or(token).to_string())
}

/// A refusal the host must fix.
fn host(words: String) -> Refusal {
    Refusal {
        words,
        host_to_fix: true,
    }
}
