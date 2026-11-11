//! A process the runner waits on for a limited time, in a process group
//! of its own: each pipe read on a thread of its own as it goes, and at
//! its limit the group asked to end, given [`GRACE`], then killed, so
//! nothing it started outlives the wait (record 0042; core.58).
//!
//! This is agentplot/sluice record 0109's design, the family's one design
//! for a git call's limit, and mirrors sluice's `crates/sluice/src/bounded.rs`
//! at `e6809f9`, its group ended as sluice's `runs::end_group` ends one.
//! Every try of the runner's git call to a remote runs so: a clone's fetch
//! and a state repository's listing.

use std::io::{self, Read};
use std::os::unix::process::CommandExt;
use std::process::{Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

/// How long one try of a call to a remote may take before it is ended:
/// agentplot/sluice record 0109's `retry::TRY_FOR`, one value for every
/// call of the family's, set by no configuration (record 0042).
pub const TRY_FOR: Duration = Duration::from_secs(30);

/// How long a group asked to end at its limit has before it is killed:
/// agentplot/sluice record 0109's `bounded::GRACE`, in which git, asked,
/// removes the lock files it holds in the repository.
pub const GRACE: Duration = Duration::from_secs(2);

/// How long the output of a group ended at its limit is waited for once
/// the group is gone: a process outside it may hold a pipe open.
const DRAINED: Duration = Duration::from_millis(200);

/// How long a group killed is waited for to be gone.
const KILLED: Duration = Duration::from_secs(5);

/// How a process ended, and what it printed until then.
#[derive(Debug)]
pub struct Ended {
    /// Its exit, by itself or by the signal that ended its group.
    pub status: ExitStatus,
    /// Whether its limit came first, and its group was ended.
    pub stopped: bool,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

/// `command` run in a process group of its own, its standard output and
/// error read, to `limit`. At the limit its group is asked to end, given
/// [`GRACE`], then killed. Its standard input is the caller's to set.
pub fn output(command: &mut Command, limit: Duration) -> io::Result<Ended> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()?;
    let out = drain(child.stdout.take());
    let err = drain(child.stderr.take());
    let until = Instant::now() + limit;
    let mut status = None;
    let done = loop {
        if status.is_none() {
            status = child.try_wait()?;
        }
        if status.is_some() && out.done() && err.done() {
            break true;
        }
        if Instant::now() >= until {
            break false;
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    if !done {
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
        status,
        stopped: !done,
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
        )
        .unwrap();
        let returned = Instant::now();
        assert!(started.elapsed() < Duration::from_secs(5));
        assert!(ended.stopped);
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
        )
        .unwrap();
        assert!(!ended.stopped);
        assert_eq!(ended.status.code(), Some(3));
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
        )
        .unwrap();
        assert!(ended.stopped);
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
        )
        .unwrap();
        let took = started.elapsed();
        assert!(ended.stopped);
        assert!(took >= GRACE, "killed after {took:?}, before its grace");
        assert!(took < Duration::from_secs(5), "took {took:?}");
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

    /// Design Decision 1: the limit and the grace are sluice's.
    #[test]
    fn the_limit_and_grace_are_sluices() {
        assert_eq!(TRY_FOR, Duration::from_secs(30));
        assert_eq!(GRACE, Duration::from_secs(2));
    }
}
