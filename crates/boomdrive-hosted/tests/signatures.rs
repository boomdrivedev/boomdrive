//! The protocol's worked contexts (the runner-protocol spec's "The README
//! names the paths and the signatures' contexts"; design Decisions 4 and
//! 5; record 0018; design Decision 2 of
//! a-cloud-host-gets-its-github-tokens-from-the-cloud): the enrollment's,
//! the connection's and a forge token's ask's contexts, signatures and
//! header, made from the worked example's fixed key, token, origin, time,
//! nonce and repository, are its worked bytes, each starting with
//! `boomdrive.v4`. The example is
//! `tests/worked-bytes.json`, of which the protocol's README shows its
//! own copy, which its writers keep to the file (record 0049).

use boomdrive_hosted::key;
use boomdrive_hosted::table::Table;
use ed25519_dalek::SigningKey;
use serde_json::Value;
use std::path::Path;

/// The worked example's values for `which`, `enrollment`, `connection` or
/// `forge_token`.
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

/// The runner-protocol spec's "The README's worked contexts" for the ask
/// (design Decision 2 of a-cloud-host-gets-its-github-tokens-from-the-cloud;
/// record 0053): the context `forge token` made from the worked key,
/// origin, time, nonce and the repository `example/app` is the worked
/// bytes, starting with `boomdrive.v4`; the key's signature over it checks
/// and is the worked one; and the ask's header is the worked header.
#[test]
fn the_asks_worked_context() {
    let values = worked("forge_token");
    let key = worked_key(&values);
    let table = table(value(&values, "url"));
    let time = values["time"].as_u64().expect("Unix seconds");
    let nonce: [u8; 16] = unhex(value(&values, "nonce")).try_into().expect("16 bytes");
    let repository = value(&values, "repository");
    assert_eq!(repository, "example/app");

    assert_eq!(
        table.forge_token_address().as_str(),
        value(&values, "address")
    );
    assert_eq!(table.origin(), value(&values, "origin"));
    let context = key::forge_token_context(
        &table.origin(),
        &key.verifying_key(),
        time,
        &nonce,
        repository,
    );
    assert_eq!(hex(&context), value(&values, "context"));
    assert!(
        context.starts_with(b"boomdrive.v4 "),
        "the context starts with boomdrive.v4"
    );
    assert_eq!(context.len(), 121, "the count of the context's bytes");
    let signature: [u8; 64] = unhex(value(&values, "signature"))
        .try_into()
        .expect("64 bytes");
    key.verifying_key()
        .verify_strict(&context, &ed25519_dalek::Signature::from_bytes(&signature))
        .expect("the signature checks over the context");
    assert_eq!(key::signature(&key, &context), key::base64url(&signature));

    assert_eq!(
        value(&values, "header"),
        format!(
            "Authorization: {}",
            key::forge_token_statement(&key, &table.origin(), time, &nonce, repository)
        )
    );
    let connection = key::connection_context(&table.origin(), &key.verifying_key(), time, &nonce);
    assert_ne!(context, connection, "an ask's context is no connection's");
}
