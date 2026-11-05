//! Version 1 is JSON Schema of draft 2020-12 in this crate, each file
//! under its own address, every reference among its own files and none
//! fetched, and `FILES` holding each file (the runner-protocol spec's
//! "Each version of the protocol is JSON Schema in the hosted
//! implementation's crate"; design Decisions 2 and 14).

mod common;

use boomdrive_hosted::protocol::v1::{ADDRESS, FILES, VERSION};
use common::{address, files, try_validator, validator, version_dir};
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
    for (name, value) in files() {
        if let Err(error) = jsonschema::draft202012::meta::validate(&value) {
            panic!("{name} fails the 2020-12 metaschema: {error}");
        }
        assert_eq!(
            value["$schema"], "https://json-schema.org/draft/2020-12/schema",
            "{name}'s draft"
        );
        assert_eq!(value["$id"], address(name).as_str(), "{name}'s $id");
    }
    assert_eq!(VERSION, 1);
    assert!(ADDRESS.ends_with("/crates/boomdrive-hosted/protocol/v1/"));
}

#[test]
fn nothing_outside_the_version_is_referenced() {
    let names: BTreeSet<&str> = FILES.iter().map(|(name, _)| *name).collect();
    for (name, value) in files() {
        let mut found = Vec::new();
        references(&value, &mut found);
        for reference in found {
            let file = reference.split('#').next().unwrap_or_default();
            assert!(
                file.is_empty() || names.contains(file),
                "{name} refers to {reference}, outside version 1's files"
            );
        }
        validator(&address(name));
        let defs = value["$defs"].as_object().cloned().unwrap_or_default();
        for def in defs.keys() {
            validator(&format!("{}#/$defs/{def}", address(name)));
        }
    }
    let outside = "https://example.com/elsewhere.schema.json";
    assert!(
        try_validator(outside).is_err(),
        "a reference outside version 1's files resolves, so it was fetched"
    );
    let unknown = format!("{ADDRESS}health.schema.json");
    assert!(try_validator(&unknown).is_err(), "{unknown} resolves");
}

#[test]
fn every_file_is_listed() {
    let on_disk: BTreeSet<String> = std::fs::read_dir(version_dir())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".schema.json"))
        .collect();
    let listed: BTreeSet<String> = FILES.iter().map(|(name, _)| name.to_string()).collect();
    assert_eq!(listed.len(), FILES.len(), "a file listed twice");
    assert_eq!(listed, on_disk);
}
