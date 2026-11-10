//! The README's worked contexts (the runner-protocol spec's "The README
//! names the two paths and the signatures' contexts"; design Decisions 4
//! and 5; record 0018): the enrollment's and the connection's contexts,
//! signatures and header, made from the README's fixed key, token,
//! origin, time and nonce, are the README's worked bytes, read from the
//! README itself, so the two cannot differ.

use boomdrive_hosted::key;
use boomdrive_hosted::table::Table;
use ed25519_dalek::SigningKey;
use std::collections::BTreeMap;
use std::path::Path;

/// The protocol's README.
fn readme() -> String {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol/README.md");
    std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()))
}

/// The lines of the `nth` ```text block after the heading `heading`.
fn block(readme: &str, heading: &str, nth: usize) -> Vec<String> {
    let after = readme
        .split_once(&format!("{heading}\n"))
        .unwrap_or_else(|| panic!("the README has no {heading:?}"))
        .1;
    let mut blocks = after.split("```text\n").skip(1);
    let block = blocks
        .nth(nth)
        .and_then(|rest| rest.split_once("\n```"))
        .unwrap_or_else(|| panic!("no text block {nth} after {heading:?}"))
        .0;
    block.lines().map(str::to_string).collect()
}

/// A worked block's values by label: each line a label, two or more
/// spaces and a value, and a line starting with spaces continuing the
/// last value.
fn values(lines: &[String]) -> BTreeMap<String, String> {
    let mut values: BTreeMap<String, String> = BTreeMap::new();
    let mut last = String::new();
    for line in lines {
        if line.starts_with(' ') {
            values
                .get_mut(&last)
                .unwrap_or_else(|| panic!("a continuation with no value: {line:?}"))
                .push_str(line.trim());
            continue;
        }
        let (label, value) = line
            .split_once("  ")
            .unwrap_or_else(|| panic!("no label and value: {line:?}"));
        last = label.trim().to_string();
        values.insert(last.clone(), value.trim().to_string());
    }
    values
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|at| u8::from_str_radix(&text[at..at + 2], 16).expect("hex"))
        .collect()
}

/// The README's key, its secret key's bytes.
fn worked_key(values: &BTreeMap<String, String>) -> SigningKey {
    let secret: [u8; 32] = unhex(&values["secret key"]).try_into().expect("32 bytes");
    let key = SigningKey::from_bytes(&secret);
    assert_eq!(hex(key.verifying_key().as_bytes()), values["public key"]);
    key
}

/// A table whose `url` is `url`.
fn table(url: &str) -> Table {
    let text = format!("url = {url:?}\n[[partitions]]\nlabel = \"home\"\nrepositories = []\n");
    Table::read(
        &toml::from_str(&text).expect("TOML"),
        Path::new("/h/config.toml"),
    )
    .expect("the table read")
}

fn value<'a>(values: &'a BTreeMap<String, String>, label: &str) -> &'a str {
    values
        .get(label)
        .unwrap_or_else(|| panic!("the README's worked bytes name no {label:?}"))
}

#[test]
fn the_enrollments_worked_bytes() {
    let readme = readme();
    let values = values(&block(&readme, "### The enrollment's worked bytes", 0));
    let key = worked_key(&values);
    let table = table(value(&values, "url"));
    let token = value(&values, "token");

    assert_eq!(table.enroll_address().as_str(), value(&values, "address"));
    assert_eq!(table.origin(), value(&values, "origin"));
    assert_eq!(key::token_digest(token), value(&values, "digest"));
    let context = key::enrollment_context(&table.origin(), token, &key.verifying_key());
    assert_eq!(hex(&context), value(&values, "context"));
    assert_eq!(
        context.len(),
        148,
        "the README's count of the context's bytes"
    );
    assert_eq!(
        key::signature(&key, &context),
        key::base64url(&unhex(value(&values, "signature")))
    );
}

#[test]
fn the_connections_worked_bytes() {
    let readme = readme();
    let heading = "### The connection's worked bytes";
    let values = values(&block(&readme, heading, 0));
    let key = worked_key(&values);
    let table = table(value(&values, "url"));
    let time: u64 = value(&values, "time").parse().expect("Unix seconds");
    let nonce: [u8; 16] = unhex(value(&values, "nonce")).try_into().expect("16 bytes");

    assert_eq!(table.connect_address().as_str(), value(&values, "address"));
    assert_eq!(table.origin(), value(&values, "origin"));
    let context = key::connection_context(&table.origin(), &key.verifying_key(), time, &nonce);
    assert_eq!(hex(&context), value(&values, "context"));
    assert_eq!(
        context.len(),
        105,
        "the README's count of the context's bytes"
    );
    assert_eq!(
        key::signature(&key, &context),
        key::base64url(&unhex(value(&values, "signature")))
    );

    let header = block(&readme, heading, 1);
    assert_eq!(
        header,
        [format!(
            "Authorization: {}",
            key::statement(&key, &table.origin(), time, &nonce)
        )]
    );
}

#[test]
fn the_examples_enroll_carries_the_worked_key_and_signature() {
    let readme = readme();
    let values = values(&block(&readme, "### The enrollment's worked bytes", 0));
    let key = worked_key(&values);
    let file =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol/v2/examples/from-runner/enroll.json");
    let example: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(file).expect("the example")).expect("JSON");
    let enroll = &example["body"]["enroll"];
    assert_eq!(
        enroll["key"],
        key::base64url(key.verifying_key().as_bytes())
    );
    assert_eq!(
        enroll["signature"],
        key::base64url(&unhex(value(&values, "signature")))
    );
}

/// The README's section under the heading `## <heading>`, up to the next
/// heading of its level.
fn section<'a>(readme: &'a str, heading: &str) -> &'a str {
    let after = readme
        .split_once(&format!("\n## {heading}\n"))
        .unwrap_or_else(|| panic!("the README has no section {heading:?}"))
        .1;
    after.split("\n## ").next().unwrap_or(after)
}

/// The runner-protocol spec's "The README of version 2" (design Decision
/// 7 of the-hosted-protocol-carries-every-end-of-a-place, and Decision 9
/// of the-hosted-protocol-carries-land-and-preview): its envelope names
/// `protocol_version` 2, its paths the subprotocol `boomdrive.v2`, its
/// files `preview.schema.json`, its copies an end's `delivery`, an end's
/// delivery alone and a preview's copy, its messages a confirm and a land
/// among a `pulled`'s acts, a preview among its wants and `previews` in a
/// `report`, and a section `Version 1` names `boomdrive.v1` and says it
/// has no land and no preview.
#[test]
fn the_readme_describes_version_2() {
    let readme = readme();
    assert!(
        section(&readme, "The forms").contains(r#"{"protocol_version": 2, "id""#),
        "the envelope names protocol_version 2"
    );
    let paths = section(&readme, "The paths");
    assert!(paths.contains("subprotocol `boomdrive.v2`"), "{paths}");
    assert!(!paths.contains("boomdrive.v1"), "{paths}");
    let copies = section(&readme, "The copies");
    for named in [
        "`key`",
        "`delivery`",
        "`expires_at`",
        "`owed`",
        "`confirmed`",
        "`session_start`",
        r#"{"of":"over","over":"dropped"|"done",…}"#,
        "**An end's delivery alone** (`end_delivery`)",
    ] {
        assert!(copies.contains(named), "the copies name {named}: {copies}");
    }
    let files = section(&readme, "The files");
    assert!(files.contains("`preview.schema.json`"), "{files}");
    for named in ["**A preview's copy**", "**A release's copy**"] {
        assert!(copies.contains(named), "the copies name {named}: {copies}");
    }
    let messages = section(&readme, "The messages");
    assert!(
        messages.contains("a confirm of a reader's ends"),
        "{messages}"
    );
    let table: Vec<&str> = messages
        .lines()
        .filter(|line| line.starts_with('|'))
        .collect();
    let row = |kind: &str| {
        table
            .iter()
            .find(|line| line.starts_with(&format!("| `{kind}` |")))
            .unwrap_or_else(|| panic!("the messages table has no row {kind}"))
            .to_string()
    };
    let pulled = row("pulled");
    assert!(pulled.contains(r#"`"land"`"#), "{pulled}");
    assert!(pulled.contains("preview"), "{pulled}");
    let report = row("report");
    assert!(report.contains("`previews`"), "{report}");
    assert!(
        !messages.contains("no land and no preview among"),
        "{messages}"
    );
    assert!(
        messages.contains(r#"`{"confirm":{"recipient":…,"deliveries":[…]}}`"#),
        "{messages}"
    );
    let first = section(&readme, "Version 1");
    assert!(first.contains("`boomdrive.v1`"), "{first}");
    assert!(first.contains("`v0.7.3`"), "{first}");
    for lacks in ["no land", "no preview", "no `previews`"] {
        assert!(first.contains(lacks), "version 1 has {lacks}: {first}");
    }
    for context in ["`boomdrive.v2 enroll`", "`boomdrive.v2 connect`"] {
        assert!(readme.contains(context), "the README names {context}");
    }
    assert!(
        !readme.replace(first, "").contains("boomdrive.v1"),
        "only the section Version 1 names boomdrive.v1"
    );
}
