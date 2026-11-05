//! The control pass (design Decision 5). It takes the runner's report,
//! reads the host once from Flume's status, hands the hello to the pull,
//! answers each want in the control plane's order, opens each terminal
//! pulled, and sends the whole report. The runner takes only what it was
//! offered, launches only through Flume's command line, and carries each
//! answer as Flume gave it (core.1, core.9, core.10). It reaches its control plane only through
//! `ControlPlane` (core.2), and acts on no place it did not launch. Work
//! under a lease that has lapsed by the pass's clock waits for a live one
//! (core.24). Each act is said on standard error, one line each, prefixed
//! with its id, and a pass that changes nothing says nothing.

use crate::config::{Address, Config};
use crate::flume::{self, Flume, LaunchArgs, Reply, Unrun};
use crate::git::Git;
use crate::hello::{self, Tools};
use crate::run::Notices;
use crate::sluice::{self, Sluice};
use crate::terminal::Relays;
use crate::{clone, facts, inbox, report};
use boomdrive_control::{
    Act, ActSort, Acted, Answer, At, Carrying, Command, CommandSort, Commanded, ControlPlane, Id,
    Inbox, InboxCarried, InboxFile, Launched, Lease, Offer, Outcome, Report, Reported, Unread,
    Unsent, Want,
};
use serde_json::Value;
use std::cell::Cell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

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
    pub sluice: &'a dyn Sluice,
    /// git, for an inbox read from its clone and a branch read from
    /// origin.
    pub git: &'a Git,
    pub clones: &'a dyn Clones,
    pub tools: &'a dyn Tools,
    /// Told of each landing the pass reads, so its repository is kept at
    /// once.
    pub notices: &'a Notices,
    /// The runner's control directory, `<data>/control`: its report file
    /// and the folder its briefs pass through.
    pub control_dir: &'a Path,
    /// The terminals relaying, kept across passes: the pass opens each
    /// terminal pulled, and closes those on a launch it lets go.
    pub relays: &'a Relays,
    /// Whether the last pull was read, kept across passes.
    pub reach: &'a Reach,
}

/// Whether the control plane's last pull was read, kept across control
/// passes, so its reach is said once each way: when a pull first cannot
/// be read, with the pull's words, and when one is next read, and at no
/// pass between (design Decision 13; core.23). The first pull is said
/// unread as one after a pull that was read.
#[derive(Debug, Default)]
pub struct Reach {
    unread: Cell<bool>,
}

impl Reach {
    /// Takes a pull that could not be read, with the pull's `words`.
    fn unread(&self, words: &str, say: &mut dyn FnMut(String)) {
        if !self.unread.replace(true) {
            say(format!(
                "boomdrive: the control plane could not be read: {words}"
            ));
        }
    }

    /// Takes a pull that was read.
    fn read(&self, say: &mut dyn FnMut(String)) {
        if self.unread.replace(false) {
            say("boomdrive: the control plane is read again".to_string());
        }
    }
}

/// What a control pass leaves for the loop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Passed {
    /// The earliest expiry of a lease the report holds not yet lapsed at
    /// the pass's end, where one has an expiry.
    pub expires: Option<At>,
    /// Whether the pass pulled and the control plane took its report,
    /// which is what `boomdrive run --once` asks of a control pass. A tool
    /// that could not run is named, and does not count against it.
    pub reported: bool,
}

impl Passed {
    /// When the control pass after this one, which began at `began` and
    /// ended at `ended`, is due, each `poll_interval`, by the clock as it
    /// reads now: [`next_due`], with the time left of the earliest lease
    /// not yet lapsed.
    pub fn next_due(&self, began: Instant, ended: Instant, poll_interval: Duration) -> Instant {
        let left = self.expires.and_then(|expires| left(expires, At::now()));
        next_due(began, ended, poll_interval, left, self.reported)
    }
}

impl Carrier<'_> {
    /// One control pass against `plane`, each line said to `say` as it is
    /// said.
    pub fn pass(&self, plane: &mut dyn ControlPlane, say: &mut dyn FnMut(String)) -> Passed {
        // The pass's clock, read once, by which the work under each lease
        // is judged (core.24).
        let now = At::now();
        let file = report::file(self.control_dir);
        let kept = match report::read(&file) {
            Ok(kept) => kept,
            Err(why) => {
                say(format!("boomdrive: {why}: nothing carried this pass"));
                return Passed {
                    expires: None,
                    reported: false,
                };
            }
        };
        let saved = kept.clone().unwrap_or_default();
        let status = self.flume.status();
        let listing = match &status {
            Ok(Reply::Given(text)) => serde_json::from_str::<Value>(text).ok(),
            Ok(Reply::Refused(words)) | Err(Unrun(words)) => {
                say(format!(
                    "boomdrive: Flume's status could not be read: {words}"
                ));
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
                self.reach.unread(&words, say);
                return Passed {
                    expires: expires(&saved, At::now()),
                    reported: false,
                };
            }
        };
        self.reach.read(say);
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
            now,
        };
        for want in &pulled.wants {
            work.want(want, say);
            if !work.save(say) {
                say("boomdrive: nothing more is carried out this pass".to_string());
                break;
            }
        }
        work.carry_statuses(say);
        work.carry_entries(say);
        for terminal in pulled.terminals {
            self.relays.open(terminal, &work.report);
        }
        let reported = work.send(plane, say);
        Passed {
            expires: expires(&work.report, At::now()),
            reported,
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
    /// The pass's clock, read once at its start.
    now: At,
}

impl Work<'_> {
    fn want(&mut self, want: &Want, say: &mut dyn FnMut(String)) {
        match want {
            Want::Offer(offer) => self.offer(offer, say),
            Want::Act(act) => self.act(act, say),
            Want::Inbox(inbox) => self.inbox(inbox, say),
            Want::Command(command) => self.command(command, say),
        }
    }

    /// Takes, leaves or refuses `offer` (design Decision 7). An offer
    /// whose lease has lapsed is left unanswered, and one already answered
    /// keeps its answer, until a pull gives it under a live lease (design
    /// Decision 13; core.24).
    fn offer(&mut self, offer: &Offer, say: &mut dyn FnMut(String)) {
        let id = &offer.id;
        if let Some(launched) = self.report.launches.get_mut(id) {
            // The lease is the one the control plane gives now.
            launched.lease = offer.lease;
            if launched.answer.outcome != Outcome::Held {
                return;
            }
        }
        if lapsed(offer.lease, self.now) {
            return;
        }
        // Without Flume's listing no mark can be read, so nothing is
        // launched.
        let Some(listing) = &self.listing else {
            return;
        };
        let found = marks(id).into_iter().find_map(|mark| {
            let place = facts::marked(listing, &mark).and_then(place_of)?;
            Some((mark, place))
        });
        if let Some((mark, place)) = found {
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
        let entry = listed.or_else(|| carried_entry(launched, place));
        reads_landed(entry.as_ref())
    }

    /// The priority `offer` is launched with: the newest an act on it set
    /// while it was not yet launched, in the control plane's order, else
    /// its own (crew's teams-cross Decision 6).
    fn priority(&self, offer: &Offer) -> Option<i64> {
        let mut set = self.wants.iter().filter_map(|want| match want {
            Want::Act(act) if act.launch == offer.id => match act.what {
                ActSort::Priority(priority) if self.done(&act.id) => Some(priority),
                _ => None,
            },
            _ => None,
        });
        set.next_back().or(offer.launch.priority)
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
    /// act the report holds is left as it stands, and one whose launch's
    /// lease in the report has lapsed is left untaken (design Decision 13;
    /// core.24).
    fn act(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        if self.report.acts.contains_key(&act.id) {
            return;
        }
        let Some(launched) = self.report.launches.get(&act.launch) else {
            // A launch the pull offers and the runner has not yet taken is
            // the control plane's to give: its act waits with it.
            if !self.offered(&act.launch) {
                let words = format!("{} is no launch of this runner's", act.launch);
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
            return;
        };
        if lapsed(launched.lease, self.now) {
            return;
        }
        if launched.answer.outcome != Outcome::Launched {
            self.act_unlaunched(act, say);
            return;
        }
        match launched.answer.place_dir.clone() {
            Some(place) => self.act_on(act, &place, say),
            None => {
                let words = format!("{} was launched at no place Flume named", act.launch);
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
        }
    }

    /// Carries out `act` as Flume's own command on the place at `place`,
    /// never forced, its answer Flume's. Where Flume could not be run,
    /// nothing is kept, and the next pass carries it out.
    fn act_on(&mut self, act: &Act, place: &str, say: &mut dyn FnMut(String)) {
        match self.carrier.flume.act(place, &flume_act(&act.what)) {
            Ok(Reply::Given(_)) => self.acted(act, Outcome::Done, None, Some(place), say),
            Ok(Reply::Refused(words)) => {
                self.acted(act, Outcome::Refused, Some(words), Some(place), say)
            }
            Err(Unrun(words)) => {
                let what = sort(&act.what);
                say(format!(
                    "{}: {what} of {} at {place}: not carried out: {words}",
                    act.id, act.launch
                ));
            }
        }
    }

    /// Answers `act` on a launch the runner holds not yet launched, from
    /// the report, running no command. A priority is done, and the launch
    /// carries it when made; a drop is done, and the launch is never made;
    /// a retry or a done is refused, naming where the launch stands.
    fn act_unlaunched(&mut self, act: &Act, say: &mut dyn FnMut(String)) {
        let launched = &self.report.launches[&act.launch];
        let (lease, answer) = (launched.lease, launched.answer.clone());
        match act.what {
            ActSort::Priority(_) => self.acted(act, Outcome::Done, None, None, say),
            ActSort::Drop => {
                self.acted(act, Outcome::Done, None, None, say);
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
                self.acted(act, Outcome::Refused, Some(words), None, say);
            }
        }
    }

    /// Records `act` answered `outcome`, with `words`, and says so: as
    /// Flume answered it on the place at `place`, where it ran there, else
    /// as the runner answered it from the report.
    fn acted(
        &mut self,
        act: &Act,
        outcome: Outcome,
        words: Option<String>,
        place: Option<&str>,
        say: &mut dyn FnMut(String),
    ) {
        let what = sort(&act.what);
        let at = place.map_or(String::new(), |place| format!(" at {place}"));
        let by = if place.is_some() { " by Flume" } else { "" };
        match (&outcome, &words) {
            (Outcome::Refused, Some(words)) => say(format!(
                "{}: {what} of {}{at}: refused{by}: {words}",
                act.id, act.launch
            )),
            _ => say(format!("{}: {what} of {}{at}: done", act.id, act.launch)),
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

    /// Carries out or refuses `command` (design Decision 15). A command
    /// the report holds is left as it stands, under the lease the control
    /// plane gives now, and its tool is not run again (core.10). One whose
    /// lease has lapsed is left untaken (design Decision 13; core.24). A
    /// sort this binary does not carry is refused, naming its tool.
    fn command(&mut self, command: &Command, say: &mut dyn FnMut(String)) {
        if let Some(commanded) = self.report.commands.get_mut(&command.id) {
            commanded.lease = command.lease;
            return;
        }
        if lapsed(command.lease, self.now) {
            return;
        }
        match &command.what {
            CommandSort::Branch {
                repository,
                branch,
                from,
            } => self.branch(command, repository, branch, from, say),
            CommandSort::Other { tool, .. } => {
                let words = format!("this binary does not carry {tool}'s commands");
                say(format!("{}: refused: {words}", command.id));
                self.commanded(command, Outcome::Refused, None, Some(words), None);
            }
        }
    }

    /// Makes `branch` from `from` in `repository` by sluice, in its clone,
    /// whatever the room (core.9), and carries sluice's answer as given,
    /// `done` where it exits 0 and `refused` otherwise, with the commit
    /// origin's `branch` then stands at, read from origin and never from
    /// sluice's words. A command for a repository the configuration does
    /// not keep is refused, naming it; one whose clone is not yet made is
    /// left untaken, for a host keeping the repository to take (core.5).
    /// Where sluice could not be run, nothing is kept, and the next pass
    /// carries it out.
    fn branch(
        &mut self,
        command: &Command,
        repository: &str,
        branch: &str,
        from: &str,
        say: &mut dyn FnMut(String),
    ) {
        let id = &command.id;
        let container = match self.carrier.clones.keeping(repository) {
            Keeping::Ready(container) => container,
            Keeping::NotYet => return,
            Keeping::NotNamed => {
                let words = format!("this host's configuration does not keep {repository}");
                say(format!("{id}: refused: {words}"));
                self.commanded(command, Outcome::Refused, None, Some(words), None);
                return;
            }
        };
        let what = format!("{id}: branch {branch} from {from} in {repository}");
        let printed = match self.carrier.sluice.branch(&container, branch, from) {
            Ok(printed) => printed,
            Err(sluice::Unrun(words)) => {
                say(format!("{what}: not carried out: {words}"));
                return;
            }
        };
        let bare = clone::bare(&container);
        let (origin_commit, unread) = match clone::origin_commit(self.carrier.git, &bare, branch) {
            Ok(commit) => (commit, None),
            Err(failure) => {
                let words = format!("origin's {branch} could not be read: {failure}");
                (None, Some(words))
            }
        };
        let on_origin = match (&origin_commit, &unread) {
            (Some(commit), _) => format!("origin holds {branch} at {commit}"),
            (None, Some(unread)) => unread.clone(),
            (None, None) => format!("origin holds no {branch}"),
        };
        let outcome = match &printed.refused {
            Some(words) => {
                say(format!("{what}: refused by sluice: {words}; {on_origin}"));
                Outcome::Refused
            }
            None => {
                say(format!("{what}: done; {on_origin}"));
                Outcome::Done
            }
        };
        let words = match (printed.refused, unread) {
            (Some(refused), Some(unread)) => Some(format!("{refused}; {unread}")),
            (refused, unread) => refused.or(unread),
        };
        self.commanded(command, outcome, Some(printed.text), words, origin_commit);
    }

    /// Records `command` answered `outcome`, with its tool's `text`, where
    /// one ran, `words` and `origin_commit`.
    fn commanded(
        &mut self,
        command: &Command,
        outcome: Outcome,
        text: Option<String>,
        words: Option<String>,
        origin_commit: Option<String>,
    ) {
        let commanded = Commanded {
            lease: command.lease,
            outcome,
            text,
            words,
            origin_commit,
            at: At::now(),
        };
        self.report.commands.insert(command.id.clone(), commanded);
    }

    /// Carries Flume's status of each launch's place (design Decision 12;
    /// core.11): `flume --json status --repository <container> <place>`'s
    /// whole answer, as the one line Flume printed. The pass's listing
    /// decides when it is read again: only where the place's entry there
    /// differs, as JSON, from its entry in the answer carried. A place the
    /// listing no longer holds is asked for once a pass, and where Flume
    /// refuses, the last status carried stands, not listed. The first
    /// status carried that reads the place landed tells the keeper, so the
    /// repository is fetched at once. Only the places the report holds are
    /// read, and only for launches the pull gives, whose repositories it
    /// names.
    fn carry_statuses(&mut self, say: &mut dyn FnMut(String)) {
        let Some(listing) = self.listing.take() else {
            return;
        };
        for offer in offers(self.wants) {
            let id = &offer.id;
            let Some(launched) = self.report.launches.get(id) else {
                continue;
            };
            let (Outcome::Launched, Some(place)) =
                (launched.answer.outcome, launched.answer.place_dir.clone())
            else {
                continue;
            };
            let Keeping::Ready(container) = self.carrier.clones.keeping(&offer.launch.repository)
            else {
                continue;
            };
            let was = carried_entry(launched, &place);
            let listed = facts::entry(&listing, &place);
            if listed.is_some() && listed == was.as_ref() {
                self.relist(id, Carrying::Status, true);
                continue;
            }
            match self.carrier.flume.status_of(&container, &place) {
                Ok(Reply::Given(text)) => {
                    let line = text.strip_suffix('\n').unwrap_or(&text);
                    self.report
                        .carry(id, Carrying::Status, line, listed.is_some(), At::now());
                }
                Ok(Reply::Refused(_)) => {
                    self.relist(id, Carrying::Status, listed.is_some());
                }
                Err(Unrun(words)) => {
                    say(format!(
                        "{id}: Flume's status of {place} could not be read: {words}"
                    ));
                }
            }
            let now = carried_entry(&self.report.launches[id], &place);
            if !reads_landed(was.as_ref()) && reads_landed(now.as_ref()) {
                if let Ok(address) = Address::parse(&offer.launch.repository) {
                    self.carrier.notices.landed(address);
                }
            }
        }
        self.listing = Some(listing);
    }

    /// Carries sluice's entry of each launch's submission (design
    /// Decision 13; core.11): of `sluice status <id> --json`, run in the
    /// launch's clone, the fields the cloud's copy of an entry holds, each
    /// as sluice printed it, and no log. The entry is the one the status
    /// carried for the launch names; a launch whose status names none runs
    /// no sluice. An entry is read again at each pass until its state is
    /// final, and carried anew only when what is carried of it changes.
    /// Where sluice refuses, the entry last carried stands, not listed.
    /// Only the launches the pull gives are read, whose repositories it
    /// names.
    fn carry_entries(&mut self, say: &mut dyn FnMut(String)) {
        for offer in offers(self.wants) {
            let id = &offer.id;
            let Some(launched) = self.report.launches.get(id) else {
                continue;
            };
            let (Outcome::Launched, Some(place)) = (
                launched.answer.outcome,
                launched.answer.place_dir.as_deref(),
            ) else {
                continue;
            };
            let entry = carried_entry(launched, place);
            let Some(submission) = entry.as_ref().and_then(facts::submission_id) else {
                continue;
            };
            let held = launched.entry.as_ref().map(|carried| carried.text.as_str());
            if held.is_some_and(|text| {
                sluice::id_of(text).as_deref() == Some(submission) && sluice::is_final(text)
            }) {
                continue;
            }
            let submission = submission.to_string();
            let Keeping::Ready(container) = self.carrier.clones.keeping(&offer.launch.repository)
            else {
                continue;
            };
            let refused = match self.carrier.sluice.entry(&container, &submission) {
                Ok(sluice::Reply::Given(answer)) => match sluice::carried(&answer) {
                    Some(text) => {
                        self.report
                            .carry(id, Carrying::Entry, &text, true, At::now());
                        continue;
                    }
                    None => "sluice's answer is no submission".to_string(),
                },
                Ok(sluice::Reply::Refused(words)) => words,
                Err(sluice::Unrun(words)) => {
                    say(format!(
                        "{id}: sluice's entry {submission} could not be read: {words}"
                    ));
                    continue;
                }
            };
            if self.relist(id, Carrying::Entry, false) {
                say(format!(
                    "{id}: sluice's entry {submission} is refused: {refused}"
                ));
            }
        }
    }

    /// Carries the inbox `inbox` names (design Decision 14; core.5,
    /// core.11). An inbox in a repository the configuration does not keep
    /// is answered with words naming it, and nothing of it read; one whose
    /// clone is not yet made waits; one on a branch origin does not hold
    /// is answered with words naming the branch. Otherwise its branch's
    /// tip in the clone is read, and, where it moved since the tip last
    /// carried, its files: where weir takes them they are carried, each
    /// brief by its digest, and where weir refuses them the tip is carried
    /// with weir's words, and the files carried before stand.
    fn inbox(&mut self, inbox: &Inbox, say: &mut dyn FnMut(String)) {
        let (id, repository, branch) = (&inbox.id, &inbox.repository, &inbox.branch);
        let container = match self.carrier.clones.keeping(repository) {
            Keeping::Ready(container) => container,
            Keeping::NotYet => return,
            Keeping::NotNamed => {
                let words = format!("this host's configuration does not keep {repository}");
                if self.carry_inbox(id, None, BTreeMap::new(), Some(words.clone())) {
                    say(format!("{id}: not carried: {words}"));
                }
                return;
            }
        };
        let held = self.report.inboxes.get(id);
        let tip = held.and_then(|carried| carried.tip.clone());
        let files = held
            .map(|carried| carried.files.clone())
            .unwrap_or_default();
        let bare = clone::bare(&container);
        match inbox::read(self.carrier.git, &bare, branch, tip.as_deref()) {
            Ok(inbox::Read::Unmoved) => {}
            Ok(inbox::Read::NoBranch) => {
                let words = format!("origin holds no branch {branch} of {repository}");
                if self.carry_inbox(id, None, BTreeMap::new(), Some(words.clone())) {
                    say(format!("{id}: not carried: {words}"));
                }
            }
            Ok(inbox::Read::Taken { tip, files }) => {
                let line = format!("{id}: the inbox on {branch} of {repository} carried at {tip}");
                if self.carry_inbox(id, Some(tip), files, None) {
                    say(line);
                }
            }
            Ok(inbox::Read::Refused { tip, words }) => {
                let line = format!(
                    "{id}: the inbox on {branch} of {repository} at {tip} is refused by weir: {words}"
                );
                if self.carry_inbox(id, Some(tip), files, Some(words)) {
                    say(line);
                }
            }
            Err(words) => {
                say(format!(
                    "{id}: the inbox on {branch} of {repository} could not be read: {words}"
                ));
            }
        }
    }

    /// Carries inbox `id` as read at `tip`, holding `files`, with `words`,
    /// its `at` moving only where one of them changes, and says whether
    /// the report changed.
    fn carry_inbox(
        &mut self,
        id: &Id,
        tip: Option<String>,
        files: BTreeMap<String, InboxFile>,
        words: Option<String>,
    ) -> bool {
        let held = self.report.inboxes.get(id);
        if held.is_some_and(|held| held.tip == tip && held.files == files && held.words == words) {
            return false;
        }
        let carried = InboxCarried {
            tip,
            files,
            words,
            at: At::now(),
        };
        self.report.inboxes.insert(id.clone(), carried);
        true
    }

    /// Marks the status or entry carried for launch `id` as its tool's
    /// last listing holds it or not, its text and time as they stand, and
    /// says whether the report changed.
    fn relist(&mut self, id: &Id, what: Carrying, listed: bool) -> bool {
        let launched = self.report.launches.get(id);
        let carried = launched.and_then(|launched| match what {
            Carrying::Status => launched.status.as_ref(),
            Carrying::Entry => launched.entry.as_ref(),
        });
        match carried.map(|carried| carried.text.clone()) {
            Some(text) => self.report.carry(id, what, &text, listed, At::now()),
            None => false,
        }
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
                say(format!("boomdrive: {why}"));
                false
            }
        }
    }

    /// Sends the whole report, which renews every lease it names, and
    /// says whether the control plane took it; one not sent is kept in the
    /// file, to be sent at the next pass.
    fn send(&mut self, plane: &mut dyn ControlPlane, say: &mut dyn FnMut(String)) -> bool {
        self.save(say);
        match plane.report(&self.report) {
            Ok(reported) => {
                self.reported(&reported, say);
                self.save(say);
                true
            }
            Err(Unsent(words)) => {
                say(format!(
                    "boomdrive: the report could not be sent: {words}: it is sent at the next pass"
                ));
                false
            }
        }
    }

    /// Takes the control plane's answer to a report (design Decision 11).
    /// What it carries no more leaves the report. What it refused, its
    /// lease no longer the runner's, leaves the report too, so the runner
    /// neither acts on it nor reports it again; its place is left on the
    /// host as Flume holds it, and named, and nothing of it is removed
    /// (core.10).
    fn reported(&mut self, reported: &Reported, say: &mut dyn FnMut(String)) {
        for id in &reported.let_go {
            if self.forget(id) {
                say(format!("{id}: let go by the control plane"));
            }
        }
        for (id, words) in &reported.refused {
            let place = self
                .report
                .launches
                .get(id)
                .and_then(|launched| launched.answer.place_dir.clone());
            if !self.forget(id) {
                continue;
            }
            match place {
                Some(place) => say(format!(
                    "{id}: let go, its lease no longer this runner's: {words}: \
                     its place {place} is left on this host as Flume holds it"
                )),
                None => say(format!(
                    "{id}: let go, its lease no longer this runner's: {words}"
                )),
            }
        }
    }

    /// Drops `id` from the report, closing every terminal on it, and says
    /// whether the report held it.
    fn forget(&mut self, id: &Id) -> bool {
        self.carrier.relays.let_go(id);
        let report = &mut self.report;
        let launch = report.launches.remove(id).is_some();
        let act = report.acts.remove(id).is_some();
        let command = report.commands.remove(id).is_some();
        let inbox = report.inboxes.remove(id).is_some();
        launch || act || command || inbox
    }
}

/// The offers among `wants`, in the control plane's order.
fn offers(wants: &[Want]) -> impl Iterator<Item = &Offer> {
    wants.iter().filter_map(|want| match want {
        Want::Offer(offer) => Some(offer),
        _ => None,
    })
}

/// The entry of the place at `place` in the status `launched` carries.
fn carried_entry(launched: &Launched, place: &str) -> Option<Value> {
    let text = &launched.status.as_ref()?.text;
    let answer = serde_json::from_str::<Value>(text).ok()?;
    facts::entry(&answer, place).cloned()
}

/// Whether a place's entry, where there is one, reads landed.
fn reads_landed(entry: Option<&Value>) -> bool {
    matches!(entry.and_then(facts::over), Some(facts::Over::Landed(_)))
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

/// `what` as Flume's own command of its name.
fn flume_act(what: &ActSort) -> flume::Act {
    match what {
        ActSort::Retry {
            stage,
            words,
            submission,
        } => flume::Act::Retry {
            stage: stage.clone(),
            words: words.clone(),
            submission: *submission,
        },
        ActSort::Drop => flume::Act::Drop,
        ActSort::Priority(priority) => flume::Act::Priority(*priority),
        ActSort::Done => flume::Act::Done,
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

/// The runner's mark on the place it launches for `id`, `boomdrive:<id>`
/// (design Decision 11).
pub fn mark(id: &Id) -> String {
    format!("boomdrive:{id}")
}

/// The marks a place launched for `id` may carry: the runner's, then the
/// one a runner put on it before the rename, `flywheel:<id>`, so a launch
/// made then is still found and never made again (core.10, core.36;
/// record 0016).
pub fn marks(id: &Id) -> [String; 2] {
    [mark(id), format!("{}:{id}", crate::BEFORE_RENAME)]
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

/// When the control pass after one that began at `began` and ended at
/// `ended` is due (design Decisions 5 and 13; core.22): `poll_interval`
/// after `began`, or, after a pass that `reported`, sooner, by the
/// midpoint between `ended` and the earliest expiry of a lease the runner
/// holds not yet lapsed, `left` after `ended`, so the lease is renewed
/// before it lapses; and never before `ended`. A lapsed lease, or a pass
/// that did not report, hastens none: while the control plane cannot be
/// reached, a pass sooner renews nothing.
pub fn next_due(
    began: Instant,
    ended: Instant,
    poll_interval: Duration,
    left: Option<Duration>,
    reported: bool,
) -> Instant {
    let polled = began + poll_interval;
    let due = match left {
        Some(left) if reported => polled.min(ended + left / 2),
        _ => polled,
    };
    due.max(ended)
}

/// How long after `now` a lease expiring at `expires` has left, or none
/// where it has lapsed by `now`.
pub fn left(expires: At, now: At) -> Option<Duration> {
    Duration::try_from(expires.time() - now.time())
        .ok()
        .filter(|left| !left.is_zero())
}

/// Whether `lease` has lapsed by `now`: it has an expiry, and `now` is
/// not before it. `At` drops a second's fraction, so an expiry `now`
/// equals has passed.
fn lapsed(lease: Lease, now: At) -> bool {
    lease.expires.is_some_and(|expires| expires <= now)
}

/// The earliest expiry of a lease `report` holds not yet lapsed at `now`.
fn expires(report: &Report, now: At) -> Option<At> {
    let launches = report.launches.values().map(|launched| launched.lease);
    let commands = report.commands.values().map(|commanded| commanded.lease);
    launches
        .chain(commands)
        .filter(|lease| !lapsed(*lease, now))
        .filter_map(|lease| lease.expires)
        .min()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn at(text: &str) -> At {
        At::parse(text).unwrap()
    }

    /// The work-into-flume spec's "Renewed before it lapses": a lease
    /// expiring 40 seconds after the end of the pass that took it, with
    /// `poll_interval` one minute, has the next pass due no later than 20
    /// seconds after that pass ended.
    #[test]
    fn renewed_before_it_lapses() {
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        let due = next_due(began, ended, minute, Some(Duration::from_secs(40)), true);
        assert_eq!(due, ended + Duration::from_secs(20));
        assert_eq!(next_due(began, ended, minute, None, true), began + minute);
        let far = Some(Duration::from_secs(600));
        assert_eq!(next_due(began, ended, minute, far, true), began + minute);
        let slow = began + Duration::from_secs(90);
        assert_eq!(next_due(began, slow, minute, None, true), slow);
    }

    /// The running spec's "A lapsed lease": with `poll_interval` one
    /// minute, a lease that lapsed before a pass that began at 10:00:00,
    /// and the control plane unreachable, the next pass begins at
    /// 10:01:00, and none between. A lapsed lease has no time left, so it
    /// hastens no pass, whether the pass reported or not.
    #[test]
    fn a_lapsed_lease() {
        let ten = at("2026-11-01T10:00:00Z");
        let lapsed = left(at("2026-11-01T09:59:00Z"), ten);
        assert_eq!(lapsed, None);
        assert_eq!(left(ten, ten), None, "an expiry of now has passed");
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        assert_eq!(
            next_due(began, ended, minute, lapsed, false),
            began + minute
        );
        assert_eq!(next_due(began, ended, minute, lapsed, true), began + minute);
    }

    /// The running spec's "A pass that did not report": with
    /// `poll_interval` one minute, a lease expiring 40 seconds after a
    /// pass that began at 10:00:00, and that pass's report not sent, the
    /// next pass begins at 10:01:00.
    #[test]
    fn a_pass_that_did_not_report() {
        let ten = at("2026-11-01T10:00:00Z");
        let left = left(at("2026-11-01T10:00:40Z"), ten);
        assert_eq!(left, Some(Duration::from_secs(40)));
        let began = Instant::now();
        let ended = began + Duration::from_secs(5);
        let minute = Duration::from_secs(60);
        assert_eq!(next_due(began, ended, minute, left, false), began + minute);
    }

    /// A lease has lapsed once the clock reads its expiry, and a lease
    /// with no expiry never lapses; only leases not yet lapsed give the
    /// earliest expiry.
    #[test]
    fn a_lease_lapsed_by_the_clock() {
        let ten = at("2026-11-01T10:00:00Z");
        let lease = |expires: Option<&str>| Lease {
            number: 1,
            expires: expires.map(at),
        };
        assert!(lapsed(lease(Some("2026-11-01T09:59:00Z")), ten));
        assert!(lapsed(lease(Some("2026-11-01T10:00:00Z")), ten));
        assert!(!lapsed(lease(Some("2026-11-01T10:00:01Z")), ten));
        assert!(!lapsed(lease(None), ten));
        let mut report = Report::default();
        let answer = Answer {
            outcome: Outcome::Launched,
            place_dir: None,
            words: None,
            at: ten,
        };
        for (id, expires) in [
            ("a", Some("2026-11-01T09:59:00Z")),
            ("b", Some("2026-11-01T10:05:00Z")),
            ("c", None),
        ] {
            report.answer(&Id::new(id), lease(expires), answer.clone());
        }
        assert_eq!(expires(&report, ten), Some(at("2026-11-01T10:05:00Z")));
    }

    fn config(repositories: &[&str]) -> Config {
        Config {
            repository_root: PathBuf::from("/code"),
            fetch_interval: std::time::Duration::from_secs(120),
            repositories: repositories
                .iter()
                .map(|address| Address::parse(address).unwrap())
                .collect(),
            control_plane: None,
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
