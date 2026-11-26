//! An offer of a hosted control plane carried through one control pass
//! over the stub `flume`, the hosted implementation reaching the hosted
//! crate's fake control plane on the loopback address (the
//! hosted-control-plane spec's "A launch Flume refuses goes up refused";
//! the work-into-flume spec's "Flume refuses").

mod fakes;
#[path = "../../boomdrive-hosted/tests/common/mod.rs"]
mod hosted;

use boomdrive::flume::Cli;
use boomdrive_hosted::key::{Memory, Store};
use boomdrive_hosted::table::Table;
use boomdrive_hosted::Hosted;
use ed25519_dalek::SigningKey;
use fakes::{Bench, Stub, KEPT, PART};
use hosted::fake::{kind, Fake, Taken};
use serde_json::{json, Value};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Flume's words refusing a kind that lands in a repository that binds
/// no merge queue.
const WORDS: &str =
    "the kind flume/fix@1 lands, and agentplot/flume binds no merge queue for it to land through";

/// The hosted implementation over `fake`, its one partition, [`PART`],
/// served [`KEPT`] and enrolled with a key of the test's own.
fn hosted(fake: &Fake) -> Hosted {
    let text = format!(
        "url = {:?}\n[[partitions]]\nlabel = {PART:?}\nrepositories = [{KEPT:?}]\n",
        fake.url()
    );
    let table = Table::read(
        &toml::from_str(&text).expect("a TOML table"),
        Path::new("/h/.config/boomdrive/config.toml"),
    )
    .expect("the table read");
    let key = SigningKey::from_bytes(&[7u8; 32]);
    fake.enrolled(PART, &key.verifying_key());
    let store = Memory::default();
    store
        .write(&table.key_name(PART), &key)
        .expect("the key kept");
    Hosted::new(
        &table,
        Arc::new(store),
        Duration::from_secs(60),
        Box::new(|_| {}),
        Box::new(|| {}),
    )
    .expect("opened")
}

/// The launch `id` of [`KEPT`] under lease 1, as version 5 offers it.
fn offer(id: &str) -> Value {
    json!({ "offer": {
        "id": id,
        "lease": { "number": 1, "expires": null },
        "launch": {
            "repository": KEPT, "kind": "flume/fix@1", "name": id, "base": "main",
            "brief": null, "stream": null, "priority": null, "intent": null,
            "recipients": [], "holds": [], "hold_submission": false,
            "after": [], "agent_settings": []
        }
    } })
}

/// Each report [`PART`]'s connection sent, with why it failed version 5,
/// where it did.
fn reports(taken: &[Taken]) -> Vec<(Value, Option<String>)> {
    taken
        .iter()
        .filter_map(|taken| match taken {
            Taken::Message {
                partition,
                message,
                refused,
            } if partition == PART && kind(message).as_deref() == Some("report") => {
                Some((message.clone(), refused.clone()))
            }
            _ => None,
        })
        .collect()
}

/// The hosted-control-plane spec's "A launch Flume refuses goes up
/// refused": Flume refuses the launch a hosted control plane offers, and
/// the report the pass sends holds it refused, under its own id, with
/// Flume's words as Flume gave them, and passes version 5.
#[test]
fn a_launch_flume_refuses_goes_up_refused() {
    let fake = Fake::start();
    fake.set_wants(PART, json!([offer("l-1")]));
    let mut plane = hosted(&fake);
    let bench = Bench::new();
    let stub = Stub::new();
    let refusal = json!({ "refused": { "refusal": { "reason": WORDS } } });
    stub.answers_text("launch", &refusal.to_string(), 1);

    let (passed, lines) = bench.pass_of(PART, &Cli::new(stub.program()), &mut plane);

    assert!(passed.reported, "{lines:?}");
    assert_eq!(lines, [format!("{PART}/l-1: refused by Flume: {WORDS}")]);
    let sent = reports(&fake.taken());
    let (last, failed) = sent.last().expect("a report sent");
    assert_eq!(failed, &None, "the report passes version 5");
    let answer = &last["body"]["report"]["launches"]["l-1"]["answer"];
    assert_eq!(
        (&answer["outcome"], &answer["words"], &answer["place_dir"]),
        (&json!("refused"), &json!(WORDS), &Value::Null),
        "{last:#}"
    );
}
