//! sluice's command line (design Decision 13), the one way the runner
//! reaches sluice (record 0001). Each call runs in a clone's container,
//! where sluice finds the repository, and its answer is read by sluice's
//! exit and kept as sluice gave it (core.1). Of a submission's entry the
//! runner carries the fields the cloud's copy of a sluice entry holds,
//! each field's text as sluice printed it, and never a log or where one
//! is kept (core.11). The runner asks sluice for no branch: a branch
//! other places land on is a Flume place's, which sluice makes and closes
//! at Flume's asking (core.52).

use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use serde_json::value::RawValue;
use std::ffi::OsStr;
use std::fmt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

/// The states after which a submission's entry no longer changes.
pub const FINAL: [&str; 4] = ["landed", "failed", "conflict", "withdrawn"];

/// The fields of sluice's submission the runner carries, in this order.
const FIELDS: [&str; 6] = ["id", "branch", "head", "state", "failed", "landed_as"];

/// The fields of a `failed` row the runner carries, in this order.
const ROW: [&str; 4] = ["task", "hash", "flaky", "system"];

/// sluice's answer, read by its exit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Reply {
    /// Exit 0 with JSON on standard output: that output as sluice printed
    /// it.
    Given(String),
    /// Any other exit: the `message` of sluice's `error` answer, as given,
    /// or, where it printed none, its standard error.
    Refused(String),
}

/// Why `sluice` gave no answer at all: it could not be run, or a signal
/// ended it. Nothing is kept of it, and the next pass asks again.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unrun(pub String);

impl fmt::Display for Unrun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// sluice as the runner reaches it: `Cli` on a host, a stub in a test.
pub trait Sluice {
    /// `sluice status <id> --json`, the submission `id` of the repository
    /// whose clone is in `container_dir`.
    fn entry(&self, container_dir: &Path, id: &str) -> Result<Reply, Unrun>;
}

/// sluice's command line, by the program at `program`: `sluice` on the
/// host's `PATH` in the binary, a stub in a test.
#[derive(Clone, Debug)]
pub struct Cli {
    program: PathBuf,
}

impl Cli {
    pub fn new(program: impl Into<PathBuf>) -> Cli {
        Cli {
            program: program.into(),
        }
    }

    /// sluice's output from `args`, run in `dir`.
    fn run<I, S>(&self, dir: &Path, args: I) -> Result<Output, Unrun>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        Command::new(&self.program)
            .args(args)
            .current_dir(dir)
            .stdin(Stdio::null())
            .output()
            .map_err(|error| Unrun(format!("sluice could not run: {error}")))
    }
}

impl Sluice for Cli {
    fn entry(&self, container_dir: &Path, id: &str) -> Result<Reply, Unrun> {
        read(&self.run(container_dir, ["status", id, "--json"])?)
    }
}

/// sluice's `error` answer, as it prints one on a refusal.
#[derive(Deserialize)]
struct Refused {
    error: Error,
}

#[derive(Deserialize)]
struct Error {
    message: String,
}

/// sluice's answer, read by its exit, or none where a signal ended it.
fn read(output: &Output) -> Result<Reply, Unrun> {
    let code = exit(output)?;
    let printed = std::str::from_utf8(&output.stdout).ok();
    Ok(match (code, printed) {
        (0, Some(text)) if serde_json::from_str::<&RawValue>(text).is_ok() => {
            Reply::Given(text.to_string())
        }
        (0, _) => Reply::Refused(stderr(output)),
        _ => Reply::Refused(refusal(output)),
    })
}

/// sluice's exit status, or none where a signal ended it.
fn exit(output: &Output) -> Result<i32, Unrun> {
    output
        .status
        .code()
        .ok_or_else(|| Unrun(format!("sluice was ended: {}", output.status)))
}

/// The words of sluice's refusal: the `message` of the `error` answer it
/// printed, as given, or, where it printed none, its standard error.
fn refusal(output: &Output) -> String {
    let printed = std::str::from_utf8(&output.stdout).ok();
    match printed.and_then(|text| serde_json::from_str::<Refused>(text).ok()) {
        Some(refused) => refused.error.message,
        None => stderr(output),
    }
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// What the runner carries of `answer`, sluice's `status <id> --json`
/// (design Decision 13): a JSON object of its `id`, `branch`, `head`,
/// `state`, `failed` and `landed_as`, each where sluice printed it, each
/// `failed` row with only its `task`, `hash`, `flaky` and `system`. A
/// value is its text as sluice printed it; an object within a row is
/// formed of its members as sluice printed each, and holds no `log`,
/// since a `flaky` reading names where the run it contradicts kept its
/// output (core.11). None where `answer` is no JSON object.
pub fn carried(answer: &str) -> Option<String> {
    let members = serde_json::from_str::<Members>(answer).ok()?.0;
    let mut fields = Vec::new();
    for name in FIELDS {
        let Some(value) = member(&members, name) else {
            continue;
        };
        let text = match name {
            "failed" => match rows(value) {
                Some(text) => text,
                None => continue,
            },
            _ => value.get().to_string(),
        };
        fields.push((name, text));
    }
    Some(object(fields))
}

/// `value`, a `failed` array, as its rows the runner carries, or none
/// where it is no array of objects.
fn rows(value: &RawValue) -> Option<String> {
    let rows = serde_json::from_str::<Vec<&RawValue>>(value.get()).ok()?;
    let mut carried = Vec::new();
    for row in rows {
        let members = serde_json::from_str::<Members>(row.get()).ok()?.0;
        let fields = ROW
            .into_iter()
            .filter_map(|name| Some((name, without_logs(member(&members, name)?))))
            .collect();
        carried.push(object(fields));
    }
    Some(format!("[{}]", carried.join(",")))
}

/// `value`'s text as sluice printed it, but that an object is formed of
/// its members, each so, with no member named `log`, and an array of its
/// elements, each so.
fn without_logs(value: &RawValue) -> String {
    if let Ok(Members(members)) = serde_json::from_str::<Members>(value.get()) {
        let fields = members
            .iter()
            .filter(|(name, _)| name != "log")
            .map(|(name, value)| (name.as_str(), without_logs(value)))
            .collect();
        return object(fields);
    }
    if let Ok(elements) = serde_json::from_str::<Vec<&RawValue>>(value.get()) {
        let elements: Vec<String> = elements.into_iter().map(without_logs).collect();
        return format!("[{}]", elements.join(","));
    }
    value.get().to_string()
}

/// The JSON object of `fields`, each a name and its value's text, in
/// order.
fn object(fields: Vec<(&str, String)>) -> String {
    let members: Vec<String> = fields
        .into_iter()
        .map(|(name, text)| format!("{}:{text}", serde_json::Value::from(name)))
        .collect();
    format!("{{{}}}", members.join(","))
}

/// The value of the first member of `members` named `name`.
fn member<'a>(members: &[(String, &'a RawValue)], name: &str) -> Option<&'a RawValue> {
    members
        .iter()
        .find(|(named, _)| named == name)
        .map(|(_, value)| *value)
}

/// The id a carried entry names, as sluice printed it, a number or a
/// string read as its text.
pub fn id_of(carried: &str) -> Option<String> {
    let value = serde_json::from_str::<serde_json::Value>(carried).ok()?;
    match value.get("id")? {
        serde_json::Value::String(id) => Some(id.clone()),
        serde_json::Value::Number(id) => Some(id.to_string()),
        _ => None,
    }
}

/// Whether a carried entry's `state` is one after which it no longer
/// changes.
pub fn is_final(carried: &str) -> bool {
    let value = serde_json::from_str::<serde_json::Value>(carried).ok();
    let state = value
        .as_ref()
        .and_then(|value| value.get("state")?.as_str());
    state.is_some_and(|state| FINAL.contains(&state))
}

/// A JSON object's members in the order printed, each value as printed.
struct Members<'a>(Vec<(String, &'a RawValue)>);

impl<'de> Deserialize<'de> for Members<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Ordered;

        impl<'de> Visitor<'de> for Ordered {
            type Value = Members<'de>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a JSON object")
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Members<'de>, A::Error> {
                let mut members = Vec::new();
                while let Some(member) = map.next_entry::<String, &'de RawValue>()? {
                    members.push(member);
                }
                Ok(Members(members))
            }
        }

        deserializer.deserialize_map(Ordered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fields_carried_and_no_others() {
        let answer = r#"{
  "schema": "sluice/submission@1",
  "id": 7,
  "branch": "main",
  "head": "abc",
  "from": {"agent": "a"},
  "state": "testing",
  "result": null
}
"#;
        assert_eq!(
            carried(answer).unwrap(),
            r#"{"id":7,"branch":"main","head":"abc","state":"testing"}"#
        );
    }

    #[test]
    fn a_rows_log_is_left_out_wherever_it_is() {
        let answer = r#"{"id": 3, "state": "failed", "failed": [
  {"task": "t", "hash": "h", "log": "/l/t.log", "not_run": "why",
   "flaky": {"here": "pass", "other": {"outcome": "fail", "log": "/l/o.log", "at": "x"}},
   "system": "linux"}
]}"#;
        assert_eq!(
            carried(answer).unwrap(),
            r#"{"id":3,"state":"failed","failed":[{"task":"t","hash":"h","flaky":{"here":"pass","other":{"outcome":"fail","at":"x"}},"system":"linux"}]}"#
        );
    }

    #[test]
    fn a_value_is_its_text_as_printed() {
        let answer = r#"{"id": 1.50, "branch": "bé", "failed": []}"#;
        assert_eq!(
            carried(answer).unwrap(),
            r#"{"id":1.50,"branch":"bé","failed":[]}"#
        );
    }

    #[test]
    fn a_failed_that_is_no_list_of_rows_is_left_out() {
        assert_eq!(
            carried(r#"{"id": 1, "failed": "/l/t.log"}"#).unwrap(),
            r#"{"id":1}"#
        );
        assert_eq!(
            carried(r#"{"id": 1, "failed": ["/l/t.log"]}"#).unwrap(),
            r#"{"id":1}"#
        );
    }

    #[test]
    fn an_answer_that_is_no_object_carries_nothing() {
        assert_eq!(carried("[1]"), None);
        assert_eq!(carried("not JSON"), None);
    }

    #[test]
    fn an_id_and_a_final_state() {
        assert_eq!(id_of(r#"{"id":412}"#).as_deref(), Some("412"));
        assert_eq!(id_of(r#"{"id":"3"}"#).as_deref(), Some("3"));
        assert_eq!(id_of(r#"{"branch":"main"}"#), None);
        for state in FINAL {
            assert!(is_final(&format!(r#"{{"state":"{state}"}}"#)), "{state}");
        }
        for state in ["queued", "waiting", "testing", "passed", "landing"] {
            assert!(!is_final(&format!(r#"{{"state":"{state}"}}"#)), "{state}");
        }
        assert!(!is_final("{}"));
    }
}
