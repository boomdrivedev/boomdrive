//! The runner's configuration (design 2): TOML naming the repositories a
//! host keeps by their GitHub address, the root their views are kept
//! under, how often they are fetched, whether each owner's views have a
//! VS Code workspace, and the control plane whose work it carries, each
//! key named for what it holds (core.12). A refusal names the file, the
//! line and the key or entry. It is built from what is wrong and never
//! from the file's text around it, a value put there by mistake among it
//! (core.4).
//!
//! The configuration has two layers, in Flume's words (record 0009 as
//! amended 2026-11-06 and 2026-11-07; core.17): the managed layer, the
//! file `--config` names or `config.toml`, and the local layer,
//! `config.local.toml` in the runner's own configuration directory, read
//! over it. The local layer sets only `repositories`, which add to the
//! managed layer's, and in a control plane's table the keys its
//! implementation lists, each replacing the managed layer's whole; a key
//! the managed layer's `pin` holds is passed over. A local file that sets
//! any other key, is not TOML, or holds a value the runner's reading or
//! the implementation's refuses, is not taken at all, and the managed
//! layer is read alone.

use crate::planes;
use serde::de::IgnoredAny;
use serde::{Deserialize, Deserializer};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
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
    /// Each repository the host keeps, in the file's order.
    pub repositories: Vec<Address>,
    /// `[control_plane]`, where the file holds it. Read at every pass, it
    /// is taken when the runner starts (design Decision 4).
    pub control_plane: Option<ControlPlaneTable>,
    /// How the local layer was read: none where there is no local file.
    pub local: LocalLayer,
}

/// How the local layer was read (design Decision 4), in the words of
/// Flume's own reading of its layers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalLayer {
    /// There is no local file: the managed layer is read alone.
    NoFile,
    /// The local layer was read over the managed one, each key the
    /// managed layer's `pin` holds passed over.
    Taken { passed_over: Vec<PinnedKey> },
    /// The local file was not taken, and the managed layer is read
    /// alone: it sets each of `keys`, which the local layer may not set,
    /// each by its dotted path, or it holds `fault`, which names the file.
    NotTaken {
        keys: Vec<String>,
        fault: Option<String>,
    },
}

/// A key the local layer sets that the managed layer's `pin` holds, passed
/// over: the key by its dotted path, and the pin holding it as written.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PinnedKey {
    pub key: String,
    pub pin: String,
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
    #[serde(default, deserialize_with = "list")]
    repositories: Option<Spanned<Entries>>,
    #[serde(default, deserialize_with = "table")]
    control_plane: Option<Keys>,
    #[serde(default, deserialize_with = "pin_list")]
    pin: Option<Spanned<Entries>>,
}

/// A local layer's `repositories` as written, each entry with where it
/// stands. Every other key it may set is read from its table.
#[derive(Deserialize)]
struct LocalFile {
    #[serde(default, deserialize_with = "list")]
    repositories: Option<Spanned<Entries>>,
}

/// `repositories` as written, each entry with where it stands.
type Entries = Vec<Spanned<Value>>;

/// A table as written, each key and value with where it stands.
type Keys = BTreeMap<Spanned<String>, Spanned<Value>>;

/// `repositories`, refused by its key where it is not a list.
fn list<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Spanned<Entries>>, D::Error> {
    Option::deserialize(deserializer)
        .map_err(|_| serde::de::Error::custom("`repositories` is not a list"))
}

/// `control_plane`, refused by its key where it is not a table.
fn table<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Keys>, D::Error> {
    Option::deserialize(deserializer)
        .map_err(|_| serde::de::Error::custom("`control_plane` is not a table"))
}

/// `pin`, refused by its key where it is not a list.
fn pin_list<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<Spanned<Entries>>, D::Error> {
    Option::deserialize(deserializer)
        .map_err(|_| serde::de::Error::custom("`pin` is not a list of dotted keys"))
}

/// A pin of the managed layer's (design Decision 6): its words as written,
/// and the path of keys they name.
struct Pin {
    written: String,
    path: Vec<String>,
}

/// A file's text, for its refusals: each names the file, and the line
/// where it has one.
struct Source<'a> {
    text: &'a str,
    file: &'a Path,
}

impl Source<'_> {
    /// The line the byte `at` stands on.
    fn line(&self, at: usize) -> usize {
        self.text.as_bytes()[..at.min(self.text.len())]
            .iter()
            .filter(|&&b| b == b'\n')
            .count()
            + 1
    }

    /// `words`, of what stands at `at`.
    fn refuse(&self, at: usize, words: String) -> String {
        format!("{}:{}: {words}", self.file.display(), self.line(at))
    }

    /// TOML's `error`, by its message and never the text it quotes.
    fn broken(&self, error: toml::de::Error) -> String {
        match error.span() {
            Some(span) => self.refuse(span.start, error.message().to_string()),
            None => format!("{}: {}", self.file.display(), error.message()),
        }
    }
}

/// Reads the configuration file `file` alone, with `~` as `home`: the
/// managed layer with no local one over it, as the modules' files are
/// read. A file that cannot be read is refused, naming it.
pub fn read(file: &Path, home: &Path) -> Result<Config, String> {
    let text =
        std::fs::read_to_string(file).map_err(|error| format!("{}: {error}", file.display()))?;
    parse(&text, file, home)
}

/// Reads the configuration in `text`, the file `file`'s, with `~` as
/// `home`, as the managed layer with no local one over it.
pub fn parse(text: &str, file: &Path, home: &Path) -> Result<Config, String> {
    parse_managed(text, file, home).map(|(config, _)| config)
}

/// The local layer's file, `config.local.toml`, in the directory of the
/// default configuration file, whatever `--config` names (design Decision
/// 1): `$XDG_CONFIG_HOME/boomdrive/`, else `~/.config/boomdrive/`.
pub fn local_path(env: &Env) -> PathBuf {
    config_path(None, env).with_file_name("config.local.toml")
}

/// Reads the managed layer, the file `managed`, and the local layer, the
/// file `local`, over it, with `~` as `home` (design Decision 4). A managed
/// file that cannot be read is refused, naming it; a local file that does
/// not exist is no local layer, and one that cannot be read is not taken.
pub fn read_layers(managed: &Path, local: &Path, home: &Path) -> Result<Config, String> {
    let text = std::fs::read_to_string(managed)
        .map_err(|error| format!("{}: {error}", managed.display()))?;
    match std::fs::read_to_string(local) {
        Ok(over) => parse_layers(&text, managed, Some(&over), local, home),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            parse_layers(&text, managed, None, local, home)
        }
        Err(error) => {
            let alone = parse(&text, managed, home)?;
            Ok(not_taken(
                alone,
                vec![],
                Some(format!("{}: {error}", local.display())),
            ))
        }
    }
}

/// Reads the managed layer in `managed`, the file `managed_file`'s, and
/// the local layer in `local`, the file `local_file`'s, over it, none
/// where there is no local file, with `~` as `home` (design Decisions 3
/// and 4). A fault of the managed layer, a pin of nothing among it,
/// refuses the configuration; a local layer at fault is not taken, and
/// the managed layer is read alone.
pub fn parse_layers(
    managed: &str,
    managed_file: &Path,
    local: Option<&str>,
    local_file: &Path,
    home: &Path,
) -> Result<Config, String> {
    let (alone, pins) = parse_managed(managed, managed_file, home)?;
    Ok(match local {
        None => alone,
        Some(local) => over(alone, &pins, local, local_file),
    })
}

/// `alone`, the managed layer's configuration, its local layer not taken
/// for `keys` or `fault`.
fn not_taken(alone: Config, keys: Vec<String>, fault: Option<String>) -> Config {
    Config {
        local: LocalLayer::NotTaken { keys, fault },
        ..alone
    }
}

/// The configuration `alone`, the managed layer's, whose pins are `pins`,
/// with the local layer `text`, the file `file`'s, read over it (design
/// Decision 4, steps 3 to 6): not taken where it is not TOML, sets a key
/// it may not, or holds a value the runner's reading or the
/// implementation's refuses; else each unit a pin holds passed over, its
/// repositories added and each unit of a control plane's table set in
/// place.
fn over(alone: Config, pins: &[Pin], text: &str, file: &Path) -> Config {
    let source = Source { text, file };
    let table: Table = match toml::from_str(text) {
        Ok(table) => table,
        Err(error) => return not_taken(alone, vec![], Some(source.broken(error))),
    };
    let found = LocalUnits::of(&table, file);
    if !found.keys.is_empty() || found.fault.is_some() {
        return not_taken(alone, found.keys, found.fault);
    }

    let mut merged = alone.clone();
    let mut passed_over = Vec::new();
    let mut set_in: Option<&'static planes::Entry> = None;
    for (path, value) in found.units {
        if let Some(pin) = pins.iter().find(|pin| path.starts_with(&pin.path)) {
            passed_over.push(PinnedKey {
                key: dotted(&path),
                pin: pin.written.clone(),
            });
            continue;
        }
        let refused = |words: String| format!("{}: `{}` {words}", file.display(), dotted(&path));
        match path.as_slice() {
            [repositories] if repositories == "repositories" => {
                let entries = match toml::from_str::<LocalFile>(text) {
                    Ok(LocalFile {
                        repositories: Some(entries),
                    }) => entries,
                    Ok(LocalFile { repositories: None }) => continue,
                    Err(error) => return not_taken(alone, vec![], Some(source.broken(error))),
                };
                let added = match addresses(entries.get_ref(), &source) {
                    Ok(added) => added,
                    Err(fault) => return not_taken(alone, vec![], Some(fault)),
                };
                for address in added {
                    if !merged.repositories.contains(&address) {
                        merged.repositories.push(address);
                    }
                }
            }
            [_, word, key] => {
                let Some(plane) = merged.control_plane.as_mut() else {
                    let fault = refused(
                        "is set, but the managed layer names no `[control_plane]`".to_string(),
                    );
                    return not_taken(alone, vec![], Some(fault));
                };
                if plane.implementation != *word {
                    let fault = refused(format!(
                        "is set, but the managed layer's `[control_plane]` names the type {:?}",
                        plane.implementation
                    ));
                    return not_taken(alone, vec![], Some(fault));
                }
                plane.implementation_table.insert(key.clone(), value);
                set_in = planes::carried(word);
            }
            _ => unreachable!("every unit is a key of the local layer's"),
        }
    }

    // The implementation's own reading of its table, as the local layer
    // leaves it: a fault the managed layer's table has alone is not the
    // local layer's, and the implementation names it at its opening.
    if let (Some(entry), Some(plane), Some(managed)) =
        (set_in, &merged.control_plane, &alone.control_plane)
    {
        if let Err(fault) = (entry.read)(&plane.implementation_table, file) {
            if (entry.read)(&managed.implementation_table, file).err() != Some(fault.clone()) {
                return not_taken(alone, vec![], Some(fault));
            }
        }
    }

    Config {
        local: LocalLayer::Taken { passed_over },
        ..merged
    }
}

/// Each key the local layer may set, by its path (design Decision 2):
/// `repositories`, and `control_plane.<type>.<key>` for each key each
/// implementation this binary carries lists.
fn local_keys() -> Vec<Vec<&'static str>> {
    let mut keys = vec![vec!["repositories"]];
    for entry in &planes::PLANES {
        for key in entry.local_keys {
            keys.push(vec!["control_plane", entry.word, key]);
        }
    }
    keys
}

/// Whether `path` is a key the local layer may set.
fn is_local_key(path: &[String]) -> bool {
    local_keys().iter().any(|key| key.as_slice() == path)
}

/// Whether `path` is a table holding a key the local layer may set.
fn leads_to_local_key(path: &[String]) -> bool {
    local_keys().iter().any(|key| {
        key.len() > path.len() && key.iter().zip(path).all(|(word, segment)| word == segment)
    })
}

/// `path` as a dotted key in TOML's spelling.
fn dotted(path: &[String]) -> String {
    path.iter()
        .map(|segment| quoted(segment))
        .collect::<Vec<_>>()
        .join(".")
}

/// A key as TOML spells it: bare where it can be, else quoted.
fn quoted(segment: &str) -> String {
    let bare = !segment.is_empty()
        && segment
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if bare {
        segment.to_string()
    } else {
        format!("{segment:?}")
    }
}

/// What a local layer's table holds (design Decision 4, step 4): each
/// unit it sets, by its path; each key it may not set, named once by its
/// dotted path, the shortest holding no key it may set; and its fault, a
/// table holding a key it may set that is no table.
#[derive(Default)]
struct LocalUnits {
    units: Vec<(Vec<String>, Value)>,
    keys: Vec<String>,
    fault: Option<String>,
}

impl LocalUnits {
    /// The units, keys and fault of the local layer's `table`, the file
    /// `file`'s.
    fn of(table: &Table, file: &Path) -> LocalUnits {
        let mut found = LocalUnits::default();
        found.walk(table, &mut vec![], file);
        found
    }

    /// Walks `table` at `path`, each table holding a key the local layer
    /// may set walked further.
    fn walk(&mut self, table: &Table, path: &mut Vec<String>, file: &Path) {
        for (name, value) in table {
            path.push(name.clone());
            match value {
                _ if is_local_key(path) => self.units.push((path.clone(), value.clone())),
                Value::Table(inner) if leads_to_local_key(path) => self.walk(inner, path, file),
                _ if leads_to_local_key(path) => {
                    self.fault.get_or_insert_with(|| {
                        format!("{}: `{}` is not a table", file.display(), dotted(path))
                    });
                }
                _ => {
                    let key = dotted(path);
                    if !self.keys.contains(&key) {
                        self.keys.push(key);
                    }
                }
            }
            path.pop();
        }
    }
}

/// The path the dotted key `key` names in TOML's spelling, read by parsing
/// `<key> = 0` as a document; none where that is not one key.
fn key_path(key: &str) -> Option<Vec<String>> {
    let mut table: Table = format!("{key} = 0").parse().ok()?;
    let mut path = vec![];
    loop {
        if table.len() != 1 {
            return None;
        }
        let (name, value) = table.into_iter().next()?;
        path.push(name);
        match value {
            Value::Table(inner) => table = inner,
            Value::Integer(0) => return Some(path),
            _ => return None,
        }
    }
}

/// The managed layer's pins as written in `entries` (design Decision 6):
/// each a dotted key naming a key the local layer may set or a table
/// holding one, and any other refused, naming it.
fn pins(entries: &Entries, source: &Source<'_>) -> Result<Vec<Pin>, String> {
    let mut pins = Vec::new();
    for entry in entries {
        let at = entry.span().start;
        let Some(text) = entry.get_ref().as_str() else {
            return Err(source.refuse(
                at,
                "`pin` entry is not a string: a pin is a dotted key".to_string(),
            ));
        };
        let Some(path) = key_path(text) else {
            return Err(source.refuse(at, format!("`pin` entry {text:?} is not a dotted key")));
        };
        if !is_local_key(&path) && !leads_to_local_key(&path) {
            let listed: Vec<String> = local_keys()
                .iter()
                .map(|key| format!("`{}`", key.join(".")))
                .collect();
            return Err(source.refuse(
                at,
                format!(
                    "`pin` entry {text:?} names no key a local layer may set: those are {}, \
                     and a table holding one",
                    listed.join(", ")
                ),
            ));
        }
        pins.push(Pin {
            written: text.to_string(),
            path,
        });
    }
    Ok(pins)
}

/// The addresses `entries` write, in their order, or the first fault: an
/// entry that is no address, or two that are one repository (design 2).
fn addresses(entries: &Entries, source: &Source<'_>) -> Result<Vec<Address>, String> {
    let mut repositories: Vec<(Address, &Spanned<Value>)> = Vec::new();
    for entry in entries {
        let at = entry.span().start;
        let Some(text) = entry.get_ref().as_str() else {
            return Err(source.refuse(
                at,
                "`repositories` entry is not a string: an address is `owner/name`".to_string(),
            ));
        };
        let address = Address::parse(text)
            .map_err(|rule| source.refuse(at, format!("`repositories` entry {text:?}: {rule}")))?;
        if let Some((_, first)) = repositories.iter().find(|(kept, _)| *kept == address) {
            return Err(source.refuse(
                at,
                format!(
                    "`repositories` entries {}, on line {}, and {text:?} are one repository: \
                     GitHub compares addresses without case",
                    named(first.get_ref()),
                    source.line(first.span().start),
                ),
            ));
        }
        repositories.push((address, entry));
    }
    Ok(repositories
        .into_iter()
        .map(|(address, _)| address)
        .collect())
}

/// Reads the managed layer in `text`, the file `file`'s, with `~` as
/// `home`: its configuration, and its pins.
fn parse_managed(text: &str, file: &Path, home: &Path) -> Result<(Config, Vec<Pin>), String> {
    let source = Source { text, file };
    let line = |at: usize| source.line(at);
    let refuse = |at: usize, words: String| source.refuse(at, words);
    let broken = |error: toml::de::Error| source.broken(error);

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

    let entries = raw.repositories.ok_or_else(|| {
        format!(
            "{}: `repositories` is required: the repositories this host keeps, \
             `repositories = []` for none",
            file.display()
        )
    })?;
    let repositories = addresses(entries.get_ref(), &source)?;

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

    let pins = match &raw.pin {
        None => Vec::new(),
        Some(entries) => pins(entries.get_ref(), &source)?,
    };

    Ok((
        Config {
            repository_root,
            fetch_interval,
            code_workspaces,
            repositories,
            control_plane,
            local: LocalLayer::NoFile,
        },
        pins,
    ))
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

    #[test]
    fn an_address_with_no_owner() {
        assert_eq!(
            read("repositories = [\"flume\"]\n"),
            Err(format!(
                "{FILE}:1: `repositories` entry \"flume\": an address is `owner/name`"
            ))
        );
    }

    #[test]
    fn one_repository_twice() {
        assert_eq!(
            read("repositories = [\n  \"agentplot/flume\",\n  \"AgentPlot/flume\",\n]\n"),
            Err(format!(
                "{FILE}:3: `repositories` entries \"agentplot/flume\", on line 2, and \
                 \"AgentPlot/flume\" are one repository: GitHub compares addresses without case"
            ))
        );
    }

    #[test]
    fn no_list() {
        assert_eq!(
            read("repository_root = \"~/Code\"\n"),
            Err(format!(
                "{FILE}: `repositories` is required: the repositories this host keeps, \
                 `repositories = []` for none"
            ))
        );
    }

    #[test]
    fn keeping_none() {
        assert_eq!(read("repositories = []\n").unwrap().repositories, vec![]);
    }

    #[test]
    fn no_views_setting() {
        let config = read("repositories = [\"agentplot/flume\"]\n").unwrap();
        assert_eq!(
            view_path(&config, &address("agentplot/flume")),
            Path::new("/h/Code/agentplot/flume")
        );
    }

    #[test]
    fn a_home_relative_root() {
        let config = read("repository_root = \"~/src\"\nrepositories = []\n").unwrap();
        assert_eq!(config.repository_root, Path::new("/h/src"));
    }

    #[test]
    fn a_relative_root() {
        for (root, named) in [("Code", "\"Code\""), ("$HOME/Code", "\"$HOME/Code\"")] {
            assert_eq!(
                read(&format!(
                    "repositories = []\nrepository_root = \"{root}\"\n"
                )),
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
            read("repositories = []\n").unwrap().fetch_interval,
            Duration::from_secs(120)
        );
    }

    /// The configuration spec's "No workspaces setting".
    #[test]
    fn no_workspaces_setting() {
        let config = read("repositories = [\"agentplot/flume\"]\n").unwrap();
        assert!(!config.code_workspaces);
        let config = read("code_workspaces = false\nrepositories = []\n").unwrap();
        assert!(!config.code_workspaces);
    }

    /// The configuration spec's "Workspaces asked for in words", and a
    /// number refused alike.
    #[test]
    fn workspaces_asked_for_in_words() {
        for (value, named) in [("\"yes\"", "\"yes\""), ("1", "not a string")] {
            assert_eq!(
                read(&format!("repositories = []\ncode_workspaces = {value}\n")),
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
                    "repositories = []\nfetch_interval = \"{interval}\"\n"
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
        let refusal = read("repositories = []\nrepositories_root = \"~/Code\"\n").unwrap_err();
        assert!(
            refusal.starts_with(&format!("{FILE}:2: unknown field `repositories_root`")),
            "{refusal}"
        );
    }

    #[test]
    fn a_stream_named() {
        assert_eq!(
            read("repositories = []\nstream = \"work\"\n"),
            Err(format!(
                "{FILE}:2: unknown field `stream`: streams are Flume's host configuration"
            ))
        );
        assert_eq!(
            read("repositories = []\n\n[streams]\nwork = 2\n"),
            Err(format!(
                "{FILE}:3: unknown field `streams`: streams are Flume's host configuration"
            ))
        );
    }

    #[test]
    fn a_kinds_file_named() {
        assert_eq!(
            read("repositories = []\nkinds = { crew = \"/k/kinds.toml\" }\n"),
            Err(format!(
                "{FILE}:2: unknown field `kinds`: kinds are Flume's host configuration"
            ))
        );
        for key in ["kind", "kinds_file"] {
            assert_eq!(
                read(&format!("repositories = []\n{key} = \"change\"\n")),
                Err(format!(
                    "{FILE}:2: unknown field `{key}`: kinds are Flume's host configuration"
                ))
            );
        }
    }

    #[test]
    fn a_key_of_before() {
        assert_eq!(
            read("views = \"~/Code\"\nfetch_every = \"2m\"\nrepositories = []\n"),
            Err(format!(
                "{FILE}:1: unknown field `views`: `repository_root` holds it now; \
                 unknown field `fetch_every`, on line 2: `fetch_interval` holds it now"
            ))
        );
        assert_eq!(
            read("repositories = []\nfetch_every = \"2m\"\n"),
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
        let refusal =
            read("repositories = []\nviews = \"~/Code\"\nghp_n0t4r34lt0k3n\n").unwrap_err();
        assert!(refusal.starts_with(&format!("{FILE}:3: ")), "{refusal}");
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_token() {
        let refusal = read("repositories = []\ntoken = \"ghp_n0t4r34lt0k3n\"\n").unwrap_err();
        assert!(
            refusal.starts_with(&format!("{FILE}:2: unknown field `token`")),
            "{refusal}"
        );
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_broken_file_is_named_by_its_line_and_never_quoted() {
        let refusal = read("repositories = []\nghp_n0t4r34lt0k3n\n").unwrap_err();
        assert!(refusal.starts_with(&format!("{FILE}:2: ")), "{refusal}");
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
    }

    #[test]
    fn a_value_of_another_kind_is_named_by_its_key() {
        assert_eq!(
            read("repositories = \"agentplot/flume\"\n"),
            Err(format!("{FILE}:1: `repositories` is not a list"))
        );
        assert_eq!(
            read("repositories = [3]\n"),
            Err(format!(
                "{FILE}:1: `repositories` entry is not a string: an address is `owner/name`"
            ))
        );
        assert_eq!(
            read("repositories = []\nrepository_root = 3\n"),
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
                 repositories = [\"agentplot/flume\", \"afterthought/crew\"]\n"
            ),
            Ok(Config {
                repository_root: PathBuf::from("/srv/code"),
                fetch_interval: Duration::from_secs(90),
                code_workspaces: true,
                repositories: vec![address("agentplot/flume"), address("afterthought/crew")],
                control_plane: None,
                local: LocalLayer::NoFile,
            })
        );
    }

    /// Design Decision 4's example of `[control_plane]`.
    const CONTROL_PLANE: &str = "repositories = []

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
        assert_eq!(read("repositories = []\n").unwrap().control_plane, None);
    }

    /// The configuration spec's "No type".
    #[test]
    fn no_type() {
        assert_eq!(
            read("repositories = []\n\n[control_plane]\npoll_interval = \"30s\"\n"),
            Err(format!(
                "{FILE}:3: `type` is required in `[control_plane]`: the word of the control \
                 plane's implementation"
            ))
        );
        assert_eq!(
            read("repositories = []\n[control_plane]\ntype = 3\n"),
            Err(format!(
                "{FILE}:3: `type` is not a string: it is the word of the control plane's \
                 implementation"
            ))
        );
    }

    /// The configuration spec's "No poll interval".
    #[test]
    fn no_poll_interval() {
        let table = read("repositories = []\n[control_plane]\ntype = \"crew\"\n")
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
                "repositories = []\n[control_plane]\ntype = \"crew\"\n\
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
                    "repositories = []\n[control_plane]\ntype = \"crew\"\n\
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
                "repositories = []\n[control_plane]\ntype = \"crew\"\n{key}\n"
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
            read("repositories = []\ncontrol_plane = \"crew\"\n"),
            Err(format!("{FILE}:2: `control_plane` is not a table"))
        );
        assert_eq!(
            read("repositories = []\n[control_plane]\ntype = \"crew\"\ncrew = 3\n"),
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

    const LOCAL: &str = "/c/config.local.toml";

    /// The managed layer `managed` with the local layer `local` over it.
    fn layers(managed: &str, local: Option<&str>) -> Result<Config, String> {
        parse_layers(
            managed,
            Path::new(FILE),
            local,
            Path::new(LOCAL),
            Path::new("/h"),
        )
    }

    /// The repositories `config` keeps, as written.
    fn kept(config: &Config) -> Vec<String> {
        config
            .repositories
            .iter()
            .map(ToString::to_string)
            .collect()
    }

    /// The hosted table's partitions' labels, as `config` hands them on.
    fn hosted_labels(config: &Config) -> Vec<String> {
        let table = &config.control_plane.as_ref().unwrap().implementation_table;
        table["partitions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|partition| partition["label"].as_str().unwrap().to_string())
            .collect()
    }

    /// The local layer not taken for `keys` and no fault.
    fn for_keys(keys: &[&str]) -> LocalLayer {
        LocalLayer::NotTaken {
            keys: keys.iter().map(ToString::to_string).collect(),
            fault: None,
        }
    }

    /// The local layer's fault, where it was not taken for one.
    fn fault_of(config: &Config) -> String {
        match &config.local {
            LocalLayer::NotTaken {
                keys,
                fault: Some(fault),
            } if keys.is_empty() => fault.clone(),
            other => panic!("not taken for a fault: {other:?}"),
        }
    }

    /// A managed layer naming the hosted control plane at an address, and
    /// `partitions` as its partitions' labels, with `top` above it.
    fn hosted(top: &str, partitions: &[&str]) -> String {
        let mut text = format!(
            "{top}repositories = [\"agentplot/flume\"]\n\n[control_plane]\ntype = \"hosted\"\n\n\
             [control_plane.hosted]\nurl = \"https://control.example\"\n"
        );
        for label in partitions {
            text += &format!(
                "\n[[control_plane.hosted.partitions]]\nlabel = \"{label}\"\nrepositories = []\n"
            );
        }
        text
    }

    /// A local layer naming `partitions` as the hosted partitions' labels,
    /// with `top` above them.
    fn hosted_local(top: &str, partitions: &[&str]) -> String {
        let mut text = top.to_string();
        for label in partitions {
            text += &format!(
                "\n[[control_plane.hosted.partitions]]\nlabel = \"{label}\"\nrepositories = []\n"
            );
        }
        text
    }

    /// The configuration spec's "No local file".
    #[test]
    fn no_local_file() {
        let config = layers("repositories = [\"agentplot/flume\"]\n", None).unwrap();
        assert_eq!(kept(&config), ["agentplot/flume"]);
        assert_eq!(config.local, LocalLayer::NoFile);
        assert_eq!(
            config,
            read("repositories = [\"agentplot/flume\"]\n").unwrap()
        );
    }

    /// The configuration spec's "The local layer's repositories".
    #[test]
    fn the_local_layers_repositories() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\"agentplot/sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(kept(&config), ["agentplot/flume", "agentplot/sluice"]);
        assert_eq!(
            config.local,
            LocalLayer::Taken {
                passed_over: vec![]
            }
        );
    }

    /// The configuration spec's "A repository both layers name": kept
    /// once, as the managed layer spells it, and neither layer refused.
    #[test]
    fn a_repository_both_layers_name() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\"AgentPlot/flume\", \"agentplot/sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(kept(&config), ["agentplot/flume", "agentplot/sluice"]);
        assert!(matches!(config.local, LocalLayer::Taken { .. }));
    }

    /// The configuration spec's "The local layer's partitions": the
    /// hosted table holds the managed layer's `url` and the local layer's
    /// partitions alone.
    #[test]
    fn the_local_layers_partitions() {
        let config = layers(&hosted("", &["home"]), Some(&hosted_local("", &["work"]))).unwrap();
        assert_eq!(hosted_labels(&config), ["work"]);
        let table = &config.control_plane.as_ref().unwrap().implementation_table;
        assert_eq!(table["url"].as_str(), Some("https://control.example"));
        assert_eq!(kept(&config), ["agentplot/flume"]);
        assert!(matches!(config.local, LocalLayer::Taken { .. }));
    }

    /// A managed hosted table naming no partitions, the local layer
    /// naming them, is taken: the implementation reads the table as the
    /// local layer leaves it.
    #[test]
    fn partitions_named_in_the_local_layer_alone() {
        let config = layers(&hosted("", &[]), Some(&hosted_local("", &["work"]))).unwrap();
        assert_eq!(hosted_labels(&config), ["work"]);
        assert!(matches!(config.local, LocalLayer::Taken { .. }));
    }

    #[test]
    fn the_local_layers_file() {
        let env = |config: Option<&str>| Env {
            home: PathBuf::from("/h"),
            config_home: config.map(PathBuf::from),
            data_home: None,
        };
        assert_eq!(
            local_path(&env(Some("/x"))),
            Path::new("/x/boomdrive/config.local.toml")
        );
        assert_eq!(
            local_path(&env(None)),
            Path::new("/h/.config/boomdrive/config.local.toml")
        );
        for unusable in ["", "relative"] {
            assert_eq!(
                local_path(&env(Some(unusable))),
                Path::new("/h/.config/boomdrive/config.local.toml"),
                "{unusable:?}"
            );
        }
    }

    /// The configuration spec's "A root in the local layer".
    #[test]
    fn a_root_in_the_local_layer() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repository_root = \"~/src\"\nrepositories = [\"agentplot/sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["repository_root"]));
        assert_eq!(config.repository_root, Path::new("/h/Code"));
        assert_eq!(kept(&config), ["agentplot/flume"]);
    }

    /// The configuration spec's "A control plane named in the local
    /// layer", and a key of an implementation's table it does not list.
    #[test]
    fn a_control_plane_named_in_the_local_layer() {
        let config = layers(
            "repositories = []\n",
            Some("repositories = []\n\n[control_plane]\ntype = \"hosted\"\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["control_plane.type"]));
        assert_eq!(config.control_plane, None);

        let config = layers(
            &hosted("", &["home"]),
            Some("[control_plane.hosted]\nurl = \"https://elsewhere.example\"\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["control_plane.hosted.url"]));

        let config = layers(
            "repositories = []\n",
            Some("pin = [\"repositories\"]\n\n[streams]\nwork = 2\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["pin", "streams"]));
    }

    /// The configuration spec's "A token in the local layer": named by
    /// its key, its value in no output.
    #[test]
    fn a_token_in_the_local_layer() {
        let config = layers(
            "repositories = []\n",
            Some("token = \"ghp_n0t4r34lt0k3n\"\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["token"]));
        assert!(
            !format!("{config:?}").contains("ghp_n0t4r34lt0k3n"),
            "{config:?}"
        );
    }

    /// The configuration spec's "A local file that is not TOML": named by
    /// its file and line, and never quoted.
    #[test]
    fn a_local_file_that_is_not_toml() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\n"),
        )
        .unwrap();
        let fault = fault_of(&config);
        assert!(fault.starts_with(&format!("{LOCAL}:")), "{fault}");
        assert_eq!(kept(&config), ["agentplot/flume"]);

        let config = layers(
            "repositories = []\n",
            Some("repositories = []\nghp_n0t4r34lt0k3n\n"),
        )
        .unwrap();
        let fault = fault_of(&config);
        assert!(fault.starts_with(&format!("{LOCAL}:2: ")), "{fault}");
        assert!(!fault.contains("ghp_n0t4r34lt0k3n"), "{fault}");
    }

    /// The configuration spec's "A local address out of form".
    #[test]
    fn a_local_address_out_of_form() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\"sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(
            fault_of(&config),
            format!("{LOCAL}:1: `repositories` entry \"sluice\": an address is `owner/name`")
        );
        assert_eq!(kept(&config), ["agentplot/flume"]);
    }

    /// A local layer naming two entries that are one repository is not
    /// taken, as a managed one naming them is refused.
    #[test]
    fn a_local_layer_naming_one_repository_twice() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\n  \"agentplot/sluice\",\n  \"AgentPlot/Sluice\",\n]\n"),
        )
        .unwrap();
        assert_eq!(
            fault_of(&config),
            format!(
                "{LOCAL}:3: `repositories` entries \"agentplot/sluice\", on line 2, and \
                 \"AgentPlot/Sluice\" are one repository: GitHub compares addresses without case"
            )
        );
        assert_eq!(kept(&config), ["agentplot/flume"]);
    }

    /// The configuration spec's "A partition the implementation refuses":
    /// the hosted implementation's reading names the label, and the
    /// managed layer's table is handed on.
    #[test]
    fn a_partition_the_implementation_refuses() {
        let config = layers(&hosted("", &["home"]), Some(&hosted_local("", &["Home"]))).unwrap();
        let fault = fault_of(&config);
        assert!(
            fault.starts_with(&format!(
                "{LOCAL}: [control_plane.hosted]: `partitions` entry 1's `label` is \"Home\""
            )),
            "{fault}"
        );
        assert_eq!(hosted_labels(&config), ["home"]);
    }

    /// A fault of the managed layer's own table, which the local layer
    /// does not make, is the implementation's to name at its opening: the
    /// local layer is taken.
    #[test]
    fn a_fault_of_the_managed_table_alone() {
        let managed = hosted("", &["home"]).replace("https://control.example", "ftp://x");
        let config = layers(&managed, Some(&hosted_local("", &["work"]))).unwrap();
        assert!(matches!(config.local, LocalLayer::Taken { .. }));
        assert_eq!(hosted_labels(&config), ["work"]);
    }

    /// A local layer naming the hosted partitions where the managed
    /// layer's control plane is crew's, or where it names none, is not
    /// taken.
    #[test]
    fn partitions_of_a_control_plane_the_managed_layer_does_not_name() {
        let config = layers(CONTROL_PLANE, Some(&hosted_local("", &["work"]))).unwrap();
        assert_eq!(
            fault_of(&config),
            format!(
                "{LOCAL}: `control_plane.hosted.partitions` is set, but the managed layer's \
                 `[control_plane]` names the type \"crew\""
            )
        );
        assert_eq!(
            config.control_plane.unwrap().implementation_table["host"].as_str(),
            Some("deneb")
        );

        let config = layers("repositories = []\n", Some(&hosted_local("", &["work"]))).unwrap();
        assert_eq!(
            fault_of(&config),
            format!(
                "{LOCAL}: `control_plane.hosted.partitions` is set, but the managed layer names \
                 no `[control_plane]`"
            )
        );
        assert_eq!(config.control_plane, None);
    }

    /// A table the local layer may set a key in that is no table is its
    /// fault.
    #[test]
    fn a_local_table_of_another_kind() {
        let config = layers("repositories = []\n", Some("control_plane = 3\n")).unwrap();
        assert_eq!(
            fault_of(&config),
            format!("{LOCAL}: `control_plane` is not a table")
        );
    }

    /// The configuration spec's "Repositories pinned".
    #[test]
    fn repositories_pinned() {
        let config = layers(
            "pin = [\"repositories\"]\nrepositories = [\"agentplot/flume\"]\n",
            Some("repositories = [\"agentplot/sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(kept(&config), ["agentplot/flume"]);
        assert_eq!(
            config.local,
            LocalLayer::Taken {
                passed_over: vec![PinnedKey {
                    key: "repositories".to_string(),
                    pin: "repositories".to_string(),
                }]
            }
        );
    }

    /// The configuration spec's "A table pinned".
    #[test]
    fn a_table_pinned() {
        let config = layers(
            &hosted("pin = [\"control_plane\"]\n", &["home"])
                .replace("[\"agentplot/flume\"]", "[]"),
            Some(&hosted_local(
                "repositories = [\"agentplot/sluice\"]\n",
                &["work"],
            )),
        )
        .unwrap();
        assert_eq!(kept(&config), ["agentplot/sluice"]);
        assert_eq!(hosted_labels(&config), ["home"]);
        assert_eq!(
            config.local,
            LocalLayer::Taken {
                passed_over: vec![PinnedKey {
                    key: "control_plane.hosted.partitions".to_string(),
                    pin: "control_plane".to_string(),
                }]
            }
        );
    }

    /// A pin in TOML's quoted spelling holds the key it names.
    #[test]
    fn a_pin_quoted() {
        let config = layers(
            &hosted(
                "pin = [\"control_plane.\\\"hosted\\\".partitions\"]\n",
                &["home"],
            ),
            Some(&hosted_local("", &["work"])),
        )
        .unwrap();
        assert_eq!(hosted_labels(&config), ["home"]);
    }

    /// The configuration spec's "A pin of nothing", and a pin of a word
    /// this binary does not carry.
    #[test]
    fn a_pin_of_nothing() {
        for (pin, line) in [("repository_root", 1), ("control_plane.elsewhere", 1)] {
            let managed = format!("pin = [\"{pin}\"]\nrepositories = []\n");
            for refusal in [
                layers(&managed, None).unwrap_err(),
                read(&managed).unwrap_err(),
            ] {
                assert_eq!(
                    refusal,
                    format!(
                        "{FILE}:{line}: `pin` entry \"{pin}\" names no key a local layer may set: \
                         those are `repositories`, `control_plane.crew.partitions`, \
                         `control_plane.hosted.partitions`, and a table holding one"
                    )
                );
            }
        }
    }

    /// A `pin` that is no list of strings, or an entry that is no dotted
    /// key, is the managed layer's fault.
    #[test]
    fn a_pin_of_another_kind() {
        assert_eq!(
            read("repositories = []\npin = \"repositories\"\n"),
            Err(format!("{FILE}:2: `pin` is not a list of dotted keys"))
        );
        assert_eq!(
            read("repositories = []\npin = [3]\n"),
            Err(format!(
                "{FILE}:2: `pin` entry is not a string: a pin is a dotted key"
            ))
        );
        assert_eq!(
            read("repositories = []\npin = [\"a b\"]\n"),
            Err(format!("{FILE}:2: `pin` entry \"a b\" is not a dotted key"))
        );
    }

    /// `pin` set in the local layer is a key it may not set.
    #[test]
    fn a_pin_in_the_local_layer() {
        let config = layers(
            "repositories = [\"agentplot/flume\"]\n",
            Some("pin = [\"repositories\"]\nrepositories = [\"agentplot/sluice\"]\n"),
        )
        .unwrap();
        assert_eq!(config.local, for_keys(&["pin"]));
        assert_eq!(kept(&config), ["agentplot/flume"]);
    }

    /// A managed layer at fault refuses the configuration before the
    /// local layer is read.
    #[test]
    fn a_managed_layer_at_fault() {
        assert_eq!(
            layers("repositories = [\"flume\"]\n", Some("repositories = []\n")),
            Err(format!(
                "{FILE}:1: `repositories` entry \"flume\": an address is `owner/name`"
            ))
        );
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
