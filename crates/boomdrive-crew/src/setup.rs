//! A drive's setup under crew, crew's side of `boomdrive setup` on a Mac
//! Nix does not manage (record 0009 as amended 2026-11-09 and 2026-11-11;
//! design Decisions 2, 3, 4, 5 and 11 of the change
//! a-persons-drive-is-set-up-by-one-command): what the host holds of a
//! drive's answers, in crew's table of the runner's file and crew's own
//! file; those answers settled and checked before any program runs; and
//! the drive set up, its state repository made private by gh where none
//! exists and the drive named in crew's file by `crew drive set`, with
//! each role's `--runs` and the drive's `--review` and never `--flume`
//! (crew records 0064 as amended 2026-11-09 and 0065). Every program is
//! found by the finder the runner hands it, and each tool's answer is
//! carried as the tool gave it (core.1, core.11).

use crate::config::{is_name, StateRepository};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// crew's five roles of a drive, in crew's order (crew's `KIND_ROLES`).
pub const ROLES: [&str; 5] = ["design", "planner", "reviewer", "ops", "operator"];

/// How a role runs, as crew's `--runs` takes it.
pub const RUNS: [&str; 2] = ["standing", "on-demand"];

/// The version of crew's file whose drives `held` reads (crew record
/// 0064 as amended 2026-11-09).
const TEAMS_VERSION: u64 = 6;

/// Each role's way of running where neither a flag nor crew's file gives
/// one (design Decision 3).
const STANDING: &str = "standing";

/// The review where neither a flag nor crew's file gives one: five changes
/// or four hours (design Decision 3).
const REVIEW: (&str, &str) = ("5", "4");

/// Where the set gives crew, named where it is not found.
const CREW_FROM: &str = "github:agentplot/boomdrive#crew";

/// The words gh says of a repository that is not there.
const NOT_THERE: &str = "Could not resolve to a Repository";

/// What setup's flags gave crew's side, each as text: none or empty where
/// a flag was not given.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Asked {
    /// `--state-repository`.
    pub state_repository: Option<String>,
    /// Each `--blueprints`, in order.
    pub blueprints: Vec<String>,
    /// Each `--runs <role> <how>`, in order.
    pub runs: Vec<(String, String)>,
    /// `--review <after_changes> <after_hours>`.
    pub review: Option<(String, String)>,
}

/// What the host holds of a drive's answers: the state repository from
/// its partition in crew's table of the runner's file, and the blueprints,
/// each role's runs and the review from its entry in crew's file. None or
/// empty where the host holds none.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Held {
    pub state_repository: Option<String>,
    pub blueprints: Vec<String>,
    /// Each role crew's file gives a way of running, by its name.
    pub runs: BTreeMap<String, String>,
    /// `after_changes` and `after_hours`, as text.
    pub review: Option<(String, String)>,
}

/// A drive's answers, settled: each given, held or setup's own, and
/// checked.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Settled {
    pub state_repository: String,
    pub blueprints: Vec<String>,
    /// Each role of [`ROLES`], in its order, with how it runs.
    pub runs: Vec<(String, String)>,
    /// `after_changes` and `after_hours`, as text.
    pub review: (String, String),
}

/// A repository the drive keeps, with the runner's clone's folder and its
/// view (core.57).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kept {
    pub repository: String,
    pub dir: PathBuf,
    pub checkout: PathBuf,
}

/// Setup refused, with words saying why, and whether the host or the
/// answers must be fixed (exit 2) or a tool refused (exit 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub words: String,
    pub host_to_fix: bool,
}

/// The programs crew's side runs, found, and this host's name as
/// `hostname -s` gives it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Prepared {
    pub host: String,
    gh: PathBuf,
    crew: PathBuf,
}

/// A drive set up: crew's table of the runner's file with the drive's
/// partition set, the answer, a line for each step, and the command that
/// starts crew's agents.
#[derive(Clone, Debug, PartialEq)]
pub struct SetUp {
    pub table: toml::Table,
    pub answer: Value,
    pub lines: Vec<String>,
    pub start: String,
}

/// crew's file on the host whose home is `home`, as crew reads it.
pub fn teams_file(home: &Path) -> PathBuf {
    home.join(".config/crew/teams.json")
}

/// What the host holds of the drive `drive`: its state repository from
/// its partition in `table`, crew's table of the runner's file, where
/// there is one, and its blueprints, runs and review from its entry in
/// crew's file, `teams_file`, at version 6. A file that is missing or
/// unreadable, at another version, or that does not name the drive gives
/// none of the file's, and a value of another shape gives none of its key.
pub fn held(table: Option<&toml::Table>, teams_file: &Path, drive: &str) -> Held {
    let state_repository = table
        .and_then(|table| table.get("partitions"))
        .and_then(toml::Value::as_array)
        .and_then(|partitions| {
            partitions.iter().find(|partition| {
                partition.get("label").and_then(toml::Value::as_str) == Some(drive)
            })
        })
        .and_then(|partition| partition.get("state_repository"))
        .and_then(toml::Value::as_str)
        .map(str::to_string);
    let mut held = Held {
        state_repository,
        ..Held::default()
    };
    let Some(entry) = teams_entry(teams_file, drive) else {
        return held;
    };
    if let Some(blueprints) = entry.get("blueprints").and_then(Value::as_array) {
        let texts: Option<Vec<String>> = blueprints
            .iter()
            .map(|b| b.as_str().map(str::to_string))
            .collect();
        held.blueprints = texts.unwrap_or_default();
    }
    if let Some(runs) = entry.get("runs").and_then(Value::as_object) {
        held.runs = runs
            .iter()
            .filter_map(|(role, how)| Some((role.clone(), how.as_str()?.to_string())))
            .collect();
    }
    if let Some(review) = entry.get("review").and_then(Value::as_object) {
        let number = |key: &str| review.get(key).and_then(Value::as_number).map(number_text);
        if let (Some(changes), Some(hours)) = (number("after_changes"), number("after_hours")) {
            held.review = Some((changes, hours));
        }
    }
    held
}

/// The drive `drive`'s entry in crew's file `teams_file`, at version 6.
fn teams_entry(teams_file: &Path, drive: &str) -> Option<serde_json::Map<String, Value>> {
    let text = std::fs::read_to_string(teams_file).ok()?;
    let data: Value = serde_json::from_str(&text).ok()?;
    if data.get("version").and_then(Value::as_u64) != Some(TEAMS_VERSION) {
        return None;
    }
    data.get("drives")?
        .as_array()?
        .iter()
        .filter_map(Value::as_object)
        .find(|entry| entry.get("label").and_then(Value::as_str) == Some(drive))
        .cloned()
}

/// A number as crew's flag takes it: a whole number without a point.
fn number_text(number: &serde_json::Number) -> String {
    if let Some(whole) = number.as_u64() {
        return whole.to_string();
    }
    match number.as_f64() {
        Some(n) if n.fract() == 0.0 && n.abs() < 1e15 => format!("{}", n as i64),
        Some(n) => n.to_string(),
        None => number.to_string(),
    }
}

/// The drive's answers settled: each `asked` gives, else each `held`
/// gives, else setup's own, the drive's first repository as its
/// blueprints, every role standing and the review `5` changes or `4`
/// hours (design Decisions 2 and 3); a `--runs` changes its role alone.
/// `repositories` are the drive's, in order. Refused as the host's to fix,
/// before any program runs, naming the flag and the value: an address out
/// of form, a blueprints repository not among `repositories`, a role not
/// of [`ROLES`] or a way of running not of [`RUNS`], and a review whose
/// changes are not a whole number of at least 1 or whose hours are not a
/// number above 0.
pub fn settle(asked: &Asked, held: &Held, repositories: &[String]) -> Result<Settled, Refusal> {
    let state_repository = asked
        .state_repository
        .clone()
        .or_else(|| held.state_repository.clone())
        .ok_or_else(|| {
            usage(
                "`--state-repository` is not given, and the runner's file names none for the \
                 drive: the account's crew state repository, `owner/name`, is needed"
                    .to_string(),
            )
        })?;
    if !in_form(&state_repository) {
        return Err(usage(format!(
            "`--state-repository` is {state_repository:?}: a repository is `owner/name`"
        )));
    }

    let (blueprints, named) = if !asked.blueprints.is_empty() {
        (asked.blueprints.clone(), "`--blueprints` names")
    } else if !held.blueprints.is_empty() {
        (
            held.blueprints.clone(),
            "crew's file gives the drive the blueprints",
        )
    } else {
        (repositories.iter().take(1).cloned().collect(), "")
    };
    if blueprints.is_empty() {
        return Err(usage(
            "the drive keeps no repository: a drive keeps at least one, its first its \
             blueprints"
                .to_string(),
        ));
    }
    let mut seen = BTreeSet::new();
    for blueprint in &blueprints {
        if !in_form(blueprint) {
            return Err(usage(format!(
                "{named} {blueprint:?}: a repository is `owner/name`"
            )));
        }
        if !seen.insert(blueprint.as_str()) {
            return Err(usage(format!(
                "{named} {blueprint} twice: each is given once"
            )));
        }
        if !repositories.contains(blueprint) {
            let fix = if asked.blueprints.is_empty() {
                ", so `--blueprints` is given"
            } else {
                ""
            };
            return Err(usage(format!(
                "{named} {blueprint}, which is no repository of the drive{fix}: the drive \
                 keeps {}",
                repositories.join(", ")
            )));
        }
    }

    let mut runs: BTreeMap<&str, String> = ROLES
        .iter()
        .map(|role| {
            let how = held
                .runs
                .get(*role)
                .filter(|how| RUNS.contains(&how.as_str()))
                .map_or(STANDING, String::as_str);
            (*role, how.to_string())
        })
        .collect();
    let mut given = BTreeSet::new();
    for (role, how) in &asked.runs {
        let Some(role) = ROLES.iter().copied().find(|known| known == role) else {
            return Err(usage(format!(
                "`--runs` names {role:?}, a role crew does not have: its roles are {}",
                roles_listed()
            )));
        };
        if !RUNS.contains(&how.as_str()) {
            return Err(usage(format!(
                "`--runs` gives the {role} {how:?}: a role runs `standing` or `on-demand`"
            )));
        }
        if !given.insert(role) {
            return Err(usage(format!(
                "`--runs` names the {role} twice: each role is given once"
            )));
        }
        runs.insert(role, how.clone());
    }

    let (changes, hours) = asked
        .review
        .clone()
        .or_else(|| held.review.clone())
        .unwrap_or_else(|| (REVIEW.0.to_string(), REVIEW.1.to_string()));
    let whole = !changes.is_empty() && changes.chars().all(|c| c.is_ascii_digit());
    if !whole || changes.parse::<u64>().map_or(true, |n| n < 1) {
        return Err(usage(format!(
            "`--review` gives {changes:?} changes: its changes, how many of the drive's changes \
             waiting for a read make a batch due, are a whole number of at least 1"
        )));
    }
    if !is_hours(&hours) {
        return Err(usage(format!(
            "`--review` gives {hours:?} hours: its hours, how many hours since the drive's last \
             batch make one due, are a number above 0"
        )));
    }

    Ok(Settled {
        state_repository,
        blueprints,
        runs: ROLES
            .iter()
            .map(|role| (role.to_string(), runs[role].clone()))
            .collect(),
        review: (changes, hours),
    })
}

/// Whether `text` is an address, `owner/name`, as a state repository's.
fn in_form(text: &str) -> bool {
    StateRepository::parse(text).is_ok()
}

/// Whether `text` is a number above 0, digits with at most one point
/// between digits, as crew's `--review` takes its hours.
fn is_hours(text: &str) -> bool {
    let (whole, part) = text.split_once('.').unwrap_or((text, "0"));
    let digits = |s: &str| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit());
    digits(whole) && digits(part) && text.parse::<f64>().is_ok_and(|n| n > 0.0)
}

/// Each of [`ROLES`], quoted, the last after `and`.
fn roles_listed() -> String {
    let quoted: Vec<String> = ROLES.iter().map(|role| format!("`{role}`")).collect();
    let (last, rest) = quoted.split_last().expect("five roles");
    format!("{} and {last}", rest.join(", "))
}

/// A refusal of the answers, the host's to fix.
fn usage(words: String) -> Refusal {
    Refusal {
        words,
        host_to_fix: true,
    }
}

/// A tool's refusal, carried with its words.
fn refused(words: String) -> Refusal {
    Refusal {
        words,
        host_to_fix: false,
    }
}

/// `crew drive set`'s arguments for the drive `drive` of the account
/// `account`, on the host `host`, its session's folder `folder`, keeping
/// `kept` (design Decision 4; crew record 0065): `drive set <drive>
/// --json`, then `--state`, each `--blueprints`, `--machine`, `--hosts
/// <host> <host> darwin`, `--session <drive>`, `--folder`, each `--repos`,
/// `--accounts <role> <account>` for each role, `--runs <role> <how>` for
/// each role, `--review`, and `--repositories <repository> <dir>
/// <checkout>` for each kept repository. No `--flume`, `--roles` or
/// `--console` is ever given.
pub fn drive_set(
    settled: &Settled,
    drive: &str,
    account: &str,
    host: &str,
    folder: &Path,
    kept: &[Kept],
) -> Vec<OsString> {
    let mut args: Vec<OsString> = Vec::new();
    let mut put = |words: &[&dyn AsRef<std::ffi::OsStr>]| {
        args.extend(words.iter().map(|word| word.as_ref().to_os_string()));
    };
    put(&[&"drive", &"set", &drive, &"--json"]);
    put(&[&"--state", &settled.state_repository.as_str()]);
    for blueprint in &settled.blueprints {
        put(&[&"--blueprints", &blueprint.as_str()]);
    }
    put(&[&"--machine", &host]);
    put(&[&"--hosts", &host, &host, &"darwin"]);
    put(&[&"--session", &drive]);
    put(&[&"--folder", &folder]);
    for repository in kept {
        put(&[&"--repos", &repository.repository.as_str()]);
    }
    for role in ROLES {
        put(&[&"--accounts", &role, &account]);
    }
    for (role, how) in &settled.runs {
        put(&[&"--runs", &role.as_str(), &how.as_str()]);
    }
    put(&[
        &"--review",
        &settled.review.0.as_str(),
        &settled.review.1.as_str(),
    ]);
    for repository in kept {
        put(&[
            &"--repositories",
            &repository.repository.as_str(),
            &repository.dir.as_path(),
            &repository.checkout.as_path(),
        ]);
    }
    args
}

/// The programs crew's side runs, `hostname`, `gh` and `crew`, found by
/// `find`, and this host's name as `hostname -s` gives it. Refused as the
/// host's to fix: each program not found, named, `crew` with where the set
/// gives it; and a name crew does not take, named, with the command that
/// sets one.
pub fn prepare(find: &dyn Fn(&str) -> Option<PathBuf>) -> Result<Prepared, Refusal> {
    let wanted = [
        (
            "hostname",
            "the system's own, which names this host to crew",
        ),
        (
            "gh",
            "GitHub's command line, which makes the state repository",
        ),
        ("crew", CREW_FROM),
    ];
    let mut found = Vec::new();
    let mut missing = Vec::new();
    for (program, from) in wanted {
        match find(program) {
            Some(path) => found.push(path),
            None => missing.push(format!("`{program}` ({from})")),
        }
    }
    if !missing.is_empty() {
        return Err(usage(format!(
            "{} not on setup's PATH",
            if missing.len() == 1 {
                format!("{} is", missing[0])
            } else {
                format!("{} are", missing.join(", "))
            }
        )));
    }
    let [hostname, gh, crew] = <[PathBuf; 3]>::try_from(found).expect("three found");
    let output = run(&hostname, &["-s"]).map_err(usage)?;
    if !output.status.success() {
        return Err(usage(format!(
            "`hostname -s` exited {}: {}",
            exit_of(&output),
            words_of(&output)
        )));
    }
    let host = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !is_name(&host) {
        return Err(usage(format!(
            "this host's name, as `hostname -s` gives it, is {host:?}, which crew does not \
             take: crew names a host in lower-case letters, digits and `-`; set one with \
             `sudo scutil --set HostName <name>`"
        )));
    }
    Ok(Prepared { host, gh, crew })
}

/// Sets up the drive `drive` of the account `account` (design Decisions 4
/// and 5): the state repository read by `gh repo view`, and made private
/// by `gh repo create` where gh cannot resolve it; then `crew drive set`
/// with [`drive_set`]'s arguments, its session's folder `folder`, keeping
/// `kept`. `table` is crew's table of the runner's file, where it has one.
/// gh's or crew's exit other than 0 is refused with its words, as the
/// tool's. Gives crew's table with the drive's partition set, every other
/// kept in its order, and this host; and the answer, holding the state
/// repository and whether it was made or there, crew's answer as crew
/// printed it, and `start`, `crew main up <drive>`.
pub fn set_up(
    prepared: &Prepared,
    settled: &Settled,
    drive: &str,
    account: &str,
    folder: &Path,
    kept: &[Kept],
    table: Option<&toml::Table>,
) -> Result<SetUp, Refusal> {
    let state = settled.state_repository.as_str();
    let tool = |words: String| refused(words);
    let viewed = run(&prepared.gh, &["repo", "view", state, "--json", "name"]).map_err(tool)?;
    let outcome = if viewed.status.success() {
        "there"
    } else if words_of(&viewed).contains(NOT_THERE) {
        let made = run(&prepared.gh, &["repo", "create", state, "--private"]).map_err(tool)?;
        if !made.status.success() {
            return Err(tool(format!(
                "`gh repo create {state} --private` exited {}: {}",
                exit_of(&made),
                words_of(&made)
            )));
        }
        "made"
    } else {
        return Err(tool(format!(
            "`gh repo view {state} --json name` exited {}: {}",
            exit_of(&viewed),
            words_of(&viewed)
        )));
    };

    let args = drive_set(settled, drive, account, &prepared.host, folder, kept);
    let crewed = run(&prepared.crew, &args).map_err(tool)?;
    if !crewed.status.success() {
        return Err(tool(format!(
            "`crew drive set {drive}` exited {}: {}",
            exit_of(&crewed),
            words_of(&crewed)
        )));
    }
    let printed = String::from_utf8_lossy(&crewed.stdout).trim().to_string();
    let crews = serde_json::from_str::<Value>(&printed).unwrap_or(Value::String(printed.clone()));
    let start = format!("crew main up {drive}");
    let said = if outcome == "made" {
        "made, private"
    } else {
        "there"
    };
    Ok(SetUp {
        table: table_with(table, &prepared.host, drive, state),
        answer: json!({
            "state_repository": {"repository": state, "outcome": outcome},
            "drive": crews,
            "start": start,
        }),
        lines: vec![
            format!("state repository {state}: {said}"),
            format!("crew drive set {drive}: {printed}"),
        ],
        start,
    })
}

/// crew's table `table`, or a new one, with `host` and the partition
/// `drive` at `state`, in its place where the table names it and else
/// after every other.
fn table_with(table: Option<&toml::Table>, host: &str, drive: &str, state: &str) -> toml::Table {
    let mut out = table.cloned().unwrap_or_default();
    out.insert("host".to_string(), toml::Value::String(host.to_string()));
    let mut partitions = match out.get("partitions") {
        Some(toml::Value::Array(partitions)) => partitions.clone(),
        _ => Vec::new(),
    };
    let mut entry = toml::Table::new();
    entry.insert("label".to_string(), toml::Value::String(drive.to_string()));
    entry.insert(
        "state_repository".to_string(),
        toml::Value::String(state.to_string()),
    );
    let at = partitions
        .iter()
        .position(|p| p.get("label").and_then(toml::Value::as_str) == Some(drive));
    match at {
        Some(at) => partitions[at] = toml::Value::Table(entry),
        None => partitions.push(toml::Value::Table(entry)),
    }
    out.insert("partitions".to_string(), toml::Value::Array(partitions));
    out
}

/// `program` run with `args`, standard input closed, its output captured;
/// or words saying why it could not be run.
fn run<S: AsRef<std::ffi::OsStr>>(program: &Path, args: &[S]) -> Result<Output, String> {
    Command::new(program)
        .args(args)
        .stdin(Stdio::null())
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

#[cfg(test)]
mod tests {
    use super::*;

    pub(super) fn texts(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| item.to_string()).collect()
    }

    fn asked_state() -> Asked {
        Asked {
            state_repository: Some("example/state".to_string()),
            ..Asked::default()
        }
    }

    /// Each role with `how`, but `except`'s with its own.
    pub(super) fn runs_all(how: &str, except: &[(&str, &str)]) -> Vec<(String, String)> {
        ROLES
            .iter()
            .map(|role| {
                let own = except.iter().find(|(r, _)| r == role).map(|(_, h)| *h);
                (role.to_string(), own.unwrap_or(how).to_string())
            })
            .collect()
    }

    /// crew's file naming the drive `home` with its blueprints, runs and
    /// review, beside the drive `work`, at `version`.
    fn teams(dir: &Path, version: u64, runs: &[(&str, &str)], review: (u64, f64)) -> PathBuf {
        let file = dir.join("teams.json");
        let runs: serde_json::Map<String, Value> = runs
            .iter()
            .map(|(role, how)| (role.to_string(), json!(how)))
            .collect();
        let data = json!({
            "version": version,
            "drives": [
                {"label": "home", "blueprints": ["agentplot/sluice"], "runs": runs,
                 "review": {"after_changes": review.0, "after_hours": review.1}},
                {"label": "work", "blueprints": ["example/app"],
                 "runs": {"design": "on-demand"},
                 "review": {"after_changes": 9, "after_hours": 9}},
            ],
        });
        std::fs::write(&file, data.to_string()).unwrap();
        file
    }

    /// crew's table naming the partitions `work` and `home`.
    fn table() -> toml::Table {
        toml::from_str(
            "host = \"host-a\"\n\
             [[partitions]]\nlabel = \"work\"\nstate_repository = \"example/work\"\n\
             [[partitions]]\nlabel = \"home\"\nstate_repository = \"example/state\"\n",
        )
        .unwrap()
    }

    /// The setup spec's "The first setup's own answers".
    #[test]
    fn the_first_setups_own_answers() {
        let dir = tempfile::tempdir().unwrap();
        let held = held(None, &dir.path().join("teams.json"), "home");
        assert_eq!(held, Held::default());
        let settled = settle(
            &asked_state(),
            &held,
            &texts(&["agentplot/flume", "agentplot/sluice"]),
        )
        .unwrap();
        assert_eq!(
            settled,
            Settled {
                state_repository: "example/state".to_string(),
                blueprints: texts(&["agentplot/flume"]),
                runs: runs_all("standing", &[]),
                review: ("5".to_string(), "4".to_string()),
            }
        );
    }

    /// The setup spec's "A later setup keeps what crew's file holds".
    #[test]
    fn a_later_setup_keeps_what_crews_file_holds() {
        let dir = tempfile::tempdir().unwrap();
        let mut runs: Vec<(&str, &str)> = ROLES.iter().map(|role| (*role, "standing")).collect();
        runs[1].1 = "on-demand";
        let file = teams(dir.path(), 6, &runs, (3, 2.0));
        let held = held(Some(&table()), &file, "home");
        assert_eq!(held.state_repository.as_deref(), Some("example/state"));
        assert_eq!(held.review, Some(("3".to_string(), "2".to_string())));
        let settled = settle(
            &Asked::default(),
            &held,
            &texts(&["agentplot/flume", "agentplot/sluice", "agentplot/crew"]),
        )
        .unwrap();
        assert_eq!(
            settled,
            Settled {
                state_repository: "example/state".to_string(),
                blueprints: texts(&["agentplot/sluice"]),
                runs: runs_all("standing", &[("planner", "on-demand")]),
                review: ("3".to_string(), "2".to_string()),
            }
        );

        // A file at another version, or one that names no such drive,
        // gives none of its own.
        let other = teams(dir.path(), 5, &runs, (3, 2.0));
        let at_five = super::held(Some(&table()), &other, "home");
        assert_eq!(
            at_five,
            Held {
                state_repository: Some("example/state".to_string()),
                ..Held::default()
            }
        );
        let unnamed = super::held(None, &teams(dir.path(), 6, &runs, (3, 2.5)), "garden");
        assert_eq!(unnamed, Held::default());
        let halves = super::held(None, &teams(dir.path(), 6, &runs, (3, 2.5)), "home");
        assert_eq!(halves.review, Some(("3".to_string(), "2.5".to_string())));
    }

    /// The setup spec's "One role changed".
    #[test]
    fn one_role_changed() {
        let dir = tempfile::tempdir().unwrap();
        let runs: Vec<(&str, &str)> = ROLES.iter().map(|role| (*role, "standing")).collect();
        let file = teams(dir.path(), 6, &runs, (5, 4.0));
        let held = held(Some(&table()), &file, "home");
        let asked = Asked {
            runs: vec![("ops".to_string(), "on-demand".to_string())],
            ..Asked::default()
        };
        let settled = settle(&asked, &held, &texts(&["agentplot/sluice"])).unwrap();
        assert_eq!(settled.runs, runs_all("standing", &[("ops", "on-demand")]));
    }

    /// The setup spec's "A blueprints repository the drive does not keep".
    #[test]
    fn a_blueprints_repository_the_drive_does_not_keep() {
        let asked = Asked {
            blueprints: texts(&["agentplot/sluice"]),
            ..asked_state()
        };
        let refusal = settle(&asked, &Held::default(), &texts(&["agentplot/flume"])).unwrap_err();
        assert!(refusal.host_to_fix);
        for words in [
            "`--blueprints`",
            "agentplot/sluice",
            "no repository of the drive",
        ] {
            assert!(refusal.words.contains(words), "{}", refusal.words);
        }
    }

    /// The setup spec's "A role crew does not have", and a way of running
    /// crew does not take.
    #[test]
    fn a_role_crew_does_not_have() {
        let asked = Asked {
            runs: vec![("main-ops".to_string(), "standing".to_string())],
            ..asked_state()
        };
        let refusal = settle(&asked, &Held::default(), &texts(&["agentplot/flume"])).unwrap_err();
        assert!(refusal.host_to_fix);
        assert!(refusal.words.contains("main-ops"), "{}", refusal.words);
        for role in ROLES {
            assert!(
                refusal.words.contains(&format!("`{role}`")),
                "{}",
                refusal.words
            );
        }
        let asked = Asked {
            runs: vec![("ops".to_string(), "sometimes".to_string())],
            ..asked_state()
        };
        let refusal = settle(&asked, &Held::default(), &texts(&["agentplot/flume"])).unwrap_err();
        assert!(refusal.words.contains("\"sometimes\""), "{}", refusal.words);
        assert!(refusal.words.contains("`on-demand`"), "{}", refusal.words);
    }

    /// The setup spec's "A review of nothing", and hours of nothing.
    #[test]
    fn a_review_of_nothing() {
        let review = |changes: &str, hours: &str| Asked {
            review: Some((changes.to_string(), hours.to_string())),
            ..asked_state()
        };
        let refusal = settle(
            &review("0", "4"),
            &Held::default(),
            &texts(&["agentplot/flume"]),
        )
        .unwrap_err();
        assert!(refusal.host_to_fix);
        assert!(refusal.words.contains("`--review`"), "{}", refusal.words);
        assert!(
            refusal.words.contains("a whole number of at least 1"),
            "{}",
            refusal.words
        );
        for hours in ["0", "0.0", "-1", "four", "1.", ".5"] {
            let refusal = settle(
                &review("5", hours),
                &Held::default(),
                &texts(&["agentplot/flume"]),
            )
            .unwrap_err();
            assert!(refusal.words.contains("a number above 0"), "{hours}");
        }
        let halves = settle(
            &review("5", "0.5"),
            &Held::default(),
            &texts(&["agentplot/flume"]),
        )
        .unwrap();
        assert_eq!(halves.review, ("5".to_string(), "0.5".to_string()));
    }
}

/// `drive_set`'s tests, named by it (task 3.2).
#[cfg(test)]
mod drive_set {
    use super::tests::{runs_all, texts};
    use super::*;

    /// The drive `home` keeping `agentplot/flume` and
    /// `agentplot/sluice` under the home `/h`.
    fn kept() -> Vec<Kept> {
        ["agentplot/flume", "agentplot/sluice"]
            .iter()
            .map(|repository| Kept {
                repository: repository.to_string(),
                dir: Path::new("/h/.local/share/boomdrive/repositories").join(repository),
                checkout: Path::new("/h/Code").join(repository),
            })
            .collect()
    }

    fn words(args: &[OsString]) -> Vec<&str> {
        args.iter().map(|arg| arg.to_str().unwrap()).collect()
    }

    /// The setup spec's "crew's flags".
    #[test]
    fn crews_flags_are_the_drives_answers() {
        let settled = Settled {
            state_repository: "example/state".to_string(),
            blueprints: texts(&["agentplot/flume"]),
            runs: runs_all("standing", &[]),
            review: ("5".to_string(), "4".to_string()),
        };
        let args = drive_set(
            &settled,
            "home",
            "own",
            "host-a",
            Path::new("/h/Code"),
            &kept(),
        );
        let mut want = vec![
            "drive",
            "set",
            "home",
            "--json",
            "--state",
            "example/state",
            "--blueprints",
            "agentplot/flume",
            "--machine",
            "host-a",
            "--hosts",
            "host-a",
            "host-a",
            "darwin",
            "--session",
            "home",
            "--folder",
            "/h/Code",
            "--repos",
            "agentplot/flume",
            "--repos",
            "agentplot/sluice",
        ];
        for role in ROLES {
            want.extend(["--accounts", role, "own"]);
        }
        for role in ROLES {
            want.extend(["--runs", role, "standing"]);
        }
        want.extend([
            "--review",
            "5",
            "4",
            "--repositories",
            "agentplot/flume",
            "/h/.local/share/boomdrive/repositories/agentplot/flume",
            "/h/Code/agentplot/flume",
            "--repositories",
            "agentplot/sluice",
            "/h/.local/share/boomdrive/repositories/agentplot/sluice",
            "/h/Code/agentplot/sluice",
        ]);
        assert_eq!(words(&args), want);
    }

    /// The setup spec's "No --flume for any role", over every role run
    /// either way.
    #[test]
    fn no_flume_is_ever_passed() {
        for role in ROLES {
            for how in RUNS {
                let other = if how == "standing" {
                    "on-demand"
                } else {
                    "standing"
                };
                let settled = Settled {
                    state_repository: "example/state".to_string(),
                    blueprints: texts(&["agentplot/flume"]),
                    runs: runs_all(other, &[(role, how)]),
                    review: ("5".to_string(), "4".to_string()),
                };
                let args = drive_set(
                    &settled,
                    "home",
                    "own",
                    "host-a",
                    Path::new("/h/Code"),
                    &kept(),
                );
                let args = words(&args);
                for flag in ["--flume", "--roles", "--console"] {
                    assert!(!args.contains(&flag), "{role} {how}: {args:?}");
                }
                let at = args
                    .windows(3)
                    .position(|w| w == ["--runs", role, how])
                    .is_some();
                assert!(at, "{role} {how}: {args:?}");
            }
        }
    }
}
