//! Each version is JSON Schema of draft 2020-12 in this crate, each file
//! under its own address, every reference among its own files and none
//! fetched, and its module's `FILES` holding each file (the
//! runner-protocol spec's "Each version of the protocol is JSON Schema in
//! the hosted implementation's crate"; design Decisions 1 and 6 of
//! the-hosted-protocol-carries-every-end-of-a-place), and version 1 held
//! to the files release `v0.7.3` holds.

mod common;

use common::VERSIONS;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

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

/// Each file under `protocol/v1/`, schemas and examples, with its SHA-256
/// as release `v0.7.3` holds it, read by `git show v0.7.3:<path> |
/// shasum -a 256` (the runner-protocol spec's "Version 1 stays as a
/// release spoke it"; core.42; record 0037).
const RELEASED_V1: &[(&str, &str)] = &[
    (
        "enrollment.schema.json",
        "8d5bc86386d3923032b8eab541ebf4b834c17fdd50d001ac791a92643e31df2e",
    ),
    (
        "examples/from-runner/enroll.json",
        "576df79a4ed96a5a33d5d7579e093ef02e49bf8fb56db250dfcfce83f293b345",
    ),
    (
        "examples/from-runner/hello-with-flume-unread.json",
        "184f6833af15b9a2206e250f6adf6e01127f4a17f7765af34611a770ce209769",
    ),
    (
        "examples/from-runner/hello.json",
        "046fa9fa53ab9698b996526d524e6141526e0858cfc27b62180c1261e8591f21",
    ),
    (
        "examples/from-runner/message_refused.json",
        "7da41731b45edf1446cc2c4e695a1fa80a7d768d5d1143c07ce8e7f1b3f93862",
    ),
    (
        "examples/from-runner/report-a-landing-kept-by-a-failed-teardown.json",
        "9ea82fcb025074b76a194ddb1457cab1e8355cdc9cabb11e26630c8532bba60e",
    ),
    (
        "examples/from-runner/report-a-live-agent-with-its-pane.json",
        "c290b0f45cdcc5e6b66e6de6c97ca6bc54a59ad0731863682fb5a896e0dc2a20",
    ),
    (
        "examples/from-runner/report-a-place-on-hold.json",
        "312e13fab96cb4bf8343801f5501d9268fedd7be14b3dbbe8591a6ee8d9def75",
    ),
    (
        "examples/from-runner/report-a-place-waiting-on-an-answer.json",
        "ffce68f679d9ee36c02a3332cba4e464e10f2d3890988f5a5181d89dd51db4d4",
    ),
    (
        "examples/from-runner/report-a-roles-run-delivered.json",
        "329aa832c196e6059f7c64fb845de91fc917cba02b7a2e639198943eb2f3ba30",
    ),
    (
        "examples/from-runner/report-a-submission-failed.json",
        "3eacb829a56431eb667f3b3f45a956875e120c484a006764f137778bfcae928b",
    ),
    (
        "examples/from-runner/report-a-submission-under-test.json",
        "eb015091eb0d3b7248d2dd807e63f7818e9da70e450092d984dde9d3a8fa1b8d",
    ),
    (
        "examples/from-runner/report-a-tidied-place.json",
        "cb627d3ad35fc8763cce6723db49b20f04ad1cfbb02fbc6c2b0a35b0ecc07c5c",
    ),
    (
        "examples/from-runner/report.json",
        "fd10539bba3cbe1dc0544c91f0dc671bbad3b78892c3be758ae12edc68fa7b4c",
    ),
    (
        "examples/from-runner/terminal_closed.json",
        "7b72b288f5fb2643612c1d96eeda2b184f4ed8010d2a7cbfbad1c13f160bb441",
    ),
    (
        "examples/from-runner/terminal_opened.json",
        "d00b27de0bcd16e8e1f180cf388fe2f10cfa00202d6a1bb24ce3fa1c6c884a3a",
    ),
    (
        "examples/from-runner/terminal_output-a-redraw-after-a-resume.json",
        "37f15ece8046f85bb43554686b02511fb0bcf65c378b4df8ec46fa88e02a1eb7",
    ),
    (
        "examples/from-runner/terminal_output-bytes-as-they-come.json",
        "290088adda7509ff29d81d71808f91ec33194baa192ef20ced8ef64664f152b7",
    ),
    (
        "examples/from-runner/terminal_output.json",
        "edaa7e2c982eb631ebed252b7ce643d6e88f5918d75c2fd1ba1c3d38743f40bb",
    ),
    (
        "examples/to-runner/enroll_refused-a-lapsed-token.json",
        "544fa184d0357f8f7faecfd02d073814780d012d9ccb7eb4238d680cf2201e2d",
    ),
    (
        "examples/to-runner/enrolled.json",
        "879a5081bc0691702265d80b2610c7cb67a5e06e5822b997fe8fb939fe3e0099",
    ),
    (
        "examples/to-runner/message_refused.json",
        "ccb2f0221dbc4bd4f07e34f71cb2f0768927cd1d884727594d6c0e4939060b03",
    ),
    (
        "examples/to-runner/pulled-with-the-report-held.json",
        "9327f4fb57cc3b4caae1503263631e639912952447d9296b67b06c67beae2585",
    ),
    (
        "examples/to-runner/pulled.json",
        "1122bc0a6c974634c1a4524ccd284ddc9128bc2ab7e43afb9a6989f7a1546eaa",
    ),
    (
        "examples/to-runner/reported.json",
        "b1b0fd6803a94297f1e6082b8f830010cd57135fc2473ea6f54e91a5bd6bd0f2",
    ),
    (
        "examples/to-runner/terminal_close.json",
        "74ac0653ff91a343c02eab70a4c35786df081d4f954bd47310b88d973acc9f66",
    ),
    (
        "examples/to-runner/terminal_input.json",
        "0f2b1b96ab83b1996ae9c3d581ee246faddebd3fe8b9dc0cce3b1d642a315938",
    ),
    (
        "examples/to-runner/terminal_open.json",
        "dd38bfea7ec4065d478a8a48b6b7ef641c68527f54fe38971472c2c0c454843e",
    ),
    (
        "examples/to-runner/terminal_resume.json",
        "e9242bf958b752336e71f118643aee88ec04da3a27c50c88a3d72f4e86f4a37b",
    ),
    (
        "from-runner.schema.json",
        "5a95908d51b7eddc26032a4c0b62e3158ee5e44d30b92403fba50c6f8d061613",
    ),
    (
        "hello.schema.json",
        "83b6c321de1131c19d69d656d6da002d17bfeca7e90930e8de0d4d6637bc5ab5",
    ),
    (
        "place-status.schema.json",
        "e2ed5a76a5ab3ef751c7170729de28a8e19b57974dcbbf291c4aa0edd8f70f0b",
    ),
    (
        "report.schema.json",
        "c596fcda3b1d944aa743987e7fa3422ef0d6ac2abc9cb6de3a70640adcd0be21",
    ),
    (
        "shared.schema.json",
        "e1ce61071a47df826f90ffecbdee36be1466aca6e54d369290bedae216e8f19a",
    ),
    (
        "submission-entry.schema.json",
        "4f71299fd13a8be2fdfff985fdf05f4ae06363ac9f9d2c3100b73985791dcfc0",
    ),
    (
        "terminal.schema.json",
        "b8bd2dc3ced3d36668e9a7300c08a72ac9bd0437ba085e0f5f62456d4c5b4beb",
    ),
    (
        "to-runner.schema.json",
        "8680b983726e2189d728a62e4b550f8eed7a34a5d49a37be7234a278035a938e",
    ),
    (
        "wants.schema.json",
        "684f4e6db2c1a09274de8bad80b464247f60e06cedb5190f55613e8c6603090e",
    ),
];

/// Every file under `dir`, at any depth, by its path relative to `root`.
fn files_under(root: &Path, dir: &Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files_under(root, &path, found);
        } else {
            let relative = path.strip_prefix(root).unwrap();
            found.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

#[test]
fn version_1_is_as_released() {
    let root = common::V1.dir();
    let mut on_disk = Vec::new();
    files_under(&root, &root, &mut on_disk);
    let on_disk: BTreeMap<String, String> = on_disk
        .into_iter()
        .map(|name| {
            let bytes = std::fs::read(root.join(&name)).unwrap();
            let digest: String = Sha256::digest(&bytes)
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect();
            (name, digest)
        })
        .collect();
    let released: BTreeMap<String, String> = RELEASED_V1
        .iter()
        .map(|(name, digest)| (name.to_string(), digest.to_string()))
        .collect();
    let mut wrong = Vec::new();
    for (name, digest) in &released {
        match on_disk.get(name) {
            None => wrong.push(format!("{name} is missing")),
            Some(found) if found != digest => wrong.push(format!("{name} differs from v0.7.3's")),
            Some(_) => {}
        }
    }
    for name in on_disk.keys().filter(|name| !released.contains_key(*name)) {
        wrong.push(format!("{name} was added"));
    }
    assert!(
        wrong.is_empty(),
        "version 1 is not as release v0.7.3 holds it: {wrong:#?}"
    );
}
