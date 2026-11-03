//! The forms of version 1's values (design Decision 4): a time of the
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
