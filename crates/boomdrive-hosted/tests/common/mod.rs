//! What every test of the protocol checks with: validators built from
//! version 1's files alone, each under its `$id`, with every other
//! address refused, so that nothing is fetched (core.42; design Decision
//! 14), and the version's examples.

#![allow(dead_code)]

use boomdrive_hosted::protocol::v1::{ADDRESS, FILES};
use jsonschema::{Draft, Registry, Resource, Retrieve, Uri, Validator};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// The retriever every registry and validator is given: it refuses every
/// address, whatever features cargo unifies into `jsonschema`, so a
/// reference outside `FILES` fails to resolve.
struct Refuse;

impl Retrieve for Refuse {
    fn retrieve(
        &self,
        uri: &Uri<String>,
    ) -> Result<Value, Box<dyn std::error::Error + Send + Sync>> {
        Err(format!("{uri} is none of version 1's files, and nothing is fetched").into())
    }
}

/// The address of `file`, its `$id`.
pub fn address(file: &str) -> String {
    format!("{ADDRESS}{file}")
}

/// The address of the definition `name` of `file`.
pub fn definition(file: &str, name: &str) -> String {
    format!("{}#/$defs/{name}", address(file))
}

/// Each file of `FILES` by its name, read as JSON.
pub fn files() -> Vec<(&'static str, Value)> {
    FILES
        .iter()
        .map(|(name, text)| {
            let value = serde_json::from_str(text).unwrap_or_else(|e| panic!("{name}: {e}"));
            (*name, value)
        })
        .collect()
}

/// Every file of `FILES` under its `$id`, and nothing else.
fn registry() -> &'static Registry<'static> {
    static REGISTRY: OnceLock<Registry<'static>> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        let resources = files()
            .into_iter()
            .map(|(name, value)| (address(name), Resource::from_contents(value)));
        Registry::new()
            .draft(Draft::Draft202012)
            .retriever(Refuse)
            .extend(resources)
            .and_then(|builder| builder.prepare())
            .unwrap_or_else(|e| panic!("version 1's files as a registry: {e}"))
    })
}

/// A validator of what `reference` names, a file or one of its
/// definitions, built from `FILES` alone, or why it could not be built.
pub fn try_validator(reference: &str) -> Result<Validator, String> {
    jsonschema::options()
        .with_draft(Draft::Draft202012)
        .with_registry(registry())
        .with_retriever(Refuse)
        .build(&serde_json::json!({ "$ref": reference }))
        .map_err(|e| format!("{reference}: {e}"))
}

/// A validator of what `reference` names.
pub fn validator(reference: &str) -> Validator {
    try_validator(reference).unwrap_or_else(|e| panic!("{e}"))
}

/// The validator of a runner's messages.
pub fn from_runner() -> Validator {
    validator(&address("from-runner.schema.json"))
}

/// The validator of a control plane's messages.
pub fn to_runner() -> Validator {
    validator(&address("to-runner.schema.json"))
}

/// Why `validator` refuses `value`, one line for each error, or nothing
/// where it passes.
pub fn errors(validator: &Validator, value: &Value) -> Vec<String> {
    validator
        .iter_errors(value)
        .map(|e| format!("{}: {e}", e.instance_path()))
        .collect()
}

/// `value` passes `validator`, or the test fails naming `what` and why.
pub fn passes(validator: &Validator, value: &Value, what: &str) {
    let errors = errors(validator, value);
    assert!(errors.is_empty(), "{what} is refused: {errors:#?}");
}

/// `value` is refused by `validator`, or the test fails naming `what`.
pub fn refused(validator: &Validator, value: &Value, what: &str) {
    assert!(!validator.is_valid(value), "{what} passes: {value:#}");
}

/// The directory of version 1's files.
pub fn version_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol/v1")
}

/// Each example of `direction`, `from-runner` or `to-runner`, by its file
/// name, read as JSON, in the order of their names.
pub fn examples(direction: &str) -> Vec<(String, Value)> {
    let dir = version_dir().join("examples").join(direction);
    let mut names: Vec<String> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".json"))
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let value = example(direction, &name);
            (name, value)
        })
        .collect()
}

/// The example `name` of `direction`, read as JSON.
pub fn example(direction: &str, name: &str) -> Value {
    let file = version_dir().join("examples").join(direction).join(name);
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", file.display()))
}
