//! `protocol/spoken.json` names each version directory once, lowest
//! first, and marks spoken exactly one version, the one the crate's code
//! checks, sends and offers, which `protocol::spoken` reads from the file
//! itself, so the file the cloud's gate reads says what the runner speaks
//! and a version is spoken by its mark alone (Boom:
//! `the-cloud-leads-the-runners-protocol`; core.42; design Decisions 1
//! and 2 of the-runner-and-the-cloud-share-a-version; design Decision 3 of
//! protocol-version-6-names-the-newest-release).

mod common;

use boomdrive_hosted::protocol::{self, speakable};
use common::{current, VERSIONS};
use serde_json::Value;
use std::path::Path;

/// `protocol/spoken.json`, read as JSON.
fn spoken() -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol/spoken.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// Each entry of the file's `versions`, as its number and whether it is
/// spoken, each entry holding those two keys alone.
fn entries() -> Vec<(u64, bool)> {
    let file = spoken();
    let top = file.as_object().expect("spoken.json is an object");
    assert_eq!(
        top.keys().collect::<Vec<_>>(),
        ["versions"],
        "spoken.json holds `versions` alone"
    );
    top["versions"]
        .as_array()
        .expect("`versions` is an array")
        .iter()
        .map(|entry| {
            let entry = entry.as_object().expect("each version is an object");
            let mut keys: Vec<&str> = entry.keys().map(String::as_str).collect();
            keys.sort_unstable();
            assert_eq!(
                keys,
                ["spoken", "version"],
                "each version holds `version` and `spoken` alone"
            );
            let number = entry["version"]
                .as_u64()
                .expect("`version` is a whole number");
            let spoken = entry["spoken"]
                .as_bool()
                .expect("`spoken` is true or false");
            (number, spoken)
        })
        .collect()
}

/// The runner-protocol spec's "The file names each version directory".
#[test]
fn the_file_names_each_version_directory() {
    let protocol = Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol");
    let mut directories: Vec<u64> = std::fs::read_dir(&protocol)
        .unwrap_or_else(|e| panic!("{}: {e}", protocol.display()))
        .map(|entry| entry.expect("an entry of protocol/"))
        .filter(|entry| entry.path().is_dir())
        .map(|entry| {
            let name = entry.file_name().into_string().expect("a name in UTF-8");
            name.strip_prefix('v')
                .and_then(|number| number.parse().ok())
                .unwrap_or_else(|| panic!("protocol/{name} is no version directory"))
        })
        .collect();
    directories.sort_unstable();

    let named: Vec<u64> = entries().iter().map(|(number, _)| *number).collect();
    assert_eq!(
        named, directories,
        "spoken.json names each version directory once, lowest first"
    );
    let carried: Vec<u64> = VERSIONS.iter().map(|v| u64::from(v.number)).collect();
    assert_eq!(
        named, carried,
        "spoken.json names each version the crate carries"
    );
}

/// The runner-protocol spec's "The runner speaks what the file marks
/// spoken".
#[test]
fn the_runner_speaks_what_the_file_marks_spoken() {
    let spoken: Vec<u64> = entries()
        .into_iter()
        .filter(|(_, spoken)| *spoken)
        .map(|(number, _)| number)
        .collect();
    let version = protocol::spoken();
    assert_eq!(
        spoken,
        [u64::from(version.number)],
        "spoken.json marks the version the runner speaks alone"
    );
    assert!(
        speakable().iter().any(|v| v.number == version.number),
        "the version marked spoken is one the code speaks"
    );
    assert_eq!(current().number, version.number);
    assert_eq!(
        version.subprotocol,
        format!("boomdrive.v{}", version.number),
        "the upgrade offers the version spoken"
    );
}

/// The runner-protocol spec's "Each version the code speaks is its own
/// files": versions 5, 6 and 7, each with its module's number,
/// subprotocol, address and files, so a mark in the file alone moves the
/// runner from one to another.
#[test]
fn each_version_the_code_speaks_is_its_own_files() {
    use boomdrive_hosted::protocol::{v5, v6, v7};
    let [five, six, seven] = speakable();
    assert_eq!(
        (five.number, five.subprotocol, five.address),
        (v5::VERSION, v5::SUBPROTOCOL, v5::ADDRESS)
    );
    assert_eq!(five.files, v5::FILES);
    assert_eq!(
        (six.number, six.subprotocol, six.address),
        (v6::VERSION, v6::SUBPROTOCOL, v6::ADDRESS)
    );
    assert_eq!(six.files, v6::FILES);
    assert_eq!(
        (seven.number, seven.subprotocol, seven.address),
        (v7::VERSION, v7::SUBPROTOCOL, v7::ADDRESS)
    );
    assert_eq!(seven.files, v7::FILES);
    assert_eq!((five.number, six.number, seven.number), (5, 6, 7));
}
