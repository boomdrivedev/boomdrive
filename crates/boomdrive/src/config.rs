//! The runner's configuration (design 2): one TOML file naming the
//! accounts a host serves, each account's drives and each drive's
//! repositories by their GitHub address (record 0028; core.5), the root
//! their views are kept under, how often they are fetched, whether each
//! drive has a VS Code workspace, and the control plane whose work it
//! carries, each key named for what it holds (core.12). A refusal names the file, the line and the key or entry. It
//! is built from what is wrong and never from the file's text around it,
//! a value put there by mistake among it (core.4).

use serde::de::IgnoredAny;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
use toml::de::{DeString, DeTable, DeValue};
use toml::{Spanned, Table, Value};

/// The configuration as a pass reads it.
#[derive(Clone, Debug, PartialEq)]
pub struct Config {
    /// The root each repository's view is kept under, at
    /// `<repository_root>/<owner>/<name>`.
    pub repository_root: PathBuf,
    /// How often every repository is fetched.
    pub fetch_interval: Duration,
    /// Whether each owner of the kept repositories has a VS Code
    /// workspace beside its views,
    /// `<repository_root>/<owner>/<owner>.code-workspace` (core.43): false
    /// where the file names none.
    pub code_workspaces: bool,
    /// Each account the host serves, in the file's order, its drives
    /// holding the repositories it keeps (record 0028).
    pub accounts: Vec<Account>,
    /// `[control_plane]`, where the file holds it. Read at every pass, it
    /// is taken when the runner starts (design Decision 4).
    pub control_plane: Option<ControlPlaneTable>,
}

impl Config {
    /// Each repository the host keeps, every drive's in the file's order:
    /// its accounts in turn, each account's drives in turn, each drive's
    /// repositories in turn (design Decision 1).
    pub fn repositories(&self) -> impl Iterator<Item = &Address> {
        self.drives().flat_map(|drive| &drive.repositories)
    }

    /// Each drive of every account, in the file's order.
    pub fn drives(&self) -> impl Iterator<Item = &Drive> {
        self.accounts.iter().flat_map(|account| &account.drives)
    }

    /// The drive that names `address`, compared without case as an
    /// address is.
    pub fn drive_of(&self, address: &Address) -> Option<&Drive> {
        self.drives()
            .find(|drive| drive.repositories.contains(address))
    }
}

/// An account the host serves, `[accounts.<name>]`: what a runner enrolls
/// with a control plane, and its drives (record 0028).
#[derive(Clone, Debug, PartialEq)]
pub struct Account {
    /// The account's name, as the file's key writes it.
    pub name: String,
    /// Each of the account's drives, in the file's order.
    pub drives: Vec<Drive>,
}

/// One account's body of work, `[accounts.<account>.drives.<name>]`: the
/// repositories it keeps (record 0028). Its stream and its branch of the
/// account's crew state are other tools' configuration, never the
/// runner's (core.5).
#[derive(Clone, Debug, PartialEq)]
pub struct Drive {
    /// The drive's name, as the file's key writes it.
    pub name: String,
    /// Each repository the drive keeps, in the file's order.
    pub repositories: Vec<Address>,
}

/// `[control_plane]` (design Decision 4): the implementation the runner
/// opens, how often it makes a control pass, and the implementation's own
/// table.
#[derive(Clone, Debug, PartialEq)]
pub struct ControlPlaneTable {
    /// `type`: the word of the implementation to open.
    pub implementation: String,
    /// How often a control pass is made.
    pub poll_interval: Duration,
    /// The table `type` names, `[control_plane.<type>]`, handed whole to
    /// the implementation, whose keys are its own (core.2): empty where
    /// the file holds none.
    pub implementation_table: Table,
}

/// The fetch interval where the file names none.
const FETCH_INTERVAL: Duration = Duration::from_secs(2 * 60);

/// The poll interval where `[control_plane]` names none.
const POLL_INTERVAL: Duration = Duration::from_secs(60);

/// Keys the runner does not read that a host's file may hold, each with
/// what its refusal adds: the key that holds it now (core.12), or whose
/// configuration it is (core.5).
const HELD_ELSEWHERE: [(&str, &str); 7] = [
    ("views", "`repository_root` holds it now"),
    ("fetch_every", "`fetch_interval` holds it now"),
    ("stream", "streams are Flume's host configuration"),
    ("streams", "streams are Flume's host configuration"),
    ("kind", "kinds are Flume's host configuration"),
    ("kinds", "kinds are Flume's host configuration"),
    ("kinds_file", "kinds are Flume's host configuration"),
];

/// A repository's GitHub address, `owner/name`, kept as written and
/// compared without case, as GitHub compares it.
#[derive(Clone, Debug)]
pub struct Address {
    owner: String,
    name: String,
}

impl Address {
    /// The address `text` writes, or the rule it breaks.
    pub fn parse(text: &str) -> Result<Address, &'static str> {
        let (owner, name) = text.split_once('/').ok_or("an address is `owner/name`")?;
        let inner_hyphens = !owner.starts_with('-') && !owner.ends_with('-');
        if owner.is_empty()
            || !inner_hyphens
            || !owner.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return Err("an owner is letters, digits and inner hyphens");
        }
        if name.is_empty()
            || name == "."
            || name == ".."
            || name.to_ascii_lowercase().ends_with(".git")
            || !name
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        {
            return Err("a name is letters, digits, `-`, `_` and `.`, \
                        and is not `.` or `..` and does not end in `.git`");
        }
        Ok(Address {
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }

    /// The owner, as written.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// The repository's name, as written.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        self.owner.eq_ignore_ascii_case(&other.owner) && self.name.eq_ignore_ascii_case(&other.name)
    }
}

impl Eq for Address {}

impl Hash for Address {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.owner.to_ascii_lowercase().hash(state);
        self.name.to_ascii_lowercase().hash(state);
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

/// How often something is done: `<n>s`, `<n>m` or `<n>h`, with `n` above
/// zero, the form flume's `--stall-after` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Interval(Duration);

impl Interval {
    /// The interval `text` writes, or none where it is not one.
    pub fn parse(text: &str) -> Option<Interval> {
        let (count, unit) = [("s", 1), ("m", 60), ("h", 60 * 60)]
            .into_iter()
            .find_map(|(suffix, unit)| Some((text.strip_suffix(suffix)?, unit)))?;
        if count.is_empty() || !count.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let seconds = count.parse::<u64>().ok()?.checked_mul(unit)?;
        (seconds > 0).then(|| Interval(Duration::from_secs(seconds)))
    }

    /// The interval as a duration.
    pub fn duration(self) -> Duration {
        self.0
    }
}

/// The file as written, each value with where it stands. A key not named
/// here is refused, so a misspelt key never silently does nothing and the
/// file has no place for a credential (core.4).
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    repository_root: Option<Spanned<Value>>,
    fetch_interval: Option<Spanned<Value>>,
    code_workspaces: Option<Spanned<Value>>,
    /// Read from the document's own tree, which keeps where each key of
    /// it stands (`accounts`).
    accounts: Option<IgnoredAny>,
    #[serde(default, deserialize_with = "table")]
    control_plane: Option<Keys>,
}

/// A table as written, each key and value with where it stands.
type Keys = BTreeMap<Spanned<String>, Spanned<Value>>;

/// `control_plane`, refused by its key where it is not a table.
fn table<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Keys>, D::Error> {
    Option::deserialize(deserializer)
        .map_err(|_| serde::de::Error::custom("`control_plane` is not a table"))
}

/// Reads the configuration file `file`, with `~` as `home`. A file that
/// cannot be read is refused, naming it.
pub fn read(file: &Path, home: &Path) -> Result<Config, String> {
    let text =
        std::fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
    parse(&text, file, home)
}

/// Reads the configuration in `text`, the file `file`'s, with `~` as
/// `home`.
pub fn parse(text: &str, file: &Path, home: &Path) -> Result<Config, String> {
    let line = |at: usize| {
        text.as_bytes()[..at.min(text.len())]
            .iter()
            .filter(|&&b| b == b'\n')
            .count()
            + 1
    };
    let refuse = |at: usize, words: String| format!("{}:{}: {words}", file.display(), line(at));
    let broken = |error: toml::de::Error| match error.span() {
        Some(span) => refuse(span.start, error.message().to_string()),
        None => format!("{}: {}", file.display(), error.message()),
    };

    // Each key held elsewhere is named at once, with where it is held, so
    // a file written for an earlier runner says all it must change.
    let keys: BTreeMap<Spanned<String>, IgnoredAny> = toml::from_str(text).map_err(broken)?;
    let mut held: Vec<(&Spanned<String>, &str)> = keys
        .keys()
        .filter_map(|key| {
            let (_, held) = HELD_ELSEWHERE
                .iter()
                .find(|(name, _)| key.get_ref() == name)?;
            Some((key, *held))
        })
        .collect();
    held.sort_by_key(|(key, _)| key.span().start);
    if let Some(((first, held_first), rest)) = held.split_first() {
        let mut words = format!("unknown field `{}`: {held_first}", first.get_ref());
        for (key, held) in rest {
            words += &format!(
                "; unknown field `{}`, on line {}: {held}",
                key.get_ref(),
                line(key.span().start)
            );
        }
        return Err(refuse(first.span().start, words));
    }

    let raw: File = toml::from_str(text).map_err(broken)?;

    let repository_root = match &raw.repository_root {
        None => home.join("Code"),
        Some(root) => root
            .get_ref()
            .as_str()
            .and_then(|root| root_path(root, home))
            .ok_or_else(|| {
                refuse(
                    root.span().start,
                    format!(
                        "`repository_root` is {}: a repository root is an absolute path or \
                         begins with `~/`",
                        named(root.get_ref())
                    ),
                )
            })?,
    };

    let fetch_interval = match &raw.fetch_interval {
        None => FETCH_INTERVAL,
        Some(interval) => interval
            .get_ref()
            .as_str()
            .and_then(Interval::parse)
            .map(Interval::duration)
            .ok_or_else(|| {
                refuse(
                    interval.span().start,
                    format!(
                        "`fetch_interval` is {}: an interval is `<n>s`, `<n>m` or `<n>h`, \
                         with n above zero",
                        named(interval.get_ref())
                    ),
                )
            })?,
    };

    let code_workspaces = match &raw.code_workspaces {
        None => false,
        Some(asked) => asked.get_ref().as_bool().ok_or_else(|| {
            refuse(
                asked.span().start,
                format!(
                    "`code_workspaces` is {}: it is `true` or `false`",
                    named(asked.get_ref())
                ),
            )
        })?,
    };

    if raw.accounts.is_none() {
        return Err(format!(
            "{}: `accounts` is required: the accounts this host serves, each naming its \
             drives and each drive its repositories, `accounts = {{}}` for none",
            file.display()
        ));
    }
    let document = DeTable::parse(text).map_err(broken)?;
    let accounts = accounts(document.get_ref(), &refuse, &line)?;

    let control_plane = match &raw.control_plane {
        None => None,
        Some(table) => {
            let at = keys
                .keys()
                .find(|key| key.get_ref() == "control_plane")
                .map_or(0, |key| key.span().start);
            Some(control_plane(table, at, &refuse)?)
        }
    };

    Ok(Config {
        repository_root,
        fetch_interval,
        code_workspaces,
        accounts,
        control_plane,
    })
}

/// A table of the document as written, each key and value with where it
/// stands.
type Tree<'i> = DeTable<'i>;

/// A table's keys with their values, in the order they stand in the file.
fn in_order<'t, 'i>(
    table: &'t Tree<'i>,
) -> Vec<(&'t Spanned<DeString<'i>>, &'t Spanned<DeValue<'i>>)> {
    let mut keys: Vec<_> = table.iter().collect();
    keys.sort_by_key(|(key, _)| key.span().start);
    keys
}

/// The table `value` holds, refused by `path`, its key standing at `at`,
/// where it holds something else.
fn table_at<'t, 'i>(
    value: &'t Spanned<DeValue<'i>>,
    path: &str,
    at: usize,
    refuse: &impl Fn(usize, String) -> String,
) -> Result<&'t Tree<'i>, String> {
    match value.get_ref() {
        DeValue::Table(table) => Ok(table),
        _ => Err(refuse(at, format!("`{path}` is not a table"))),
    }
}

/// The value of `name` in `table`, with its key, where it holds one.
fn entry<'t, 'i>(
    table: &'t Tree<'i>,
    name: &str,
) -> Option<(&'t Spanned<DeString<'i>>, &'t Spanned<DeValue<'i>>)> {
    table.iter().find(|(key, _)| key.get_ref() == name)
}

/// `accounts` as the document holds it, `document` (design Decisions 1
/// and 2): each account with its drives and each drive with its
/// repositories, in the file's order. Each refusal is made by `refuse`,
/// naming its key by its path, and `line` gives the line a place stands
/// on.
fn accounts(
    document: &Tree<'_>,
    refuse: &impl Fn(usize, String) -> String,
    line: &impl Fn(usize) -> usize,
) -> Result<Vec<Account>, String> {
    let Some((key, value)) = entry(document, "accounts") else {
        return Ok(Vec::new());
    };
    let table = table_at(value, "accounts", key.span().start, refuse)?;
    let mut accounts = Vec::new();
    for (name, value) in in_order(table) {
        let at = name.span().start;
        let path = format!("accounts.{}", name.get_ref());
        let account = table_at(value, &path, at, refuse)?;
        let Some((drives_key, drives)) = entry(account, "drives") else {
            return Err(refuse(
                at,
                format!(
                    "`{path}.drives` is required: the account's drives, `drives = {{}}` for none"
                ),
            ));
        };
        let drives = table_at(
            drives,
            &format!("{path}.drives"),
            drives_key.span().start,
            refuse,
        )?;
        let mut kept = Vec::new();
        for (name, value) in in_order(drives) {
            let at = name.span().start;
            let path = format!("{path}.drives.{}", name.get_ref());
            let drive = table_at(value, &path, at, refuse)?;
            kept.push(Drive {
                name: name.get_ref().to_string(),
                repositories: repositories(drive, &path, at, refuse, line)?,
            });
        }
        accounts.push(Account {
            name: path["accounts.".len()..].to_string(),
            drives: kept,
        });
    }
    Ok(accounts)
}

/// The repositories of the drive `drive`, whose key, at `path`, stands at
/// `at`, each an address, in the file's order.
fn repositories(
    drive: &Tree<'_>,
    path: &str,
    at: usize,
    refuse: &impl Fn(usize, String) -> String,
    line: &impl Fn(usize) -> usize,
) -> Result<Vec<Address>, String> {
    let path = format!("{path}.repositories");
    let Some((key, list)) = entry(drive, "repositories") else {
        return Err(refuse(
            at,
            format!("`{path}` is required: the drive's repositories, `repositories = []` for none"),
        ));
    };
    let DeValue::Array(entries) = list.get_ref() else {
        return Err(refuse(key.span().start, format!("`{path}` is not a list")));
    };
    let mut kept: Vec<(Address, &Spanned<DeValue<'_>>)> = Vec::new();
    for entry in entries.iter() {
        let at = entry.span().start;
        let Some(text) = entry.get_ref().as_str() else {
            return Err(refuse(
                at,
                format!("`{path}` entry is not a string: an address is `owner/name`"),
            ));
        };
        let address = Address::parse(text)
            .map_err(|rule| refuse(at, format!("`{path}` entry {text:?}: {rule}")))?;
        if let Some((_, first)) = kept.iter().find(|(kept, _)| *kept == address) {
            return Err(refuse(
                at,
                format!(
                    "`{path}` entries {:?}, on line {}, and {text:?} are one repository: \
                     GitHub compares addresses without case",
                    first.get_ref().as_str().unwrap_or_default(),
                    line(first.span().start),
                ),
            ));
        }
        kept.push((address, entry));
    }
    Ok(kept.into_iter().map(|(address, _)| address).collect())
}

/// `[control_plane]` as written in `table`, whose key stands at `at`,
/// each refusal made by `refuse` (design Decision 4). It holds `type`,
/// `poll_interval` and the table `type` names, and nothing else.
fn control_plane(
    table: &Keys,
    at: usize,
    refuse: &impl Fn(usize, String) -> String,
) -> Result<ControlPlaneTable, String> {
    let find = |name: &str| table.iter().find(|(key, _)| key.get_ref() == name);

    let implementation = match find("type") {
        None => {
            return Err(refuse(
                at,
                "`type` is required in `[control_plane]`: the word of the control plane's \
                 implementation"
                    .to_string(),
            ))
        }
        Some((_, word)) => match word.get_ref().as_str() {
            Some(word) => word.to_string(),
            None => {
                return Err(refuse(
                    word.span().start,
                    "`type` is not a string: it is the word of the control plane's \
                     implementation"
                        .to_string(),
                ))
            }
        },
    };

    let poll_interval = match find("poll_interval") {
        None => POLL_INTERVAL,
        Some((_, interval)) => interval
            .get_ref()
            .as_str()
            .and_then(Interval::parse)
            .map(Interval::duration)
            .ok_or_else(|| {
                refuse(
                    interval.span().start,
                    format!(
                        "`poll_interval` is {}: an interval is `<n>s`, `<n>m` or `<n>h`, \
                         with n above zero",
                        named(interval.get_ref())
                    ),
                )
            })?,
    };

    let read = ["type", "poll_interval", implementation.as_str()];
    if let Some((key, _)) = table
        .iter()
        .filter(|(key, _)| !read.contains(&key.get_ref().as_str()))
        .min_by_key(|(key, _)| key.span().start)
    {
        return Err(refuse(
            key.span().start,
            format!(
                "unknown field `{}` in `[control_plane]`: it holds `type`, `poll_interval` \
                 and the table `type` names",
                key.get_ref()
            ),
        ));
    }

    let implementation_table = match find(&implementation) {
        None => Table::new(),
        Some((key, value)) => match value.get_ref() {
            Value::Table(own) => own.clone(),
            _ => {
                return Err(refuse(
                    key.span().start,
                    format!(
                        "`{implementation}` in `[control_plane]` is not a table: it is \
                         `[control_plane.{implementation}]`, the implementation's own table"
                    ),
                ))
            }
        },
    };

    Ok(ControlPlaneTable {
        implementation,
        poll_interval,
        implementation_table,
    })
}

/// A repository root: an absolute path, or `~/…` beneath `home`. Nothing
/// else is expanded, so `$HOME/Code` is no root.
fn root_path(root: &str, home: &Path) -> Option<PathBuf> {
    match root.strip_prefix("~/") {
        Some(beneath) => Some(home.join(beneath)),
        None => Path::new(root).is_absolute().then(|| PathBuf::from(root)),
    }
}

/// A value as a refusal names it: a string as written, escaped, and
/// anything else by what it is not.
fn named(value: &Value) -> String {
    match value.as_str() {
        Some(text) => format!("{text:?}"),
        None => "not a string".to_string(),
    }
}

/// A repository's view, `<repository_root>/<owner>/<name>` (design 7).
pub fn view_path(config: &Config, address: &Address) -> PathBuf {
    config
        .repository_root
        .join(address.owner())
        .join(address.name())
}

/// What the runner reads of its environment: the home, and the XDG base
/// directories as set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Env {
    /// `HOME`.
    pub home: PathBuf,
    /// `XDG_CONFIG_HOME`, where it is set.
    pub config_home: Option<PathBuf>,
    /// `XDG_DATA_HOME`, where it is set.
    pub data_home: Option<PathBuf>,
}

impl Env {
    /// The environment `var` gives, refused where `HOME` is unset or
    /// empty.
    pub fn read(var: impl Fn(&str) -> Option<OsString>) -> Result<Env, String> {
        let home = var("HOME")
            .filter(|home| !home.is_empty())
            .ok_or("HOME is not set")?;
        Ok(Env {
            home: home.into(),
            config_home: var("XDG_CONFIG_HOME").map(PathBuf::from),
            data_home: var("XDG_DATA_HOME").map(PathBuf::from),
        })
    }
}

/// The configuration file: the one `--config` names, else
/// `$XDG_CONFIG_HOME/boomdrive/config.toml`, else
/// `~/.config/boomdrive/config.toml`.
pub fn config_path(arg: Option<&Path>, env: &Env) -> PathBuf {
    match arg {
        Some(file) => file.to_path_buf(),
        None => xdg(env.config_home.as_deref(), &env.home, ".config")
            .join("boomdrive")
            .join("config.toml"),
    }
}

/// The runner's data directory, `$XDG_DATA_HOME/boomdrive`, else
/// `~/.local/share/boomdrive` (design 3). Where that is not there and the
/// one a runner kept before the rename is, `flywheel` beside it, it is
/// that one: its clones hold the places and views made in them, which
/// moving it would break, and it holds the report (core.14, core.36;
/// record 0016).
pub fn data_dir(env: &Env) -> PathBuf {
    let under = xdg(env.data_home.as_deref(), &env.home, ".local/share");
    let dir = under.join("boomdrive");
    let before = under.join(crate::BEFORE_RENAME);
    if dir.symlink_metadata().is_err() && before.is_dir() {
        return before;
    }
    dir
}

/// An XDG base directory: the variable's, else `under` the home. One that
/// is empty or not absolute is taken as unset, as the XDG base directory
/// specification says.
fn xdg(dir: Option<&Path>, home: &Path, under: &str) -> PathBuf {
    match dir {
        Some(dir) if dir.is_absolute() => dir.to_path_buf(),
        _ => home.join(under),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "/c/config.toml";

    fn read(text: &str) -> Result<Config, String> {
        parse(text, Path::new(FILE), Path::new("/h"))
    }

    fn address(text: &str) -> Address {
        Address::parse(text).unwrap()
    }

    /// `text`'s drives, each named with its account and its
    /// repositories as written, in the order kept.
    fn drives(text: &str) -> Vec<(String, String, Vec<String>)> {
        read(text)
            .unwrap()
            .accounts
            .iter()
            .flat_map(|account| {
                account.drives.iter().map(|drive| {
                    (
                        account.name.clone(),
                        drive.name.clone(),
                        drive.repositories.iter().map(Address::to_string).collect(),
                    )
                })
            })
            .collect()
    }

    /// The configuration spec's "One drive".
    #[test]
    fn one_drive() {
        let text = "[accounts.own.drives.home]\nrepositories = [\"agentplot/flume\"]\n";
        assert_eq!(
            drives(text),
            [(
                "own".to_string(),
                "home".to_string(),
                vec!["agentplot/flume".to_string()]
            )]
        );
        let config = read(text).unwrap();
        let flume = address("agentplot/flume");
        assert_eq!(
            config.drive_of(&flume).map(|drive| drive.name.as_str()),
            Some("home")
        );
        assert_eq!(config.drive_of(&address("agentplot/sluice")), None);
    }

    /// The configuration spec's "Drives of two accounts": every drive's
    /// repositories are kept in the file's order, whatever the order of
    /// the accounts' and drives' names.
    #[test]
    fn drives_of_two_accounts() {
        let config = read(
            "[accounts.own.drives.home]\n\
             repositories = [\"agentplot/flume\", \"afterthought/crew\"]\n\
             \n\
             [accounts.client.drives.garden]\n\
             repositories = [\"agentplot/sluice\"]\n",
        )
        .unwrap();
        assert_eq!(
            config
                .repositories()
                .map(Address::to_string)
                .collect::<Vec<_>>(),
            ["agentplot/flume", "afterthought/crew", "agentplot/sluice"]
        );
        assert_eq!(
            config
                .accounts
                .iter()
                .map(|account| account.name.as_str())
                .collect::<Vec<_>>(),
            ["own", "client"]
        );
        assert_eq!(
            config
                .drive_of(&address("AgentPlot/Sluice"))
                .map(|drive| drive.name.as_str()),
            Some("garden")
        );
        assert_eq!(
            drives(
                "[accounts.own.drives.work]\nrepositories = []\n\
                 [accounts.own.drives.home]\nrepositories = []\n"
            )
            .into_iter()
            .map(|(_, drive, _)| drive)
            .collect::<Vec<_>>(),
            ["work", "home"]
        );
    }

    /// The configuration spec's "No list", which has no `accounts`.
    #[test]
    fn no_accounts() {
        assert_eq!(
            read("repository_root = \"~/Code\"\n"),
            Err(format!(
                "{FILE}: `accounts` is required: the accounts this host serves, each naming \
                 its drives and each drive its repositories, `accounts = {{}}` for none"
            ))
        );
    }

    /// An account with no `drives`.
    #[test]
    fn an_account_with_no_drives() {
        assert_eq!(
            read("accounts = {}\n").map(|config| config.accounts),
            Ok(vec![])
        );
        assert_eq!(
            read("[accounts.own]\n"),
            Err(format!(
                "{FILE}:1: `accounts.own.drives` is required: the account's drives, \
                 `drives = {{}}` for none"
            ))
        );
        assert_eq!(
            drives("[accounts.own]\ndrives = {}\n"),
            Vec::<(String, String, Vec<String>)>::new()
        );
    }

    /// The configuration spec's "A drive with no list".
    #[test]
    fn a_drive_with_no_list() {
        assert_eq!(
            read("repository_root = \"~/Code\"\n\n[accounts.own.drives.home]\n"),
            Err(format!(
                "{FILE}:3: `accounts.own.drives.home.repositories` is required: the drive's \
                 repositories, `repositories = []` for none"
            ))
        );
    }

    /// The configuration spec's "Keeping none".
    #[test]
    fn keeping_none() {
        let config = read("accounts = {}\n").unwrap();
        assert_eq!(config.accounts, vec![]);
        assert_eq!(config.repositories().count(), 0);
        assert_eq!(read("[accounts]\n").unwrap().accounts, vec![]);
    }

    /// The configuration spec's "A drive keeping none".
    #[test]
    fn a_drive_keeping_none() {
        assert_eq!(
            drives("[accounts.own.drives.home]\nrepositories = []\n"),
            [("own".to_string(), "home".to_string(), vec![])]
        );
    }

    /// Each level is a table, and a drive's `repositories` a list of
    /// strings, each refused by its path.
    #[test]
    fn a_level_of_another_kind_is_named_by_its_path() {
        for (text, refusal) in [
            ("accounts = []\n", "1: `accounts` is not a table"),
            (
                "accounts = { own = 3 }\n",
                "1: `accounts.own` is not a table",
            ),
            (
                "[accounts.own]\ndrives = []\n",
                "2: `accounts.own.drives` is not a table",
            ),
            (
                "[[accounts.own.drives.home]]\nrepositories = []\n",
                "1: `accounts.own.drives.home` is not a table",
            ),
            (
                "[accounts.own.drives.home]\nrepositories = \"agentplot/flume\"\n",
                "2: `accounts.own.drives.home.repositories` is not a list",
            ),
            (
                "[accounts.own.drives.home]\nrepositories = [3]\n",
                "2: `accounts.own.drives.home.repositories` entry is not a string: an address \
                 is `owner/name`",
            ),
        ] {
            assert_eq!(read(text), Err(format!("{FILE}:{refusal}")), "{text}");
        }
    }

    #[test]
    fn a_home_relative_root() {
        let config = read("repository_root = \"~/src\"\naccounts = {}\n").unwrap();
        assert_eq!(config.repository_root, Path::new("/h/src"));
    }

    #[test]
    fn a_relative_root() {
        for (root, named) in [("Code", "\"Code\""), ("$HOME/Code", "\"$HOME/Code\"")] {
            assert_eq!(
                read(&format!("accounts = {{}}\nrepository_root = \"{root}\"\n")),
                Err(format!(
                    "{FILE}:2: `repository_root` is {named}: a repository root is an absolute \
                     path or begins with `~/`"
                ))
            );
        }
    }

    #[test]
    fn no_period() {
        assert_eq!(
            read("accounts = {}\n").unwrap().fetch_interval,
            Duration::from_secs(120)
        );
    }

    /// The configuration spec's "No workspaces setting".
    #[test]
    fn no_workspaces_setting() {
        let config =
            read("[accounts.own.drives.home]\nrepositories = [\"agentplot/flume\"]\n").unwrap();
        assert!(!config.code_workspaces);
        let config = read("code_workspaces = false\naccounts = {}\n").unwrap();
        assert!(!config.code_workspaces);
    }

    /// The configuration spec's "Workspaces asked for in words", and a
    /// number refused alike.
    #[test]
    fn workspaces_asked_for_in_words() {
        for (value, named) in [("\"yes\"", "\"yes\""), ("1", "not a string")] {
            assert_eq!(
                read(&format!("accounts = {{}}\ncode_workspaces = {value}\n")),
                Err(format!(
                    "{FILE}:2: `code_workspaces` is {named}: it is `true` or `false`"
                ))
            );
        }
    }

    #[test]
    fn a_period_of_nothing() {
        for interval in ["0m", "soon"] {
            assert_eq!(
                read(&format!(
                    "accounts = {{}}\nfetch_interval = \"{interval}\"\n"
                )),
                Err(format!(
                    "{FILE}:2: `fetch_interval` is \"{interval}\": an interval is `<n>s`, \
                     `<n>m` or `<n>h`, with n above zero"
                ))
            );
        }
    }

    #[test]
    fn a_misspelt_key() {
        let refusal = read("accounts = {}\nrepositories_root = \"~/Code\"\n").unwrap_err();
        assert!(
            refusal.starts_with(&format!("{FILE}:2: unknown field `repositories_root`")),
            "{refusal}"
        );
    }

    #[test]
    fn a_stream_named() {
        assert_eq!(
            read("accounts = {}\nstream = \"work\"\n"),
            Err(format!(
                "{FILE}:2: unknown field `stream`: streams are Flume's host configuration"
            ))
        );
        assert_eq!(
            read("accounts = {}\n\n[streams]\nwork = 2\n"),
            Err(format!(
                "{FILE}:3: unknown field `streams`: streams are Flume's host configuration"
            ))
        );
    }

    #[test]
    fn a_kinds_file_named() {
        assert_eq!(
            read("accounts = {}\nkinds = { crew = \"/k/kinds.toml\" }\n"),
            Err(format!(
                "{FILE}:2: unknown field `kinds`: kinds are Flume's host configuration"
            ))
        );
        for key in ["kind", "kinds_file"] {
            assert_eq!(
                read(&format!("accounts = {{}}\n{key} = \"change\"\n")),
                Err(format!(
                    "{FILE}:2: unknown field `{key}`: kinds are Flume's host configuration"
                ))
            );
        }
    }

    #[test]
    fn a_key_of_before() {
        assert_eq!(
            read("views = \"~/Code\"\nfetch_every = \"2m\"\naccounts = {}\n"),
            Err(format!(
                "{FILE}:1: unknown field `views`: `repository_root` holds it now; \
                 unknown field `fetch_every`, on line 2: `fetch_interval` holds it now"
            ))
        );
        assert_eq!(
            read("accounts = {}\nfetch_every = \"2m\"\n"),
            Err(format!(
                "{FILE}:2: unknown field `fetch_every`: `fetch_interval` holds it now"
            ))
        );
    }

    #[test]
    fn a_key_held_elsewhere_is_named_before_what_else_is_wrong() {
        assert_eq!(
            read("views = 3\n"),
            Err(format!(
                "{FILE}:1: unknown field `views`: `repository_root` holds it now"
            ))
        );
        let refusal = read("accounts = {}\nviews = \"~/Code\"\nghp_n0t4r34lt0k3n\n").unwrap_err();
        assert!(refusal.starts_with(&format!("{FILE}:3: ")), "{refusal}");
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_token() {
        let refusal = read("accounts = {}\ntoken = \"ghp_n0t4r34lt0k3n\"\n").unwrap_err();
        assert!(
            refusal.starts_with(&format!("{FILE}:2: unknown field `token`")),
            "{refusal}"
        );
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_broken_file_is_named_by_its_line_and_never_quoted() {
        let refusal = read("accounts = {}\nghp_n0t4r34lt0k3n\n").unwrap_err();
        assert!(refusal.starts_with(&format!("{FILE}:2: ")), "{refusal}");
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_value_of_another_kind_is_named_by_its_key() {
        assert_eq!(
            read("accounts = {}\nrepository_root = 3\n"),
            Err(format!(
                "{FILE}:2: `repository_root` is not a string: a repository root is an \
                 absolute path or begins with `~/`"
            ))
        );
    }

    #[test]
    fn a_whole_file() {
        assert_eq!(
            read(
                "repository_root = \"/srv/code\"\nfetch_interval = \"90s\"\n\
                 code_workspaces = true\n\
                 \n\
                 [accounts.own.drives.home]\n\
                 repositories = [\"agentplot/flume\", \"afterthought/crew\"]\n"
            ),
            Ok(Config {
                repository_root: PathBuf::from("/srv/code"),
                fetch_interval: Duration::from_secs(90),
                code_workspaces: true,
                accounts: vec![Account {
                    name: "own".to_string(),
                    drives: vec![Drive {
                        name: "home".to_string(),
                        repositories: vec![
                            address("agentplot/flume"),
                            address("afterthought/crew")
                        ],
                    }],
                }],
                control_plane: None,
            })
        );
    }

    /// Design Decision 4's example of `[control_plane]`.
    const CONTROL_PLANE: &str = "accounts = {}

[control_plane]
type = \"crew\"
poll_interval = \"1m\"

[control_plane.crew]
host = \"deneb\"

[[control_plane.crew.partitions]]
label = \"madswan\"
state_repository = \"afterthought/madswan-state\"
";

    /// The configuration spec's "crew named": the table `type` names is
    /// handed on whole.
    #[test]
    fn the_control_plane_named() {
        let table = read(CONTROL_PLANE).unwrap().control_plane.unwrap();
        let own: Table = toml::from_str(
            "host = \"deneb\"\n\n[[partitions]]\nlabel = \"madswan\"\n\
             state_repository = \"afterthought/madswan-state\"\n",
        )
        .unwrap();
        assert_eq!(
            table,
            ControlPlaneTable {
                implementation: "crew".to_string(),
                poll_interval: Duration::from_secs(60),
                implementation_table: own,
            }
        );
    }

    /// The configuration spec's "No control plane".
    #[test]
    fn no_control_plane() {
        assert_eq!(read("accounts = {}\n").unwrap().control_plane, None);
    }

    /// The configuration spec's "No type".
    #[test]
    fn no_type() {
        assert_eq!(
            read("accounts = {}\n\n[control_plane]\npoll_interval = \"30s\"\n"),
            Err(format!(
                "{FILE}:3: `type` is required in `[control_plane]`: the word of the control \
                 plane's implementation"
            ))
        );
        assert_eq!(
            read("accounts = {}\n[control_plane]\ntype = 3\n"),
            Err(format!(
                "{FILE}:3: `type` is not a string: it is the word of the control plane's \
                 implementation"
            ))
        );
    }

    /// The configuration spec's "No poll interval".
    #[test]
    fn no_poll_interval() {
        let table = read("accounts = {}\n[control_plane]\ntype = \"crew\"\n")
            .unwrap()
            .control_plane
            .unwrap();
        assert_eq!(table.poll_interval, Duration::from_secs(60));
        assert_eq!(table.implementation_table, Table::new());
    }

    #[test]
    fn poll_intervals() {
        for (interval, seconds) in [("30s", 30), ("5m", 300), ("1h", 3600)] {
            let table = read(&format!(
                "accounts = {{}}\n[control_plane]\ntype = \"crew\"\n\
                 poll_interval = \"{interval}\"\n"
            ))
            .unwrap()
            .control_plane
            .unwrap();
            assert_eq!(
                table.poll_interval,
                Duration::from_secs(seconds),
                "{interval}"
            );
        }
    }

    /// The configuration spec's "A poll interval of nothing".
    #[test]
    fn a_poll_interval_of_nothing() {
        for (interval, named) in [
            ("\"0s\"", "\"0s\""),
            ("\"soon\"", "\"soon\""),
            ("60", "not a string"),
        ] {
            assert_eq!(
                read(&format!(
                    "accounts = {{}}\n[control_plane]\ntype = \"crew\"\n\
                     poll_interval = {interval}\n"
                )),
                Err(format!(
                    "{FILE}:4: `poll_interval` is {named}: an interval is `<n>s`, `<n>m` or \
                     `<n>h`, with n above zero"
                ))
            );
        }
    }

    /// `[control_plane]` holds `type`, `poll_interval` and the table
    /// `type` names, and a key of anything else is refused, its value in
    /// no output.
    #[test]
    fn a_key_the_control_plane_table_does_not_hold() {
        for (key, line) in [
            ("token = \"ghp_n0t4r34lt0k3n\"", 4),
            ("[control_plane.hosted]\ntoken = \"ghp_n0t4r34lt0k3n\"", 4),
        ] {
            let refusal = read(&format!(
                "accounts = {{}}\n[control_plane]\ntype = \"crew\"\n{key}\n"
            ))
            .unwrap_err();
            let name = if key.starts_with('[') {
                "hosted"
            } else {
                "token"
            };
            assert_eq!(
                refusal,
                format!(
                    "{FILE}:{line}: unknown field `{name}` in `[control_plane]`: it holds \
                     `type`, `poll_interval` and the table `type` names"
                )
            );
        }
    }

    #[test]
    fn a_control_plane_of_another_kind_is_named_by_its_key() {
        assert_eq!(
            read("accounts = {}\ncontrol_plane = \"crew\"\n"),
            Err(format!("{FILE}:2: `control_plane` is not a table"))
        );
        assert_eq!(
            read("accounts = {}\n[control_plane]\ntype = \"crew\"\ncrew = 3\n"),
            Err(format!(
                "{FILE}:4: `crew` in `[control_plane]` is not a table: it is \
                 `[control_plane.crew]`, the implementation's own table"
            ))
        );
    }

    #[test]
    fn addresses() {
        for good in ["a/b", "agent-plot/flu.me_2", "A1/x.y.z"] {
            assert!(Address::parse(good).is_ok(), "{good}");
        }
        for bad in [
            "flume", "a/b/c", "/b", "-a/b", "a-/b", "a_b/c", "a/", "a/.", "a/..", "a/b.git",
            "a/b.GIT", "a/b c",
        ] {
            assert!(Address::parse(bad).is_err(), "{bad}");
        }
        assert_eq!(address("AgentPlot/Flume"), address("agentplot/flume"));
        assert_eq!(address("AgentPlot/Flume").to_string(), "AgentPlot/Flume");
    }

    #[test]
    fn intervals() {
        for (text, seconds) in [("30s", 30), ("2m", 120), ("1h", 3600)] {
            assert_eq!(
                Interval::parse(text).map(Interval::duration),
                Some(Duration::from_secs(seconds))
            );
        }
        for bad in [
            "",
            "m",
            "0s",
            "2",
            "2d",
            "-1m",
            "+1m",
            "1.5h",
            " 2m",
            "99999999999999999h",
        ] {
            assert_eq!(Interval::parse(bad), None, "{bad}");
        }
    }

    #[test]
    fn the_file_and_the_data_directory() {
        let env = |config: Option<&str>, data: Option<&str>| Env {
            home: PathBuf::from("/h"),
            config_home: config.map(PathBuf::from),
            data_home: data.map(PathBuf::from),
        };
        let unset = env(None, None);
        assert_eq!(
            config_path(None, &unset),
            Path::new("/h/.config/boomdrive/config.toml")
        );
        assert_eq!(data_dir(&unset), Path::new("/h/.local/share/boomdrive"));
        let set = env(Some("/x/config"), Some("/x/data"));
        assert_eq!(
            config_path(None, &set),
            Path::new("/x/config/boomdrive/config.toml")
        );
        assert_eq!(data_dir(&set), Path::new("/x/data/boomdrive"));
        assert_eq!(
            config_path(Some(Path::new("/etc/boomdrive.toml")), &set),
            Path::new("/etc/boomdrive.toml")
        );
        for unusable in ["", "relative"] {
            let taken_as_unset = env(Some(unusable), Some(unusable));
            assert_eq!(
                config_path(None, &taken_as_unset),
                config_path(None, &unset)
            );
            assert_eq!(data_dir(&taken_as_unset), data_dir(&unset));
        }
    }

    #[test]
    fn a_data_directory_from_before_the_rename_is_kept() {
        let under = tempfile::tempdir().unwrap();
        let env = Env {
            home: PathBuf::from("/h"),
            config_home: None,
            data_home: Some(under.path().to_path_buf()),
        };
        let now = under.path().join("boomdrive");
        let before = under.path().join("flywheel");
        assert_eq!(data_dir(&env), now, "neither is there");
        std::fs::create_dir(&before).unwrap();
        assert_eq!(data_dir(&env), before, "only the one from before is there");
        std::fs::create_dir(&now).unwrap();
        assert_eq!(data_dir(&env), now, "both are there");
    }

    #[test]
    fn the_environment_needs_a_home() {
        fn vars(home: Option<&'static str>) -> impl Fn(&str) -> Option<OsString> {
            move |name| match name {
                "HOME" => home.map(OsString::from),
                "XDG_DATA_HOME" => Some(OsString::from("/d")),
                _ => None,
            }
        }
        assert_eq!(
            Env::read(vars(Some("/h"))),
            Ok(Env {
                home: PathBuf::from("/h"),
                config_home: None,
                data_home: Some(PathBuf::from("/d")),
            })
        );
        assert_eq!(Env::read(vars(None)), Err("HOME is not set".to_string()));
        assert_eq!(
            Env::read(vars(Some(""))),
            Err("HOME is not set".to_string())
        );
    }
}
