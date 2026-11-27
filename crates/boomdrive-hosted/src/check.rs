//! The validators of each version the runner speaks (design Decisions 9
//! and 10; core.42): built once for each version from its files alone, each under its `$id`, with every other
//! address refused, so nothing is fetched. A check's words name the
//! instance's path and the schema's, and never a value, since a value may
//! be text the control plane must not hold (core.11).

use crate::protocol::Version;
use jsonschema::{Draft, Registry, Resource, Retrieve, Uri, Validator};
use serde_json::Value;
use std::sync::OnceLock;

/// The retriever every registry and validator is given: it refuses every
/// address, so a reference outside a version's files fails to resolve.
struct Refuse(u32);

impl Retrieve for Refuse {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!(
            "{uri} is none of version {}'s files, and nothing is fetched",
            self.0
        )
        .into())
    }
}

/// A version's registry and validators, each built at its first use.
pub(crate) struct Checks {
    registry: OnceLock<Registry<'static>>,
    from_runner: OnceLock<Validator>,
    to_runner: OnceLock<Validator>,
    message_id: OnceLock<Validator>,
    hello: OnceLock<Validator>,
    report: OnceLock<Validator>,
    place_status: OnceLock<Validator>,
    end_record: OnceLock<Validator>,
    submission_entry: OnceLock<Validator>,
    preview: OnceLock<Validator>,
    release: OnceLock<Validator>,
    host_part: OnceLock<Validator>,
    repository_status: OnceLock<Validator>,
}

impl Checks {
    pub(crate) const fn new() -> Checks {
        Checks {
            registry: OnceLock::new(),
            from_runner: OnceLock::new(),
            to_runner: OnceLock::new(),
            message_id: OnceLock::new(),
            hello: OnceLock::new(),
            report: OnceLock::new(),
            place_status: OnceLock::new(),
            end_record: OnceLock::new(),
            submission_entry: OnceLock::new(),
            preview: OnceLock::new(),
            release: OnceLock::new(),
            host_part: OnceLock::new(),
            repository_status: OnceLock::new(),
        }
    }
}

/// Every file of `version` under its `$id`, and nothing else.
fn registry(version: &'static Version) -> &'static Registry<'static> {
    version.checks.registry.get_or_init(|| {
        let resources = version.files.iter().map(|(name, text)| {
            // Each file is the crate's own, read by its tests.
            let value: Value = serde_json::from_str(text).unwrap_or(Value::Null);
            (
                format!("{}{name}", version.address),
                Resource::from_contents(value),
            )
        });
        // The version's files are the crate's own, which its tests build.
        Registry::new()
            .draft(Draft::Draft202012)
            .retriever(Refuse(version.number))
            .extend(resources)
            .and_then(|builder| builder.prepare())
            .expect("the version's files make a registry, as its tests check")
    })
}

/// The validator of what `reference` names under `version`'s address: a
/// file, or one of its definitions.
fn validator(version: &'static Version, reference: &str) -> Validator {
    jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(registry(version))
        .with_retriever(Refuse(version.number))
        .build(&serde_json::json!({ "$ref": format!("{}{reference}", version.address) }))
        .expect("each of the version's references resolves among its files, as its tests check")
}

/// The validator of every message a runner sends, its envelope
/// `from-runner.schema.json`.
pub fn from_runner(version: &'static Version) -> &'static Validator {
    version
        .checks
        .from_runner
        .get_or_init(|| validator(version, "from-runner.schema.json"))
}

/// The validator of every message a control plane sends, its envelope
/// `to-runner.schema.json`.
pub fn to_runner(version: &'static Version) -> &'static Validator {
    version
        .checks
        .to_runner
        .get_or_init(|| validator(version, "to-runner.schema.json"))
}

/// The validator of a message's own id, `shared.schema.json`'s
/// `message_id`, by which a message that fails its envelope is still
/// named in its refusal.
pub fn message_id(version: &'static Version) -> &'static Validator {
    version
        .checks
        .message_id
        .get_or_init(|| validator(version, "shared.schema.json#/$defs/message_id"))
}

/// The validator of a hello's body, `hello.schema.json`'s `hello`, whose
/// failure names the path deep in it.
pub fn hello(version: &'static Version) -> &'static Validator {
    version
        .checks
        .hello
        .get_or_init(|| validator(version, "hello.schema.json#/$defs/hello"))
}

/// The validator of a report's body, `report.schema.json`'s `report`,
/// whose failure names the path deep in it.
pub fn report(version: &'static Version) -> &'static Validator {
    version
        .checks
        .report
        .get_or_init(|| validator(version, "report.schema.json#/$defs/report"))
}

/// The validator of a place's status as its copy, `place-status.schema.json`'s
/// `place_status`.
pub fn place_status(version: &'static Version) -> &'static Validator {
    version
        .checks
        .place_status
        .get_or_init(|| validator(version, crate::copy::PLACE_STATUS))
}

/// The validator of one end of a place's status copy,
/// `place-status.schema.json`'s `end_record`: a form version 5 names, or
/// an end's delivery alone.
pub fn end_record(version: &'static Version) -> &'static Validator {
    version
        .checks
        .end_record
        .get_or_init(|| validator(version, crate::copy::END_RECORD))
}

/// The validator of sluice's entry as its copy,
/// `submission-entry.schema.json`'s `submission_entry`.
pub fn submission_entry(version: &'static Version) -> &'static Validator {
    version
        .checks
        .submission_entry
        .get_or_init(|| validator(version, crate::copy::SUBMISSION_ENTRY))
}

/// The validator of a preview as its copy, `preview.schema.json`'s
/// `preview`.
pub fn preview(version: &'static Version) -> &'static Validator {
    version
        .checks
        .preview
        .get_or_init(|| validator(version, crate::copy::PREVIEW))
}

/// The validator of a release as its copy, `preview.schema.json`'s
/// `release`.
pub fn release(version: &'static Version) -> &'static Validator {
    version
        .checks
        .release
        .get_or_init(|| validator(version, crate::copy::RELEASE))
}

/// The validator of Flume's host part as its copy, `hello.schema.json`'s
/// `host_part`.
pub fn host_part(version: &'static Version) -> &'static Validator {
    version
        .checks
        .host_part
        .get_or_init(|| validator(version, crate::copy::HOST_PART))
}

/// The validator of Flume's status of a repository as its copy,
/// `hello.schema.json`'s `repository_status_copy`, which version 7 names
/// and no version before it.
pub fn repository_status(version: &'static Version) -> &'static Validator {
    version
        .checks
        .repository_status
        .get_or_init(|| validator(version, crate::copy::REPOSITORY_STATUS))
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
    use crate::protocol::V5;
    use serde_json::json;

    #[test]
    fn a_message_of_each_direction_passes_its_own() {
        let enrolled = json!({
            "protocol_version": V5.number,
            "id": "plane:1",
            "body": { "enrolled": { "host": "h-1" } }
        });
        assert_eq!(failure(to_runner(&V5), &enrolled), None);
        assert!(failure(from_runner(&V5), &enrolled).is_some());
    }

    #[test]
    fn a_failure_names_paths_and_no_value() {
        let refused = json!({
            "protocol_version": V5.number,
            "id": "plane:1",
            "body": { "enroll_refused": { "enroll": "runner:1", "words": "lapsed", "secret": "n0t-sh0wn" } }
        });
        let words =
            failure(to_runner(&V5), &refused).expect("a field the version does not name fails");
        assert!(words.starts_with('/'), "{words}");
        assert!(words.contains("fails the schema's"), "{words}");
        assert!(!words.contains("n0t-sh0wn"), "{words}");
    }
}
