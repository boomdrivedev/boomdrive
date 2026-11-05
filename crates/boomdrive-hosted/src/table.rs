//! The hosted table, `[control_plane.hosted]` (design Decision 2): the
//! control plane's address, and each partition served, one enrollment
//! each, with the repositories that partition is served (core.26). Every
//! key is named for what it holds (core.12), and the table names no
//! credential: each partition's key is made by `boomdrive enroll` and kept
//! in the host's credential store (core.4, core.32). A refusal names the
//! file and the key, and never quotes the `url` or a key the
//! implementation does not read, a value put there by mistake among it.

use crate::key::KeyName;
use std::path::Path;
use toml::Value;
use url::{Host, Url};

/// The table as the runner's configuration names it.
const TABLE: &str = "[control_plane.hosted]";

/// The keys the hosted implementation reads, at the table's top and in
/// each partition.
const KEYS: [&str; 2] = ["url", "partitions"];
const PARTITION_KEYS: [&str; 2] = ["label", "repositories"];

/// The keys a credential would be put under, each refused with where the
/// host's key is.
const CREDENTIALS: [&str; 3] = ["token", "token_file", "key"];

/// The most characters a label holds.
const LABEL_LENGTH: usize = 32;

/// The two paths under the control plane's address (the protocol's
/// README, *The paths*).
const ENROLL_PATH: &str = "runner/enroll";
const CONNECT_PATH: &str = "runner/connect";

/// The hosted table, read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table {
    /// The control plane's address, under which its two paths go.
    pub url: Url,
    /// Each partition served, in the table's order.
    pub partitions: Vec<Partition>,
}

/// A partition served: one enrollment, its label the host's own name for
/// it, and the repositories, `owner/name`, it is served.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Partition {
    pub label: String,
    pub repositories: Vec<String>,
}

impl Table {
    /// Reads the hosted table, `table`, from the configuration file
    /// `file`.
    pub fn read(table: &toml::Table, file: &Path) -> Result<Table, String> {
        let refuse = |words: String| format!("{}: {TABLE}: {words}", file.display());
        unread(table, &KEYS, "").map_err(refuse)?;
        let url = address(table.get("url")).map_err(refuse)?;

        let entries = match table.get("partitions") {
            None => {
                return Err(refuse(
                    "names no `partitions`: at least one partition is served, each a `label` \
                     and its `repositories`"
                        .to_string(),
                ))
            }
            Some(Value::Array(entries)) if !entries.is_empty() => entries,
            Some(Value::Array(_)) => {
                return Err(refuse(
                    "`partitions` is empty: at least one partition is served".to_string(),
                ))
            }
            Some(_) => {
                return Err(refuse(
                    "`partitions` is not a list of tables: write each as \
                     `[[control_plane.hosted.partitions]]`"
                        .to_string(),
                ))
            }
        };

        let mut partitions: Vec<Partition> = Vec::new();
        for (n, entry) in entries.iter().enumerate() {
            let which = format!("`partitions` entry {}", n + 1);
            let Value::Table(entry) = entry else {
                return Err(refuse(format!("{which} is not a table")));
            };
            let label = match entry.get("label") {
                None => {
                    return Err(refuse(format!(
                        "{which} names no `label`: the host's own name for the partition is \
                         required"
                    )))
                }
                Some(Value::String(label)) if is_label(label) => label.clone(),
                Some(Value::String(label)) => {
                    return Err(refuse(format!(
                        "{which}'s `label` is {label:?}: a label is 1 to {LABEL_LENGTH} \
                         lowercase letters, digits and `-`, starting with a letter"
                    )))
                }
                Some(_) => return Err(refuse(format!("{which}'s `label` is not a string"))),
            };
            let which = format!("`partitions` entry `{label}`");
            unread(entry, &PARTITION_KEYS, &format!("{which} ")).map_err(refuse)?;
            let repositories = match entry.get("repositories") {
                None => {
                    return Err(refuse(format!(
                        "{which} names no `repositories`: the repositories this partition is \
                         served, each `owner/name`, are required, and may be `[]`"
                    )))
                }
                Some(Value::Array(repositories)) => repositories,
                Some(_) => {
                    return Err(refuse(format!(
                        "{which}'s `repositories` is not a list of addresses, each `owner/name`"
                    )))
                }
            };
            let repositories = repositories
                .iter()
                .map(|repository| match repository {
                    Value::String(text) if is_address(text) => Ok(text.clone()),
                    Value::String(text) => Err(refuse(format!(
                        "{which}'s `repositories` entry {text:?}: an address is `owner/name`"
                    ))),
                    _ => Err(refuse(format!(
                        "{which}'s `repositories` entry is not a string: an address is \
                         `owner/name`"
                    ))),
                })
                .collect::<Result<Vec<String>, String>>()?;
            if partitions.iter().any(|served| served.label == label) {
                return Err(refuse(format!(
                    "`partitions` names `{label}` twice: each partition is enrolled once"
                )));
            }
            partitions.push(Partition {
                label,
                repositories,
            });
        }
        Ok(Table { url, partitions })
    }

    /// The partition labelled `label`.
    pub fn partition(&self, label: &str) -> Option<&Partition> {
        self.partitions
            .iter()
            .find(|partition| partition.label == label)
    }

    /// The control plane's origin, which each signature names: WHATWG's
    /// serialization, its scheme, `://` and its host, then `:` and the
    /// port where the port is not the scheme's own, lowercased (the
    /// protocol's README, *The signatures*).
    pub fn origin(&self) -> String {
        self.url.origin().ascii_serialization().to_lowercase()
    }

    /// What `label`'s key is kept under in the host's credential store:
    /// the label and the control plane's host (design Decision 3).
    pub fn key_name(&self, label: &str) -> KeyName {
        let plane = self.url.host_str().unwrap_or_default().to_lowercase();
        KeyName::new(label, &plane)
    }

    /// Where an enrollment is sent: `runner/enroll` under the `url`.
    pub fn enroll_address(&self) -> Url {
        self.joined(ENROLL_PATH)
    }

    /// Where each partition's socket is opened: `runner/connect` under
    /// the `url`, `wss`, or `ws` where the `url` is `http` to a loopback
    /// address.
    pub fn connect_address(&self) -> Url {
        let mut address = self.joined(CONNECT_PATH);
        let scheme = match address.scheme() {
            "http" => "ws",
            _ => "wss",
        };
        // Each of `http`, `https`, `ws` and `wss` is a special scheme,
        // which WHATWG lets one change into another.
        let _ = address.set_scheme(scheme);
        address
    }

    /// `path` under the `url`, whose path gains a `/` where it ends
    /// without one.
    fn joined(&self, path: &str) -> Url {
        let mut base = self.url.clone();
        if !base.path().ends_with('/') {
            let with_slash = format!("{}/", base.path());
            base.set_path(&with_slash);
        }
        // `path` is relative and holds nothing a join refuses.
        base.join(path).unwrap_or(base)
    }
}

/// The `url` `value` gives, or why it is refused, its value never quoted:
/// a user written into it may hold a password.
fn address(value: Option<&Value>) -> Result<Url, String> {
    let rule = "the control plane's address is `https://`, or `http://` to a loopback address, \
                with no user, query or fragment";
    let text = match value {
        None => return Err(format!("names no `url`: {rule}, and is required")),
        Some(Value::String(text)) => text,
        Some(_) => return Err("`url` is not a string".to_string()),
    };
    let url = Url::parse(text).map_err(|e| format!("`url` is not an address ({e}): {rule}"))?;
    let loopback = match url.host() {
        Some(Host::Ipv4(ip)) => ip.is_loopback(),
        Some(Host::Ipv6(ip)) => ip.is_loopback(),
        Some(Host::Domain(domain)) => domain.eq_ignore_ascii_case("localhost"),
        None => false,
    };
    match (url.scheme(), url.host()) {
        (_, None) => return Err(format!("`url` names no host: {rule}")),
        ("https", _) => {}
        ("http", _) if loopback => {}
        ("http", _) => {
            return Err(format!(
                "`url` is `http://` to a host that is not loopback: {rule}"
            ))
        }
        (scheme, _) => return Err(format!("`url` is `{scheme}:`: {rule}")),
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(format!(
            "`url` holds a user: {rule}, and a credential is never in the table"
        ));
    }
    if url.query().is_some() {
        return Err(format!("`url` holds a query: {rule}"));
    }
    if url.fragment().is_some() {
        return Err(format!("`url` holds a fragment: {rule}"));
    }
    Ok(url)
}

/// Whether `text` is a label: 1 to [`LABEL_LENGTH`] lowercase letters,
/// digits and `-`, starting with a letter. A label holds no `/`, so an id
/// it prefixes splits at its first (design Decision 8).
pub fn is_label(text: &str) -> bool {
    text.len() <= LABEL_LENGTH
        && text.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Whether `text` is a repository's GitHub address, `owner/name`.
fn is_address(text: &str) -> bool {
    let Some((owner, name)) = text.split_once('/') else {
        return false;
    };
    let owner_ok = !owner.is_empty()
        && !owner.starts_with('-')
        && !owner.ends_with('-')
        && owner.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
    let name_ok = !name.is_empty()
        && name != "."
        && name != ".."
        && !name.to_ascii_lowercase().ends_with(".git")
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'));
    owner_ok && name_ok
}

/// Refuses each key of `table` that `read` does not hold, named and never
/// quoted, a credential's key saying where the host's key is (core.4).
fn unread(table: &toml::Table, read: &[&str], within: &str) -> Result<(), String> {
    let refused: Vec<String> = table
        .keys()
        .filter(|key| !read.contains(&key.as_str()))
        .map(|key| {
            let elsewhere = match key.as_str() {
                key if CREDENTIALS.contains(&key) => {
                    ": the host's key is kept in its credential store, made by `boomdrive enroll`"
                }
                "kinds" | "kind" | "kinds_file" => ": kinds are Flume's host configuration",
                "stream" | "streams" => ": streams are Flume's host configuration",
                _ => "",
            };
            format!(
                "{within}names `{key}`, which the hosted implementation does not read{elsewhere}"
            )
        })
        .collect();
    match refused.is_empty() {
        true => Ok(()),
        false => Err(refused.join("; ")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "/h/.config/boomdrive/config.toml";

    /// The hosted table as `text`, written under `[control_plane.hosted]`,
    /// read.
    fn read_table(text: &str) -> Result<Table, String> {
        let table: toml::Table = toml::from_str(text).expect("a TOML table");
        Table::read(&table, Path::new(FILE))
    }

    const PARTITIONS: &str =
        "[[partitions]]\nlabel = \"home\"\nrepositories = [\"example/app\"]\n\n\
                              [[partitions]]\nlabel = \"work\"\n\
                              repositories = [\"example/service\", \"example/app\"]\n";

    fn with_url(url: &str) -> String {
        format!("url = {url:?}\n\n{PARTITIONS}")
    }

    #[test]
    fn the_table_read() {
        let table = read_table(&with_url("https://control.example")).unwrap();
        assert_eq!(table.url.as_str(), "https://control.example/");
        assert_eq!(
            table.partitions,
            [
                Partition {
                    label: "home".to_string(),
                    repositories: vec!["example/app".to_string()],
                },
                Partition {
                    label: "work".to_string(),
                    repositories: vec!["example/service".to_string(), "example/app".to_string()],
                },
            ]
        );
        assert_eq!(table.partition("work"), Some(&table.partitions[1]));
        assert_eq!(table.partition("garden"), None);
    }

    #[test]
    fn a_partition_served_no_repository() {
        let table =
            read_table("url = \"https://control.example\"\n[[partitions]]\nlabel = \"home\"\nrepositories = []\n")
                .unwrap();
        assert_eq!(table.partitions[0].repositories, Vec::<String>::new());
    }

    #[test]
    fn the_paths_go_under_the_url() {
        let table = read_table(&with_url("https://control.example/boomdrive")).unwrap();
        assert_eq!(
            table.enroll_address().as_str(),
            "https://control.example/boomdrive/runner/enroll"
        );
        assert_eq!(
            table.connect_address().as_str(),
            "wss://control.example/boomdrive/runner/connect"
        );
        assert_eq!(table.origin(), "https://control.example");
        let with_slash = read_table(&with_url("https://control.example/boomdrive/")).unwrap();
        assert_eq!(with_slash.enroll_address(), table.enroll_address());
        let bare = read_table(&with_url("https://control.example")).unwrap();
        assert_eq!(
            bare.connect_address().as_str(),
            "wss://control.example/runner/connect"
        );
    }

    #[test]
    fn the_origin_lowercased_with_a_port_not_the_schemes_own() {
        let table = read_table(&with_url("HTTPS://Control.Example:8443/Boomdrive")).unwrap();
        assert_eq!(table.origin(), "https://control.example:8443");
        let default_port = read_table(&with_url("https://control.example:443")).unwrap();
        assert_eq!(default_port.origin(), "https://control.example");
        assert_eq!(table.key_name("home").account(), "home@control.example");
    }

    #[test]
    fn a_loopback_control_plane_in_the_clear() {
        for url in [
            "http://127.0.0.1:4100",
            "http://[::1]:4100/boomdrive",
            "http://localhost:4100",
        ] {
            let table = read_table(&with_url(url)).unwrap();
            assert!(
                table.connect_address().as_str().starts_with("ws://"),
                "{url}"
            );
        }
    }

    #[test]
    fn a_control_plane_in_the_clear() {
        let refusal = read_table(&with_url("http://control.example")).unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.hosted]: `url` is `http://` to a host that is not \
                 loopback: the control plane's address is `https://`, or `http://` to a loopback \
                 address, with no user, query or fragment"
            )
        );
    }

    #[test]
    fn a_url_holding_a_user_a_query_or_a_fragment() {
        let user = read_table(&with_url("https://someone:hunter2@control.example")).unwrap_err();
        assert!(user.contains("`url` holds a user"), "{user}");
        assert!(!user.contains("hunter2"), "{user}");
        assert!(read_table(&with_url("https://control.example/?a=b"))
            .unwrap_err()
            .contains("`url` holds a query"));
        assert!(read_table(&with_url("https://control.example/#a"))
            .unwrap_err()
            .contains("`url` holds a fragment"));
        assert!(read_table(&with_url("wss://control.example"))
            .unwrap_err()
            .contains("`url` is `wss:`"));
        assert!(read_table(&with_url("control.example"))
            .unwrap_err()
            .contains("`url` is not an address"));
    }

    #[test]
    fn no_url() {
        assert_eq!(
            read_table(PARTITIONS),
            Err(format!(
                "{FILE}: [control_plane.hosted]: names no `url`: the control plane's address is \
                 `https://`, or `http://` to a loopback address, with no user, query or \
                 fragment, and is required"
            ))
        );
    }

    #[test]
    fn no_partitions() {
        assert_eq!(
            read_table("url = \"https://control.example\"\n"),
            Err(format!(
                "{FILE}: [control_plane.hosted]: names no `partitions`: at least one partition \
                 is served, each a `label` and its `repositories`"
            ))
        );
        assert_eq!(
            read_table("url = \"https://control.example\"\npartitions = []\n"),
            Err(format!(
                "{FILE}: [control_plane.hosted]: `partitions` is empty: at least one partition \
                 is served"
            ))
        );
    }

    #[test]
    fn a_token_in_the_table() {
        for line in [
            "token = \"n0t-4-r34l-t0k3n\"",
            "token_file = \"/h/token\"",
            "key = \"n0t-4-r34l-k3y\"",
        ] {
            let (key, value) = line.split_once(" = ").unwrap();
            let value = value.trim_matches('"');
            let refusal = read_table(&format!("{line}\n{}", with_url("https://control.example")))
                .unwrap_err();
            assert_eq!(
                refusal,
                format!(
                    "{FILE}: [control_plane.hosted]: names `{key}`, which the hosted \
                     implementation does not read: the host's key is kept in its credential \
                     store, made by `boomdrive enroll`"
                )
            );
            assert!(!refusal.contains(value), "{refusal}");
            let in_partition = read_table(&format!(
                "url = \"https://control.example\"\n[[partitions]]\nlabel = \"home\"\n\
                 repositories = []\n{line}\n"
            ))
            .unwrap_err();
            assert!(
                in_partition.contains(&format!(
                    "`partitions` entry `home` names `{key}`, which the hosted implementation \
                     does not read: the host's key is kept in its credential store, made by \
                     `boomdrive enroll`"
                )),
                "{in_partition}"
            );
            assert!(!in_partition.contains(value), "{in_partition}");
        }
    }

    #[test]
    fn a_key_the_table_does_not_read() {
        let refusal = read_table(&format!(
            "poll_every = \"1m\"\n{}",
            with_url("https://control.example")
        ))
        .unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.hosted]: names `poll_every`, which the hosted \
                 implementation does not read"
            )
        );
    }

    #[test]
    fn a_label_twice() {
        let refusal = read_table(&format!(
            "{}\n[[partitions]]\nlabel = \"home\"\nrepositories = []\n",
            with_url("https://control.example")
        ))
        .unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.hosted]: `partitions` names `home` twice: each partition \
                 is enrolled once"
            )
        );
    }

    #[test]
    fn a_label_the_table_does_not_take() {
        for label in [
            "",
            "Home",
            "1home",
            "home/work",
            "home work",
            &"h".repeat(33),
        ] {
            let refusal = read_table(&format!(
                "url = \"https://control.example\"\n[[partitions]]\nlabel = {label:?}\n\
                 repositories = []\n"
            ))
            .unwrap_err();
            assert!(
                refusal.contains(&format!(
                    "`partitions` entry 1's `label` is {label:?}: a label is 1 to 32 lowercase \
                     letters, digits and `-`, starting with a letter"
                )),
                "{refusal}"
            );
        }
        assert!(is_label(&"h".repeat(32)));
        assert!(is_label("home-2"));
    }

    #[test]
    fn a_repository_not_owner_and_name() {
        for text in [
            "app",
            "example/",
            "/app",
            "example/app.git",
            "example/app/more",
        ] {
            let refusal = read_table(&format!(
                "url = \"https://control.example\"\n[[partitions]]\nlabel = \"home\"\n\
                 repositories = [{text:?}]\n"
            ))
            .unwrap_err();
            assert_eq!(
                refusal,
                format!(
                    "{FILE}: [control_plane.hosted]: `partitions` entry `home`'s `repositories` \
                     entry {text:?}: an address is `owner/name`"
                )
            );
        }
        assert!(read_table(
            "url = \"https://control.example\"\n[[partitions]]\nlabel = \"home\"\n"
        )
        .unwrap_err()
        .contains("`partitions` entry `home` names no `repositories`"));
    }
}
