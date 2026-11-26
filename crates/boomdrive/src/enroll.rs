//! `boomdrive enroll` (the change a-host-enrolls-from-one-command; record
//! 0006; core.31, core.39): one whole command, which the control plane's
//! page gives, enrolls the partition of a drive at a control plane, the
//! drive's name its label and its account's, with the one-time token read
//! from standard input, never from an argument or the environment, and
//! never shown. It reads the configuration as `run` reads it, its local
//! layer over its managed one, and takes one of three ways (design
//! Decision 3): a file that names the enrollment already is left as it is;
//! a file that is missing, or the runner's own, is written on macOS once
//! the control plane enrolls the partition, and setup's agent loaded or
//! restarted; and any other file is changed in nothing, its keys to add
//! named. Every refusal of the host comes before the token is read (design
//! Decision 4). It reaches the implementation's enrollment through
//! [`planes::enrolling_entry`] alone (core.2), takes no lock, and writes
//! nothing in the data directory (core.35).

use crate::config::{self, Account, Address, Config, Drive, Env};
use crate::planes::{self, Enrolled, Enrolling, Entry, Refusal, Token};
use crate::setup;
use serde_json::{json, Value};
use std::io::{BufRead, IsTerminal, Write};
use std::path::Path;
use std::process::ExitCode;

/// What the command asks: the control plane's address, the drive, and
/// each repository the drive keeps.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Asked {
    pub control_plane: String,
    pub drive: String,
    pub repositories: Vec<String>,
}

/// The host enroll runs on: its system, as `std::env::consts::OS` names
/// it, its environment, and the managed file `--config` names, where it
/// names one.
pub struct Host<'a> {
    pub system: &'a str,
    pub env: &'a Env,
    pub config: Option<&'a Path>,
}

/// A partition enrolled: its label, the control plane's id for the host,
/// the runner's file and its outcome, and setup's agent and its outcome,
/// where enroll wrote the file (design Decision 6).
#[derive(Clone, Debug, PartialEq)]
pub struct Answer {
    pub partition: String,
    pub host: String,
    pub config: Value,
    pub service: Option<Value>,
}

impl Answer {
    /// The answer as `--json` prints it, which only gains fields
    /// (core.39).
    pub fn json(&self) -> Value {
        let mut enrolled = json!({
            "partition": self.partition,
            "host": self.host,
            "config": self.config,
        });
        if let Some(service) = &self.service {
            enrolled["service"] = service.clone();
        }
        json!({ "enrolled": enrolled })
    }
}

/// What the runner's managed file is (design Decision 3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    /// No file there.
    Missing,
    /// The runner's own, its first line setup's.
    Runners,
    /// A link, as Nix's activation makes it.
    Link,
    /// A file a person wrote.
    Persons,
}

/// Enrolls what `asked` names, answering in words or, with `json`, as one
/// JSON object, and exiting 0 enrolled, 1 refused by the control plane or
/// unreachable, and 2 for a host that must be fixed (core.39).
pub fn main(file: Option<&Path>, json: bool, asked: &Asked) -> ExitCode {
    let answer =
        Env::read(|name| std::env::var_os(name))
            .map_err(host)
            .and_then(|env| {
                let host_of = Host {
                    system: std::env::consts::OS,
                    env: &env,
                    config: file,
                };
                let launchctl = setup::finder(std::env::var_os("PATH"))("launchctl");
                enroll(
                    asked,
                    &host_of,
                    |table, file, label, token| {
                        let (_, enrolling) = planes::enrolling_entry().ok_or_else(|| {
                            host("this binary carries no control plane that enrolls".to_string())
                        })?;
                        (enrolling.enroll)(table, file, label, token)
                    },
                    |home, managed, restart| {
                        let launchctl = launchctl.clone().ok_or_else(|| {
                        host("`launchctl` (macOS's own, `/bin/launchctl`) is not on enroll's PATH"
                            .to_string())
                    })?;
                        setup::load_agent(home, managed, &launchctl, restart)
                    },
                    || read_token(&asked.drive),
                )
            });
    match answer {
        Ok(answer) if json => {
            println!("{}", answer.json());
            ExitCode::SUCCESS
        }
        Ok(answer) => {
            println!("{}: enrolled as host {}", answer.partition, answer.host);
            println!(
                "{}: {}",
                answer.config["file"].as_str().unwrap_or_default(),
                answer.config["outcome"].as_str().unwrap_or_default()
            );
            if let Some(service) = &answer.service {
                println!(
                    "{}: {}",
                    service["file"].as_str().unwrap_or_default(),
                    service["outcome"].as_str().unwrap_or_default()
                );
            }
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

/// Enrolls what `asked` names on `host_of`: `enroller` enrolls the
/// partition from the implementation's own table, with the token `token`
/// gives, and `agent` loads setup's agent for the home and the file
/// written, restarting it where asked. Every refusal of the host comes
/// before `token` is asked (design Decisions 3 and 4).
pub fn enroll<E, A>(
    asked: &Asked,
    host_of: &Host<'_>,
    enroller: E,
    agent: A,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Answer, Refusal>
where
    E: FnOnce(&toml::Table, &Path, &str, Token<'_>) -> Result<Enrolled, Refusal>,
    A: FnOnce(&Path, &Path, bool) -> Result<Value, Refusal>,
{
    let (entry, enrolling) = planes::enrolling_entry()
        .ok_or_else(|| host("this binary carries no control plane that enrolls".to_string()))?;
    let label = asked.drive.as_str();
    if !config::in_form(label) {
        return Err(host(format!(
            "`--drive {label}`: a drive's {}",
            config::NAME_FORM
        )));
    }
    for repository in &asked.repositories {
        Address::parse(repository)
            .map_err(|form| host(format!("`--repository {repository}`: {form}")))?;
    }
    let env = host_of.env;
    let managed = config::config_path(host_of.config, env);
    let local = config::local_path(env);
    // The enrollment's own table, checked before anything is read of the
    // host: its address, its label and its repositories.
    (enrolling.with_partition)(
        None,
        &asked.control_plane,
        label,
        &asked.repositories,
        &managed,
    )
    .map_err(host)?;

    let kind = kind_of(&managed);
    let layers = match kind {
        Kind::Missing => None,
        _ => Some(config::read_layers(&managed, &local, &env.home).map_err(host)?),
    };

    // Way 1: the file names the enrollment already.
    if let Some(layers) = &layers {
        if let Some(table) = names_it(layers, entry, enrolling, asked, &managed) {
            let enrolled = enroller(&table, &managed, label, Box::new(token))?;
            return Ok(Answer {
                partition: label.to_string(),
                host: enrolled.host,
                config: json!({"file": managed.display().to_string(), "outcome": "unchanged"}),
                service: None,
            });
        }
    }

    // Way 3: a file enroll does not write, or a host off macOS.
    let held = layers
        .as_ref()
        .and_then(|layers| layers.control_plane.as_ref())
        .and_then(|plane| plane.table(entry.word));
    if !matches!(kind, Kind::Missing | Kind::Runners) || host_of.system != "macos" {
        let table = (enrolling.with_partition)(
            held,
            &asked.control_plane,
            label,
            &asked.repositories,
            &managed,
        )
        .map_err(host)?;
        return Err(host(to_add(
            kind,
            host_of.system,
            &managed,
            entry,
            asked,
            &table,
        )));
    }

    // Way 2: the runner's own file, written once the partition is
    // enrolled.
    let own = match kind {
        Kind::Runners => Some(config::read(&managed, &env.home).map_err(host)?),
        _ => None,
    };
    let held = own
        .as_ref()
        .and_then(|own| own.control_plane.as_ref())
        .and_then(|plane| plane.table(entry.word));
    let table = (enrolling.with_partition)(
        held,
        &asked.control_plane,
        label,
        &asked.repositories,
        &managed,
    )
    .map_err(host)?;
    let others: Vec<(String, toml::Table)> = own
        .as_ref()
        .and_then(|own| own.control_plane.as_ref())
        .map(|plane| {
            plane
                .implementations
                .iter()
                .filter(|(word, _)| word != entry.word)
                .cloned()
                .collect()
        })
        .unwrap_or_default();
    let accounts = with_drive(
        own.map(|own| own.accounts).unwrap_or_default(),
        asked,
        entry.word,
    )?;
    let text = setup::runner_file(&accounts, entry.word, &table, &others);
    config::parse(&text, &managed, &env.home).map_err(host)?;

    let enrolled = enroller(&table, &managed, label, Box::new(token))?;
    let written = setup::write_runner_file(&managed, &text).map_err(|refusal| {
        host(format!(
            "the control plane enrolled `{label}` as host {}, but the runner's file could not \
             be written: {}",
            enrolled.host, refusal.words
        ))
    })?;
    let outcome = written["outcome"].as_str().unwrap_or_default().to_string();
    let service = agent(&env.home, &managed, outcome == "written")?;
    Ok(Answer {
        partition: label.to_string(),
        host: enrolled.host,
        config: json!({"file": written["file"], "outcome": outcome}),
        service: Some(service),
    })
}

/// What the runner's managed file at `managed` is.
fn kind_of(managed: &Path) -> Kind {
    match managed.symlink_metadata() {
        Err(_) => Kind::Missing,
        Ok(meta) if meta.file_type().is_symlink() => Kind::Link,
        Ok(_) if setup::is_setups(managed) => Kind::Runners,
        Ok(_) => Kind::Persons,
    }
}

/// The implementation's own table of `layers` where both layers name the
/// enrollment `asked` asks: the table names it, and the drive, under an
/// account the implementation serves, keeps each repository.
fn names_it(
    layers: &Config,
    entry: &Entry,
    enrolling: &Enrolling,
    asked: &Asked,
    managed: &Path,
) -> Option<toml::Table> {
    let table = layers.control_plane.as_ref()?.table(entry.word)?;
    if !(enrolling.names)(
        table,
        managed,
        &asked.control_plane,
        &asked.drive,
        &asked.repositories,
    ) {
        return None;
    }
    let kept = layers
        .accounts
        .iter()
        .filter(|account| account.control_plane.as_deref() == Some(entry.word))
        .flat_map(|account| &account.drives)
        .find(|drive| drive.name == asked.drive)?;
    asked
        .repositories
        .iter()
        .all(|wanted| {
            Address::parse(wanted).is_ok_and(|wanted| kept.repositories.contains(&wanted))
        })
        .then(|| table.clone())
}

/// `accounts` with the account and the drive of `asked`'s name, served by
/// the implementation `word`, keeping `asked`'s repositories; refused
/// where an account of that name is served by another control plane.
fn with_drive(
    mut accounts: Vec<Account>,
    asked: &Asked,
    word: &str,
) -> Result<Vec<Account>, Refusal> {
    let drive = Drive {
        name: asked.drive.clone(),
        repositories: asked
            .repositories
            .iter()
            .filter_map(|repository| Address::parse(repository).ok())
            .collect(),
    };
    match accounts
        .iter_mut()
        .find(|account| account.name == asked.drive)
    {
        Some(account)
            if account
                .control_plane
                .as_deref()
                .is_some_and(|plane| plane != word) =>
        {
            Err(host(format!(
                "the account `{}` is served by `{}`, and an enrollment names its account after \
                 the drive: give the drive a name no account of this host has",
                account.name,
                account.control_plane.as_deref().unwrap_or_default()
            )))
        }
        Some(account) => {
            account.control_plane = Some(word.to_string());
            match account
                .drives
                .iter_mut()
                .find(|held| held.name == drive.name)
            {
                Some(held) => *held = drive,
                None => account.drives.push(drive),
            }
            Ok(accounts)
        }
        None => {
            accounts.push(Account {
                name: asked.drive.clone(),
                control_plane: Some(word.to_string()),
                drives: vec![drive],
            });
            Ok(accounts)
        }
    }
}

/// The words of way 3: the file `managed`, of `kind`, on `system`, is not
/// enroll's to write, so enroll names the keys to add, the implementation
/// `entry`'s table `table` among them, as the module's options where Nix
/// writes the file or the host is not macOS, and as TOML where a person
/// wrote it.
fn to_add(
    kind: Kind,
    system: &str,
    managed: &Path,
    entry: &Entry,
    asked: &Asked,
    table: &toml::Table,
) -> String {
    let file = managed.display();
    let label = &asked.drive;
    let mut by_owner = toml::Table::new();
    for address in asked
        .repositories
        .iter()
        .filter_map(|repository| Address::parse(repository).ok())
    {
        let names = by_owner
            .entry(address.owner().to_string())
            .or_insert_with(|| toml::Value::Array(Vec::new()));
        if let toml::Value::Array(names) = names {
            names.push(toml::Value::String(address.name().to_string()));
        }
    }
    let after = "then run the same command again: it enrolls this host, and the token is not \
                 spent";
    let nix_lines = |by_owner: toml::Table| {
        let mut drive = toml::Table::new();
        drive.insert("repositories".to_string(), toml::Value::Table(by_owner));
        let mut drives = toml::Table::new();
        drives.insert(label.clone(), toml::Value::Table(drive));
        let mut account = toml::Table::new();
        account.insert(
            "control_plane".to_string(),
            toml::Value::String(entry.word.to_string()),
        );
        account.insert("drives".to_string(), toml::Value::Table(drives));
        format!(
            "  services.boomdrive.accounts.{label} = {};\n  \
             services.boomdrive.settings.control_plane.{} = {};",
            nix(&toml::Value::Table(account)),
            entry.word,
            nix(&toml::Value::Table(table.clone()))
        )
    };
    match kind {
        Kind::Link => format!(
            "{file} is a link, as Nix's module makes it, so enroll changes nothing and reads no \
             token: set\n{}\nin this host's configuration and switch, {after}",
            nix_lines(by_owner)
        ),
        Kind::Persons => {
            let mut account = toml::Table::new();
            account.insert(
                "control_plane".to_string(),
                toml::Value::String(entry.word.to_string()),
            );
            let mut keeps = toml::Table::new();
            keeps.insert(
                "repositories".to_string(),
                toml::Value::Array(
                    asked
                        .repositories
                        .iter()
                        .cloned()
                        .map(toml::Value::String)
                        .collect(),
                ),
            );
            let mut drives = toml::Table::new();
            drives.insert(label.clone(), toml::Value::Table(keeps));
            account.insert("drives".to_string(), toml::Value::Table(drives));
            let mut accounts = toml::Table::new();
            accounts.insert(label.clone(), toml::Value::Table(account));
            let mut plane = toml::Table::new();
            plane.insert(entry.word.to_string(), toml::Value::Table(table.clone()));
            let mut settings = toml::Table::new();
            settings.insert("accounts".to_string(), toml::Value::Table(accounts));
            settings.insert("control_plane".to_string(), toml::Value::Table(plane));
            format!(
                "{file} is a file a person wrote, so enroll changes nothing and reads no token: \
                 set these keys in it\n{}\n{after}",
                toml::to_string(&settings).unwrap_or_default().trim_end()
            )
        }
        Kind::Missing | Kind::Runners => format!(
            "enroll writes the runner's file on macOS, and this host is {system}: a host of \
             {system} sets up its runner by the flake's module, `services.boomdrive`; set\n{}\n\
             in this host's configuration and switch, {after}",
            nix_lines(by_owner)
        ),
    }
}

/// `value` as a Nix expression: a string quoted, a list and an attribute
/// set on one line.
fn nix(value: &toml::Value) -> String {
    match value {
        toml::Value::String(text) => {
            format!(
                "\"{}\"",
                text.replace('\\', "\\\\")
                    .replace('"', "\\\"")
                    .replace("${", "\\${")
            )
        }
        toml::Value::Array(items) => {
            let items: Vec<String> = items.iter().map(nix).collect();
            format!("[ {} ]", items.join(" "))
        }
        toml::Value::Table(table) => {
            let pairs: Vec<String> = table
                .iter()
                .map(|(key, value)| format!("{} = {};", nix_key(key), nix(value)))
                .collect();
            format!("{{ {} }}", pairs.join(" "))
        }
        other => other.to_string(),
    }
}

/// `key` as a Nix attribute's name, quoted where it is no identifier.
fn nix_key(key: &str) -> String {
    let plain = key.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '\'');
    if plain {
        key.to_string()
    } else {
        nix(&toml::Value::String(key.to_string()))
    }
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

#[cfg(test)]
#[path = "enroll_tests.rs"]
mod tests;
