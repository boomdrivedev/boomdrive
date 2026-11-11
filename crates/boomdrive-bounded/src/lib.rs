//! A process the runner waits on for a limited time, in a process group
//! of its own: each pipe read on a thread of its own as it goes, and at
//! its limit or its halt, whichever comes first, the group asked to end,
//! given [`GRACE`], then killed, so nothing it started outlives the wait
//! (record 0042 as amended on 2026-11-11; core.58). A [`Halt`] is the
//! runner's stop: one value the stop sets once and every call holding it
//! reads, and a call begun once it is set runs no process.
//!
//! This is agentplot/sluice record 0109's design, the family's one design
//! for a git call's limit, and mirrors sluice's `crates/sluice/src/bounded.rs`
//! at `e6809f9`, its group ended as sluice's `runs::end_group` ends one.
//! Every try of the runner's git call to a remote runs so: a clone's fetch
//! and a state repository's listing.

use std::io::{self, Read};
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// How long one try of a call to a remote may take before it is ended:
/// agentplot/sluice record 0109's `retry::TRY_FOR`, one value for every
/// call of the family's, set by no configuration (record 0042).
pub const TRY_FOR: Duration = Duration::from_secs(30);

/// How long a group asked to end at its limit or its halt has before it
/// is killed:
/// agentplot/sluice record 0109's `bounded::GRACE`, in which git, asked,
/// removes the lock files it holds in the repository.
pub const GRACE: Duration = Duration::from_secs(2);

/// How long the output of a group ended at its limit or its halt is
/// waited for once the group is gone: a process outside it may hold a pipe open.
const DRAINED: Duration = Duration::from_millis(200);

/// How long a group killed is waited for to be gone.
const KILLED: Duration = Duration::from_secs(5);

/// The runner's stop as every call it runs reads it: a handle to one
/// shared flag, which its clones share, set once by the stop and never
/// cleared. `Halt::default()` gives one never set (record 0042 as amended
/// on 2026-11-11, its design Decision 1).
#[derive(Clone, Debug, Default)]
pub struct Halt(Arc<AtomicBool>);

impl Halt {
    /// Set the halt, for this handle and every clone of it; setting it
    /// again changes nothing.
    pub fn halt(&self) {
        self.0.store(true, Ordering::SeqCst);
    }

    /// Whether the halt is set.
    pub fn halted(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
}

/// What ended a call before it ended by itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    /// Its limit came first.
    AtLimit,
    /// Its halt came first, or was set before it began.
    Halted,
}

/// How a process ended, and what it printed until then.
#[derive(Debug)]
pub struct Ended {
    /// Its exit, by itself or by the signal that ended its group; none
    /// only where no process ran, its halt set before it began.
    pub status: Option<ExitStatus>,
    /// Whichever of its limit and its halt came first, and its group was
    /// ended; none where it ended by itself.
    pub stopped: Option<Stopped>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// `command` run in a process group of its own, its standard output and
/// error read, to `limit` and `halt`. At whichever comes first its group
/// is asked to end, given [`GRACE`], then killed; a halt that comes while
/// the group is being ended at its limit changes nothing. Where `halt` is
/// set before it begins, no process runs, and it is said halted with no
/// exit and nothing printed. Its standard input is the caller's to set.
pub fn output(command: &mut Command, limit: Duration, halt: &Halt) -> io::Result<Ended> {
    if halt.halted() {
        return Ok(Ended {
            status: None,
            stopped: Some(Stopped::Halted),
            stdout: Vec::new(),
            stderr: Vec::new(),
        });
    }
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()?;
    let out = drain(child.stdout.take());
    let err = drain(child.stderr.take());
    let until = Instant::now() + limit;
    let mut status = None;
    let stopped = loop {
        if status.is_none() {
            status = child.try_wait()?;
        }
        if status.is_some() && out.done() && err.done() {
            break None;
        }
        if Instant::now() >= until {
            break Some(Stopped::AtLimit);
        }
        if halt.halted() {
            break Some(Stopped::Halted);
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    if stopped.is_some() {
        // The group's id is its leader's, which the spawn made it.
        let group = child.id() as libc::pid_t;
        end_group(group, GRACE, &mut || {
            if status.is_none() {
                status = child.try_wait().ok().flatten();
            }
            status.is_some()
        });
        let until = Instant::now() + DRAINED;
        while !(out.done() && err.done()) && Instant::now() < until {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    let status = match status {
        Some(status) => status,
        None => child.wait()?,
    };
    Ok(Ended {
        status: Some(status),
        stopped,
        stdout: out.taken(),
        stderr: err.taken(),
    })
}

/// The words of a call stopped at its limit, as sluice says them:
/// `<call> with <remote> was still going after <limit>, the most a try
/// may take, and was stopped`.
pub fn stopped(call: &str, remote: &str, limit: Duration) -> String {
    format!(
        "{call} with {remote} was still going after {}, the most a try may take, and was stopped",
        span(limit)
    )
}

/// The words of a call ended by its halt, the runner's stop: `<call>
/// with <remote> was stopped as the runner stopped` (record 0042 as
/// amended on 2026-11-11, its design Decision 5).
pub fn halted(call: &str, remote: &str) -> String {
    format!("{call} with {remote} was stopped as the runner stopped")
}

/// A limit in the words a person reads: `30s`, or `300ms` under a second,
/// as sluice's `bounded::span` gives it.
fn span(limit: Duration) -> String {
    if limit >= Duration::from_secs(1) && limit.subsec_millis() == 0 {
        format!("{}s", limit.as_secs())
    } else {
        format!("{}ms", limit.as_millis())
    }
}

/// Ask the group `group` to end, give it `grace`, then kill what is left.
/// `reap` reaps the leader, whose parent the caller is, since a leader
/// left a zombie keeps its group alive; it returns whether the leader is
/// reaped.
fn end_group(group: libc::pid_t, grace: Duration, reap: &mut dyn FnMut() -> bool) {
    let alive = |reap: &mut dyn FnMut() -> bool| {
        reap();
        signal(group, 0)
    };
    if !alive(reap) {
        return;
    }
    signal(group, libc::SIGTERM);
    let until = Instant::now() + grace;
    while Instant::now() < until {
        if !alive(reap) {
            return;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    signal(group, libc::SIGKILL);
    let until = Instant::now() + KILLED;
    while alive(reap) && Instant::now() < until {
        std::thread::sleep(Duration::from_millis(20));
    }
}

/// `sig` sent to the group `group`, `0` asking only whether it is there:
/// false once no process of it is.
fn signal(group: libc::pid_t, sig: libc::c_int) -> bool {
    // SAFETY: killpg takes two integers and touches no memory of ours.
    let sent = unsafe { libc::killpg(group, sig) };
    sent == 0 || io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

/// A pipe read to its end by a thread of its own, into a buffer the
/// caller may take before the end comes.
struct Drain {
    read: Arc<Mutex<Vec<u8>>>,
    thread: Option<JoinHandle<()>>,
}

impl Drain {
    fn done(&self) -> bool {
        self.thread.as_ref().is_none_or(JoinHandle::is_finished)
    }

    /// What was read so far.
    fn taken(self) -> Vec<u8> {
        let mut read = self.read.lock().unwrap_or_else(|p| p.into_inner());
        std::mem::take(&mut *read)
    }
}

/// A thread reading `pipe` to its end.
fn drain(pipe: Option<impl Read + Send + 'static>) -> Drain {
    let read = Arc::new(Mutex::new(Vec::new()));
    let thread = pipe.map(|mut pipe| {
        let read = read.clone();
        std::thread::spawn(move || {
            let mut chunk = [0u8; 8192];
            loop {
                match pipe.read(&mut chunk) {
                    Ok(0) => break,
                    Ok(n) => read
                        .lock()
                        .unwrap_or_else(|p| p.into_inner())
                        .extend_from_slice(&chunk[..n]),
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(_) => break,
                }
            }
        })
    });
    Drain { read, thread }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether the process `pid` is gone.
    fn gone(pid: libc::pid_t) -> bool {
        // SAFETY: kill with no signal takes two integers and touches no
        // memory of ours.
        let sent = unsafe { libc::kill(pid, 0) };
        sent != 0 && io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
    }

    /// The clones spec's "What a call started ends with it": a process
    /// that starts another, neither ever ending, is ended at its limit with
    /// its group, and what each pipe gave until then is kept.
    #[test]
    fn a_process_that_never_ends_is_ended_with_its_group() {
        let started = Instant::now();
        let ended = output(
            Command::new("sh")
                .args([
                    "-c",
                    "echo begun; echo said >&2; sleep 60 & echo $!; sleep 60",
                ])
                .stdin(Stdio::null()),
            Duration::from_millis(300),
            &Halt::default(),
        )
        .unwrap();
        let returned = Instant::now();
        assert!(started.elapsed() < Duration::from_secs(5));
        assert_eq!(ended.stopped, Some(Stopped::AtLimit));
        let stdout = String::from_utf8(ended.stdout).unwrap();
        let mut lines = stdout.lines();
        assert_eq!(lines.next(), Some("begun"), "{stdout:?}");
        let pid: libc::pid_t = lines
            .next()
            .and_then(|pid| pid.parse().ok())
            .unwrap_or_else(|| panic!("no pid printed: {stdout:?}"));
        assert_eq!(lines.next(), None, "{stdout:?}");
        assert_eq!(ended.stderr, b"said\n");
        while !gone(pid) {
            assert!(
                returned.elapsed() < Duration::from_secs(1),
                "the background sleep {pid} outlived its call"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// The clones spec's "A call that ends within its limit": its exit and
    /// everything it printed are given, and it is not said stopped.
    #[test]
    fn a_process_that_ends_gives_its_exit_and_all_it_printed() {
        let ended = output(
            Command::new("sh")
                .args(["-c", "echo out; echo err >&2; exit 3"])
                .stdin(Stdio::null()),
            Duration::from_secs(10),
            &Halt::default(),
        )
        .unwrap();
        assert_eq!(ended.stopped, None);
        assert_eq!(ended.status.and_then(|status| status.code()), Some(3));
        assert_eq!(ended.stdout, b"out\n");
        assert_eq!(ended.stderr, b"err\n");
    }

    /// The clones spec's "A call asked to end is given its grace": what it
    /// said as it ended is kept, and it is said stopped.
    #[test]
    fn a_process_asked_to_end_says_so_in_its_grace() {
        let ended = output(
            Command::new("sh")
                .args(["-c", "trap 'echo asked; exit 0' TERM; sleep 60"])
                .stdin(Stdio::null()),
            Duration::from_millis(300),
            &Halt::default(),
        )
        .unwrap();
        assert_eq!(ended.stopped, Some(Stopped::AtLimit));
        assert_eq!(ended.stdout, b"asked\n");
    }

    /// The clones spec's "A call that will not end is killed": once its
    /// grace has passed.
    #[test]
    fn a_process_that_will_not_end_is_killed() {
        let started = Instant::now();
        let ended = output(
            Command::new("sh")
                .args(["-c", "trap '' TERM; while :; do sleep 0.1; done"])
                .stdin(Stdio::null()),
            Duration::from_millis(300),
            &Halt::default(),
        )
        .unwrap();
        let took = started.elapsed();
        assert_eq!(ended.stopped, Some(Stopped::AtLimit));
        assert!(took >= GRACE, "killed after {took:?}, before its grace");
        assert!(took < Duration::from_secs(5), "took {took:?}");
    }

    /// `halt` set from a thread of its own `after` from now.
    fn halting(halt: &Halt, after: Duration) -> JoinHandle<()> {
        let halt = halt.clone();
        std::thread::spawn(move || {
            std::thread::sleep(after);
            halt.halt();
        })
    }

    /// The clones spec's "What a halted call started ends with it": a
    /// process that starts another, neither ever ending, is ended with its
    /// group at its halt, well before its limit, and what each pipe gave
    /// until then is kept (record 0042 as amended on 2026-11-11, its design
    /// Decision 2).
    #[test]
    fn a_halted_process_is_ended_with_its_group() {
        let halt = Halt::default();
        let started = Instant::now();
        let halter = halting(&halt, Duration::from_millis(300));
        let ended = output(
            Command::new("sh")
                .args([
                    "-c",
                    "echo begun; echo said >&2; sleep 60 & echo $!; sleep 60",
                ])
                .stdin(Stdio::null()),
            Duration::from_secs(60),
            &halt,
        )
        .unwrap();
        let returned = Instant::now();
        halter.join().unwrap();
        assert!(started.elapsed() < Duration::from_secs(5));
        assert_eq!(ended.stopped, Some(Stopped::Halted));
        assert!(ended.status.is_some());
        let stdout = String::from_utf8(ended.stdout).unwrap();
        let mut lines = stdout.lines();
        assert_eq!(lines.next(), Some("begun"), "{stdout:?}");
        let pid: libc::pid_t = lines
            .next()
            .and_then(|pid| pid.parse().ok())
            .unwrap_or_else(|| panic!("no pid printed: {stdout:?}"));
        assert_eq!(lines.next(), None, "{stdout:?}");
        assert_eq!(ended.stderr, b"said\n");
        while !gone(pid) {
            assert!(
                returned.elapsed() < Duration::from_secs(1),
                "the background sleep {pid} outlived its call"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    /// The clones spec's "A halted call is given its grace": what it said
    /// as it ended is kept, and it is said halted.
    #[test]
    fn a_halted_process_is_given_its_grace() {
        let halt = Halt::default();
        let halter = halting(&halt, Duration::from_millis(300));
        let ended = output(
            Command::new("sh")
                .args(["-c", "trap 'echo asked; exit 0' TERM; sleep 60"])
                .stdin(Stdio::null()),
            Duration::from_secs(60),
            &halt,
        )
        .unwrap();
        halter.join().unwrap();
        assert_eq!(ended.stopped, Some(Stopped::Halted));
        assert_eq!(ended.stdout, b"asked\n");
    }

    /// The clones spec's "A call begun after its halt": it runs no
    /// process, returns at once with no exit and nothing printed, and is
    /// said halted (record 0042 as amended on 2026-11-11, its design
    /// Decision 3).
    #[test]
    fn a_process_begun_after_its_halt_runs_nothing() {
        let folder = std::env::temp_dir().join(format!(
            "boomdrive-bounded-halted-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        std::fs::create_dir_all(&folder).unwrap();
        let file = folder.join("touched");
        let halt = Halt::default();
        halt.halt();
        let started = Instant::now();
        let ended = output(
            Command::new("sh")
                .arg("-c")
                .arg(format!("touch '{}'", file.display()))
                .stdin(Stdio::null()),
            Duration::from_secs(60),
            &halt,
        )
        .unwrap();
        let took = started.elapsed();
        let touched = file.exists();
        std::fs::remove_dir_all(&folder).unwrap();
        assert_eq!(ended.stopped, Some(Stopped::Halted));
        assert!(ended.status.is_none(), "{:?}", ended.status);
        assert!(ended.stdout.is_empty());
        assert!(ended.stderr.is_empty());
        assert!(took < Duration::from_millis(100), "took {took:?}");
        assert!(!touched, "the call ran though its halt was set");
    }

    /// Design Decisions 1 and 3: a stopped call names its call, its
    /// remote and its limit.
    #[test]
    fn a_stopped_calls_words_name_its_call_remote_and_limit() {
        assert_eq!(
            stopped("git fetch", "origin", TRY_FOR),
            "git fetch with origin was still going after 30s, the most a try may take, and was \
             stopped"
        );
        assert_eq!(
            stopped(
                "git ls-remote",
                "https://example.invalid/s.git",
                Duration::from_millis(300)
            ),
            "git ls-remote with https://example.invalid/s.git was still going after 300ms, the \
             most a try may take, and was stopped"
        );
    }

    /// Record 0042's amendment of 2026-11-11, its design Decision 5: a
    /// halted call names its call, its remote and the runner's stop.
    #[test]
    fn a_halted_calls_words_name_its_call_and_remote() {
        assert_eq!(
            halted("git fetch", "origin"),
            "git fetch with origin was stopped as the runner stopped"
        );
        assert_eq!(
            halted("git ls-remote", "https://example.invalid/s.git"),
            "git ls-remote with https://example.invalid/s.git was stopped as the runner stopped"
        );
    }

    /// Design Decision 1: the limit and the grace are sluice's.
    #[test]
    fn the_limit_and_grace_are_sluices() {
        assert_eq!(TRY_FOR, Duration::from_secs(30));
        assert_eq!(GRACE, Duration::from_secs(2));
    }
}
