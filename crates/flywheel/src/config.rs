//! The runner's configuration (design 2): one TOML file naming the
//! repositories a host keeps by their GitHub address, where their views
//! are kept and how often they are fetched. A refusal names the file, the
//! line and the key or entry. It is built from what is wrong and never
//! from the file's text around it, a value put there by mistake among it
//! (core.4).

use serde::{Deserialize, Deserializer};
use std::ffi::OsString;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::time::Duration;
use toml::{Spanned, Value};

/// The configuration as a pass reads it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Where each repository's view is kept, at `<views>/<owner>/<name>`.
    pub views: PathBuf,
    /// How often every repository is fetched.
    pub fetch_every: Duration,
    /// Each repository the host keeps, in the file's order.
    pub repositories: Vec<Address>,
}

/// The period where the file names none.
const FETCH_EVERY: Duration = Duration::from_secs(2 * 60);

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

/// How often repositories are fetched: `<n>s`, `<n>m` or `<n>h`, with `n`
/// above zero, the form flume's `--stall-after` reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Period(Duration);

impl Period {
    /// The period `text` writes, or none where it is not one.
    pub fn parse(text: &str) -> Option<Period> {
        let (count, unit) = [("s", 1), ("m", 60), ("h", 60 * 60)]
            .into_iter()
            .find_map(|(suffix, unit)| Some((text.strip_suffix(suffix)?, unit)))?;
        if count.is_empty() || !count.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let seconds = count.parse::<u64>().ok()?.checked_mul(unit)?;
        (seconds > 0).then(|| Period(Duration::from_secs(seconds)))
    }

    /// The period as a duration.
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
    views: Option<Spanned<Value>>,
    fetch_every: Option<Spanned<Value>>,
    #[serde(default, deserialize_with = "list")]
    repositories: Option<Spanned<Entries>>,
}

/// `repositories` as written, each entry with where it stands.
type Entries = Vec<Spanned<Value>>;

/// `repositories`, refused by its key where it is not a list.
fn list<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Spanned<Entries>>, D::Error> {
    Option::deserialize(deserializer)
        .map_err(|_| serde::de::Error::custom("`repositories` is not a list"))
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
    let raw: File = toml::from_str(text).map_err(|error| match error.span() {
        Some(span) => refuse(span.start, error.message().to_string()),
        None => format!("{}: {}", file.display(), error.message()),
    })?;

    let views = match &raw.views {
        None => home.join("Code"),
        Some(views) => views
            .get_ref()
            .as_str()
            .and_then(|root| views_root(root, home))
            .ok_or_else(|| {
                refuse(
                    views.span().start,
                    format!(
                        "`views` is {}: a views root is an absolute path or begins with `~/`",
                        named(views.get_ref())
                    ),
                )
            })?,
    };

    let fetch_every = match &raw.fetch_every {
        None => FETCH_EVERY,
        Some(period) => period
            .get_ref()
            .as_str()
            .and_then(Period::parse)
            .map(Period::duration)
            .ok_or_else(|| {
                refuse(
                    period.span().start,
                    format!(
                        "`fetch_every` is {}: a period is `<n>s`, `<n>m` or `<n>h`, \
                         with n above zero",
                        named(period.get_ref())
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
    let mut repositories: Vec<(Address, &Spanned<Value>)> = Vec::new();
    for entry in entries.get_ref() {
        let at = entry.span().start;
        let Some(text) = entry.get_ref().as_str() else {
            return Err(refuse(
                at,
                "`repositories` entry is not a string: an address is `owner/name`".to_string(),
            ));
        };
        let address = Address::parse(text)
            .map_err(|rule| refuse(at, format!("`repositories` entry {text:?}: {rule}")))?;
        if let Some((_, first)) = repositories.iter().find(|(kept, _)| *kept == address) {
            return Err(refuse(
                at,
                format!(
                    "`repositories` entries {}, on line {}, and {text:?} are one repository: \
                     GitHub compares addresses without case",
                    named(first.get_ref()),
                    line(first.span().start),
                ),
            ));
        }
        repositories.push((address, entry));
    }

    Ok(Config {
        views,
        fetch_every,
        repositories: repositories
            .into_iter()
            .map(|(address, _)| address)
            .collect(),
    })
}

/// A views root: an absolute path, or `~/…` beneath `home`. Nothing else
/// is expanded, so `$HOME/Code` is no root.
fn views_root(root: &str, home: &Path) -> Option<PathBuf> {
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

/// A repository's view, `<views>/<owner>/<name>` (design 7).
pub fn view_path(config: &Config, address: &Address) -> PathBuf {
    config.views.join(address.owner()).join(address.name())
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
/// `$XDG_CONFIG_HOME/flywheel/config.toml`, else
/// `~/.config/flywheel/config.toml`.
pub fn config_path(arg: Option<&Path>, env: &Env) -> PathBuf {
    match arg {
        Some(file) => file.to_path_buf(),
        None => xdg(env.config_home.as_deref(), &env.home, ".config")
            .join("flywheel")
            .join("config.toml"),
    }
}

/// The runner's data directory, `$XDG_DATA_HOME/flywheel`, else
/// `~/.local/share/flywheel` (design 3).
pub fn data_dir(env: &Env) -> PathBuf {
    xdg(env.data_home.as_deref(), &env.home, ".local/share").join("flywheel")
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
            read("views = \"~/Code\"\n"),
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
        let config = read("views = \"~/src\"\nrepositories = []\n").unwrap();
        assert_eq!(config.views, Path::new("/h/src"));
    }

    #[test]
    fn a_relative_root() {
        for (root, named) in [("Code", "\"Code\""), ("$HOME/Code", "\"$HOME/Code\"")] {
            assert_eq!(
                read(&format!("repositories = []\nviews = \"{root}\"\n")),
                Err(format!(
                    "{FILE}:2: `views` is {named}: a views root is an absolute path or \
                     begins with `~/`"
                ))
            );
        }
    }

    #[test]
    fn no_period() {
        assert_eq!(
            read("repositories = []\n").unwrap().fetch_every,
            Duration::from_secs(120)
        );
    }

    #[test]
    fn a_period_of_nothing() {
        for period in ["0m", "soon"] {
            assert_eq!(
                read(&format!("repositories = []\nfetch_every = \"{period}\"\n")),
                Err(format!(
                    "{FILE}:2: `fetch_every` is \"{period}\": a period is `<n>s`, `<n>m` or \
                     `<n>h`, with n above zero"
                ))
            );
        }
    }

    #[test]
    fn a_misspelt_key() {
        let refusal = read("repositories = []\nview = \"~/Code\"\n").unwrap_err();
        assert!(
            refusal.starts_with(&format!("{FILE}:2: unknown field `view`")),
            "{refusal}"
        );
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
            read("repositories = []\nviews = 3\n"),
            Err(format!(
                "{FILE}:2: `views` is not a string: a views root is an absolute path or \
                 begins with `~/`"
            ))
        );
    }

    #[test]
    fn a_whole_file() {
        assert_eq!(
            read(
                "views = \"/srv/views\"\nfetch_every = \"90s\"\n\
                 repositories = [\"agentplot/flume\", \"afterthought/crew\"]\n"
            ),
            Ok(Config {
                views: PathBuf::from("/srv/views"),
                fetch_every: Duration::from_secs(90),
                repositories: vec![address("agentplot/flume"), address("afterthought/crew")],
            })
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
    fn periods() {
        for (text, seconds) in [("30s", 30), ("2m", 120), ("1h", 3600)] {
            assert_eq!(
                Period::parse(text).map(Period::duration),
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
            assert_eq!(Period::parse(bad), None, "{bad}");
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
            Path::new("/h/.config/flywheel/config.toml")
        );
        assert_eq!(data_dir(&unset), Path::new("/h/.local/share/flywheel"));
        let set = env(Some("/x/config"), Some("/x/data"));
        assert_eq!(
            config_path(None, &set),
            Path::new("/x/config/flywheel/config.toml")
        );
        assert_eq!(data_dir(&set), Path::new("/x/data/flywheel"));
        assert_eq!(
            config_path(Some(Path::new("/etc/flywheel.toml")), &set),
            Path::new("/etc/flywheel.toml")
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
