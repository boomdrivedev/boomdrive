//! The hosted control plane's forge tokens (design Decision 7 of
//! a-cloud-host-gets-its-github-tokens-from-the-cloud; record 0053;
//! core.33): under a hosted control plane the Boomdrive App is the control
//! plane's, so a served repository's token is asked of it by one
//! `POST runner/forge-token` of version 4, under the key of a partition
//! served the repository, each partition so served asked in the table's
//! order until one gives a token. Each ask is signed afresh over the
//! context `forge token`, which binds its repository, with the partition's
//! key read from the host's store at that ask (core.31, core.32). The
//! store is opened at each token asked, so opening the tokens reads the
//! hosted table alone.
//!
//! An ask opens no socket and no part, keeps nothing, and says no line:
//! its answer, a token, the App's absence or a refusal, is its caller's,
//! and no words of it hold a token, a statement or a key (core.4,
//! core.11, core.35). Every answer is checked against version 4 before it
//! is read, whatever its status (core.42), and the asks together stop at
//! [`ASK_FOR`].

use crate::check;
use crate::key::{self, Store};
use crate::protocol::v4::{self, FromRunner, Message, MessageId, ToRunner, VERSION};
use crate::table::{Partition, Table};
use boomdrive_control::{ForgeToken, ForgeTokens, TokenAsked};
use serde_json::Value;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use ureq::tls::{RootCerts, TlsConfig};
use uuid::Uuid;

/// How long the asks of one token may take together, from the first
/// ask's start to the last answer's last byte, so a `boomdrive
/// forge-token get` answers within the thirty seconds sluice gives a
/// credential command (sluice record 0108), whatever the control plane
/// does.
pub const ASK_FOR: Duration = Duration::from_secs(20);

/// The most bytes of the control plane's answer read.
const ANSWER_LIMIT: u64 = 64 * 1024;

/// The path an ask goes to, as its words name it.
const PATH: &str = "`runner/forge-token`";

/// What opens the store each partition's key is read from, or says why it
/// could not be opened: the host's own, [`key::store_for_host`], or a
/// test's.
pub type OpenStore = Box<dyn Fn() -> Result<Box<dyn Store>, String> + Send + Sync>;

/// The hosted control plane's forge tokens: its table, what opens the
/// host's store, and how long the asks of one token may take.
pub struct Tokens {
    table: Table,
    store: OpenStore,
    ask_for: Duration,
}

/// What one partition's ask came to, where it gave no token.
enum Passed {
    /// The control plane's word that the App's installation does not cover
    /// the repository, with its words.
    NotInstalled(String),
    /// A refusal of the control plane's, or an answer that is none.
    Refused(String),
    /// The partition could not be asked: no store, or no key of it.
    NotAsked(String),
}

impl Tokens {
    /// The forge tokens of the hosted table `table`, each partition's key
    /// read from the store `store` opens at each token asked; the asks of
    /// one token stopped at [`ASK_FOR`].
    pub fn new(table: Table, store: OpenStore) -> Tokens {
        Tokens {
            table,
            store,
            ask_for: ASK_FOR,
        }
    }

    /// `self`, the asks of one token stopped at `limit`, a test's shorter
    /// limit. No configuration key, environment variable or flag sets it
    /// (core.17).
    pub fn asking_for(self, limit: Duration) -> Tokens {
        Tokens {
            ask_for: limit,
            ..self
        }
    }

    /// One partition's ask of `repository`'s token, the asks having begun
    /// at `began`: its key read, a statement signed for it, and one
    /// request sent, its answer read once it passes version 4 and names
    /// the ask and the repository.
    fn ask(
        &self,
        store: &Result<Box<dyn Store>, String>,
        partition: &Partition,
        repository: &str,
        began: Instant,
    ) -> Result<ForgeToken, Passed> {
        let label = &partition.label;
        let enroll = self.table.enroll_command(label);
        let store = store.as_ref().map_err(|words| {
            Passed::NotAsked(format!(
                "the host's credential store could not be opened: {words}; its key is made by \
                 `{enroll}`"
            ))
        })?;
        let key = match store.read(&self.table.key_name(label)) {
            Ok(Some(key)) => key,
            Ok(None) => {
                return Err(Passed::NotAsked(format!(
                    "this host's store holds no key of `{label}`: enroll it with `{enroll}`"
                )))
            }
            Err(words) => {
                return Err(Passed::NotAsked(format!(
                    "{words}; its key is made by `{enroll}`"
                )))
            }
        };

        let origin = self.table.origin();
        let nonce = key::nonce().map_err(Passed::NotAsked)?;
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|since| since.as_secs())
            .unwrap_or_default();
        let statement = key::forge_token_statement(&key, &origin, time, &nonce, repository);
        let id = MessageId::new(Uuid::now_v7().to_string());
        let message = Message::new(
            id.clone(),
            FromRunner::ForgeToken(v4::ForgeToken {
                repository: repository.to_string(),
            }),
        );
        let body = serde_json::to_value(&message)
            .map_err(|e| Passed::NotAsked(format!("the ask could not be written: {e}")))?;
        if let Some(words) = check::failure(check::from_runner(), &body) {
            return Err(Passed::NotAsked(format!(
                "the ask fails version {VERSION}'s check, so is not sent: {words}"
            )));
        }

        let left = self.ask_for.saturating_sub(began.elapsed());
        if left.is_zero() {
            return Err(Passed::Refused(format!(
                "no ask went to {PATH}: the asks stop at {}s",
                self.ask_for.as_secs_f64()
            )));
        }
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .http_status_as_error(false)
            .max_redirects(0)
            .timeout_global(Some(left))
            .tls_config(
                TlsConfig::builder()
                    .root_certs(RootCerts::PlatformVerifier)
                    .build(),
            )
            .build()
            .into();
        let mut response = agent
            .post(self.table.forge_token_address().as_str())
            .header("Authorization", &statement)
            .content_type("application/json")
            .send(body.to_string())
            .map_err(|e| {
                Passed::Refused(format!(
                    "the control plane at {origin} could not be asked at {PATH}: {e}"
                ))
            })?;
        let status = response.status().as_u16();
        let text = response
            .body_mut()
            .with_config()
            .limit(ANSWER_LIMIT)
            .read_to_string()
            .map_err(|e| {
                Passed::Refused(format!(
                    "the control plane's answer at {PATH}, {status}, could not be read: {e}"
                ))
            })?;
        let answer: Value = serde_json::from_str(&text).map_err(|_| {
            Passed::Refused(format!(
                "the control plane answered {status} at {PATH} with no message of version \
                 {VERSION}"
            ))
        })?;
        if let Some(words) = check::failure(check::to_runner(), &answer) {
            return Err(Passed::Refused(format!(
                "the control plane's answer at {PATH}, {status}, fails version {VERSION}'s \
                 check: {words}"
            )));
        }
        let kind = answer
            .get("body")
            .and_then(Value::as_object)
            .and_then(|body| body.keys().next().cloned())
            .unwrap_or_default();
        let answer: Message<ToRunner> = serde_json::from_value(answer).map_err(|e| {
            Passed::Refused(format!(
                "the control plane's answer at {PATH}, {status}, could not be read: {e}"
            ))
        })?;
        match (status, answer.body) {
            (200, ToRunner::ForgeTokenGiven(given)) => {
                if given.forge_token != id || given.repository != repository {
                    return Err(Passed::Refused(format!(
                        "the control plane's answer at {PATH}, {status}, gives a token of {} \
                         for the ask {}, not of {repository} for this ask",
                        given.repository, given.forge_token
                    )));
                }
                Ok(ForgeToken::new(given.token, given.expires_at.time()))
            }
            (401 | 403 | 502, ToRunner::ForgeTokenRefused(refused)) => {
                if refused.forge_token != id || refused.repository != repository {
                    return Err(Passed::Refused(format!(
                        "the control plane's answer at {PATH}, {status}, refuses a token of {} \
                         for the ask {}, not of {repository} for this ask",
                        refused.repository, refused.forge_token
                    )));
                }
                match refused.not_installed {
                    true => Err(Passed::NotInstalled(refused.words)),
                    false => Err(Passed::Refused(format!(
                        "the control plane refused {repository}'s token: {}",
                        refused.words
                    ))),
                }
            }
            (400, ToRunner::MessageRefused(refused)) if refused.message == id => {
                Err(Passed::Refused(format!(
                    "the control plane refused the ask at {PATH}: {}",
                    refused.words
                )))
            }
            (status, _) => Err(Passed::Refused(format!(
                "the control plane answered {status} at {PATH} with `{kind}`, which is no \
                 answer to this ask"
            ))),
        }
    }
}

impl ForgeTokens for Tokens {
    /// `repository`'s token, asked under each partition served it in the
    /// table's order until one gives it: none served refuses at once; the
    /// App's absence where every partition asked says its installation does
    /// not cover the repository; a refusal the host must fix where no
    /// partition could be asked; and otherwise a refusal, each partition's
    /// words joined in the table's order, each naming its label.
    fn token(&self, repository: &str) -> TokenAsked {
        let served: Vec<&Partition> = self
            .table
            .partitions
            .iter()
            .filter(|partition| partition.repositories.iter().any(|r| r == repository))
            .collect();
        if served.is_empty() {
            return TokenAsked::Refused {
                words: format!(
                    "{repository} is a repository no partition of {} is served",
                    self.table.role.table()
                ),
                host_to_fix: false,
            };
        }
        let began = Instant::now();
        let store = (self.store)();
        let mut passed = Vec::new();
        for partition in served {
            match self.ask(&store, partition, repository, began) {
                Ok(token) => return TokenAsked::Token(token),
                Err(why) => passed.push((partition.label.as_str(), why)),
            }
        }
        let words = passed
            .iter()
            .map(|(label, why)| {
                let (Passed::NotInstalled(words)
                | Passed::Refused(words)
                | Passed::NotAsked(words)) = why;
                format!("`{label}`: {words}")
            })
            .collect::<Vec<_>>()
            .join("; ");
        let asked: Vec<&Passed> = passed
            .iter()
            .map(|(_, why)| why)
            .filter(|why| !matches!(why, Passed::NotAsked(_)))
            .collect();
        if asked.is_empty() {
            TokenAsked::Refused {
                words,
                host_to_fix: true,
            }
        } else if asked
            .iter()
            .all(|why| matches!(why, Passed::NotInstalled(_)))
        {
            TokenAsked::NotInstalled(words)
        } else {
            TokenAsked::Refused {
                words,
                host_to_fix: false,
            }
        }
    }
}
