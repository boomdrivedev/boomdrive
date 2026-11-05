//! A partition's enrollment (design Decision 4; record 0006; core.31):
//! its key made in memory, registered with the control plane by one
//! `POST runner/enroll` that bears the one-time token, and kept in the
//! host's credential store only once `enrolled` answers. The answer is
//! checked against version 1 before it is read (design Decision 10). The
//! enrollment writes no configuration and nothing in the data directory
//! (core.35), and no output of it holds the token (core.4).

use crate::check;
use crate::key::{self, Store};
use crate::protocol::v1::{Enroll, FromRunner, Message, MessageId, ToRunner};
use crate::table::Table;
use serde_json::Value;
use std::time::Duration;
use ureq::tls::{RootCerts, TlsConfig};
use uuid::Uuid;

/// The most bytes of the control plane's answer read.
const ANSWER_LIMIT: u64 = 64 * 1024;

/// How long the request may take, from its connection to its answer's
/// last byte.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// A partition enrolled: its label, and the control plane's id for the
/// host, which the runner shows and keeps nowhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enrolled {
    pub partition: String,
    pub host: String,
}

/// An enrollment refused, with words saying why, and whether the host must
/// be fixed, its configuration, its store or its token, before an
/// enrollment can go (core.39's exit 2), or the control plane refused it or
/// could not be reached (exit 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub words: String,
    pub host_to_fix: bool,
}

impl Refusal {
    /// A refusal the host must fix.
    pub fn host(words: String) -> Refusal {
        Refusal {
            words,
            host_to_fix: true,
        }
    }

    /// A refusal by the control plane, or of reaching it.
    pub fn control_plane(words: String) -> Refusal {
        Refusal {
            words,
            host_to_fix: false,
        }
    }
}

/// Enrolls the partition `label` of `table` with the one-time token
/// `token` gives, or words saying why it could not be read, naming the
/// host `machine`, its key kept in the store `store` finds. The token is
/// asked for only once the partition and the store are found, and nothing
/// is sent before the token is found good; the store is as it was unless
/// `enrolled` answers.
pub fn enroll(
    table: &Table,
    label: &str,
    token: impl FnOnce() -> Result<String, String>,
    store: impl FnOnce() -> Result<Box<dyn Store>, String>,
    machine: &str,
) -> Result<Enrolled, Refusal> {
    if table.partition(label).is_none() {
        let labels: Vec<String> = table
            .partitions
            .iter()
            .map(|partition| format!("`{}`", partition.label))
            .collect();
        return Err(Refusal::host(format!(
            "[control_plane.hosted] names no partition `{label}`: it names {}",
            labels.join(", ")
        )));
    }
    let store = store().map_err(Refusal::host)?;
    let token = token().map_err(Refusal::host)?;
    let token = token.as_str();
    if token.is_empty() {
        return Err(Refusal::host(
            "the enrollment token is empty: give the partition's one-time token on standard input"
                .to_string(),
        ));
    }
    if !token.bytes().all(|byte| byte.is_ascii_graphic()) {
        return Err(Refusal::host(
            "the enrollment token holds a character no header carries: give the token as the \
             control plane issued it"
                .to_string(),
        ));
    }

    let key = key::made().map_err(Refusal::host)?;
    let public = key.verifying_key();
    let origin = table.origin();
    let id = MessageId::new(Uuid::now_v7().to_string());
    let message = Message::new(
        id,
        FromRunner::Enroll(Enroll {
            key: key::base64url(public.as_bytes()),
            signature: key::signature(&key, &key::enrollment_context(&origin, token, &public)),
            machine: machine.to_string(),
        }),
    );
    let body = serde_json::to_value(&message)
        .map_err(|e| Refusal::host(format!("the enrollment could not be written: {e}")))?;
    if let Some(words) = check::failure(check::from_runner(), &body) {
        return Err(Refusal::host(format!(
            "the enrollment fails version 1's check, so is not sent: {words}"
        )));
    }

    let agent: ureq::Agent = ureq::Agent::config_builder()
        .http_status_as_error(false)
        .max_redirects(0)
        .timeout_global(Some(REQUEST_TIMEOUT))
        .tls_config(
            TlsConfig::builder()
                .root_certs(RootCerts::PlatformVerifier)
                .build(),
        )
        .build()
        .into();
    let mut response = agent
        .post(table.enroll_address().as_str())
        .header("Authorization", &format!("Bearer {token}"))
        .content_type("application/json")
        .send(body.to_string())
        .map_err(|e| {
            Refusal::control_plane(format!("the control plane at {origin} is unreachable: {e}"))
        })?;
    let status = response.status().as_u16();
    let text = response
        .body_mut()
        .with_config()
        .limit(ANSWER_LIMIT)
        .read_to_string()
        .map_err(|e| {
            Refusal::control_plane(format!(
                "the control plane's answer, {status}, could not be read: {e}"
            ))
        })?;
    let answer: Value = serde_json::from_str(&text).map_err(|_| {
        Refusal::control_plane(format!(
            "the control plane answered {status} with no message of version 1"
        ))
    })?;
    if let Some(words) = check::failure(check::to_runner(), &answer) {
        return Err(Refusal::control_plane(format!(
            "the control plane's answer, {status}, fails version 1's check: {words}"
        )));
    }
    let kind = answer
        .get("body")
        .and_then(Value::as_object)
        .and_then(|body| body.keys().next().cloned())
        .unwrap_or_default();
    let answer: Message<ToRunner> = serde_json::from_value(answer).map_err(|e| {
        Refusal::control_plane(format!(
            "the control plane's answer, {status}, could not be read: {e}"
        ))
    })?;

    match (status, answer.body) {
        (200, ToRunner::Enrolled(enrolled)) => {
            store.write(&table.key_name(label), &key).map_err(|words| {
                Refusal::host(format!(
                    "the control plane enrolled `{label}` as host {}, but its key could not be \
                     kept: {words}. The control plane holds a key this host does not, and a \
                     new token enrolls the partition again",
                    enrolled.host
                ))
            })?;
            Ok(Enrolled {
                partition: label.to_string(),
                host: enrolled.host,
            })
        }
        (401 | 403, ToRunner::EnrollRefused(refused)) => Err(Refusal::control_plane(refused.words)),
        (400, ToRunner::MessageRefused(refused)) => Err(Refusal::control_plane(format!(
            "the control plane refused the enrollment's message: {}",
            refused.words
        ))),
        (status, _) => Err(Refusal::control_plane(format!(
            "the control plane answered {status} with `{kind}`, which is no answer to an \
             enrollment"
        ))),
    }
}
