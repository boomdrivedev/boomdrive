//! `boomdrive forge-token get|erase` (design Decision 4; record 0007 as
//! amended on 2026-11-09 and 2026-11-11; core.33): a served repository's
//! forge token, in git's credential-helper protocol, for every tool on the
//! host that asks, git and sluice among them.
//!
//! It reads the configuration as `run` reads it, its local layer over its
//! managed one. On a host whose control plane holds the App every
//! operation is refused with the implementation's own words, until a
//! version of the protocol carries a token (core.2). `get` reads git's
//! `protocol`, `host` and `path`, and for `https`, `github.com` and a
//! repository of a drive the host serves mints a token in its own process,
//! gives it on standard output alone, and keeps it no longer: its asker
//! keeps what it was given. `erase` reads what it is sent and answers
//! nothing; since no process keeps a token it gave, it drops nothing and
//! revokes nothing at GitHub, and the next `get` mints afresh, never giving
//! the erased token. Nothing is written to the data directory, a file or a
//! log (core.4, core.35).
//!
//! It takes no `--json`, departing from core.39 for this one command (its
//! details say so): git reads a helper's standard output as the protocol,
//! and a refusal written there in JSON would be read as a malformed
//! credential. A refusal is one line on standard error,
//! `boomdrive forge-token: <words>`, and nothing on standard output, and
//! the command exits as core.39 has it: 0 answered, 1 refused, 2 for a
//! usage error or a host that must be fixed.

use crate::config::{self, Address, Config, Env};
use crate::forge::{Forge, Minted, Refused};
use crate::planes;
use std::ffi::OsString;
use std::io::{BufRead, Write};
use std::path::Path;
use std::process::ExitCode;

/// What git or sluice asks of the helper.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum Operation {
    /// Give a served repository's token.
    Get,
    /// Take back a token given: drops nothing and revokes nothing.
    Erase,
}

/// Runs `boomdrive forge-token <operation>` with the configuration `file`,
/// else the default one, with the local layer over it, on standard input
/// and standard output, minting from GitHub's API. A configuration that
/// cannot be read is a host to fix.
pub fn main(file: Option<&Path>, operation: Operation) -> ExitCode {
    let answered = read(file, |name| std::env::var_os(name)).and_then(|config| {
        answer(
            &config,
            operation,
            &mut std::io::stdin().lock(),
            &mut std::io::stdout().lock(),
            &Forge::new(),
        )
    });
    match answered {
        Ok(()) => ExitCode::SUCCESS,
        Err(refused) => {
            eprintln!("boomdrive forge-token: {}", refused.words);
            ExitCode::from(if refused.host_to_fix { 2 } else { 1 })
        }
    }
}

/// The configuration `file`, else the default one, with the local layer
/// over it, in the environment `var` gives, as `run` reads it; a host to
/// fix where it cannot be read.
fn read(file: Option<&Path>, var: impl Fn(&str) -> Option<OsString>) -> Result<Config, Refused> {
    let env = Env::read(var).map_err(host)?;
    let file = config::config_path(file, &env);
    let local = config::local_path(&env);
    config::read_layers(&file, &local, &env.home).map_err(host)
}

/// Answers `operation` under the configuration `config`, git's request read
/// from `input` and the answer written to `output`, minting with `forge`
/// (design Decision 4, steps 2 to 6): or why it is refused.
pub fn answer(
    config: &Config,
    operation: Operation,
    input: &mut dyn BufRead,
    output: &mut dyn Write,
    forge: &Forge,
) -> Result<(), Refused> {
    let holds_the_app = config
        .control_plane
        .as_ref()
        .and_then(|plane| planes::carried(&plane.implementation))
        .and_then(|entry| entry.holds_the_app);
    if let Some(words) = holds_the_app {
        return Err(refused(words.to_string()));
    }
    let Some(app) = &config.forge else {
        return Err(host(
            "the configuration names no [forge], the Boomdrive App's app_id and key_file"
                .to_string(),
        ));
    };
    let request = Request::read(input)?;
    if operation == Operation::Erase {
        return Ok(());
    }

    let protocol = request.protocol.unwrap_or_default();
    let forge_host = request.host.unwrap_or_default();
    if protocol != "https" || !forge_host.eq_ignore_ascii_case("github.com") {
        return Err(refused(format!(
            "git asked for {protocol}://{forge_host}, and the runner gives tokens for \
             https://github.com alone"
        )));
    }
    let Some(path) = request.path else {
        return Err(refused(
            "git named no repository: set credential.useHttpPath so git sends its path".to_string(),
        ));
    };
    let address =
        repository(&path).ok_or_else(|| refused(format!("{path} is no repository's address")))?;
    if !config.repositories().any(|kept| *kept == address) {
        return Err(refused(format!(
            "{address} is a repository of no drive this host serves"
        )));
    }
    let token = match forge.mint(app, &address)? {
        Minted::Token(token) => token,
        Minted::NotInstalled => {
            return Err(refused(format!(
                "GitHub finds no installation of app {} on {address}: install the app on it",
                app.app_id
            )))
        }
    };
    write!(
        output,
        "username=x-access-token\npassword={}\npassword_expiry_utc={}\n",
        token.secret(),
        token.ends().unix_timestamp()
    )
    .and_then(|()| output.flush())
    .map_err(|error| refused(format!("the answer could not be written: {error}")))
}

/// What git sends a helper that the runner reads: every other key,
/// `capability[]` and `wwwauth[]` among them, is passed over, as git passes
/// it over.
#[derive(Default)]
struct Request {
    protocol: Option<String>,
    host: Option<String>,
    path: Option<String>,
}

impl Request {
    /// The request in `input`, `key=value` lines to a blank line or its
    /// end, as git writes it.
    fn read(input: &mut dyn BufRead) -> Result<Request, Refused> {
        let mut request = Request::default();
        for line in input.lines() {
            let line =
                line.map_err(|error| refused(format!("what git sent could not be read: {error}")))?;
            let line = line.strip_suffix('\r').unwrap_or(&line);
            if line.is_empty() {
                break;
            }
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let kept = match key {
                "protocol" => &mut request.protocol,
                "host" => &mut request.host,
                "path" => &mut request.path,
                _ => continue,
            };
            *kept = Some(value.to_string());
        }
        Ok(request)
    }
}

/// The repository git's `path` names, `owner/name` with a trailing `/` or
/// `.git` taken off; none where it names none.
fn repository(path: &str) -> Option<Address> {
    let path = path.trim_end_matches('/');
    let path = match path.len().checked_sub(4) {
        Some(at) if path.is_char_boundary(at) && path[at..].eq_ignore_ascii_case(".git") => {
            &path[..at]
        }
        _ => path,
    };
    Address::parse(path).ok()
}

/// A refusal the host must fix.
fn host(words: String) -> Refused {
    Refused {
        words,
        host_to_fix: true,
    }
}

/// A refusal of what was asked.
fn refused(words: String) -> Refused {
    Refused {
        words,
        host_to_fix: false,
    }
}

#[cfg(test)]
mod tests {
    use super::repository;

    #[test]
    fn a_path_is_read_as_an_address() {
        for path in [
            "agentplot/flume",
            "agentplot/flume.git",
            "agentplot/flume/",
            "agentplot/flume.GIT/",
        ] {
            assert_eq!(
                repository(path).map(|address| address.to_string()),
                Some("agentplot/flume".to_string()),
                "{path}"
            );
        }
        for path in [
            "",
            "agentplot",
            "agentplot/flume/extra",
            "/agentplot/flume",
            ".git",
        ] {
            assert_eq!(repository(path), None, "{path}");
        }
    }
}
