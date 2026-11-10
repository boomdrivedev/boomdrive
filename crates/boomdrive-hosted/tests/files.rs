//! Each version is JSON Schema of draft 2020-12 in this crate, each file
//! under its own address, every reference among its own files and none
//! fetched, and its module's `FILES` holding each file (the
//! runner-protocol spec's "Each version of the protocol is JSON Schema in
//! the hosted implementation's crate"; design Decisions 1 and 6 of
//! the-hosted-protocol-carries-every-end-of-a-place).

mod common;

use common::VERSIONS;
use serde_json::Value;
use std::collections::BTreeSet;

/// Every `$ref` in `value`, at any depth.
fn references(value: &Value, found: &mut Vec<String>) {
    match value {
        Value::Object(members) => {
            for (key, member) in members {
                match (key.as_str(), member) {
                    ("$ref", Value::String(reference)) => found.push(reference.clone()),
                    _ => references(member, found),
                }
            }
        }
        Value::Array(elements) => elements.iter().for_each(|e| references(e, found)),
        _ => {}
    }
}

#[test]
fn each_file_is_a_schema_of_its_draft() {
    for version in VERSIONS {
        let n = version.number;
        for (name, value) in version.files() {
            if let Err(error) = jsonschema::draft202012::meta::validate(&value) {
                panic!("version {n}'s {name} fails the 2020-12 metaschema: {error}");
            }
            assert_eq!(
                value["$schema"], "https://json-schema.org/draft/2020-12/schema",
                "version {n}'s {name}'s draft"
            );
            assert_eq!(
                value["$id"],
                version.address(name).as_str(),
                "version {n}'s {name}'s $id"
            );
        }
        assert!(
            version
                .address
                .ends_with(&format!("/crates/boomdrive-hosted/protocol/v{n}/")),
            "version {n}'s address {}",
            version.address
        );
    }
    assert_eq!(common::V1.number, 1);
    assert_eq!(common::V2.number, 2);
}

#[test]
fn nothing_outside_the_version_is_referenced() {
    for version in VERSIONS {
        let n = version.number;
        let names: BTreeSet<&str> = version.files.iter().map(|(name, _)| *name).collect();
        for (name, value) in version.files() {
            let mut found = Vec::new();
            references(&value, &mut found);
            for reference in found {
                let file = reference.split('#').next().unwrap_or_default();
                assert!(
                    file.is_empty() || names.contains(file),
                    "version {n}'s {name} refers to {reference}, outside its files"
                );
            }
            version.validator(&version.address(name));
            let defs = value["$defs"].as_object().cloned().unwrap_or_default();
            for def in defs.keys() {
                version.validator(&version.definition(name, def));
            }
        }
        let outside = "https://example.com/elsewhere.schema.json";
        assert!(
            version.try_validator(outside).is_err(),
            "a reference outside version {n}'s files resolves, so it was fetched"
        );
        let unknown = version.address("health.schema.json");
        assert!(
            version.try_validator(&unknown).is_err(),
            "{unknown} resolves"
        );
        let other = VERSIONS
            .iter()
            .find(|v| v.number != n)
            .expect("another version");
        let across = other.address("from-runner.schema.json");
        assert!(
            version.try_validator(&across).is_err(),
            "version {}'s file resolves among version {n}'s",
            other.number
        );
    }
}

#[test]
fn every_file_is_listed() {
    for version in VERSIONS {
        let on_disk: BTreeSet<String> = std::fs::read_dir(version.dir())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".schema.json"))
            .collect();
        let listed: BTreeSet<String> = version
            .files
            .iter()
            .map(|(name, _)| name.to_string())
            .collect();
        assert_eq!(
            listed.len(),
            version.files.len(),
            "a file of version {} listed twice",
            version.number
        );
        assert_eq!(listed, on_disk, "version {}'s files", version.number);
    }
}
