//! `boomdrive enroll` (design Decision 4; record 0006; core.31, core.39):
//! the partition `<label>` of the configuration's control plane enrolled
//! once, with the one-time token read from standard input, never from an
//! argument or the environment, and never shown. It reads the
//! configuration as `run` reads it, its local layer over its managed one, reaches the implementation's
//! enrollment through `planes::enroll` alone (core.2), takes no lock, and
//! writes no configuration and nothing in the data directory (core.35).
//! With `--observer`, the partition `<label>` of `[observer]` is enrolled
//! the same way, through `planes::enroll_observer`, whatever
//! `[control_plane]`'s `type` (design Decision 4 of
//! a-runner-reports-to-an-observer-it-takes-no-work-from; record 0048).

use crate::config::{self, Env};
use crate::planes::{self, Enrolled, Refusal};
use serde_json::json;
use std::ffi::OsString;
use std::io::{BufRead, IsTerminal, Write};
use std::path::Path;
use std::process::ExitCode;

/// Enrolls `label`, of the observer where `observer`, answering in words
/// or, with `json`, as one JSON object, and exiting 0 enrolled, 1 refused
/// by the control plane or unreachable, and 2 for a host that must be
/// fixed (core.39).
pub fn main(file: Option<&Path>, json: bool, observer: bool, label: &str) -> ExitCode {
    let var = |name: &str| std::env::var_os(name);
    let answer = match observer {
        false => enroll(file, label, var, || read_token(label)),
        true => enroll_observer(file, label, var, || read_token(label)),
    };
    let named = match observer {
        false => label.to_string(),
        true => format!("observer {label}"),
    };
    match answer {
        Ok(enrolled) if json => {
            println!(
                "{}",
                json!({"enrolled": {"partition": label, "host": enrolled.host}})
            );
            ExitCode::SUCCESS
        }
        Ok(enrolled) => {
            println!("{named}: enrolled as host {}", enrolled.host);
            ExitCode::SUCCESS
        }
        Err(refusal) => {
            if json {
                println!(
                    "{}",
                    json!({"refused": {"refusal": {"reason": refusal.words}}})
                );
            } else {
                eprintln!("boomdrive: {}", refusal.words);
            }
            ExitCode::from(if refusal.host_to_fix { 2 } else { 1 })
        }
    }
}

/// Enrolls the partition `label` of the control plane the configuration
/// `file` names, else the default one, with the local layer over it
/// (design Decision 8), in the environment `var` gives,
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
    let local = config::local_path(&env);
    let config = config::read_layers(&file, &local, &env.home).map_err(host)?;
    let Some(table) = config.control_plane else {
        return Err(host(format!(
            "{}: names no `[control_plane]`, so no partition of one to enroll",
            file.display()
        )));
    };
    planes::enroll(
        &table.implementation,
        &table.implementation_table,
        &file,
        label,
        token,
    )
}

/// Enrolls the partition `label` of the observer the configuration `file`
/// names, else the default one, with the local layer over it, in the
/// environment `var` gives, with the token `token` gives, as [`enroll`]
/// enrolls a control plane's, whatever `[control_plane]`'s `type` (design
/// Decision 4 of a-runner-reports-to-an-observer-it-takes-no-work-from). A
/// configuration that cannot be read, or names no `[observer]`, is refused
/// as the host's to fix, before the token is asked for.
pub fn enroll_observer(
    file: Option<&Path>,
    label: &str,
    var: impl Fn(&str) -> Option<OsString>,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Enrolled, Refusal> {
    let env = Env::read(var).map_err(host)?;
    let file = config::config_path(file, &env);
    let local = config::local_path(&env);
    let config = config::read_layers(&file, &local, &env.home).map_err(host)?;
    let Some(table) = config.observer else {
        return Err(host(format!(
            "{}: names no `[observer]`, so no partition of an observer to enroll",
            file.display()
        )));
    };
    planes::enroll_observer(&table, &file, label, token)
}

/// The one-time token: asked for on standard error and read without echo
/// where standard input is a terminal, and otherwise standard input's
/// first line, its line ending left off.
fn read_token(label: &str) -> Result<String, String> {
    let unread = |e: std::io::Error| format!("the enrollment token could not be read: {e}");
    if std::io::stdin().is_terminal() {
        eprint!("{label}'s enrollment token: ");
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
