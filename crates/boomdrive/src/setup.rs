//! `boomdrive setup` (record 0009 as amended 2026-11-09 and 2026-11-11;
//! core.16; the design of the change a-persons-drive-is-set-up-by-one-command):
//! one drive of one account set up on a Mac Nix does not manage, by one
//! command an agent can run, each question a flag, which changes nothing
//! run again with the same answers. Its answers come from its flags, then
//! the host as setup left it, then a terminal, then setup's own (design
//! Decisions 1 to 3). It refuses a host Nix manages before anything runs
//! (Decision 7), and needs GitHub's sign-in and names the others (Decision
//! 10). It sets up the drive through the implementation that sets one up,
//! reached through `planes` alone (core.2; Decision 11), then writes the
//! runner's managed layer by the keys Nix's module writes (Decision 6),
//! links the set's herdr plugins from the person's profile with herdr's
//! own command (Decision 8; core.29), and loads the runner as a launchd
//! agent of the login, as nix-darwin's (Decision 9). It answers in one
//! JSON object with `--json`, and exits 0 answered, 1 refused by a tool or
//! on a host Nix manages, and 2 for a usage error or a host to fix
//! (core.39).

use crate::clone;
use crate::config::{self, Account, Address, Drive, Env};
use crate::planes::{self, Asked, DriveSetUp, Find, Kept, Refusal};
use serde_json::{json, Value};
use std::ffi::OsString;
use std::io::{BufRead, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output, Stdio};

/// The first line of the runner's file setup writes, by which setup knows
/// its own file (design Decision 6).
pub const FIRST_LINE: &str = "# Written by boomdrive setup; a change goes through boomdrive setup.";

/// The agent's label, under the project's own public address (design
/// Decision 9).
pub const LABEL: &str = "io.github.agentplot.boomdrive";

/// Where a person's own runner file is moved, beside it, once.
pub const ASIDE: &str = "config.toml.before-setup";

/// Where the set gives its herdr plugins in one package.
const PLUGINS_FROM: &str = "github:agentplot/boomdrive#herdr-plugins";

/// What setup's flags gave, each as text: none or empty where a flag was
/// not given.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Flags {
    pub account: Option<String>,
    pub drive: Option<String>,
    pub state_repository: Option<String>,
    /// Each `--repository`, the drive's whole list.
    pub repositories: Vec<String>,
    /// Each `--add-repository`.
    pub add_repositories: Vec<String>,
    /// Each `--remove-repository`.
    pub remove_repositories: Vec<String>,
    /// Each `--blueprints`, passed to the implementation as given.
    pub blueprints: Vec<String>,
    /// Each `--runs <role> <how>`, passed to the implementation as given.
    pub runs: Vec<(String, String)>,
    /// `--review <after_changes> <after_hours>`, passed to the
    /// implementation as given.
    pub review: Option<(String, String)>,
}

/// The answers setup gathers itself: the account, the drive, the state
/// repository where its flag or a terminal gave it, and the drive's
/// repositories, owner by owner in the order of the owners' names, as
/// Nix's module orders them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answers {
    pub account: String,
    pub drive: String,
    /// None where the host holds it for the drive.
    pub state_repository: Option<String>,
    pub repositories: Vec<Address>,
}

/// A terminal's question, asked on standard error, and its answer read
/// from standard input, trimmed: empty where none was given.
pub type Ask<'a> = &'a mut dyn FnMut(&str) -> Result<String, String>;

/// What setup knows of its host before it runs anything: the system, the
/// home, the runner's managed file and the finder of programs on its
/// `PATH`.
pub struct Host<'a> {
    pub system: &'a str,
    pub home: &'a Path,
    pub managed: &'a Path,
    pub find: Find<'a>,
}

/// A sign-in, a plugin or a link missing, with how the person makes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Missing {
    pub what: String,
    pub fix: String,
}

impl Missing {
    fn json(&self) -> Value {
        json!({"what": self.what, "fix": self.fix})
    }
}

/// Sets up a drive, answering in words or, with `json`, as one JSON
/// object, and exiting 0 answered, 1 refused by a tool or on a host Nix
/// manages, and 2 for a usage error or a host to fix (core.39).
pub fn main(flags: &Flags, json: bool) -> ExitCode {
    let mut say = |line: &str| {
        if !json {
            println!("{line}");
        }
    };
    match set_up(flags, &mut say) {
        Ok(answer) => {
            if json {
                println!("{}", json!({ "set_up": answer }));
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

/// Sets up the drive the flags, the host and a terminal give, saying a
/// line for each step to `say`, ending with the command that starts the
/// implementation's agents; gives the answer, or why it was refused.
fn set_up(flags: &Flags, say: &mut dyn FnMut(&str)) -> Result<Value, Refusal> {
    let env = Env::read(|name| std::env::var_os(name)).map_err(host)?;
    let home = env.home.clone();
    let managed = config::config_path(None, &env);
    let local = config::local_path(&env);
    let data = config::data_dir(&env);
    let found = finder(std::env::var_os("PATH"));
    let find: Find<'_> = &found;

    // The host's checks, before any program runs (design Decision 7).
    check_host(&Host {
        system: std::env::consts::OS,
        home: &home,
        managed: &managed,
        find,
    })?;
    let own = own_file(&managed, &home)?;

    let Some((entry, setup)) = planes::set_up_entry() else {
        return Err(host(
            "this binary carries no control plane that sets up a drive".to_string(),
        ));
    };
    let table = own.as_ref().and_then(|own| {
        own.control_plane
            .as_ref()
            .filter(|plane| plane.implementation == entry.word)
            .map(|plane| plane.implementation_table.clone())
    });
    let accounts = own.map(|own| own.accounts).unwrap_or_default();

    // The answers: the flags, the host, then a terminal (design Decisions 1
    // and 2).
    let held_state = |drive: &str| {
        (setup.held)(table.as_ref(), &home, drive)
            .state_repository()
            .map(str::to_string)
    };
    let mut terminal = |question: &str| ask_terminal(question);
    let ask: Option<Ask<'_>> = if std::io::stdin().is_terminal() {
        Some(&mut terminal)
    } else {
        None
    };
    let answers = answers(flags, &accounts, &held_state, ask)?;
    let repositories: Vec<String> = answers
        .repositories
        .iter()
        .map(ToString::to_string)
        .collect();

    // The implementation's answers settled and its programs found, then
    // GitHub's sign-in, before anything is made or written (design
    // Decisions 10 and 11).
    let held = (setup.held)(table.as_ref(), &home, &answers.drive);
    let asked = Asked {
        state_repository: answers.state_repository.clone(),
        blueprints: flags.blueprints.clone(),
        runs: flags.runs.clone(),
        review: flags.review.clone(),
    };
    let settled = (setup.settle)(&asked, &held, &repositories)?;
    let prepared = (setup.prepare)(find)?;
    let gh = find("gh").ok_or_else(|| host("`gh` is not on setup's PATH".to_string()))?;
    let signed_in = run(&gh, &["auth", "status", "--hostname", "github.com"], &[]).map_err(host)?;
    if !signed_in.status.success() {
        return Err(host(format!(
            "GitHub's sign-in is missing, which setup needs to make the state repository: \
             sign in with `gh auth login` ({})",
            words_of(&signed_in)
        )));
    }
    let mut missing = sign_ins(find);

    // The drive set up by its implementation.
    let root = config::parse("", &managed, &home)
        .map_err(host)?
        .repository_root;
    let kept: Vec<Kept> = answers
        .repositories
        .iter()
        .map(|address| Kept {
            repository: address.to_string(),
            dir: clone::container(&data, address),
            checkout: root.join(address.owner()).join(address.name()),
        })
        .collect();
    let done: DriveSetUp = (setup.set_up)(
        &prepared,
        &settled,
        &planes::Drive {
            drive: &answers.drive,
            account: &answers.account,
            folder: &root,
            kept: &kept,
            table: table.as_ref(),
        },
    )?;
    say(&format!(
        "drive {} of {}: {}",
        answers.drive,
        answers.account,
        repositories.join(", ")
    ));
    for line in &done.lines {
        say(line);
    }

    // The runner's file (design Decision 6).
    let accounts = with_drive(accounts, &answers);
    let text = runner_file(&accounts, entry.word, &done.table);
    let config = write_runner_file(&managed, &text)?;
    config::read_layers(&managed, &local, &home).map_err(host)?;
    if let Some(aside) = config["aside"].as_str() {
        say(&format!(
            "{}: a person's own file, moved to {aside}",
            managed.display()
        ));
    }
    say(&format!(
        "{}: {}",
        managed.display(),
        config["outcome"].as_str().unwrap_or_default()
    ));

    // herdr's plugins (design Decision 8).
    let herdr = find("herdr").ok_or_else(|| host("`herdr` is not on setup's PATH".to_string()))?;
    let (plugins, plugins_missing) = plugins(&home, &herdr);
    for plugin in &plugins {
        say(&format!(
            "herdr plugin {}: {}",
            plugin["plugin"].as_str().unwrap_or_default(),
            plugin["outcome"].as_str().unwrap_or_default()
        ));
    }
    missing.extend(plugins_missing);

    // The agent (design Decision 9).
    let launchctl =
        find("launchctl").ok_or_else(|| host("`launchctl` is not on setup's PATH".to_string()))?;
    let service = load_agent(&home, &managed, &launchctl)?;
    say(&format!(
        "{}: {}",
        service["file"].as_str().unwrap_or_default(),
        service["outcome"].as_str().unwrap_or_default()
    ));

    for gap in &missing {
        say(&format!("missing: {}; {}", gap.what, gap.fix));
    }
    say(&format!("start: {}", done.start));
    Ok(json!({
        "account": answers.account,
        "drive": answers.drive,
        "repositories": repositories,
        "control_plane": done.answer,
        "config": config,
        "service": service,
        "plugins": plugins,
        "missing": missing.iter().map(Missing::json).collect::<Vec<_>>(),
    }))
}

/// A refusal the host or the answers must fix.
fn host(words: String) -> Refusal {
    Refusal {
        words,
        host_to_fix: true,
    }
}

/// A tool's refusal, or a host Nix manages.
fn refused(words: String) -> Refusal {
    Refusal {
        words,
        host_to_fix: false,
    }
}

/// The finder of programs on `path`, a `PATH`: the first executable file
/// of the name in its folders, in order.
pub fn finder(path: Option<OsString>) -> impl Fn(&str) -> Option<PathBuf> {
    let folders: Vec<PathBuf> = path
        .map(|path| std::env::split_paths(&path).collect())
        .unwrap_or_default();
    move |name| {
        folders
            .iter()
            .map(|folder| folder.join(name))
            .find(|path| is_executable(path))
    }
}

/// Whether `path` is a file anyone may run.
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(path)
        .is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

/// Asks `question` on standard error and reads its answer from standard
/// input, trimmed.
fn ask_terminal(question: &str) -> Result<String, String> {
    let unread = |e: std::io::Error| format!("an answer could not be read: {e}");
    eprint!("{question}: ");
    std::io::stderr().flush().map_err(unread)?;
    let mut line = String::new();
    std::io::stdin()
        .lock()
        .read_line(&mut line)
        .map_err(unread)?;
    Ok(line.trim().to_string())
}

/// The account, the drive, the state repository and the drive's
/// repositories (design Decision 1): each from its flag, else from
/// `written`, the accounts of the runner's file setup wrote, where it names
/// the drive or names one drive alone, else, for the state repository,
/// none where `held` gives the one the host holds for the drive, else from
/// `ask`, a terminal's questions, asked in that order, the repositories one
/// a line until an empty one. `--add-repository` adds each after the
/// drive's own, one it holds changing nothing, and `--remove-repository`
/// removes each, one it does not hold changing nothing. Refused as a usage
/// error: an answer missing with no terminal, naming each missing flag;
/// two drives in `written` and no drive named with no terminal, naming
/// them; the whole list beside a change to it; a name or an address out of
/// form; a drive `written` keeps under another account; a drive left with
/// no repository; and a repository `written` keeps in another drive.
pub fn answers(
    flags: &Flags,
    written: &[Account],
    held: &dyn Fn(&str) -> Option<String>,
    mut ask: Option<Ask<'_>>,
) -> Result<Answers, Refusal> {
    let changes = !flags.add_repositories.is_empty() || !flags.remove_repositories.is_empty();
    if !flags.repositories.is_empty() && changes {
        return Err(host(
            "`--repository` gives the drive's whole list, and `--add-repository` and \
             `--remove-repository` change the list the host holds: give the one or the others"
                .to_string(),
        ));
    }
    let drives: Vec<(&str, &Drive)> = written
        .iter()
        .flat_map(|account| {
            account
                .drives
                .iter()
                .map(move |drive| (account.name.as_str(), drive))
        })
        .collect();
    let named = |name: &str| drives.iter().find(|(_, drive)| drive.name == name).copied();
    let mut entry = match &flags.drive {
        Some(name) => named(name),
        None if drives.len() == 1 => Some(drives[0]),
        None => None,
    };
    let mut missing: Vec<&str> = Vec::new();

    let account = match (&flags.account, entry) {
        (Some(account), _) => Some(account.clone()),
        (None, Some((account, _))) => Some(account.to_string()),
        (None, None) => asked(&mut ask, "Account (the account the drive is of)")?,
    };
    match &account {
        Some(account) => in_form("--account", "an account's", account)?,
        None => missing.push("--account"),
    }

    let drive = match (&flags.drive, entry) {
        (Some(drive), _) => Some(drive.clone()),
        (None, Some((_, drive))) => Some(drive.name.clone()),
        (None, None) if drives.len() > 1 && ask.is_none() => {
            return Err(host(format!(
                "`--drive` is not given, and the runner's file names the drives {}: name the \
                 one to set up with `--drive`",
                drives
                    .iter()
                    .map(|(_, drive)| drive.name.as_str())
                    .collect::<Vec<_>>()
                    .join(" and ")
            )));
        }
        (None, None) => asked(&mut ask, "Drive (its name)")?,
    };
    match &drive {
        Some(drive) => {
            in_form("--drive", "a drive's", drive)?;
            entry = entry.or_else(|| named(drive.as_str()));
        }
        None => missing.push("--drive"),
    }
    if let (Some(account), Some((owner, held_drive))) = (&account, entry) {
        if owner != account {
            return Err(host(format!(
                "the runner's file keeps the drive {} in the account {owner}, not {account}: a \
                 drive is one account's",
                held_drive.name
            )));
        }
    }

    let mut state_repository = flags.state_repository.clone();
    let is_held = drive.as_deref().is_some_and(|drive| held(drive).is_some());
    if state_repository.is_none() && !is_held {
        state_repository = asked(
            &mut ask,
            "State repository (the account's crew state repository, owner/name)",
        )?;
        if state_repository.is_none() {
            missing.push("--state-repository");
        }
    }

    let base: Option<Vec<String>> = if !flags.repositories.is_empty() {
        Some(flags.repositories.clone())
    } else if let Some((_, held_drive)) = entry {
        Some(
            held_drive
                .repositories
                .iter()
                .map(ToString::to_string)
                .collect(),
        )
    } else if changes {
        Some(Vec::new())
    } else if let Some(ask) = ask.as_mut() {
        let mut listed = Vec::new();
        loop {
            let answer =
                (*ask)("Repository (owner/name; an empty line ends the list)").map_err(host)?;
            if answer.is_empty() {
                break;
            }
            listed.push(answer);
        }
        Some(listed).filter(|listed| !listed.is_empty())
    } else {
        None
    };
    if base.is_none() {
        missing.push("--repository");
    }

    if !missing.is_empty() {
        let flags: Vec<String> = missing.iter().map(|flag| format!("`{flag}`")).collect();
        let (last, rest) = flags.split_last().expect("a flag missing");
        let listed = if rest.is_empty() {
            last.clone()
        } else {
            format!("{} and {last}", rest.join(", "))
        };
        return Err(host(format!(
            "setup is not given {listed}{}: give each as its flag",
            if ask.is_none() {
                ", and standard input is no terminal to ask"
            } else {
                ""
            }
        )));
    }
    let (account, drive) = (account.expect("an account"), drive.expect("a drive"));

    let mut repositories: Vec<Address> = Vec::new();
    for text in base.expect("the repositories") {
        let address = parsed("--repository", &text)?;
        if repositories.contains(&address) {
            return Err(host(format!(
                "`--repository` names {text} twice: a drive keeps each repository once"
            )));
        }
        repositories.push(address);
    }
    for text in &flags.add_repositories {
        let address = parsed("--add-repository", text)?;
        if !repositories.contains(&address) {
            repositories.push(address);
        }
    }
    for text in &flags.remove_repositories {
        let address = parsed("--remove-repository", text)?;
        repositories.retain(|kept| *kept != address);
    }
    if repositories.is_empty() {
        return Err(host(format!(
            "`--remove-repository` leaves the drive {drive} with no repository: a drive keeps \
             at least one repository"
        )));
    }
    for address in &repositories {
        let other = drives.iter().find(|(_, held_drive)| {
            held_drive.name != drive && held_drive.repositories.contains(address)
        });
        if let Some((_, other)) = other {
            return Err(host(format!(
                "the runner's file keeps {address} in the drive {}: a repository is kept by one \
                 drive",
                other.name
            )));
        }
    }
    Ok(Answers {
        account,
        drive,
        state_repository,
        repositories: by_owner(repositories),
    })
}

/// `question`'s answer from `ask`, none where there is no terminal or the
/// answer is empty.
fn asked(ask: &mut Option<Ask<'_>>, question: &str) -> Result<Option<String>, Refusal> {
    match ask {
        None => Ok(None),
        Some(ask) => {
            let answer = (*ask)(question).map_err(host)?;
            Ok(Some(answer).filter(|answer| !answer.is_empty()))
        }
    }
}

/// Refuses `name`, given by `flag`, where it is not of an account's or a
/// drive's form.
fn in_form(flag: &str, whose: &str, name: &str) -> Result<(), Refusal> {
    if config::in_form(name) {
        return Ok(());
    }
    Err(host(format!(
        "`{flag}` is {name:?}: {whose} name is 1 to 32 lower-case letters, digits and `-`, \
         starting with a letter"
    )))
}

/// The address `text`, given by `flag`, or a refusal naming both.
fn parsed(flag: &str, text: &str) -> Result<Address, Refusal> {
    Address::parse(text).map_err(|rule| host(format!("`{flag}` names {text:?}: {rule}")))
}

/// `repositories`, owner by owner in the order of the owners' names, each
/// owner's in the order given, as Nix's module orders a drive's.
fn by_owner(mut repositories: Vec<Address>) -> Vec<Address> {
    repositories.sort_by(|a, b| a.owner().cmp(b.owner()));
    repositories
}

/// Refuses, before any program runs (design Decision 7): a system other
/// than macOS, as the host's to fix; a runner's managed file that is a
/// link, as Nix's, writing nothing; `gh`, `herdr` or `launchctl` not on
/// setup's `PATH`, each with where it comes from; and a file at the managed
/// path setup did not write beside a `config.toml.before-setup` already
/// there, naming both.
pub fn check_host(host_of: &Host<'_>) -> Result<(), Refusal> {
    if host_of.system != "macos" {
        return Err(host(format!(
            "setup sets up a drive on macOS, and this host is {}: a NixOS host is set up by \
             the flake's module, `services.boomdrive`",
            host_of.system
        )));
    }
    let managed = host_of.managed;
    if managed
        .symlink_metadata()
        .is_ok_and(|meta| meta.file_type().is_symlink())
    {
        return Err(refused(format!(
            "{} is a link, as Nix's module makes it, so this host's runner is Nix's and setup \
             writes nothing here: a drive of this host is set in `services.boomdrive.accounts`",
            managed.display()
        )));
    }
    let wanted = [
        ("gh", "GitHub's command line, `nix profile add nixpkgs#gh`"),
        (
            "herdr",
            "`nix profile add github:agentplot/boomdrive#herdr`",
        ),
        ("launchctl", "macOS's own, `/bin/launchctl`"),
    ];
    let missing: Vec<String> = wanted
        .iter()
        .filter(|(program, _)| (host_of.find)(program).is_none())
        .map(|(program, from)| format!("`{program}` ({from})"))
        .collect();
    if !missing.is_empty() {
        return Err(host(format!(
            "{} not on setup's PATH",
            if missing.len() == 1 {
                format!("{} is", missing[0])
            } else {
                format!("{} are", missing.join(", "))
            }
        )));
    }
    let aside = managed.with_file_name(ASIDE);
    if managed.exists() && !is_setups(managed) && aside.symlink_metadata().is_ok() {
        return Err(host(format!(
            "{} is a file setup did not write, and {}, where setup moves such a file once, is \
             there too: both are left as they are; move one of them away",
            managed.display(),
            aside.display()
        )));
    }
    Ok(())
}

/// Whether the file at `path` is setup's own, its first line setup's.
fn is_setups(path: &Path) -> bool {
    std::fs::read_to_string(path).is_ok_and(|text| text.lines().next() == Some(FIRST_LINE))
}

/// The runner's file setup wrote, read as the runner reads its managed
/// layer, none where there is no file or it is not setup's.
fn own_file(managed: &Path, home: &Path) -> Result<Option<config::Config>, Refusal> {
    if !is_setups(managed) {
        return Ok(None);
    }
    config::read(managed, home).map(Some).map_err(host)
}

/// The sign-ins crew's agents need, read after GitHub's (design Decision
/// 10): Claude Code's, `loggedIn` in `claude auth status --json`, and
/// Pending You's, the line of `pendingyou status` holding `Sign-in:`
/// beginning `ok` once trimmed. Each missing, or whose program is not
/// found, is named with how the person signs in.
pub fn sign_ins(find: Find<'_>) -> Vec<Missing> {
    let mut missing = Vec::new();
    let claude =
        find("claude").and_then(|claude| run(&claude, &["auth", "status", "--json"], &[]).ok());
    let claude_in = claude.is_some_and(|output| {
        serde_json::from_slice::<Value>(&output.stdout)
            .is_ok_and(|status| status["loggedIn"] == Value::Bool(true))
    });
    if !claude_in {
        missing.push(Missing {
            what: "Claude Code's sign-in".to_string(),
            fix: "claude auth login".to_string(),
        });
    }
    let pending = find("pendingyou").and_then(|pendingyou| run(&pendingyou, &["status"], &[]).ok());
    let pending_in = pending.is_some_and(|output| {
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .find(|line| line.contains("Sign-in:"))
            .is_some_and(|line| line.trim().starts_with("ok"))
    });
    if !pending_in {
        missing.push(Missing {
            what: "Pending You's sign-in".to_string(),
            fix: "npx -y pendingyou@latest init".to_string(),
        });
    }
    missing
}

/// `written` with the drive of `answers` set: its repositories replaced
/// where its account names it, and else the drive, and its account where
/// none is, added.
fn with_drive(mut written: Vec<Account>, answers: &Answers) -> Vec<Account> {
    let drive = Drive {
        name: answers.drive.clone(),
        repositories: answers.repositories.clone(),
    };
    match written
        .iter_mut()
        .find(|account| account.name == answers.account)
    {
        Some(account) => match account
            .drives
            .iter_mut()
            .find(|held| held.name == drive.name)
        {
            Some(held) => *held = drive,
            None => account.drives.push(drive),
        },
        None => written.push(Account {
            name: answers.account.clone(),
            drives: vec![drive],
        }),
    }
    written
}

/// The runner's managed layer as setup writes it (design Decision 6): its
/// first line setup's, then `accounts`, each account's drives and each
/// drive's repositories, owner by owner, and `[control_plane]`, its `type`
/// `word` and its table `table`, laid out as Nix's module lays out the
/// same keys (`nix/modules/toml.nix`): top-level keys before tables, each
/// sorted by name, a table holding only tables named by theirs, and TOML's
/// basic strings.
pub fn runner_file(accounts: &[Account], word: &str, table: &toml::Table) -> String {
    let mut settings = toml::Table::new();
    let mut accounts_table = toml::Table::new();
    for account in accounts {
        let mut drives = toml::Table::new();
        for drive in &account.drives {
            let repositories = by_owner(drive.repositories.clone())
                .iter()
                .map(|address| toml::Value::String(address.to_string()))
                .collect();
            let mut held = toml::Table::new();
            held.insert("repositories".to_string(), toml::Value::Array(repositories));
            drives.insert(drive.name.clone(), toml::Value::Table(held));
        }
        let mut account_table = toml::Table::new();
        account_table.insert("drives".to_string(), toml::Value::Table(drives));
        accounts_table.insert(account.name.clone(), toml::Value::Table(account_table));
    }
    settings.insert("accounts".to_string(), toml::Value::Table(accounts_table));
    let mut plane = toml::Table::new();
    plane.insert("type".to_string(), toml::Value::String(word.to_string()));
    plane.insert(word.to_string(), toml::Value::Table(table.clone()));
    settings.insert("control_plane".to_string(), toml::Value::Table(plane));

    let (keys, sections) = lines_of(&[], &settings);
    let mut lines = vec![FIRST_LINE.to_string()];
    lines.extend(keys);
    lines.extend(sections);
    lines.join("\n") + "\n"
}

/// Whether `value` is written as a table of its own: a table, or a list
/// of tables.
fn nested(value: &toml::Value) -> bool {
    match value {
        toml::Value::Table(_) => true,
        toml::Value::Array(items) => !items.is_empty() && items.iter().all(toml::Value::is_table),
        _ => false,
    }
}

/// A table's lines, at `path`: its keys', then its tables', each sorted by
/// name.
fn lines_of(path: &[&str], table: &toml::Table) -> (Vec<String>, Vec<String>) {
    let mut names: Vec<&String> = table.keys().collect();
    names.sort();
    let keys = names
        .iter()
        .filter(|name| !nested(&table[name.as_str()]))
        .map(|name| format!("{} = {}", key(name), inline(&table[name.as_str()])))
        .collect();
    let mut sections = Vec::new();
    for name in names.iter().filter(|name| nested(&table[name.as_str()])) {
        let mut at = path.to_vec();
        at.push(name.as_str());
        match &table[name.as_str()] {
            toml::Value::Table(inner) => {
                let (keys, inner_sections) = lines_of(&at, inner);
                if !keys.is_empty() || inner_sections.is_empty() {
                    sections.push(String::new());
                    sections.push(format!("[{}]", header(&at)));
                    sections.extend(keys);
                }
                sections.extend(inner_sections);
            }
            toml::Value::Array(items) => {
                for item in items.iter().filter_map(toml::Value::as_table) {
                    let (keys, inner_sections) = lines_of(&at, item);
                    sections.push(String::new());
                    sections.push(format!("[[{}]]", header(&at)));
                    sections.extend(keys);
                    sections.extend(inner_sections);
                }
            }
            _ => {}
        }
    }
    (keys, sections)
}

/// A table's header, its keys joined by `.`.
fn header(path: &[&str]) -> String {
    path.iter()
        .map(|name| key(name))
        .collect::<Vec<_>>()
        .join(".")
}

/// A key as TOML writes it: bare where it can be, else quoted.
fn key(name: &str) -> String {
    let bare = !name.is_empty()
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    if bare {
        name.to_string()
    } else {
        basic(name)
    }
}

/// A value written on its key's line.
fn inline(value: &toml::Value) -> String {
    match value {
        toml::Value::String(text) => basic(text),
        toml::Value::Array(items) => {
            let items: Vec<String> = items.iter().map(inline).collect();
            format!("[{}]", items.join(", "))
        }
        other => other.to_string(),
    }
}

/// TOML's basic string: `"`, `\` and every control character escaped, as
/// `toml.nix` escapes them.
fn basic(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\u{8}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 32 || c as u32 == 127 => {
                out.push_str(&format!("\\u{:04X}", c as u32));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Writes `text` as the runner's file `managed`, whole, by a file renamed
/// into place, and only where its bytes differ; a person's own file there
/// moved to `config.toml.before-setup` first (design Decision 6). Gives
/// the file, its outcome, `written` or `unchanged`, and where a person's
/// file was moved, or null.
fn write_runner_file(managed: &Path, text: &str) -> Result<Value, Refusal> {
    let unwritten = |e: std::io::Error| host(format!("{}: {e}", managed.display()));
    let folder = managed.parent().expect("the file's folder");
    std::fs::create_dir_all(folder).map_err(unwritten)?;
    let mut aside = Value::Null;
    if managed.exists() && !is_setups(managed) {
        let to = managed.with_file_name(ASIDE);
        std::fs::rename(managed, &to).map_err(unwritten)?;
        aside = json!(to.display().to_string());
    }
    let outcome = if write_if_changed(managed, text.as_bytes()).map_err(unwritten)? {
        "written"
    } else {
        "unchanged"
    };
    Ok(json!({"file": managed.display().to_string(), "outcome": outcome, "aside": aside}))
}

/// Writes `bytes` to `file` by a file beside it renamed into place, only
/// where the file's bytes differ: whether it was written.
fn write_if_changed(file: &Path, bytes: &[u8]) -> std::io::Result<bool> {
    if std::fs::read(file).is_ok_and(|held| held == bytes) {
        return Ok(false);
    }
    let name = file.file_name().expect("a file's name").to_string_lossy();
    let temporary = file.with_file_name(format!(".{name}.setup-{}", std::process::id()));
    std::fs::write(&temporary, bytes)?;
    std::fs::rename(&temporary, file).inspect_err(|_| {
        let _ = std::fs::remove_file(&temporary);
    })?;
    Ok(true)
}

/// Links the set's herdr plugins from the person's profile (design
/// Decision 8; core.29): each folder under `share/herdr/plugins` of
/// `~/.nix-profile`, else `~/.local/state/nix/profile`, by its path
/// through the profile, with `herdr plugin link`, but one whose resolved
/// path is the resolved `plugin_root` of a plugin `herdr plugin list
/// --json` gives. herdr runs with no `HERDR_` variable and standard input
/// closed. Gives each folder's outcome, `linked`, `there` or `refused`
/// with herdr's words; and, where no profile holds the folder, the
/// plugins as missing.
pub fn plugins(home: &Path, herdr: &Path) -> (Vec<Value>, Option<Missing>) {
    let profiles = [
        home.join(".nix-profile"),
        home.join(".local/state/nix/profile"),
    ];
    let Some(folder) = profiles
        .iter()
        .map(|profile| profile.join("share/herdr/plugins"))
        .find(|folder| folder.is_dir())
    else {
        return (
            Vec::new(),
            Some(Missing {
                what: "the set's herdr plugins".to_string(),
                fix: format!("nix profile add {PLUGINS_FROM}"),
            }),
        );
    };
    let mut folders: Vec<PathBuf> = std::fs::read_dir(&folder)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter(|path| path.is_dir())
                .collect()
        })
        .unwrap_or_default();
    folders.sort();
    if folders.is_empty() {
        return (
            Vec::new(),
            Some(Missing {
                what: "the set's herdr plugins".to_string(),
                fix: format!("nix profile add {PLUGINS_FROM}"),
            }),
        );
    }
    let listed = herdr_run(herdr, &["plugin", "list", "--json"]);
    let roots: Result<Vec<PathBuf>, String> = match listed {
        Ok(output) if output.status.success() => Ok(plugin_roots(&output.stdout)),
        Ok(output) => Err(format!("`herdr plugin list --json`: {}", words_of(&output))),
        Err(words) => Err(words),
    };
    let outcomes = folders
        .iter()
        .map(|plugin| {
            let name = plugin
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
                .unwrap_or_default();
            let shown = plugin.display().to_string();
            let roots = match &roots {
                Ok(roots) => roots,
                Err(words) => {
                    return json!({"plugin": name, "folder": shown, "outcome": "refused", "words": words});
                }
            };
            let resolved = resolve(plugin);
            if roots.contains(&resolved) {
                return json!({"plugin": name, "folder": shown, "outcome": "there"});
            }
            let mut args: Vec<&std::ffi::OsStr> = vec!["plugin".as_ref(), "link".as_ref()];
            args.push(plugin.as_os_str());
            match herdr_run(herdr, &args) {
                Ok(output) if output.status.success() => {
                    json!({"plugin": name, "folder": shown, "outcome": "linked"})
                }
                Ok(output) => json!({
                    "plugin": name, "folder": shown, "outcome": "refused", "words": words_of(&output)
                }),
                Err(words) => {
                    json!({"plugin": name, "folder": shown, "outcome": "refused", "words": words})
                }
            }
        })
        .collect();
    (outcomes, None)
}

/// Each `plugin_root` herdr's list gives, resolved.
fn plugin_roots(listed: &[u8]) -> Vec<PathBuf> {
    let Ok(answer) = serde_json::from_slice::<Value>(listed) else {
        return Vec::new();
    };
    let plugins = answer["result"]["plugins"]
        .as_array()
        .or_else(|| answer["plugins"].as_array())
        .or_else(|| answer.as_array());
    plugins
        .into_iter()
        .flatten()
        .filter_map(|plugin| plugin["plugin_root"].as_str())
        .map(|root| resolve(Path::new(root)))
        .collect()
}

/// `path` with its links resolved, or as it is where it cannot be.
fn resolve(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

/// herdr run with `args`, every `HERDR_` variable removed and standard
/// input closed, as the modules' activation runs it.
fn herdr_run<S: AsRef<std::ffi::OsStr>>(herdr: &Path, args: &[S]) -> Result<Output, String> {
    let removed: Vec<OsString> = std::env::vars_os()
        .map(|(name, _)| name)
        .filter(|name| name.to_string_lossy().starts_with("HERDR_"))
        .collect();
    run(herdr, args, &removed)
}

/// The agent's plist for the home `home`, running the runner with the
/// file `config_file` (design Decision 9): nix-darwin's agent's keys, its
/// program waiting on `/bin/wait4path /nix/store` and then running
/// `boomdrive` by name, which the fixed `PATH` finds in the person's
/// profile.
pub fn agent(home: &Path, config_file: &Path) -> String {
    let home_text = home.display().to_string();
    let path = [
        format!("{home_text}/.nix-profile/bin"),
        format!("{home_text}/.local/state/nix/profile/bin"),
        "/nix/var/nix/profiles/default/bin".to_string(),
        format!("{home_text}/.local/bin"),
        "/opt/homebrew/bin".to_string(),
        "/usr/local/bin".to_string(),
        "/usr/bin".to_string(),
        "/bin".to_string(),
        "/usr/sbin".to_string(),
        "/sbin".to_string(),
    ]
    .join(":");
    let command = format!(
        "/bin/wait4path /nix/store && exec boomdrive run --config {}",
        shell_quoted(&config_file.display().to_string())
    );
    let log = format!("{home_text}/Library/Logs/boomdrive.log");
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>AbandonProcessGroup</key>
	<true/>
	<key>EnvironmentVariables</key>
	<dict>
		<key>HOME</key>
		<string>{home}</string>
		<key>PATH</key>
		<string>{path}</string>
	</dict>
	<key>KeepAlive</key>
	<true/>
	<key>Label</key>
	<string>{LABEL}</string>
	<key>ProcessType</key>
	<string>Interactive</string>
	<key>ProgramArguments</key>
	<array>
		<string>/bin/sh</string>
		<string>-c</string>
		<string>{command}</string>
	</array>
	<key>RunAtLoad</key>
	<true/>
	<key>StandardErrorPath</key>
	<string>{log}</string>
	<key>StandardOutPath</key>
	<string>{log}</string>
	<key>ThrottleInterval</key>
	<integer>30</integer>
</dict>
</plist>
"#,
        home = xml(&home_text),
        path = xml(&path),
        command = xml(&command),
        log = xml(&log),
    )
}

/// `text` as one word of the shell: as it is where it holds nothing the
/// shell reads, else in single quotes.
fn shell_quoted(text: &str) -> String {
    let plain = !text.is_empty()
        && text.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(c, '/' | '.' | '_' | '-' | '+' | ',' | ':' | '@' | '%')
        });
    if plain {
        text.to_string()
    } else {
        format!("'{}'", text.replace('\'', r"'\''"))
    }
}

/// `text` escaped for an XML string.
fn xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Writes the agent's plist to `~/Library/LaunchAgents` only where its
/// bytes differ, then loads it into the login's domain (design Decision
/// 9): bootstrapped where `launchctl print` finds no agent, booted out
/// first where the plist changed and one is loaded, and neither where it
/// is loaded and unchanged. A refusal of `launchctl`'s is the tool's, with
/// its words. Gives the plist, whether it was written, and the agent's
/// outcome, `loaded`, `reloaded` or `running`.
fn load_agent(home: &Path, config_file: &Path, launchctl: &Path) -> Result<Value, Refusal> {
    let plist = home
        .join("Library/LaunchAgents")
        .join(format!("{LABEL}.plist"));
    let unwritten = |e: std::io::Error| host(format!("{}: {e}", plist.display()));
    std::fs::create_dir_all(plist.parent().expect("the plist's folder")).map_err(unwritten)?;
    let changed =
        write_if_changed(&plist, agent(home, config_file).as_bytes()).map_err(unwritten)?;
    // SAFETY: getuid has no preconditions and cannot fail.
    let uid = unsafe { libc::getuid() };
    let domain = format!("gui/{uid}");
    let service = format!("{domain}/{LABEL}");
    let tool = |words: String| refused(words);
    let loaded = run(launchctl, &["print", service.as_str()], &[])
        .map_err(tool)?
        .status
        .success();
    let plist_text = plist.display().to_string();
    let outcome = if loaded && !changed {
        "running"
    } else {
        if loaded {
            let out = run(launchctl, &["bootout", service.as_str()], &[]).map_err(tool)?;
            if !out.status.success() {
                return Err(tool(format!(
                    "`launchctl bootout {service}` exited {}: {}",
                    exit_of(&out),
                    words_of(&out)
                )));
            }
        }
        let out = run(
            launchctl,
            &["bootstrap", domain.as_str(), plist_text.as_str()],
            &[],
        )
        .map_err(tool)?;
        if !out.status.success() {
            return Err(tool(format!(
                "`launchctl bootstrap {domain} {plist_text}` exited {}: {}",
                exit_of(&out),
                words_of(&out)
            )));
        }
        if loaded {
            "reloaded"
        } else {
            "loaded"
        }
    };
    Ok(json!({
        "file": plist_text,
        "plist": if changed { "written" } else { "unchanged" },
        "outcome": outcome,
    }))
}

/// `program` run with `args`, each of `removed` taken from its
/// environment, standard input closed, its output captured; or words
/// saying why it could not be run.
fn run<S: AsRef<std::ffi::OsStr>>(
    program: &Path,
    args: &[S],
    removed: &[OsString],
) -> Result<Output, String> {
    let mut command = Command::new(program);
    command.args(args).stdin(Stdio::null());
    for name in removed {
        command.env_remove(name);
    }
    command
        .output()
        .map_err(|error| format!("{} could not be run: {error}", program.display()))
}

/// A program's words: its standard error, else its standard output,
/// trimmed.
fn words_of(output: &Output) -> String {
    let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !err.is_empty() {
        return err;
    }
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// A program's exit, its code or the signal that ended it.
fn exit_of(output: &Output) -> String {
    match output.status.code() {
        Some(code) => code.to_string(),
        None => "on a signal".to_string(),
    }
}

/// `answers`' tests, named by it (task 4.1).
#[cfg(test)]
mod answers {
    use super::*;

    /// The accounts of a runner's file setup wrote: each drive of `drives`
    /// in the account `own`, with its repositories.
    fn written(drives: &[(&str, &[&str])]) -> Vec<Account> {
        vec![Account {
            name: "own".to_string(),
            drives: drives
                .iter()
                .map(|(name, repositories)| Drive {
                    name: name.to_string(),
                    repositories: repositories
                        .iter()
                        .map(|text| Address::parse(text).unwrap())
                        .collect(),
                })
                .collect(),
        }]
    }

    /// The host holds no state repository.
    fn none(_: &str) -> Option<String> {
        None
    }

    /// The host holds `example/state` for every drive.
    fn held(_: &str) -> Option<String> {
        Some("example/state".to_string())
    }

    fn texts(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    fn kept(answers: &Answers) -> Vec<String> {
        answers
            .repositories
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    /// The setup spec's "An answer missing with no terminal".
    #[test]
    fn an_answer_missing_with_no_terminal_names_each_flag() {
        let flags = Flags {
            account: Some("own".to_string()),
            drive: Some("home".to_string()),
            ..Flags::default()
        };
        let refusal = answers(&flags, &[], &none, None).unwrap_err();
        assert!(refusal.host_to_fix);
        for flag in ["`--state-repository`", "`--repository`", "no terminal"] {
            assert!(refusal.words.contains(flag), "{}", refusal.words);
        }
        for flag in ["`--account`", "`--drive`"] {
            assert!(!refusal.words.contains(flag), "{}", refusal.words);
        }
        let refusal = answers(&Flags::default(), &[], &none, None).unwrap_err();
        assert!(
            refusal
                .words
                .contains("`--account`, `--drive`, `--state-repository` and `--repository`"),
            "{}",
            refusal.words
        );
    }

    /// The setup spec's "Asked on a terminal".
    #[test]
    fn a_terminal_is_asked_in_order() {
        let flags = Flags {
            account: Some("own".to_string()),
            ..Flags::default()
        };
        let mut said = vec!["home", "example/state", "agentplot/flume", ""].into_iter();
        let mut questions: Vec<String> = Vec::new();
        let mut ask = |question: &str| {
            questions.push(question.to_string());
            Ok(said.next().expect("an answer").to_string())
        };
        let got = answers(&flags, &[], &none, Some(&mut ask)).unwrap();
        assert_eq!(
            got,
            Answers {
                account: "own".to_string(),
                drive: "home".to_string(),
                state_repository: Some("example/state".to_string()),
                repositories: vec![Address::parse("agentplot/flume").unwrap()],
            }
        );
        let asked: Vec<&str> = questions
            .iter()
            .map(|question| question.split(' ').next().unwrap())
            .collect();
        assert_eq!(asked, ["Drive", "State", "Repository", "Repository"]);
    }

    /// The setup spec's "The whole list beside a change to it".
    #[test]
    fn the_whole_list_beside_a_change_to_it() {
        for (add, remove) in [(true, false), (false, true)] {
            let flags = Flags {
                account: Some("own".to_string()),
                drive: Some("home".to_string()),
                state_repository: Some("example/state".to_string()),
                repositories: texts(&["agentplot/flume"]),
                add_repositories: if add {
                    texts(&["agentplot/sluice"])
                } else {
                    vec![]
                },
                remove_repositories: if remove {
                    texts(&["agentplot/flume"])
                } else {
                    vec![]
                },
                ..Flags::default()
            };
            let refusal = answers(&flags, &[], &none, None).unwrap_err();
            assert!(refusal.host_to_fix);
            for flag in [
                "`--repository`",
                "`--add-repository`",
                "`--remove-repository`",
            ] {
                assert!(refusal.words.contains(flag), "{}", refusal.words);
            }
        }
    }

    /// The setup spec's "Two drives and no drive named", and the drive
    /// named taking its account and repositories from the file.
    #[test]
    fn two_drives_and_no_drive_named() {
        let file = written(&[("home", &["agentplot/flume"]), ("work", &["example/app"])]);
        let refusal = answers(&Flags::default(), &file, &held, None).unwrap_err();
        assert!(refusal.host_to_fix);
        for words in ["`--drive`", "home", "work"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
        let flags = Flags {
            drive: Some("work".to_string()),
            ..Flags::default()
        };
        let got = answers(&flags, &file, &held, None).unwrap();
        assert_eq!(got.account, "own");
        assert_eq!(got.state_repository, None);
        assert_eq!(kept(&got), ["example/app"]);
    }

    /// The setup spec's "The last repository removed", and one removed
    /// beside another.
    #[test]
    fn the_last_repository_removed() {
        let file = written(&[("home", &["agentplot/flume"])]);
        let flags = Flags {
            remove_repositories: texts(&["agentplot/flume"]),
            ..Flags::default()
        };
        let refusal = answers(&flags, &file, &held, None).unwrap_err();
        assert!(refusal.host_to_fix);
        assert!(
            refusal
                .words
                .contains("a drive keeps at least one repository"),
            "{}",
            refusal.words
        );
        let file = written(&[("home", &["agentplot/flume", "agentplot/sluice"])]);
        let flags = Flags {
            remove_repositories: texts(&["agentplot/sluice", "example/app"]),
            ..Flags::default()
        };
        let got = answers(&flags, &file, &held, None).unwrap();
        assert_eq!(kept(&got), ["agentplot/flume"]);
    }

    /// The setup spec's "A repository added twice", and one added after
    /// the drive's own, owner by owner.
    #[test]
    fn a_repository_added_twice() {
        let file = written(&[("home", &["agentplot/flume", "agentplot/sluice"])]);
        let flags = Flags {
            add_repositories: texts(&["agentplot/sluice"]),
            ..Flags::default()
        };
        let got = answers(&flags, &file, &held, None).unwrap();
        assert_eq!(got.account, "own");
        assert_eq!(got.drive, "home");
        assert_eq!(kept(&got), ["agentplot/flume", "agentplot/sluice"]);
        let flags = Flags {
            add_repositories: texts(&["example/app", "agentplot/crew"]),
            ..Flags::default()
        };
        let got = answers(&flags, &file, &held, None).unwrap();
        assert_eq!(
            kept(&got),
            [
                "agentplot/flume",
                "agentplot/sluice",
                "agentplot/crew",
                "example/app"
            ]
        );
    }

    /// A repository the runner's file keeps in another drive, and a drive
    /// it keeps under another account.
    #[test]
    fn a_repository_in_another_drive() {
        let file = written(&[("home", &["agentplot/flume"]), ("work", &["example/app"])]);
        let flags = Flags {
            drive: Some("home".to_string()),
            add_repositories: texts(&["example/app"]),
            ..Flags::default()
        };
        let refusal = answers(&flags, &file, &held, None).unwrap_err();
        assert!(refusal.host_to_fix);
        for words in ["example/app", "the drive work"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
        let flags = Flags {
            account: Some("other".to_string()),
            drive: Some("home".to_string()),
            ..Flags::default()
        };
        let refusal = answers(&flags, &file, &held, None).unwrap_err();
        assert!(
            refusal.words.contains("the account own"),
            "{}",
            refusal.words
        );
    }

    /// An account out of form is refused naming it, and a drive and an
    /// address too.
    #[test]
    fn an_account_out_of_form() {
        let every = Flags {
            account: Some("own".to_string()),
            drive: Some("home".to_string()),
            state_repository: Some("example/state".to_string()),
            repositories: texts(&["agentplot/flume"]),
            ..Flags::default()
        };
        let cases = [
            (
                Flags {
                    account: Some("Own".to_string()),
                    ..every.clone()
                },
                ["`--account`", "\"Own\""],
            ),
            (
                Flags {
                    drive: Some("-home".to_string()),
                    ..every.clone()
                },
                ["`--drive`", "\"-home\""],
            ),
            (
                Flags {
                    repositories: texts(&["flume"]),
                    ..every.clone()
                },
                ["`--repository`", "\"flume\""],
            ),
        ];
        for (flags, words) in cases {
            let refusal = answers(&flags, &[], &none, None).unwrap_err();
            assert!(refusal.host_to_fix);
            for word in words {
                assert!(refusal.words.contains(word), "{}", refusal.words);
            }
        }
        assert!(answers(&every, &[], &none, None).is_ok());
    }
}

/// `check_host`'s tests, named by it (task 5.1).
#[cfg(test)]
mod check_host {
    use super::*;

    /// A scratch home whose runner's file is at its default path.
    struct Home {
        dir: tempfile::TempDir,
    }

    impl Home {
        fn new() -> Home {
            Home {
                dir: tempfile::tempdir().unwrap(),
            }
        }

        fn home(&self) -> PathBuf {
            self.dir.path().to_path_buf()
        }

        fn managed(&self) -> PathBuf {
            self.home().join(".config/boomdrive/config.toml")
        }

        fn write(&self, path: &Path, text: &str) {
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }

        /// `check_host` on `system`, finding every program but `lacking`.
        fn check(&self, system: &str, lacking: &[&str]) -> Result<(), Refusal> {
            let find = |name: &str| {
                Some(PathBuf::from("/stand-in").join(name)).filter(|_| !lacking.contains(&name))
            };
            check_host(&Host {
                system,
                home: &self.home(),
                managed: &self.managed(),
                find: &find,
            })
        }
    }

    /// The setup spec's "On Linux".
    #[test]
    fn on_linux_setup_is_refused() {
        let home = Home::new();
        let refusal = home.check("linux", &[]).unwrap_err();
        assert!(refusal.host_to_fix);
        for words in ["linux", "`services.boomdrive`"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
        assert_eq!(home.check("macos", &[]), Ok(()));
    }

    /// A runner's file that is a link is Nix's: refused, never as the
    /// host's to fix, naming the file and `services.boomdrive.accounts`.
    #[test]
    fn a_linked_runner_file_is_nixs() {
        let home = Home::new();
        let target = home.home().join("store-file.toml");
        home.write(&target, "[accounts]\n");
        std::fs::create_dir_all(home.managed().parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(&target, home.managed()).unwrap();
        let refusal = home.check("macos", &[]).unwrap_err();
        assert!(!refusal.host_to_fix);
        let file = home.managed().display().to_string();
        for words in [file.as_str(), "`services.boomdrive.accounts`"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
    }

    /// Each program missing is named with where it comes from.
    #[test]
    fn a_program_missing_is_named() {
        let home = Home::new();
        let refusal = home.check("macos", &["herdr"]).unwrap_err();
        assert!(refusal.host_to_fix);
        for words in ["`herdr`", "github:agentplot/boomdrive#herdr"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
        let refusal = home.check("macos", &["gh", "launchctl"]).unwrap_err();
        for words in ["`gh`", "`launchctl`", "/bin/launchctl"] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
    }

    /// The setup spec's "Both files there", as the host's to fix; and a
    /// person's file alone, or setup's beside an aside, passed.
    #[test]
    fn both_files_there_are_named() {
        let home = Home::new();
        home.write(&home.managed(), "[accounts]\n");
        assert_eq!(home.check("macos", &[]), Ok(()));
        let aside = home.managed().with_file_name(ASIDE);
        home.write(&aside, "[accounts]\n");
        let refusal = home.check("macos", &[]).unwrap_err();
        assert!(refusal.host_to_fix);
        let (file, aside_text) = (
            home.managed().display().to_string(),
            aside.display().to_string(),
        );
        for words in [file.as_str(), aside_text.as_str()] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
        home.write(&home.managed(), &format!("{FIRST_LINE}\n[accounts]\n"));
        assert_eq!(home.check("macos", &[]), Ok(()));
    }
}
