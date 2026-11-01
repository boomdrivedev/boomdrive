//! The control pass (design Decision 5). It takes the runner's report,
//! reads the host once from Flume's status, hands the hello to the pull,
//! answers each want in the control plane's order, and sends the whole
//! report. The runner takes only what it was offered, launches only
//! through Flume's command line, and carries each answer as Flume gave it
//! (core.1, core.9, core.10). It reaches its control plane only through
//! `ControlPlane` (core.2), and acts on no place it did not launch. Each
//! act is said on standard error, one line each, prefixed with its id, and
//! a pass that changes nothing says nothing.

use crate::config::{Address, Config};
use crate::flume::{Flume, LaunchArgs, Reply, Unrun};
use crate::hello::{self, Tools};
use crate::run::Notices;
use crate::{facts, report};
use flywheel_control::{
    Act, ActSort, Acted, Answer, At, ControlPlane, Id, Launched, Offer, Outcome, Report, Unread,
    Unsent, Want,
};
use serde_json::Value;
use std::path::{Path, PathBuf};

/// How the runner keeps the repository a launch names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Keeping {
    /// The configuration names it, and its clone is made in this
    /// container.
    Ready(PathBuf),
    /// The configuration names it, and its clone is not yet made.
    NotYet,
    /// The configuration does not name it.
    NotNamed,
}

/// The repositories the runner keeps, as a control pass reads them.
pub trait Clones {
    /// How the runner keeps the repository at `address`, `owner/name`.
    fn keeping(&self, address: &str) -> Keeping;
    /// Each address the configuration names whose clone is made, in the
    /// configuration's order and as it writes each.
    fn made(&self) -> Vec<String>;
}

/// The runner's clones against its configuration: `Clones` on a host.
#[derive(Clone, Debug)]
pub struct Configured {
    named: Vec<Address>,
    clones: Vec<(Address, PathBuf)>,
}

impl Configured {
    /// `clones`, as `clone::find` gives them, against `config`.
    pub fn new(config: &Config, clones: Vec<(Address, PathBuf)>) -> Configured {
        Configured {
            named: config.repositories.clone(),
            clones,
        }
    }

    fn container(&self, address: &Address) -> Option<&PathBuf> {
        self.clones
            .iter()
            .find(|(made, _)| made == address)
            .map(|(_, container)| container)
    }
}

impl Clones for Configured {
    fn keeping(&self, address: &str) -> Keeping {
        let Ok(address) = Address::parse(address) else {
            return Keeping::NotNamed;
        };
        if !self.named.contains(&address) {
            return Keeping::NotNamed;
        }
        match self.container(&address) {
            Some(container) => Keeping::Ready(container.clone()),
            None => Keeping::NotYet,
        }
    }

    fn made(&self) -> Vec<String> {
        self.named
            .iter()
            .filter(|address| self.container(address).is_some())
            .map(Address::to_string)
            .collect()
    }
}

/// What a control pass runs over.
pub struct Carrier<'a> {
    pub flume: &'a dyn Flume,
    pub clones: &'a dyn Clones,
    pub tools: &'a dyn Tools,
    /// Told of each landing the pass reads, so its repository is kept at
    /// once.
    pub notices: &'a Notices,
    /// The runner's control directory, `<data>/control`: its report file
    /// and the folder its briefs pass through.
    pub control_dir: &'a Path,
}

/// What a control pass leaves for the loop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Passed {
    /// The earliest expiry of a lease the report holds, where one has an
    /// expiry.
    pub expires: Option<At>,
    /// Whether a call the pass made failed, and was named.
    pub failed: bool,
}

impl Carrier<'_> {
    /// One control pass against `plane`, each line said to `say` as it is
    /// said.
    pub fn pass(&self, plane: &mut dyn ControlPlane, say: &mut dyn FnMut(String)) -> Passed {
        let file = report::file(self.control_dir);
        let kept = match report::read(&file) {
            Ok(kept) => kept,
            Err(why) => {
                say(format!("flywheel: {why}: nothing carried this pass"));
                return Passed {
                    expires: None,
                    failed: true,
                };
            }
        };
        let saved = kept.clone().unwrap_or_default();
        let mut failed = false;
        let status = self.flume.status();
        let listing = match &status {
            Ok(Reply::Given(text)) => serde_json::from_str::<Value>(text).ok(),
            Ok(Reply::Refused(words)) | Err(Unrun(words)) => {
                say(format!(
                    "flywheel: Flume's status could not be read: {words}"
                ));
                failed = true;
                None
            }
        };
        let hello = hello::hello(
            self.tools,
            self.clones.made(),
            hello::capacity(&status),
            &saved,
        );
        let pulled = match plane.pull(&hello, &saved) {
            Ok(pulled) => pulled,
            Err(Unread(words)) => {
                say(format!(
                    "flywheel: the control plane could not be read: {words}: nothing carried this pass"
                ));
                return Passed {
                    expires: expires(&saved),
                    failed: true,
                };
            }
        };
        // A report file lost with the data directory is taken from the
        // control plane, which holds every answer the runner reported.
        let report = kept.or(pulled.report).unwrap_or_default();
        let mut work = Work {
            carrier: self,
            wants: &pulled.wants,
            file,
            saved,
            report,
            room: Room::read(listing.as_ref()),
            listing,
            failed,
        };
        for want in &pulled.wants {
            work.want(want, say);
            if !work.save(say) {
                say("flywheel: nothing more is carried out this pass".to_string());
                break;
            }
        }
        work.send(plane, say);
        Passed {
            expires: expires(&work.report),
            failed: work.failed,
        }
    }
}

/// One pass's work: the report as it stands, and what was read of the
/// host.
struct Work<'p> {
    carrier: &'p Carrier<'p>,
    /// What the pull gave, in the control plane's order.
    wants: &'p [Want],
    file: PathBuf,
    /// The report as the file holds it.
    saved: Report,
    report: Report,
    /// The pass's one `flume --json status`, where Flume gave it.
    listing: Option<Value>,
    room: Room,
    failed: bool,
}

impl Work<'_> {
    fn want(&mut self, want: &Want, say: &mut dyn FnMut(String)) {
        match want {
            Want::Offer(offer) => self.offer(offer, say),
            Want::Act(act) => self.act(act, say),
            Want::Command(_) | Want::Inbox(_) => {}
        }
    }

    /// Takes, leaves or refuses `offer` (design Decision 7).
    fn offer(&mut self, offer: &Offer, say: &mut dyn FnMut(String)) {
        let id = &offer.id;
        if let Some(launched) = self.report.launches.get_mut(id) {
            // The lease is the one the control plane gives now.
            launched.lease = offer.lease;
            if launched.answer.outcome != Outcome::Held {
                return;
            }
        }
        // Without Flume's listing no mark can be read, so nothing is
        // launched.
        let Some(listing) = &self.listing else {
            return;
        };
        let mark = mark(id);
        if let Some(place) = facts::marked(listing, &mark).and_then(place_of) {
            say(format!(
                "{id}: launched at {place}, as its mark {mark} shows"
            ));
            self.answer(offer, Outcome::Launched, Some(place), None);
            return;
        }
        let repository = &offer.launch.repository;
        let container = match self.carrier.clones.keeping(repository) {
            Keeping::Ready(container) => container,
            Keeping::NotYet => return,
            Keeping::NotNamed => {
                let words = format!("this host's configuration does not keep {repository}");
                say(format!("{id}: refused: {words}"));
                self.answer(offer, Outcome::Refused, None, Some(words));
                return;
            }
        };
        if let Some(words) = self.waits(&offer.launch.after) {
            if self.answer(offer, Outcome::Held, None, Some(words.clone())) {
                say(format!("{id}: held: {words}"));
            }
            return;
        }
        let stream = offer.launch.stream.as_deref();
        if let Some(words) = self.room.lacks(stream) {
            // A new offer with no room is left for the control plane to
            // give where it will; one already held stays held.
            if self.report.launches.contains_key(id)
                && self.answer(offer, Outcome::Held, None, Some(words.clone()))
            {
                say(format!("{id}: held: {words}"));
            }
            return;
        }
        self.launch(offer, &container, say);
    }

    /// Launches `offer` in the clone at `container` (design Decision 9).
    fn launch(&mut self, offer: &Offer, container: &Path, say: &mut dyn FnMut(String)) {
        let id = &offer.id;
        let args = launch_args(offer, container, self.priority(offer));
        let briefs = self.carrier.control_dir.join("briefs");
        match self.carrier.flume.launch(&args, &briefs) {
            Ok(Reply::Given(text)) => {
                let answer = serde_json::from_str::<Value>(&text).ok();
                let place = answer
                    .as_ref()
                    .and_then(facts::launched)
                    .and_then(facts::location)
                    .map(str::to_string);
                match &place {
                    Some(place) => say(format!("{id}: launched at {place}")),
                    None => say(format!(
                        "{id}: launched, though Flume's answer names no place"
                    )),
                }
                self.room.take(offer.launch.stream.as_deref());
                self.answer(offer, Outcome::Launched, place, None);
            }
            Ok(Reply::Refused(words)) => {
                say(format!("{id}: refused by Flume: {words}"));
                self.answer(offer, Outcome::Refused, None, Some(words));
            }
            Err(Unrun(words)) => {
                say(format!("{id}: not launched: {words}"));
                self.failed = true;
            }
        }
    }

    /// What `after` still waits on, each launch with where it stands, or
    /// none where every one's place has read landed. The runner reads only
    /// whether Flume says a place landed (core.1).
    fn waits(&self, after: &[Id]) -> Option<String> {
        let waiting: Vec<String> = after
            .iter()
            .filter_map(|launch| match self.standing(launch) {
                Standing::Landed => None,
                standing => Some(format!("{launch} ({})", standing.words())),
            })
            .collect();
        (!waiting.is_empty()).then(|| format!("waits on {}", waiting.join(", ")))
    }

    /// Where launch `id` stands, as the report holds it and Flume's status
    /// reads its place: the pass's listing where it holds the place, else
    /// the status last carried.
    fn standing(&self, id: &Id) -> Standing {
        let Some(launched) = self.report.launches.get(id) else {
            return Standing::Unlaunched;
        };
        let answer = &launched.answer;
        match answer.outcome {
            Outcome::Held => Standing::Unlaunched,
            Outcome::Refused => Standing::Refused(answer.words.clone()),
            Outcome::Done => Standing::Dropped,
            Outcome::Launched => match &answer.place_dir {
                Some(place) if self.landed(launched, place) => Standing::Landed,
                place => Standing::Launched(place.clone()),
            },
        }
    }

    /// Whether the place at `place` reads landed in the pass's listing,
    /// where it holds the place, else in the status `launched` carries.
    fn landed(&self, launched: &Launched, place: &str) -> bool {
        let listed = self
            .listing
            .as_ref()
            .and_then(|listing| facts::entry(listing, place).cloned());
        let carried = || {
            let text = &launched.status.as_ref()?.text;
            let answer = serde_json::from_str::<Value>(text).ok()?;
            facts::entry(&answer, place).cloned()
        };
        let entry = listed.or_else(carried);
        matches!(
            entry.as_ref().and_then(facts::over),
            Some(facts::Over::Landed(_))
        )
    }

    /// The priority `offer` is launched with: the newest an act on it set
    /// while it was not yet launched, in the control plane's order, else
    /// its own (crew's teams-cross Decision 6).
    fn priority(&self, offer: &Offer) -> Option<i64> {
        let set = self.wants.iter().filter_map(|want| match want {
            Want::Act(act) if act.launch == offer.id => match act.what {
                ActSort::Priority(priority) if self.done(&act.id) => Some(priority),
                _ => None,
            },
            _ => None,
        });
        set.last().or(offer.launch.priority)
    }

    /// Whether the report holds act `id` done.
    fn done(&self, id: &Id) -> bool {
        self.report
            .acts
            .get(id)
            .is_some_and(|acted| acted.outcome == Outcome::Done)
    }

    /// Whether the pull offers launch `id`.
    fn offered(&self, id: &Id) -> bool {
        self.wants
            .iter()
            .any(|want| matches!(want, Want::Offer(offer) if offer.id == *id))
    }

    /// Carries out or answers `act` on its launch (design Decision 9). An
    /// act the report holds is left as it stands.
    fn act(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        if self.report.acts.contains_key(&act.id) {
            return;
        }
        let Some(launched) = self.report.launches.get(&act.launch) else {
            // A launch the pull offers and the runner has not yet taken is
            // the control plane's to give: its act waits with it.
            if !self.offered(&act.launch) {
                let words = format!("{} is no launch of this runner's", act.launch);
                self.acted(act, Outcome::Refused, Some(words), say);
            }
            return;
        };
        if launched.answer.outcome == Outcome::Launched {
            return;
        }
        self.act_unlaunched(act, say);
    }

    /// Answers `act` on a launch the runner holds not yet launched, from
    /// the report, running no command. A priority is done, and the launch
    /// carries it when made; a drop is done, and the launch is never made;
    /// a retry or a done is refused, naming where the launch stands.
    fn act_unlaunched(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        let launched = &self.report.launches[&act.launch];
        let (lease, answer) = (launched.lease, launched.answer.clone());
        match act.what {
            ActSort::Priority(_) => self.acted(act, Outcome::Done, None, say),
            ActSort::Drop => {
                self.acted(act, Outcome::Done, None, say);
                if answer.outcome == Outcome::Held {
                    let words = format!("dropped by {} before it was launched", act.id);
                    say(format!("{}: done: {words}", act.launch));
                    let dropped = Answer {
                        outcome: Outcome::Done,
                        place_dir: None,
                        words: Some(words),
                        at: At::now(),
                    };
                    self.report.answer(&act.launch, lease, dropped);
                }
            }
            ActSort::Retry { .. } | ActSort::Done => {
                let outcome = match answer.outcome {
                    Outcome::Held => "held",
                    Outcome::Done => "done",
                    Outcome::Refused | Outcome::Launched => "refused",
                };
                let why = answer
                    .words
                    .map_or(String::new(), |words| format!(": {words}"));
                let words = format!("{} is {outcome}, not launched{why}", act.launch);
                self.acted(act, Outcome::Refused, Some(words), say);
            }
        }
    }

    /// Records `act` answered `outcome`, with `words`, and says so.
    fn acted(
        &mut self,
        act: &Act,
        outcome: Outcome,
        words: Option<String>,
        say: &mut dyn FnMut(String),
    ) {
        let what = sort(&act.what);
        match (&outcome, &words) {
            (Outcome::Refused, Some(words)) => say(format!(
                "{}: {what} of {}: refused: {words}",
                act.id, act.launch
            )),
            _ => say(format!("{}: {what} of {}: done", act.id, act.launch)),
        }
        let acted = Acted {
            launch: act.launch.clone(),
            lease: act.lease,
            outcome,
            words,
            at: At::now(),
        };
        self.report.acts.insert(act.id.clone(), acted);
    }

    /// Gives `offer` the answer `outcome`, at `place_dir`, with `words`.
    fn answer(
        &mut self,
        offer: &Offer,
        outcome: Outcome,
        place_dir: Option<String>,
        words: Option<String>,
    ) -> bool {
        let answer = Answer {
            outcome,
            place_dir,
            words,
            at: At::now(),
        };
        self.report.answer(&offer.id, offer.lease, answer)
    }

    /// Saves the report where it changed since the file was last written,
    /// and says whether the file holds it.
    fn save(&mut self, say: &mut dyn FnMut(String)) -> bool {
        if self.report == self.saved {
            return true;
        }
        match report::save(&self.file, &self.report) {
            Ok(()) => {
                self.saved = self.report.clone();
                true
            }
            Err(why) => {
                say(format!("flywheel: {why}"));
                self.failed = true;
                false
            }
        }
    }

    /// Sends the whole report, which renews every lease it names; one not
    /// sent is kept in the file, to be sent at the next pass.
    fn send(&mut self, plane: &mut dyn ControlPlane, say: &mut dyn FnMut(String)) {
        self.save(say);
        if let Err(Unsent(words)) = plane.report(&self.report) {
            say(format!(
                "flywheel: the report could not be sent: {words}: it is sent at the next pass"
            ));
            self.failed = true;
        }
    }
}

/// Where a launch stands, for what waits on it.
enum Standing {
    /// Not answered, or held: not yet launched.
    Unlaunched,
    /// Launched, at the place Flume named where it named one, and not
    /// landed.
    Launched(Option<String>),
    Landed,
    Refused(Option<String>),
    /// Dropped before it was launched.
    Dropped,
}

impl Standing {
    /// Where it stands, in the runner's own words.
    fn words(&self) -> String {
        match self {
            Standing::Unlaunched => "not yet launched".to_string(),
            Standing::Launched(Some(place)) => format!("launched at {place}, not landed"),
            Standing::Launched(None) => "launched, not landed".to_string(),
            Standing::Landed => "landed".to_string(),
            Standing::Refused(Some(words)) => format!("refused: {words}"),
            Standing::Refused(None) => "refused".to_string(),
            Standing::Dropped => "dropped, so its place will never land".to_string(),
        }
    }
}

/// An act's sort, as its lines name it.
fn sort(what: &ActSort) -> String {
    match what {
        ActSort::Retry { .. } => "retry".to_string(),
        ActSort::Drop => "drop".to_string(),
        ActSort::Priority(priority) => format!("priority {priority}"),
        ActSort::Done => "done".to_string(),
    }
}

/// The location of the place whose entry is `entry`: its `place`, or,
/// for the record Flume keeps of a place tidied while an end of it is
/// pending, `<location>@<at>`, the location it stood at.
fn place_of(entry: &Value) -> Option<String> {
    let named = facts::location(entry)?;
    let tidied = entry.pointer("/phase/tidied").is_some();
    let stood = named
        .rsplit_once('@')
        .filter(|(_, at)| tidied && !at.is_empty() && at.bytes().all(|byte| byte.is_ascii_digit()));
    Some(stood.map_or(named, |(location, _)| location).to_string())
}

/// Room on the host and in each of its streams, as the pass's one
/// `flume --json status` gives it, counted down as the pass launches
/// (design Decision 8; core.9). Room is Flume's: the runner reads it and
/// adds no bound of its own.
struct Room {
    host: Option<facts::Room>,
    streams: Vec<facts::StreamRoom>,
}

impl Room {
    fn read(listing: Option<&Value>) -> Room {
        Room {
            host: listing.and_then(facts::host_room),
            streams: listing.map(facts::streams).unwrap_or_default(),
        }
    }

    /// Why a launch running in `stream`, or naming none, has no room, or
    /// none where it has room. The host has room while its room is
    /// unbounded or above zero. A stream Flume lists has room while the
    /// host has, its own room is unbounded or above zero, and no place
    /// waits in its queue. A stream Flume does not list is no room
    /// question: Flume's refusal of it is the launch's answer. A launch
    /// naming no stream runs in the one Flume's host configuration gives
    /// it, which its status does not say, so it is judged by the host's
    /// room alone.
    fn lacks(&self, stream: Option<&str>) -> Option<String> {
        if !open(self.host) {
            return Some("waits for room on this host".to_string());
        }
        let listed = stream.and_then(|name| self.streams.iter().find(|s| s.name == name))?;
        if open(listed.room) && listed.queue_empty == Some(true) {
            return None;
        }
        Some(format!("waits for room in the stream {}", listed.name))
    }

    /// Counts one launch against the host's room and `stream`'s, where
    /// each is a number.
    fn take(&mut self, stream: Option<&str>) {
        count_down(&mut self.host);
        if let Some(listed) =
            stream.and_then(|name| self.streams.iter_mut().find(|s| s.name == name))
        {
            count_down(&mut listed.room);
        }
    }
}

/// Whether `room`, as Flume gives it, is unbounded or above zero. A room
/// Flume's status does not give as either is none.
fn open(room: Option<facts::Room>) -> bool {
    match room {
        Some(facts::Room::Unbounded) => true,
        Some(facts::Room::Count(count)) => count > 0,
        None => false,
    }
}

fn count_down(room: &mut Option<facts::Room>) {
    if let Some(facts::Room::Count(count)) = room {
        *count -= 1;
    }
}

/// The runner's mark on the place it launches for `id`, `flywheel:<id>`
/// (design Decision 11).
pub fn mark(id: &Id) -> String {
    format!("flywheel:{id}")
}

/// What `flume launch` is given for `offer` in the clone at `container`:
/// every field as the launch gives it, its priority `priority`, and the
/// runner's mark. It names no stream, priority or intent of the runner's
/// own (core.9), and no kinds file (core.5).
fn launch_args(offer: &Offer, container: &Path, priority: Option<i64>) -> LaunchArgs {
    let launch = &offer.launch;
    LaunchArgs {
        container_dir: container.to_path_buf(),
        kind: launch.kind.clone(),
        name: launch.name.clone(),
        base: launch.base.clone(),
        stream: launch.stream.clone(),
        priority,
        intent: launch.intent.clone(),
        recipients: launch.recipients.clone(),
        holds: launch
            .holds
            .iter()
            .map(|hold| (hold.stage.clone(), hold.words.clone()))
            .collect(),
        hold_submission: launch.hold_submission,
        brief: launch.brief.clone(),
        agent_settings: launch
            .agent_settings
            .iter()
            .map(|setting| (setting.field.clone(), setting.value.clone()))
            .collect(),
        mark: mark(&offer.id),
    }
}

/// The earliest expiry of a lease `report` holds.
fn expires(report: &Report) -> Option<At> {
    let launches = report.launches.values().map(|launched| launched.lease);
    let commands = report.commands.values().map(|commanded| commanded.lease);
    launches
        .chain(commands)
        .filter_map(|lease| lease.expires)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn config(repositories: &[&str]) -> Config {
        Config {
            repository_root: PathBuf::from("/code"),
            fetch_interval: std::time::Duration::from_secs(120),
            repositories: repositories
                .iter()
                .map(|address| Address::parse(address).unwrap())
                .collect(),
        }
    }

    #[test]
    fn a_repository_is_kept_as_the_configuration_and_the_clones_have_it() {
        let flume = Address::parse("agentplot/flume").unwrap();
        let clones = Configured::new(
            &config(&["afterthought/crew", "agentplot/flume"]),
            vec![(flume, PathBuf::from("/d/repositories/agentplot/flume"))],
        );
        assert_eq!(
            clones.keeping("Agentplot/Flume"),
            Keeping::Ready(Path::new("/d/repositories/agentplot/flume").into())
        );
        assert_eq!(clones.keeping("afterthought/crew"), Keeping::NotYet);
        assert_eq!(clones.keeping("agentplot/sluice"), Keeping::NotNamed);
        assert_eq!(clones.keeping("not an address"), Keeping::NotNamed);
        assert_eq!(clones.made(), ["agentplot/flume"]);
    }

    #[test]
    fn a_clone_the_configuration_drops_is_not_kept() {
        let flume = Address::parse("agentplot/flume").unwrap();
        let clones = Configured::new(
            &config(&[]),
            vec![(flume, PathBuf::from("/d/repositories/agentplot/flume"))],
        );
        assert_eq!(clones.keeping("agentplot/flume"), Keeping::NotNamed);
        assert!(clones.made().is_empty());
    }
}
