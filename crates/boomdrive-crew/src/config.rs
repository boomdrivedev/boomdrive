//! crew's table, `[control_plane.crew]` (design Decision 4): crew's name
//! for this host, and each partition served with its state repository.
//! Every key is named for what it holds (core.12), and the table names no
//! credential: a state repository is reached through the host's own git
//! credential helper (core.4). A refusal names the file and the key, and
//! never quotes a key the implementation does not read, a value put there
//! by mistake among it.

use std::fmt;
use std::path::Path;
use toml::{Table, Value};

/// The table as the runner's configuration names it.
const TABLE: &str = "[control_plane.crew]";

/// The keys crew's implementation reads, at the table's top and in each
/// partition.
const KEYS: [&str; 2] = ["host", "partitions"];
const PARTITION_KEYS: [&str; 2] = ["label", "state_repository"];

/// crew's table, read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// This host as crew's hosts file names it: the `Host` of the
    /// requests it answers and of the role records whose runs it keeps.
    pub host: String,
    /// Each partition served, in the table's order.
    pub partitions: Vec<Partition>,
}

/// A partition served: its label, as crew writes it, and its state
/// repository.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Partition {
    pub label: String,
    pub state_repository: StateRepository,
}

/// A state repository's GitHub address, `owner/name`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateRepository {
    pub owner: String,
    pub name: String,
}

impl StateRepository {
    /// The address `text` writes, or the rule it breaks.
    pub fn parse(text: &str) -> Result<StateRepository, &'static str> {
        let rule = "a state repository is `owner/name`";
        let (owner, name) = text.split_once('/').ok_or(rule)?;
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
        if !owner_ok || !name_ok {
            return Err(rule);
        }
        Ok(StateRepository {
            owner: owner.to_string(),
            name: name.to_string(),
        })
    }

    /// Where the repository is reached on `forge`, a base address ending
    /// in `/`: `https://github.com/<owner>/<name>.git` on GitHub.
    pub fn url(&self, forge: &str) -> String {
        format!("{forge}{}/{}.git", self.owner, self.name)
    }
}

impl fmt::Display for StateRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.name)
    }
}

/// Whether `text` is a name as crew writes a host's or a label: lower-case
/// letters and digits, then those and hyphens.
pub fn is_name(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

/// Reads crew's table, `table`, from the configuration file `file`.
pub fn read(table: &Table, file: &Path) -> Result<Config, String> {
    let refuse = |words: String| format!("{}: {TABLE}: {words}", file.display());
    unread(table, &KEYS, "").map_err(refuse)?;

    let host = match table.get("host") {
        None => {
            return Err(refuse(
                "names no `host`: crew's name for this host, as crew's hosts file names it, \
                 is required"
                    .to_string(),
            ))
        }
        Some(Value::String(host)) if is_name(host) => host.clone(),
        Some(Value::String(host)) => {
            return Err(refuse(format!(
                "`host` is {host:?}: a host is named as crew names it, lower-case letters, \
                 digits and inner hyphens"
            )))
        }
        Some(_) => return Err(refuse("`host` is not a string".to_string())),
    };

    let entries = match table.get("partitions") {
        None => {
            return Err(refuse(
                "names no `partitions`: at least one partition is served, each a `label` \
                 and a `state_repository`"
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
                 `[[control_plane.crew.partitions]]`"
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
                    "{which} names no `label`: the partition's label, as crew writes it, \
                     is required"
                )))
            }
            Some(Value::String(label)) if is_name(label) => label.clone(),
            Some(Value::String(label)) => {
                return Err(refuse(format!(
                    "{which}'s `label` is {label:?}: a label is lower-case letters, digits \
                     and inner hyphens, as crew writes it"
                )))
            }
            Some(_) => return Err(refuse(format!("{which}'s `label` is not a string"))),
        };
        let which = format!("`partitions` entry `{label}`");
        unread(entry, &PARTITION_KEYS, &format!("{which} ")).map_err(refuse)?;
        let state_repository = match entry.get("state_repository") {
            None => {
                return Err(refuse(format!(
                    "{which} names no `state_repository`: the repository holding the \
                     partition's drive branch, `owner/name`, is required"
                )))
            }
            Some(Value::String(text)) => StateRepository::parse(text).map_err(|rule| {
                refuse(format!("{which}'s `state_repository` is {text:?}: {rule}"))
            })?,
            Some(_) => {
                return Err(refuse(format!(
                    "{which}'s `state_repository` is not a string"
                )))
            }
        };
        if partitions.iter().any(|served| served.label == label) {
            return Err(refuse(format!(
                "`partitions` names `{label}` twice: each partition is served once"
            )));
        }
        partitions.push(Partition {
            label,
            state_repository,
        });
    }
    Ok(Config { host, partitions })
}

/// Refuses each key of `table` that `read` does not hold, named and never
/// quoted, `kinds` saying whose configuration kinds are (core.5).
fn unread(table: &Table, read: &[&str], within: &str) -> Result<(), String> {
    let refused: Vec<String> = table
        .keys()
        .filter(|key| !read.contains(&key.as_str()))
        .map(|key| {
            let elsewhere = match key.as_str() {
                "kinds" | "kind" | "kinds_file" => ": kinds are Flume's host configuration",
                "stream" | "streams" => ": streams are Flume's host configuration",
                _ => "",
            };
            format!("{within}names `{key}`, which crew's implementation does not read{elsewhere}")
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

    const FILE: &str = "/home/u/.config/boomdrive/config.toml";

    /// crew's table as `text`, written under `[control_plane.crew]`, read.
    fn read_table(text: &str) -> Result<Config, String> {
        let table: Table = toml::from_str(text).expect("a TOML table");
        read(&table, Path::new(FILE))
    }

    fn madswan() -> Partition {
        Partition {
            label: "madswan".to_string(),
            state_repository: StateRepository::parse("afterthought/madswan-state").unwrap(),
        }
    }

    const EXAMPLE: &str = "host = \"deneb\"\n\n[[partitions]]\nlabel = \"madswan\"\n\
                           state_repository = \"afterthought/madswan-state\"\n";

    #[test]
    fn the_table_read() {
        assert_eq!(
            read_table(EXAMPLE),
            Ok(Config {
                host: "deneb".to_string(),
                partitions: vec![madswan()],
            })
        );
        assert_eq!(
            madswan().state_repository.url("https://github.com/"),
            "https://github.com/afterthought/madswan-state.git"
        );
    }

    #[test]
    fn two_partitions_in_the_tables_order() {
        let config = read_table(&format!(
            "{EXAMPLE}\n[[partitions]]\nlabel = \"swancloud\"\n\
             state_repository = \"afterthought/swancloud-state\"\n"
        ))
        .unwrap();
        let labels: Vec<&str> = config.partitions.iter().map(|p| p.label.as_str()).collect();
        assert_eq!(labels, ["madswan", "swancloud"]);
    }

    #[test]
    fn no_host() {
        assert_eq!(
            read_table(
                "[[partitions]]\nlabel = \"madswan\"\n\
                 state_repository = \"afterthought/madswan-state\"\n"
            ),
            Err(format!(
                "{FILE}: [control_plane.crew]: names no `host`: crew's name for this host, as \
                 crew's hosts file names it, is required"
            ))
        );
    }

    #[test]
    fn no_partitions() {
        assert_eq!(
            read_table("host = \"deneb\"\n"),
            Err(format!(
                "{FILE}: [control_plane.crew]: names no `partitions`: at least one partition \
                 is served, each a `label` and a `state_repository`"
            ))
        );
        assert_eq!(
            read_table("host = \"deneb\"\npartitions = []\n"),
            Err(format!(
                "{FILE}: [control_plane.crew]: `partitions` is empty: at least one partition \
                 is served"
            ))
        );
    }

    #[test]
    fn no_label() {
        assert_eq!(
            read_table(
                "host = \"deneb\"\n[[partitions]]\n\
                 state_repository = \"afterthought/madswan-state\"\n"
            ),
            Err(format!(
                "{FILE}: [control_plane.crew]: `partitions` entry 1 names no `label`: the \
                 partition's label, as crew writes it, is required"
            ))
        );
    }

    #[test]
    fn no_state_repository() {
        assert_eq!(
            read_table("host = \"deneb\"\n[[partitions]]\nlabel = \"madswan\"\n"),
            Err(format!(
                "{FILE}: [control_plane.crew]: `partitions` entry `madswan` names no \
                 `state_repository`: the repository holding the partition's drive branch, \
                 `owner/name`, is required"
            ))
        );
    }

    #[test]
    fn a_label_twice() {
        let refusal = read_table(&format!("{EXAMPLE}\n[[partitions]]\nlabel = \"madswan\"\nstate_repository = \"afterthought/other\"\n"))
            .unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.crew]: `partitions` names `madswan` twice: each \
                 partition is served once"
            )
        );
    }

    #[test]
    fn a_state_repository_not_owner_and_name() {
        for text in [
            "madswan-state",
            "afterthought/",
            "/madswan-state",
            "a/b.git",
            "a/b/c",
        ] {
            let refusal = read_table(&format!(
                "host = \"deneb\"\n[[partitions]]\nlabel = \"madswan\"\n\
                 state_repository = {text:?}\n"
            ))
            .unwrap_err();
            assert_eq!(
                refusal,
                format!(
                    "{FILE}: [control_plane.crew]: `partitions` entry `madswan`'s \
                     `state_repository` is {text:?}: a state repository is `owner/name`"
                )
            );
        }
    }

    #[test]
    fn a_host_or_a_label_crew_would_not_write() {
        assert!(read_table(&EXAMPLE.replace("deneb", "Deneb"))
            .unwrap_err()
            .contains("`host` is \"Deneb\": a host is named as crew names it"));
        assert!(read_table(&EXAMPLE.replace("\"madswan\"", "\"mad swan\""))
            .unwrap_err()
            .contains("`partitions` entry 1's `label` is \"mad swan\""));
        assert_eq!(
            read_table("host = 3\n"),
            Err(format!(
                "{FILE}: [control_plane.crew]: `host` is not a string"
            ))
        );
    }

    #[test]
    fn a_kinds_file_named() {
        let refusal = read_table(&format!(
            "kinds = {{ crew = \"/k/kinds.toml\" }}\n{EXAMPLE}"
        ))
        .unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.crew]: names `kinds`, which crew's implementation \
                 does not read: kinds are Flume's host configuration"
            )
        );
        assert!(!refusal.contains("/k/kinds.toml"), "{refusal}");
    }

    #[test]
    fn a_kinds_file_named_in_a_partition() {
        let refusal =
            read_table(&format!("{EXAMPLE}kinds_file = \"/k/kinds.toml\"\n")).unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.crew]: `partitions` entry `madswan` names \
                 `kinds_file`, which crew's implementation does not read: kinds are Flume's \
                 host configuration"
            )
        );
    }

    #[test]
    fn a_token_in_crews_table() {
        let refusal = read_table(&format!("token = \"ghp_n0t4r34lt0k3n\"\n{EXAMPLE}")).unwrap_err();
        assert_eq!(
            refusal,
            format!(
                "{FILE}: [control_plane.crew]: names `token`, which crew's implementation \
                 does not read"
            )
        );
        assert!(!refusal.contains("ghp_n0t4r34lt0k3n"), "{refusal}");
        let in_partition =
            read_table(&format!("{EXAMPLE}token = \"ghp_n0t4r34lt0k3n\"\n")).unwrap_err();
        assert!(
            !in_partition.contains("ghp_n0t4r34lt0k3n"),
            "{in_partition}"
        );
        assert!(in_partition.contains("names `token`"), "{in_partition}");
    }
}
