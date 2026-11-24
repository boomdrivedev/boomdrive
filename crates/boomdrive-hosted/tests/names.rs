//! No version names a grant or a credential (the runner-protocol spec's
//! "No version names a grant or a credential"; design Decision 14 of
//! the-runner-dials-the-hosted-control-plane): a connection's account, and
//! what it may do, come from its enrollment, never from a message
//! (core.4; boomdrive-cloud records 0047 and 0065). Version 4's forge
//! token alone is a credential by design, and its names alone are
//! excepted (design Decision 3 of
//! a-cloud-host-gets-its-github-tokens-from-the-cloud; record 0053).

mod common;

use common::VERSIONS;
use serde_json::Value;
use std::collections::BTreeSet;

/// The words no name of any version holds.
const UNNAMED: [&str; 7] = [
    "grant",
    "role",
    "permission",
    "token",
    "credential",
    "secret",
    "password",
];

/// The names of the forge token of versions 4 and 5, which hold `token` by design: its
/// three kinds, each answer's `forge_token`, and `forge_token_given`'s
/// `token`, the one field of any version that names a credential (record
/// 0053).
const FORGE_TOKEN: [&str; 4] = [
    "forge_token",
    "forge_token_given",
    "forge_token_refused",
    "token",
];

/// Every name in `value`, at any depth: each member's name, which takes
/// in each `properties` key and each `$defs` key, and each field a
/// `required` lists.
fn names(value: &Value, found: &mut BTreeSet<String>) {
    match value {
        Value::Object(members) => {
            for (key, member) in members {
                found.insert(key.clone());
                if let ("required", Value::Array(fields)) = (key.as_str(), member) {
                    found.extend(fields.iter().filter_map(Value::as_str).map(str::to_string));
                }
                names(member, found);
            }
        }
        Value::Array(elements) => elements.iter().for_each(|e| names(e, found)),
        _ => {}
    }
}

/// The unnamed word `name` holds, alone or as a word of it, in the
/// singular or the plural.
fn unnamed_word(name: &str) -> Option<&'static str> {
    let name = name.to_lowercase();
    name.split(['_', '-', '$']).find_map(|word| {
        let singular = word.strip_suffix('s').unwrap_or(word);
        UNNAMED
            .into_iter()
            .find(|unnamed| *unnamed == word || *unnamed == singular)
    })
}

#[test]
fn every_name_read() {
    for version in VERSIONS {
        let n = version.number;
        let excepted: &[&str] = if n >= 4 { &FORGE_TOKEN } else { &[] };
        for (file, value) in version.files() {
            let mut found = BTreeSet::new();
            names(&value, &mut found);
            for name in found
                .iter()
                .filter(|name| !excepted.contains(&name.as_str()))
            {
                if let Some(word) = unnamed_word(name) {
                    panic!("version {n}'s {file} names {name:?}, a {word}'s name");
                }
            }
        }
        let mut all = BTreeSet::new();
        version
            .files()
            .iter()
            .for_each(|(_, value)| names(value, &mut all));
        for name in ["protocol_version", "hello", "share", "sealed", "lease"] {
            assert!(all.contains(name), "the walk of version {n} misses {name}");
        }
        for name in FORGE_TOKEN {
            assert_eq!(
                all.contains(name),
                n >= 4,
                "version {n} {} {name}",
                if n >= 4 { "does not name" } else { "names" }
            );
        }
    }
    for name in FORGE_TOKEN {
        assert!(unnamed_word(name).is_some(), "{name} is not caught");
    }
}

/// The runner-protocol spec's "A word caught in any name": a word is
/// caught alone or as a word of a name, in either case and in the plural.
#[test]
fn a_word_caught_in_any_name() {
    for name in ["roles", "grant", "Secret", "credential_file"] {
        assert!(unnamed_word(name).is_some(), "{name} is not caught");
    }
}
