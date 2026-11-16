//! Each version is JSON Schema of draft 2020-12 in this crate, each file
//! under its own address, every reference among its own files and none
//! fetched, and its module's `FILES` holding each file (the
//! runner-protocol spec's "Each version of the protocol is JSON Schema in
//! the hosted implementation's crate"; design Decisions 1 and 6 of
//! the-hosted-protocol-carries-every-end-of-a-place), and each version a
//! release has spoken held to the files that release holds: version 1 to
//! `v0.7.3`'s and version 2 to `v0.9.0`'s (core.42).

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

/// Each file under `protocol/v2/`, schemas and examples, with its SHA-256
/// as release `v0.9.0` holds it, read by `git show v0.9.0:<path> |
/// shasum -a 256`. Every release from `v0.8.0`, the first to speak
/// version 2, holds the same bytes (the runner-protocol spec's "Version 2
/// stays as a release spoke it"; core.42; record 0037).
const RELEASED_V2: &[(&str, &str)] = &[
    (
        "enrollment.schema.json",
        "362233c8ed7c06dc98d9a5c7f62cc1b80336908d67e0ad217a6d0b5460ef7d0c",
    ),
    (
        "examples/from-runner/enroll.json",
        "b6dbcbc9a1a3b998394256112bb593e559a5491ca9dd9105a74d06527c578b42",
    ),
    (
        "examples/from-runner/hello-with-flume-unread.json",
        "6280a5bbb0da3271fd11ae37eaddc5707e2f954d7acebc812014f024e65f7530",
    ),
    (
        "examples/from-runner/hello.json",
        "e83cb2927ac815d1ecdfbcd508dd0b4a1395b2b8ad2f7e763f6ed9504daeba1b",
    ),
    (
        "examples/from-runner/message_refused.json",
        "fbd9010be7bd3a25b189979b45e3e49d0153e4e189ef05eff70b1d6c9d2b79a9",
    ),
    (
        "examples/from-runner/report-a-done-place-held-for-the-runner.json",
        "22a0bcf32c16b03d4dc4cd6668805ba8ed862b058ed45db909472076b88d8335",
    ),
    (
        "examples/from-runner/report-a-dropped-place-held-for-the-runner.json",
        "8d551f6ab561b880c3534952d942e80b40e355c1e8b7ebdc7c21c9467c53d519",
    ),
    (
        "examples/from-runner/report-a-landing-kept-by-a-failed-teardown.json",
        "984a5e2de800557aa54fcc93f63c752a669cd7ef212cdcd99f456c44db3f2e07",
    ),
    (
        "examples/from-runner/report-a-live-agent-with-its-pane.json",
        "918e426863728bf8ad781d0033fe1a6cc1ab540445cf5b0683b19576db29196a",
    ),
    (
        "examples/from-runner/report-a-place-on-hold.json",
        "ad4efa1017c6f9c11bef1d88e0309ebfedf48339f6a61643a458fa402856b388",
    ),
    (
        "examples/from-runner/report-a-place-waiting-on-an-answer.json",
        "d236d628e87f9b7d33faafb2500dc6628e9eb8adb1470297feee4662c9071010",
    ),
    (
        "examples/from-runner/report-a-preview-brought-up.json",
        "a77f2f7dbd1c7edbf6e988db158ddfa6a06002d2edb961e67ceddf1d356cfb04",
    ),
    (
        "examples/from-runner/report-a-preview-held.json",
        "af1c0d79f6ee56f32181eb74731e7fd35443618e476dda7a93b287591f1b5faa",
    ),
    (
        "examples/from-runner/report-a-preview-put-away.json",
        "fadfcd6274c4314c6ad311fca5918bef9148560955f62ba7374581455802a8bb",
    ),
    (
        "examples/from-runner/report-a-preview-sluice-refused.json",
        "684dcee02d36478d3e58b39b83ed186201be4ff205a6ee8f68739ce2ef333355",
    ),
    (
        "examples/from-runner/report-a-roles-run-delivered.json",
        "a1678a03ff8707c7057cfc32b9aea643a1c52fc9d2eb1a0c45c7724c81675772",
    ),
    (
        "examples/from-runner/report-a-submission-failed.json",
        "847e103d170e463fd39be573bde1afab1282cf1eec114c4ac41a686d56874706",
    ),
    (
        "examples/from-runner/report-a-submission-under-test.json",
        "499c418ddecdf60b76a68bdf36347233551a17579bf1b2812fa887a46b4c79bd",
    ),
    (
        "examples/from-runner/report-a-tidied-place.json",
        "bd0aa28868650766e5372909a426f418e9bd16369b6a4e460751ede31aca026c",
    ),
    (
        "examples/from-runner/report-an-end-by-its-delivery-alone.json",
        "e0771355e71dbb8d9d6265a783a6d6142a9951a3af154917fffed7d32e38fe3c",
    ),
    (
        "examples/from-runner/report-an-end-handed-over-at-a-sessions-start.json",
        "8511608808cad446bb036768b5a4c6c3cbc3817d79c384493a226ef20210e312",
    ),
    (
        "examples/from-runner/report-an-end-owed-expired-and-one-confirmed.json",
        "f3c9683c1e02bacccf5687484ef1f197f95bab9ea93f0e74c8c72f7d35dd80e5",
    ),
    (
        "examples/from-runner/report.json",
        "3383568219b9c7727f9242b0e42166fafe8f42ae66688efecd99d6611a304ce9",
    ),
    (
        "examples/from-runner/terminal_closed.json",
        "de63df269a5ed837d01b8b144dd6b0ef7c7a56b385dfb7d084cf50390a89d41d",
    ),
    (
        "examples/from-runner/terminal_opened.json",
        "1e39ef3ce25147b8187c43251fde28eea9cf76bf87e055e7890a9b8e32ecb886",
    ),
    (
        "examples/from-runner/terminal_output-a-redraw-after-a-resume.json",
        "da5a885b2fb6cc0ac4263978a7cf0382d17fd341ee9a568c0bd8dca5cff5e3a3",
    ),
    (
        "examples/from-runner/terminal_output-bytes-as-they-come.json",
        "58b3a2c3ccefe205063293fb1576e11b74acb1c1528f1e428e17796990932320",
    ),
    (
        "examples/from-runner/terminal_output.json",
        "db868d698482fbcb1002e9708673164a5046cdc4d485cd15c705ba0c707a0b49",
    ),
    (
        "examples/to-runner/enroll_refused-a-lapsed-token.json",
        "8560fb8e56860ae57cc7b039ef855a727e1d71f9537c0e7c118f041f05e72c43",
    ),
    (
        "examples/to-runner/enrolled.json",
        "5c26b72f9b683c1031aef7694df34e571e0b776d7ecaed1dbdc15889309f1759",
    ),
    (
        "examples/to-runner/message_refused.json",
        "44ca0baddb83b8b3325f42ec16d29af2dd84bd7eea79df8c2025509aa0fcc91f",
    ),
    (
        "examples/to-runner/pulled-with-a-confirm.json",
        "ef24929649effbac868707c9a98f906add0959bc9f00f4496f4d5f3f749c807c",
    ),
    (
        "examples/to-runner/pulled-with-a-land.json",
        "1c636204950c04a4bc6e2048cb203bb374cefedc76470fb10c320688035b9846",
    ),
    (
        "examples/to-runner/pulled-with-a-preview-and-its-done.json",
        "064fa055c49eeffd0c0092473e928d78f649302dcf40e32c5853f97c46f7a87b",
    ),
    (
        "examples/to-runner/pulled-with-the-report-held.json",
        "d9496301f0243f4d798256e7e590a9dfdce7013607aaa5b0ac48d54c9af0044d",
    ),
    (
        "examples/to-runner/pulled.json",
        "cd1a6612b116b0a16a0592f6fddd6dbd1239025b12ad139e44c68bc19290e3ba",
    ),
    (
        "examples/to-runner/reported.json",
        "28f129fe99cec9860301d224aa0726cdd2342d17df3fd1960f06823368fa4e5a",
    ),
    (
        "examples/to-runner/terminal_close.json",
        "3712fe25bc1080e79ef5e3c2aa4673a1a3f5df8e878ac1e5912f6d95c3ffe84a",
    ),
    (
        "examples/to-runner/terminal_input.json",
        "1b5104cfd52646260e7d6340908eaf683413d80367922ba351f21c4ab9101109",
    ),
    (
        "examples/to-runner/terminal_open.json",
        "4d9b5d8d5ce58e87026343dcb1545132cbd3549ce3e93b8a8d10f94e0b30337f",
    ),
    (
        "examples/to-runner/terminal_resume.json",
        "5b0e8b21cd3b4d693d8b98e17199459c6b51e4519e9f52ee74b82a1559bb8721",
    ),
    (
        "from-runner.schema.json",
        "29c511f05ed3ac1ba6aa96ff96565a3450a122603abce64fb81bd6af24edbf3b",
    ),
    (
        "hello.schema.json",
        "a44a7fe4a338c14ad131343dcc5f1ad79dd85d2cefb1eff5e4827464e7f70e1d",
    ),
    (
        "place-status.schema.json",
        "18a0186e3e15ef1d0b5f559cfc2c517cd3acd1efff0a9a077d68ba5a009a9b65",
    ),
    (
        "preview.schema.json",
        "11217ce5d0f3ee060578420b606f18c4a24166607f470be6c4c4a6d7b1871ef0",
    ),
    (
        "report.schema.json",
        "b8b39b3f5c0d6ee4b4a6843ba5d3dd3ac664f35253b77b1b45a7c3353beaafe8",
    ),
    (
        "shared.schema.json",
        "ca90b26bef37bdf1e341083da5b43b964b1e127e2a1ae25eef61874394b993fa",
    ),
    (
        "submission-entry.schema.json",
        "d55e15133d8008588b4b8aea0ec1c6dae332b40c4a9307ad4d00fdf56ee9f223",
    ),
    (
        "terminal.schema.json",
        "4d1f6be2ed9fd6578375f53ecc9538b9ef3c09ebddb25ab2913c01a46ea9eb2f",
    ),
    (
        "to-runner.schema.json",
        "c42a461337e95a9ab6e048b94fd763064f02f79d9a506ef28d478d855d37d476",
    ),
    (
        "wants.schema.json",
        "6a37ac187ae795163af44f36fbab1930b8cb911654d043a4a635d4851b241fa0",
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

/// Holds every file under `version`'s directory, schemas and examples,
/// to its digest as release `tag` holds it, `released`: none missing,
/// none differing and none added (core.42).
fn held_as_released(version: common::Version, tag: &str, released: &[(&str, &str)]) {
    let root = version.dir();
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
    let released: BTreeMap<String, String> = released
        .iter()
        .map(|(name, digest)| (name.to_string(), digest.to_string()))
        .collect();
    let mut wrong = Vec::new();
    for (name, digest) in &released {
        match on_disk.get(name) {
            None => wrong.push(format!("{name} is missing")),
            Some(found) if found != digest => wrong.push(format!("{name} differs from {tag}'s")),
            Some(_) => {}
        }
    }
    for name in on_disk.keys().filter(|name| !released.contains_key(*name)) {
        wrong.push(format!("{name} was added"));
    }
    assert!(
        wrong.is_empty(),
        "version {} is not as release {tag} holds it: {wrong:#?}",
        version.number
    );
}

#[test]
fn version_1_is_as_released() {
    held_as_released(common::V1, "v0.7.3", RELEASED_V1);
}

#[test]
fn version_2_is_as_released() {
    held_as_released(common::V2, "v0.9.0", RELEASED_V2);
}
