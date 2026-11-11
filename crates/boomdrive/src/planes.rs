//! The control planes this binary carries (design Decisions 1 and 4;
//! core.2): each implementation's word, as `[control_plane]`'s `type`
//! names it, against the function that opens it and the one that enrolls
//! a partition of it, where it has one. The list is data, and the one
//! place outside an implementation that names one: every other module
//! reaches the control plane through `ControlPlane` alone, served as
//! parts, and `boomdrive enroll` through [`enroll`].

use boomdrive_bounded::Halt;
use boomdrive_control::{Part, Wake};
use std::path::Path;
use std::time::Duration;

/// Where an implementation says what it did and could not do, one line
/// each.
pub type Say = Box<dyn FnMut(String) + Send>;

/// What makes the say of each part an implementation opens, by its
/// label.
pub type SayFor<'a> = &'a dyn Fn(&str) -> Say;

/// What makes the wake of each part an implementation opens, by its
/// label, which it calls when its control plane pushes that part's work.
pub type WakeFor<'a> = &'a dyn Fn(&str) -> Wake;

/// Opens an implementation from its own table, `[control_plane.<type>]`,
/// of the configuration file `file`, keeping what it keeps under the
/// runner's data directory, each part making a control pass each
/// `poll_interval`, as its parts: at least one, each label once, in its
/// table's order, each handed the say and the wake made for its label
/// (record 0026; design Decision 1), and each try of a git call to a
/// remote it makes ended at `halt`, the runner's stop (record 0042 as
/// amended on 2026-11-11).
pub type Opener = fn(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
    halt: &Halt,
) -> Result<Vec<Part>, String>;

/// Enrolls the partition `label` of an implementation's own table, of the
/// configuration file `file`, with the one-time token `token` gives, or
/// words saying why it could not be read. The implementation asks for the
/// token only once it has found what it refuses before it.
pub type Enroller = fn(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: Token<'_>,
) -> Result<Enrolled, Refusal>;

/// What gives an enrollment's one-time token when it is asked for, or
/// words saying why it could not be read.
pub type Token<'a> = Box<dyn FnOnce() -> Result<String, String> + 'a>;

/// Reads an implementation's own table, `[control_plane.<type>]`, of the
/// configuration file `file`, opening nothing and asking nothing of the
/// host: words saying what it refuses, or nothing.
pub type Reader = fn(table: &toml::Table, file: &Path) -> Result<(), String>;

/// An implementation this binary carries.
pub struct Entry {
    /// The word `[control_plane]`'s `type` names it by.
    pub word: &'static str,
    pub open: Opener,
    /// Its enrollment, none where its partitions enroll nothing.
    pub enroll: Option<Enroller>,
    /// The keys of its table the runner's local layer may set, each taken
    /// whole (record 0009 as amended 2026-11-07; core.17).
    pub local_keys: &'static [&'static str],
    /// Its own reading of its table, which a local layer's table must
    /// pass to be taken.
    pub read: Reader,
}

/// Each implementation this binary carries, by its word.
pub const PLANES: [Entry; 2] = [
    Entry {
        word: "crew",
        open: crew,
        enroll: None,
        local_keys: &boomdrive_crew::config::LOCAL_KEYS,
        read: crew_read,
    },
    Entry {
        word: "hosted",
        open: hosted,
        enroll: Some(hosted_enroll),
        local_keys: &boomdrive_hosted::table::LOCAL_KEYS,
        read: hosted_read,
    },
];

/// A partition enrolled: the control plane's id for the host, which the
/// runner shows and keeps nowhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enrolled {
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

/// crew's implementation, a part for each partition, which never wakes
/// the runner: crew's control plane pushes nothing, its records are read
/// at each pull, and nothing of it waits on `poll_interval`. Each listing
/// of its state repository is ended at `halt` (record 0042 as amended on
/// 2026-11-11).
fn crew(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    _poll_interval: Duration,
    say_for: SayFor<'_>,
    _wake_for: WakeFor<'_>,
    halt: &Halt,
) -> Result<Vec<Part>, String> {
    boomdrive_crew::open(table, file, data_dir, say_for, halt)
}

/// crew's reading of its table.
fn crew_read(table: &toml::Table, file: &Path) -> Result<(), String> {
    boomdrive_crew::config::read(table, file).map(drop)
}

/// The hosted implementation's reading of its table.
fn hosted_read(table: &toml::Table, file: &Path) -> Result<(), String> {
    boomdrive_hosted::table::Table::read(table, file).map(drop)
}

/// The hosted control plane's implementation, a part for each partition,
/// whose connections' backoff stops at `poll_interval` (core.22), and
/// which keeps nothing under the data directory (core.35). It makes no
/// git call, and takes no halt.
fn hosted(
    table: &toml::Table,
    file: &Path,
    _data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
    _halt: &Halt,
) -> Result<Vec<Part>, String> {
    boomdrive_hosted::open(table, file, poll_interval, say_for, wake_for)
}

/// The hosted implementation's enrollment, its key kept in the host's own
/// credential store (core.32).
fn hosted_enroll(
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: Token<'_>,
) -> Result<Enrolled, Refusal> {
    boomdrive_hosted::enroll(table, file, label, token)
        .map(|enrolled| Enrolled {
            host: enrolled.host,
        })
        .map_err(|refused| Refusal {
            words: refused.words,
            host_to_fix: refused.host_to_fix,
        })
}

/// The entry whose word is `word`, none where this binary carries none.
pub fn carried(word: &str) -> Option<&'static Entry> {
    PLANES.iter().find(|entry| entry.word == word)
}

/// The entry whose word is `word`, or why there is none, naming the
/// words this binary carries.
fn entry(word: &str, file: &Path) -> Result<&'static Entry, String> {
    carried(word).ok_or_else(|| {
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
/// `data_dir`, each part making a control pass each `poll_interval`, as
/// its parts, each handed the say and the wake `say_for` and `wake_for`
/// make for its label, and the runner's halt, `halt`, which ends each try
/// of a git call to a remote it makes (record 0042 as amended on
/// 2026-11-11); or why it could not be opened. A word this binary does
/// not carry is refused, naming those it does.
pub fn open(
    word: &str,
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
    halt: &Halt,
) -> Result<Vec<Part>, String> {
    (entry(word, file)?.open)(
        table,
        file,
        data_dir,
        poll_interval,
        say_for,
        wake_for,
        halt,
    )
}

/// Enrolls the partition `label` of the implementation whose word is
/// `word`, from its own table, `table`, of the configuration file `file`,
/// with the one-time token `token` gives, or words saying why it could
/// not be read (design Decision 4). A word this binary does not carry, or
/// one whose implementation enrolls nothing, is refused as the host's to
/// fix before the token is asked for.
pub fn enroll(
    word: &str,
    table: &toml::Table,
    file: &Path,
    label: &str,
    token: impl FnOnce() -> Result<String, String>,
) -> Result<Enrolled, Refusal> {
    let host = |words| Refusal {
        words,
        host_to_fix: true,
    };
    let entry = entry(word, file).map_err(host)?;
    let Some(enroller) = entry.enroll else {
        return Err(host(format!(
            "{}: `type` is {word:?}, a control plane that enrolls nothing: `boomdrive enroll` \
             enrolls a partition of {}",
            file.display(),
            listed(PLANES.iter().filter(|entry| entry.enroll.is_some()))
        )));
    };
    enroller(table, file, label, Box::new(token))
}
