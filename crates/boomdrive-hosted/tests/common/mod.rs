//! What every test of the protocol checks with: validators built from one
//! version's files alone, each under its `$id`, with every other address
//! refused, so that nothing is fetched (core.42; design Decision 14), the
//! version's examples, and the fake hosted control plane. Each version is
//! a [`Version`]; the free functions are [`CURRENT`]'s, the version the
//! runner speaks.

#![allow(dead_code)]

pub mod fake;

use boomdrive_hosted::protocol::{v1, v2};
use jsonschema::{Draft, Registry, Resource, Retrieve, Uri, Validator};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// One version of the protocol, as its module names it.
#[derive(Clone, Copy, Debug)]
pub struct Version {
    /// Its number, every message's `protocol_version`.
    pub number: u32,
    /// The address each file's `$id` is under.
    pub address: &'static str,
    /// Each of its schema files by its name, with its text.
    pub files: &'static [(&'static str, &'static str)],
}

/// Version 1, as releases up to `v0.7.3` spoke it.
pub const V1: Version = Version {
    number: v1::VERSION,
    address: v1::ADDRESS,
    files: v1::FILES,
};

/// Version 2 (record 0037).
pub const V2: Version = Version {
    number: v2::VERSION,
    address: v2::ADDRESS,
    files: v2::FILES,
};

/// Each version, in order.
pub const VERSIONS: [Version; 2] = [V1, V2];

/// The version the runner speaks, which the free functions check.
pub const CURRENT: Version = V2;

/// The retriever every registry and validator is given: it refuses every
/// address, whatever features cargo unifies into `jsonschema`, so a
/// reference outside a version's files fails to resolve.
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

impl Version {
    /// The address of `file`, its `$id`.
    pub fn address(&self, file: &str) -> String {
        format!("{}{file}", self.address)
    }

    /// The address of the definition `name` of `file`.
    pub fn definition(&self, file: &str, name: &str) -> String {
        format!("{}#/$defs/{name}", self.address(file))
    }

    /// Each of its files by its name, read as JSON.
    pub fn files(&self) -> Vec<(&'static str, Value)> {
        self.files
            .iter()
            .map(|(name, text)| {
                let value = serde_json::from_str(text).unwrap_or_else(|e| panic!("{name}: {e}"));
                (*name, value)
            })
            .collect()
    }

    /// Every one of its files under its `$id`, and nothing else.
    fn registry(&self) -> &'static Registry<'static> {
        static REGISTRIES: [OnceLock<Registry<'static>>; 2] = [OnceLock::new(), OnceLock::new()];
        let cell = &REGISTRIES[(self.number - 1) as usize];
        cell.get_or_init(|| {
            let resources = self
                .files()
                .into_iter()
                .map(|(name, value)| (self.address(name), Resource::from_contents(value)));
            Registry::new()
                .draft(Draft::Draft202012)
                .retriever(Refuse(self.number))
                .extend(resources)
                .and_then(|builder| builder.prepare())
                .unwrap_or_else(|e| panic!("version {}'s files as a registry: {e}", self.number))
        })
    }

    /// A validator of what `reference` names, a file or one of its
    /// definitions, built from its files alone, or why it could not be
    /// built.
    pub fn try_validator(&self, reference: &str) -> Result<Validator, String> {
        jsonschema::options()
            .with_draft(Draft::Draft202012)
            .with_registry(self.registry())
            .with_retriever(Refuse(self.number))
            .build(&serde_json::json!({ "$ref": reference }))
            .map_err(|e| format!("{reference}: {e}"))
    }

    /// A validator of what `reference` names.
    pub fn validator(&self, reference: &str) -> Validator {
        self.try_validator(reference)
            .unwrap_or_else(|e| panic!("{e}"))
    }

    /// The validator of a runner's messages.
    pub fn from_runner(&self) -> Validator {
        self.validator(&self.address("from-runner.schema.json"))
    }

    /// The validator of a control plane's messages.
    pub fn to_runner(&self) -> Validator {
        self.validator(&self.address("to-runner.schema.json"))
    }

    /// The directory of its files.
    pub fn dir(&self) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("protocol/v{}", self.number))
    }

    /// Each example of `direction`, `from-runner` or `to-runner`, by its
    /// file name, read as JSON, in the order of their names.
    pub fn examples(&self, direction: &str) -> Vec<(String, Value)> {
        let dir = self.dir().join("examples").join(direction);
        let mut names: Vec<String> = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".json"))
            .collect();
        names.sort();
        names
            .into_iter()
            .map(|name| {
                let value = self.example(direction, &name);
                (name, value)
            })
            .collect()
    }

    /// The example `name` of `direction`, read as JSON.
    pub fn example(&self, direction: &str, name: &str) -> Value {
        let file = self.dir().join("examples").join(direction).join(name);
        let text =
            std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
        serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", file.display()))
    }
}

/// The address of `file` in the current version, its `$id`.
pub fn address(file: &str) -> String {
    CURRENT.address(file)
}

/// The address of the definition `name` of `file` in the current version.
pub fn definition(file: &str, name: &str) -> String {
    CURRENT.definition(file, name)
}

/// Each file of the current version by its name, read as JSON.
pub fn files() -> Vec<(&'static str, Value)> {
    CURRENT.files()
}

/// A validator of what `reference` names in the current version, or why
/// it could not be built.
pub fn try_validator(reference: &str) -> Result<Validator, String> {
    CURRENT.try_validator(reference)
}

/// A validator of what `reference` names in the current version.
pub fn validator(reference: &str) -> Validator {
    CURRENT.validator(reference)
}

/// The validator of a runner's messages in the current version.
pub fn from_runner() -> Validator {
    CURRENT.from_runner()
}

/// The validator of a control plane's messages in the current version.
pub fn to_runner() -> Validator {
    CURRENT.to_runner()
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

/// The directory of the current version's files.
pub fn version_dir() -> PathBuf {
    CURRENT.dir()
}

/// Each example of `direction` in the current version, by its file name.
pub fn examples(direction: &str) -> Vec<(String, Value)> {
    CURRENT.examples(direction)
}

/// The example `name` of `direction` in the current version.
pub fn example(direction: &str, name: &str) -> Value {
    CURRENT.example(direction, name)
}
