//! The validators of version 2 (design Decisions 9 and 10; core.42):
//! built once from its files alone, each under its `$id`, with every other
//! address refused, so nothing is fetched. A check's words name the
//! instance's path and the schema's, and never a value, since a value may
//! be text the control plane must not hold (core.11).

use crate::protocol::v2::{ADDRESS, FILES};
use jsonschema::{Draft, Registry, Resource, Retrieve, Uri, Validator};
use serde_json::Value;
use std::sync::OnceLock;

/// The retriever every registry and validator is given: it refuses every
/// address, so a reference outside [`FILES`] fails to resolve.
struct Refuse;

impl Retrieve for Refuse {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!("{uri} is none of version 2's files, and nothing is fetched").into())
    }
}

/// Every file of [`FILES`] under its `$id`, and nothing else.
fn registry() -> &'static Registry<'static> {
    static REGISTRY: OnceLock<Registry<'static>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let resources = FILES.iter().map(|(name, text)| {
            // Each file is the crate's own, read by its tests.
            let value: Value = serde_json::from_str(text).unwrap_or(Value::Null);
            (format!("{ADDRESS}{name}"), Resource::from_contents(value))
        });
        // Version 2's files are the crate's own, which its tests build.
        Registry::new()
            .draft(Draft::Draft202012)
            .retriever(Refuse)
            .extend(resources)
            .and_then(|builder| builder.prepare())
            .expect("version 2's files make a registry, as its tests check")
    })
}

/// The validator of what `reference` names under [`ADDRESS`]: a file, or
/// one of its definitions.
fn validator(reference: &str) -> Validator {
    jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(registry())
        .with_retriever(Refuse)
        .build(&serde_json::json!({ "$ref": format!("{ADDRESS}{reference}") }))
        .expect("each of version 2's references resolves among its files, as its tests check")
}

/// The validator of every message a runner sends, its envelope
/// `from-runner.schema.json`.
pub fn from_runner() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator("from-runner.schema.json"))
}

/// The validator of every message a control plane sends, its envelope
/// `to-runner.schema.json`.
pub fn to_runner() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator("to-runner.schema.json"))
}

/// The validator of a message's own id, `shared.schema.json`'s
/// `message_id`, by which a message that fails its envelope is still
/// named in its refusal.
pub fn message_id() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator("shared.schema.json#/$defs/message_id"))
}

/// The validator of a hello's body, `hello.schema.json`'s `hello`, whose
/// failure names the path deep in it.
pub fn hello() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator("hello.schema.json#/$defs/hello"))
}

/// The validator of a report's body, `report.schema.json`'s `report`,
/// whose failure names the path deep in it.
pub fn report() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator("report.schema.json#/$defs/report"))
}

/// The validator of a place's status as its copy, `place-status.schema.json`'s
/// `place_status`.
pub fn place_status() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::PLACE_STATUS))
}

/// The validator of one end of a place's status copy,
/// `place-status.schema.json`'s `end_record`: a form version 2 names, or
/// an end's delivery alone.
pub fn end_record() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::END_RECORD))
}

/// The validator of sluice's entry as its copy,
/// `submission-entry.schema.json`'s `submission_entry`.
pub fn submission_entry() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::SUBMISSION_ENTRY))
}

/// The validator of a preview as its copy, `preview.schema.json`'s
/// `preview`.
pub fn preview() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::PREVIEW))
}

/// The validator of a release as its copy, `preview.schema.json`'s
/// `release`.
pub fn release() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::RELEASE))
}

/// The validator of Flume's host part as its copy, `hello.schema.json`'s
/// `host_part`.
pub fn host_part() -> &'static Validator {
    static VALIDATOR: OnceLock<Validator> = OnceLock::new();
    VALIDATOR.get_or_init(|| validator(crate::copy::HOST_PART))
}

/// Why `value` fails `validator`: its first error's path in the value and
/// in the schema, and no value; or none, where it passes.
pub fn failure(validator: &Validator, value: &Value) -> Option<String> {
    validator.iter_errors(value).next().map(|error| {
        let at = error.instance_path().to_string();
        let at = match at.is_empty() {
            true => "/".to_string(),
            false => at,
        };
        format!("{at} fails the schema's {}", error.schema_path())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn a_message_of_each_direction_passes_its_own() {
        let enrolled = json!({
            "protocol_version": 2,
            "id": "plane:1",
            "body": { "enrolled": { "host": "h-1" } }
        });
        assert_eq!(failure(to_runner(), &enrolled), None);
        assert!(failure(from_runner(), &enrolled).is_some());
    }

    #[test]
    fn a_failure_names_paths_and_no_value() {
        let refused = json!({
            "protocol_version": 2,
            "id": "plane:1",
            "body": { "enroll_refused": { "enroll": "runner:1", "words": "lapsed", "secret": "n0t-sh0wn" } }
        });
        let words = failure(to_runner(), &refused).expect("a field version 2 does not name fails");
        assert!(words.starts_with('/'), "{words}");
        assert!(words.contains("fails the schema's"), "{words}");
        assert!(!words.contains("n0t-sh0wn"), "{words}");
    }
}
