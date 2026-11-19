//! The forms of version 3's values (design Decision 4): a time of the
//! runner's or the control plane's is RFC 3339 in UTC to the second, and
//! an integer is one JavaScript reads exactly, a lease's number 1 or more
//! (the runner-protocol spec's "A message holds only what its version
//! names").

mod common;

use common::{definition, passes, refused, validator};
use serde_json::json;

const SHARED: &str = "shared.schema.json";

#[test]
fn a_time_not_in_utc_to_the_second() {
    let lease = validator(&definition(SHARED, "lease"));
    let expiring = |at: &str| json!({"number": 1, "expires": at});
    passes(&lease, &expiring("2026-11-01T10:00:00Z"), "a time in UTC");
    passes(&lease, &json!({"number": 1, "expires": null}), "no expiry");
    refused(
        &lease,
        &expiring("2026-11-01T12:00:00+02:00"),
        "a lease expiring at a time with an offset other than Z",
    );
    refused(
        &lease,
        &expiring("2026-11-01T10:00:00.5Z"),
        "a lease expiring at a time with a fraction of a second",
    );
    let time = validator(&definition(SHARED, "time"));
    for wrong in ["2026-11-01 10:00:00Z", "2026-13-01T10:00:00Z", "soon"] {
        refused(&time, &json!(wrong), wrong);
    }
}

#[test]
fn an_integer_past_what_javascript_reads_exactly() {
    let lease = validator(&definition(SHARED, "lease"));
    let numbered = |number: serde_json::Value| json!({"number": number, "expires": null});
    passes(
        &lease,
        &numbered(json!(9_007_199_254_740_991_u64)),
        "a lease numbered 2^53 - 1",
    );
    refused(
        &lease,
        &numbered(json!(9_007_199_254_740_992_u64)),
        "a lease numbered 2^53",
    );
    refused(&lease, &numbered(json!(0)), "a lease numbered 0");
    let signed = validator(&definition(SHARED, "signed_integer"));
    passes(&signed, &json!(-9_007_199_254_740_991_i64), "-(2^53 - 1)");
    refused(&signed, &json!(-9_007_199_254_740_992_i64), "-(2^53)");
    refused(&signed, &json!(1.5), "a fraction");
}

/// The runner-protocol spec's "A forge token's examples" (design
/// Decision 1 of a-cloud-host-gets-its-github-tokens-from-the-cloud):
/// version 4's ask and its three answers each pass their direction's
/// envelope of version 4.
#[test]
fn a_forge_tokens_examples_pass() {
    let v4 = common::V4;
    let (from, to) = (v4.messages_from_runner(), v4.messages_to_runner());
    let ask = v4.example("from-runner", "forge_token.json");
    passes(&from, &ask, "from-runner/forge_token.json");
    for name in [
        "forge_token_given.json",
        "forge_token_refused-not-installed.json",
        "forge_token_refused-a-repository-no-drive-serves.json",
    ] {
        passes(&to, &v4.example("to-runner", name), name);
    }
}

/// The runner-protocol spec's "A forge token's ask or answer that is not
/// whole" (design Decision 1 of
/// a-cloud-host-gets-its-github-tokens-from-the-cloud): each kind is
/// closed, every field required, an address `owner/name`, a time in UTC
/// to the second, and a token of one character or more.
#[test]
fn a_forge_tokens_ask_or_answer_that_is_not_whole() {
    let v4 = common::V4;
    let (from, to) = (v4.messages_from_runner(), v4.messages_to_runner());
    let ask = v4.example("from-runner", "forge_token.json");
    let mut with_a_path = ask.clone();
    with_a_path["body"]["forge_token"]["path"] = json!("/data/example/app");
    refused(&from, &with_a_path, "an ask with a path added");
    let mut bare = ask.clone();
    bare["body"]["forge_token"]["repository"] = json!("flume");
    refused(&from, &bare, "an ask whose repository is flume");

    let given = v4.example("to-runner", "forge_token_given.json");
    let mut no_end = given.clone();
    no_end["body"]["forge_token_given"]
        .as_object_mut()
        .unwrap()
        .remove("expires_at");
    refused(&to, &no_end, "a token given with no expires_at");
    let mut offset = given.clone();
    offset["body"]["forge_token_given"]["expires_at"] = json!("2026-11-18T12:00:00+02:00");
    refused(&to, &offset, "a token given whose expires_at has an offset");
    let mut empty = given.clone();
    empty["body"]["forge_token_given"]["token"] = json!("");
    refused(&to, &empty, "a token given whose token is empty");

    let mut unsaid = v4.example("to-runner", "forge_token_refused-not-installed.json");
    unsaid["body"]["forge_token_refused"]
        .as_object_mut()
        .unwrap()
        .remove("not_installed");
    refused(&to, &unsaid, "a refusal with no not_installed");
}

/// The runner-protocol spec's "A forge token is no message of version 1, 2
/// or 3" (design Decision 5 of
/// a-cloud-host-gets-its-github-tokens-from-the-cloud): version 4's ask and
/// two of its answers, each naming an earlier version, are refused by that
/// version's envelope of their direction, which a release has spoken and
/// which never changes to carry a token (core.33, core.42; record 0053).
#[test]
fn no_forge_token_in_version_1_2_or_3() {
    let v4 = common::V4;
    let messages = [
        ("from-runner", "forge_token.json"),
        ("to-runner", "forge_token_given.json"),
        ("to-runner", "forge_token_refused-not-installed.json"),
    ];
    for version in [common::V3, common::V2, common::V1] {
        let n = version.number;
        for (direction, name) in messages {
            let validator = match direction {
                "from-runner" => version.messages_from_runner(),
                _ => version.messages_to_runner(),
            };
            let mut message = v4.example(direction, name);
            message["protocol_version"] = json!(n);
            refused(
                &validator,
                &message,
                &format!("{direction}/{name} as a message of version {n}"),
            );
        }
    }
}
