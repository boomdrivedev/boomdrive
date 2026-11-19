//! The protocol's worked contexts (the runner-protocol spec's "The README
//! names the two paths and the signatures' contexts"; design Decisions 4
//! and 5; record 0018): the enrollment's and the connection's contexts,
//! signatures and header, made from the worked example's fixed key,
//! token, origin, time and nonce, are its worked bytes. The example is
//! `tests/worked-bytes.json`, of which the protocol's README shows its
//! own copy, which its writers keep to the file (record 0049).

use boomdrive_hosted::key;
use boomdrive_hosted::table::Table;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use std::path::Path;

/// The worked example's values for `which`, `enrollment` or `connection`.
fn worked(which: &str) -> Value {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/worked-bytes.json");
    let text = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("{}: {e}", file.display()));
    let mut example: Value = serde_json::from_str(&text).expect("JSON");
    example[which].take()
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

/// The worked example's key, its secret key's bytes.
fn worked_key(values: &Value) -> SigningKey {
    let secret: [u8; 32] = unhex(value(values, "secret_key"))
        .try_into()
        .expect("32 bytes");
    let key = SigningKey::from_bytes(&secret);
    assert_eq!(
        hex(key.verifying_key().as_bytes()),
        value(values, "public_key")
    );
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

fn value<'a>(values: &'a Value, label: &str) -> &'a str {
    values[label]
        .as_str()
        .unwrap_or_else(|| panic!("the worked bytes name no {label:?}"))
}

#[test]
fn the_enrollments_worked_bytes() {
    let values = worked("enrollment");
    let key = worked_key(&values);
    let table = table(value(&values, "url"));
    let token = value(&values, "token");

    assert_eq!(table.enroll_address().as_str(), value(&values, "address"));
    assert_eq!(table.origin(), value(&values, "origin"));
    assert_eq!(key::token_digest(token), value(&values, "digest"));
    let context = key::enrollment_context(&table.origin(), token, &key.verifying_key());
    assert_eq!(hex(&context), value(&values, "context"));
    assert_eq!(context.len(), 148, "the count of the context's bytes");
    assert_eq!(
        key::signature(&key, &context),
        key::base64url(&unhex(value(&values, "signature")))
    );
}

#[test]
fn the_connections_worked_bytes() {
    let values = worked("connection");
    let key = worked_key(&values);
    let table = table(value(&values, "url"));
    let time = values["time"].as_u64().expect("Unix seconds");
    let nonce: [u8; 16] = unhex(value(&values, "nonce")).try_into().expect("16 bytes");

    assert_eq!(table.connect_address().as_str(), value(&values, "address"));
    assert_eq!(table.origin(), value(&values, "origin"));
    let context = key::connection_context(&table.origin(), &key.verifying_key(), time, &nonce);
    assert_eq!(hex(&context), value(&values, "context"));
    assert_eq!(context.len(), 105, "the count of the context's bytes");
    assert_eq!(
        key::signature(&key, &context),
        key::base64url(&unhex(value(&values, "signature")))
    );

    assert_eq!(
        value(&values, "header"),
        format!(
            "Authorization: {}",
            key::statement(&key, &table.origin(), time, &nonce)
        )
    );
}

#[test]
fn the_examples_enroll_carries_the_worked_key_and_signature() {
    let values = worked("enrollment");
    let key = worked_key(&values);
    let file =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("protocol/v4/examples/from-runner/enroll.json");
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
