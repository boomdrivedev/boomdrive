//! The control planes this binary carries (design Decisions 1 and 4;
//! core.2): each implementation's word, as `[control_plane]`'s `type`
//! names it, against the function that opens it and the one that enrolls
//! a partition of it, where it has one. The list is data, and the one
//! place outside an implementation that names one: every other module
//! reaches the control plane through `ControlPlane` alone, served as
//! parts, `boomdrive enroll` through [`enroll`], and `boomdrive setup`
//! through [`set_up_entry`] (record 0009 as amended 2026-11-11; design
//! Decision 11 of the change a-persons-drive-is-set-up-by-one-command).

use boomdrive_control::{Part, Wake};
use std::path::{Path, PathBuf};
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
/// (record 0026; design Decision 1).
pub type Opener = fn(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
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
    /// Its setup of a person's drive, none where it sets up none (record
    /// 0009 as amended 2026-11-11).
    pub set_up: Option<SetUp>,
}

/// An implementation's setup of a drive on a host Nix does not manage
/// (record 0009 as amended 2026-11-11; design Decision 11 of the change
/// a-persons-drive-is-set-up-by-one-command): what the host holds of the
/// drive's answers, those answers settled, the programs it runs found,
/// and the drive set up. What it holds, settles and finds is its own,
/// carried by the runner from one step to the next unread.
pub struct SetUp {
    /// What the host holds of the drive `drive`: from `table`, the
    /// implementation's own table of the runner's file where there is one,
    /// and from its tool's own files under the home `home`.
    pub held: fn(table: Option<&toml::Table>, home: &Path, drive: &str) -> Held,
    /// The drive's answers, each `asked` gives, else `held`, else the
    /// implementation's own, checked against the drive's `repositories`
    /// before any program runs; refused as the host's to fix.
    pub settle: fn(asked: &Asked, held: &Held, repositories: &[String]) -> Result<Settled, Refusal>,
    /// The programs it runs, each found by `find` on setup's `PATH`, and
    /// what it reads of the host before it writes anything.
    pub prepare: fn(find: Find<'_>) -> Result<Prepared, Refusal>,
    /// The drive set up, a tool's refusal carried with its words.
    pub set_up: fn(
        prepared: &Prepared,
        settled: &Settled,
        drive: &Drive<'_>,
    ) -> Result<DriveSetUp, Refusal>,
}

/// What finds a program on setup's `PATH` by its name, none where it is
/// not there.
pub type Find<'a> = &'a dyn Fn(&str) -> Option<PathBuf>;

/// What setup's flags gave an implementation, each as text: none or empty
/// where a flag was not given.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Asked {
    /// `--state-repository`.
    pub state_repository: Option<String>,
    /// Each `--blueprints`, in order.
    pub blueprints: Vec<String>,
    /// Each `--runs <role> <how>`, in order.
    pub runs: Vec<(String, String)>,
    /// `--review <after_changes> <after_hours>`.
    pub review: Option<(String, String)>,
}

/// What the host holds of a drive's answers, as an implementation read
/// it.
#[derive(Clone, Debug)]
pub struct Held(HeldBy);

#[derive(Clone, Debug)]
enum HeldBy {
    Crew(boomdrive_crew::setup::Held),
}

impl Held {
    /// The state repository the host holds for the drive, where it holds
    /// one: setup asks a terminal for it only where neither its flag nor
    /// this gives it.
    pub fn state_repository(&self) -> Option<&str> {
        match &self.0 {
            HeldBy::Crew(held) => held.state_repository.as_deref(),
        }
    }
}

/// A drive's answers, settled by an implementation.
#[derive(Clone, Debug)]
pub struct Settled(SettledBy);

#[derive(Clone, Debug)]
enum SettledBy {
    Crew(boomdrive_crew::setup::Settled),
}

/// The programs an implementation runs, found, and what it read of the
/// host.
#[derive(Clone, Debug)]
pub struct Prepared(PreparedBy);

#[derive(Clone, Debug)]
enum PreparedBy {
    Crew(boomdrive_crew::setup::Prepared),
}

/// The drive setup sets up: its name, its account, the folder its
/// repositories' views are kept under, each repository it keeps, and the
/// implementation's own table of the runner's file, where there is one.
pub struct Drive<'a> {
    pub drive: &'a str,
    pub account: &'a str,
    pub folder: &'a Path,
    pub kept: &'a [Kept],
    pub table: Option<&'a toml::Table>,
}

/// A repository the drive keeps, with the runner's clone's folder and its
/// view (core.57).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Kept {
    pub repository: String,
    pub dir: PathBuf,
    pub checkout: PathBuf,
}

/// A drive set up by an implementation: its own table of the runner's
/// file, with the drive set; its answer, which setup carries as given; a
/// line for each step; and the command that starts its agents, which
/// setup names and runs not (core.13).
#[derive(Clone, Debug, PartialEq)]
pub struct DriveSetUp {
    pub table: toml::Table,
    pub answer: serde_json::Value,
    pub lines: Vec<String>,
    pub start: String,
}

/// Each implementation this binary carries, by its word.
pub const PLANES: [Entry; 2] = [
    Entry {
        word: "crew",
        open: crew,
        enroll: None,
        local_keys: &boomdrive_crew::config::LOCAL_KEYS,
        read: crew_read,
        set_up: Some(SetUp {
            held: crew_held,
            settle: crew_settle,
            prepare: crew_prepare,
            set_up: crew_set_up,
        }),
    },
    Entry {
        word: "hosted",
        open: hosted,
        enroll: Some(hosted_enroll),
        local_keys: &boomdrive_hosted::table::LOCAL_KEYS,
        read: hosted_read,
        set_up: None,
    },
];

/// A partition enrolled: the control plane's id for the host, which the
/// runner shows and keeps nowhere.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enrolled {
    pub host: String,
}

/// An enrollment or a drive's setup refused, with words saying why, and
/// whether the host must be fixed, its configuration, its store, its token
/// or setup's answers, before it can go (core.39's exit 2), or the control
/// plane or a tool refused it or could not be reached (exit 1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub words: String,
    pub host_to_fix: bool,
}

/// crew's implementation, a part for each partition, which never wakes
/// the runner: crew's control plane pushes nothing, its records are read
/// at each pull, and nothing of it waits on `poll_interval`.
fn crew(
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    _poll_interval: Duration,
    say_for: SayFor<'_>,
    _wake_for: WakeFor<'_>,
) -> Result<Vec<Part>, String> {
    boomdrive_crew::open(table, file, data_dir, say_for)
}

/// crew's reading of its table.
fn crew_read(table: &toml::Table, file: &Path) -> Result<(), String> {
    boomdrive_crew::config::read(table, file).map(drop)
}

/// crew's reading of what the host holds of a drive: its partition in
/// crew's table, and its entry in crew's own file under `home`.
fn crew_held(table: Option<&toml::Table>, home: &Path, drive: &str) -> Held {
    let teams = boomdrive_crew::setup::teams_file(home);
    Held(HeldBy::Crew(boomdrive_crew::setup::held(
        table, &teams, drive,
    )))
}

/// crew's settling of a drive's answers.
fn crew_settle(asked: &Asked, held: &Held, repositories: &[String]) -> Result<Settled, Refusal> {
    let HeldBy::Crew(held) = &held.0;
    let asked = boomdrive_crew::setup::Asked {
        state_repository: asked.state_repository.clone(),
        blueprints: asked.blueprints.clone(),
        runs: asked.runs.clone(),
        review: asked.review.clone(),
    };
    boomdrive_crew::setup::settle(&asked, held, repositories)
        .map(|settled| Settled(SettledBy::Crew(settled)))
        .map_err(crew_refusal)
}

/// crew's finding of `hostname`, `gh` and `crew`, and of this host's name.
fn crew_prepare(find: Find<'_>) -> Result<Prepared, Refusal> {
    boomdrive_crew::setup::prepare(find)
        .map(|prepared| Prepared(PreparedBy::Crew(prepared)))
        .map_err(crew_refusal)
}

/// crew's setup of a drive: its state repository, `crew drive set`, and
/// its table.
fn crew_set_up(
    prepared: &Prepared,
    settled: &Settled,
    drive: &Drive<'_>,
) -> Result<DriveSetUp, Refusal> {
    let PreparedBy::Crew(prepared) = &prepared.0;
    let SettledBy::Crew(settled) = &settled.0;
    let kept: Vec<boomdrive_crew::setup::Kept> = drive
        .kept
        .iter()
        .map(|kept| boomdrive_crew::setup::Kept {
            repository: kept.repository.clone(),
            dir: kept.dir.clone(),
            checkout: kept.checkout.clone(),
        })
        .collect();
    boomdrive_crew::setup::set_up(
        prepared,
        settled,
        drive.drive,
        drive.account,
        drive.folder,
        &kept,
        drive.table,
    )
    .map(|done| DriveSetUp {
        table: done.table,
        answer: done.answer,
        lines: done.lines,
        start: done.start,
    })
    .map_err(crew_refusal)
}

/// crew's refusal of a setup, as the interface's.
fn crew_refusal(refused: boomdrive_crew::setup::Refusal) -> Refusal {
    Refusal {
        words: refused.words,
        host_to_fix: refused.host_to_fix,
    }
}

/// The hosted implementation's reading of its table.
fn hosted_read(table: &toml::Table, file: &Path) -> Result<(), String> {
    boomdrive_hosted::table::Table::read(table, file).map(drop)
}

/// The hosted control plane's implementation, a part for each partition,
/// whose connections' backoff stops at `poll_interval` (core.22), and
/// which keeps nothing under the data directory (core.35).
fn hosted(
    table: &toml::Table,
    file: &Path,
    _data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
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

/// The one entry that sets up a drive, with its setup, none where no
/// implementation this binary carries sets one up.
pub fn set_up_entry() -> Option<(&'static Entry, &'static SetUp)> {
    PLANES
        .iter()
        .find_map(|entry| Some((entry, entry.set_up.as_ref()?)))
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
/// make for its label; or why it could not be opened. A word this binary
/// does not carry is refused, naming those it does.
pub fn open(
    word: &str,
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    poll_interval: Duration,
    say_for: SayFor<'_>,
    wake_for: WakeFor<'_>,
) -> Result<Vec<Part>, String> {
    (entry(word, file)?.open)(table, file, data_dir, poll_interval, say_for, wake_for)
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
