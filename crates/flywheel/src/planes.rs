//! The control planes this binary carries (design Decisions 1 and 4;
//! core.2): each implementation's word, as `[control_plane]`'s `type`
//! names it, against the function that opens it and the one that enrols
//! a partition of it, where it has one. The list is data, and the one
//! place outside an implementation that names one: every other module
//! reaches the control plane through `ControlPlane` alone, and
//! `flywheel enroll` through [`enrol`].

use flywheel_control::{ControlPlane, Wake};
use std::path::Path;
use std::time::Duration;

/// A control plane as the runner holds it, on its loop's one thread.
pub type Plane = Box<dyn ControlPlane + Send>;

/// Where an implementation says what it did and could not do, one line
/// each.
pub type Say = Box<dyn FnMut(String) + Send>;

/// Opens an implementation from its own table, `[control_plane.<type>]`,
/// of the configuration file `file`, keeping what it keeps under the
/// runner's data directory, making a control pass each `poll_interval`,
/// and handed `wake` for when its control plane pushes work.
pub type Opener = fn(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say: Say,
    wake: Wake,
) -> Result<Plane, String>;

/// Enrols the partition `label` of an implementation's own table, of the
/// configuration file `file`, with its one-time `token`.
pub type Enroller =
    fn(table: &toml::Table, file: &Path, label: &str, token: &str) -> Result<Enrolled, Refusal>;

/// An implementation this binary carries.
pub struct Entry {
    /// The word `[control_plane]`'s `type` names it by.
    pub word: &'static str,
    pub open: Opener,
    /// Its enrolment, none where its partitions enrol nothing.
    pub enrol: Option<Enroller>,
}

/// Each implementation this binary carries, by its word.
pub const PLANES: [Entry; 2] = [
    Entry {
        word: "crew",
        open: crew,
        enrol: None,
    },
    Entry {
        word: "hosted",
        open: hosted,
        enrol: Some(hosted_enrol),
    },
];

/// A partition enrolled: the control plane's id for the host, which the
/// runner shows and keeps nowhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enrolled {
    pub host: String,
}

/// An enrolment refused, with words saying why, and whether the host must
/// be fixed, its configuration, its store or its token, before an
/// enrolment can go (core.39's exit 2), or the control plane refused it or
/// could not be reached (exit 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub words: String,
    pub host_to_fix: bool,
}

/// crew's implementation, which never wakes the runner: crew's control
/// plane pushes nothing, its records are read at each pull, and nothing
/// of it waits on `poll_interval`.
fn crew(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    _poll_interval: Duration,
    say: Say,
    _wake: Wake,
) -> Result<Plane, String> {
    Ok(Box::new(flywheel_crew::open(table, file, data_dir, say)?))
}

/// The hosted control plane's implementation, whose connections' backoff
/// stops at `poll_interval` (core.22), and which keeps nothing under the
/// data directory (core.35).
fn hosted(
    table: &toml::Table,
    file: &Path,
    _data_dir: &Path,
    poll_interval: Duration,
    say: Say,
    wake: Wake,
) -> Result<Plane, String> {
    Ok(Box::new(flywheel_hosted::open(
        table,
        file,
        poll_interval,
        say,
        wake,
    )?))
}

/// The hosted implementation's enrolment, its key kept in the host's own
/// credential store (core.32).
fn hosted_enrol(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: &str,
) -> Result<Enrolled, Refusal> {
    flywheel_hosted::enrol(table, file, label, token)
        .map(|enrolled| Enrolled {
            host: enrolled.host,
        })
        .map_err(|refused| Refusal {
            words: refused.words,
            host_to_fix: refused.host_to_fix,
        })
}

/// The entry whose word is `word`, or why there is none, naming the
/// words this binary carries.
fn entry(word: &str, file: &Path) -> Result<&'static Entry, String> {
    PLANES
        .iter()
        .find(|entry| entry.word == word)
        .ok_or_else(|| {
            format!(
                "{}: `type` is {word:?}, a control plane this binary does not carry: it carries {}",
                file.display(),
                listed(PLANES.iter())
            )
        })
}

/// Each entry's word, quoted, joined by commas.
fn listed<'a>(entries: impl Iterator<Item = &'a Entry>) -> String {
    let words: Vec<String> = entries.map(|entry| format!("`{}`", entry.word)).collect();
    words.join(", ")
}

/// Opens the implementation whose word is `word` from its own table,
/// `table`, of the configuration file `file`, keeping what it keeps under
/// `data_dir`, making a control pass each `poll_interval`, and handed
/// `wake`; or why it could not be opened. A word this binary does not
/// carry is refused, naming those it does.
pub fn open(
    word: &str,
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say: Say,
    wake: Wake,
) -> Result<Plane, String> {
    (entry(word, file)?.open)(table, file, data_dir, poll_interval, say, wake)
}

/// Enrols the partition `label` of the implementation whose word is
/// `word`, from its own table, `table`, of the configuration file `file`,
/// with the one-time token `token` gives (design Decision 4). A word this
/// binary does not carry, or one whose implementation enrols nothing, is
/// refused as the host's to fix before the token is asked for.
pub fn enrol(
    word: &str,
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: impl FnOnce() -> Result<String, Refusal>,
) -> Result<Enrolled, Refusal> {
    let host = |words| Refusal {
        words,
        host_to_fix: true,
    };
    let entry = entry(word, file).map_err(host)?;
    let Some(enroller) = entry.enrol else {
        return Err(host(format!(
            "{}: `type` is {word:?}, a control plane that enrols nothing: `flywheel enroll` \
             enrols a partition of {}",
            file.display(),
            listed(PLANES.iter().filter(|entry| entry.enrol.is_some()))
        )));
    };
    enroller(table, file, label, &token()?)
}
