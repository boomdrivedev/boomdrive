//! Each clone's lock (record 0026; design Decision 4): one reader-writer
//! lock for each address the runner keeps, compared without case as an
//! `Address` compares. The keeper holds a repository's lock alone while it
//! keeps or lets go of it, and a part's launch into the repository's clone
//! holds it shared with every other launch, so a launch waits only for
//! the keep of its own repository, and the keeper only for the launches
//! into it under way. A keep waiting for the lock holds every launch that
//! comes after it, so launches never keep the keeper out. Nothing else
//! takes a lock, and none is held across a pass, a pull or a report.

use crate::config::Address;
use std::collections::HashMap;
use std::sync::{Condvar, Mutex, MutexGuard};

/// Each kept address's lock.
#[derive(Debug, Default)]
pub struct Locks {
    held: Mutex<HashMap<String, Held>>,
    changed: Condvar,
}

/// How one address's lock is held.
#[derive(Debug, Default)]
struct Held {
    /// The launches holding it.
    launches: usize,
    /// Whether a keep holds it.
    kept: bool,
    /// The keeps waiting for it.
    waiting: usize,
}

impl Held {
    fn idle(&self) -> bool {
        self.launches == 0 && !self.kept && self.waiting == 0
    }

    /// Whether a launch may take it now: no keep holds it or waits for it.
    fn launchable(&self) -> bool {
        !self.kept && self.waiting == 0
    }
}

/// A lock held, let go when dropped.
#[derive(Debug)]
#[must_use = "the lock is let go when this is dropped"]
pub struct Hold<'a> {
    locks: &'a Locks,
    key: String,
    keep: bool,
}

/// The key `address`'s lock is held under: its address without case.
fn key(address: &Address) -> String {
    format!("{}/{}", address.owner(), address.name()).to_ascii_lowercase()
}

impl Locks {
    pub fn new() -> Locks {
        Locks::default()
    }

    fn held(&self) -> MutexGuard<'_, HashMap<String, Held>> {
        self.held.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Holds `address`'s lock alone, for the keeper's keep or let-go of
    /// it, once no launch into it and no other keep of it holds it.
    pub fn keep(&self, address: &Address) -> Hold<'_> {
        let key = key(address);
        let mut held = self.held();
        held.entry(key.clone()).or_default().waiting += 1;
        loop {
            let entry = held.get_mut(&key).expect("the entry waited on");
            if entry.launches == 0 && !entry.kept {
                entry.waiting -= 1;
                entry.kept = true;
                return Hold {
                    locks: self,
                    key,
                    keep: true,
                };
            }
            held = self.changed.wait(held).unwrap_or_else(|e| e.into_inner());
        }
    }

    /// Holds `address`'s lock shared, for a launch into its clone, once no
    /// keep of it holds it or waits for it.
    pub fn launch(&self, address: &Address) -> Hold<'_> {
        let key = key(address);
        let mut held = self.held();
        loop {
            let entry = held.entry(key.clone()).or_default();
            if entry.launchable() {
                entry.launches += 1;
                return Hold {
                    locks: self,
                    key,
                    keep: false,
                };
            }
            held = self.changed.wait(held).unwrap_or_else(|e| e.into_inner());
        }
    }

    /// Holds `address`'s lock shared, for a launch into its clone, where no
    /// keep of it holds it or waits for it; none otherwise, at once.
    pub fn try_launch(&self, address: &Address) -> Option<Hold<'_>> {
        let key = key(address);
        let mut held = self.held();
        let entry = held.entry(key.clone()).or_default();
        if !entry.launchable() {
            return None;
        }
        entry.launches += 1;
        Some(Hold {
            locks: self,
            key,
            keep: false,
        })
    }
}

impl Drop for Hold<'_> {
    fn drop(&mut self) {
        let mut held = self.locks.held();
        if let Some(entry) = held.get_mut(&self.key) {
            match self.keep {
                true => entry.kept = false,
                false => entry.launches -= 1,
            }
            if entry.idle() {
                held.remove(&self.key);
            }
        }
        drop(held);
        self.locks.changed.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{mpsc, Arc};
    use std::thread;
    use std::time::Duration;

    /// Long enough for a thread that could take a lock to take it.
    const A_WHILE: Duration = Duration::from_millis(200);

    fn address(text: &str) -> Address {
        Address::parse(text).unwrap()
    }

    /// Whether `take`, run on a thread of its own, takes its lock within
    /// a while. The thread lets it go at once; where it is held, the
    /// thread is left waiting, and the test fails.
    fn taken_within_a_while(
        locks: &Arc<Locks>,
        take: impl Fn(&Locks) -> Hold<'_> + Send + 'static,
    ) -> bool {
        let (took, taken) = mpsc::channel();
        let locks = Arc::clone(locks);
        thread::spawn(move || {
            drop(take(&locks));
            let _ = took.send(());
        });
        taken.recv_timeout(A_WHILE).is_ok()
    }

    /// A keep of an address, in any case, holds a launch into it and
    /// another keep of it, until it is let go.
    #[test]
    fn a_keep_holds_a_launch_and_a_keep_of_its_address() {
        let locks = Locks::new();
        let kept = locks.keep(&address("agentplot/Flume"));
        assert!(locks.try_launch(&address("agentplot/flume")).is_none());
        thread::scope(|scope| {
            let (took, taken) = mpsc::channel();
            let launching = took.clone();
            let locks = &locks;
            scope.spawn(move || {
                let _hold = locks.launch(&address("Agentplot/flume"));
                let _ = launching.send("launch");
            });
            scope.spawn(move || {
                let _hold = locks.keep(&address("AGENTPLOT/FLUME"));
                let _ = took.send("keep");
            });
            assert!(taken.recv_timeout(A_WHILE).is_err(), "held by the keep");
            drop(kept);
            let mut both = vec![
                taken.recv_timeout(Duration::from_secs(5)).unwrap(),
                taken.recv_timeout(Duration::from_secs(5)).unwrap(),
            ];
            both.sort();
            assert_eq!(both, ["keep", "launch"]);
        });
    }

    /// Launches into one clone hold none of each other.
    #[test]
    fn launches_of_one_address_hold_none_of_each_other() {
        let locks = Arc::new(Locks::new());
        let first = locks.launch(&address("agentplot/flume"));
        assert!(taken_within_a_while(&locks, |locks| locks.launch(&address("agentplot/FLUME"))));
        let second = locks.try_launch(&address("agentplot/flume"));
        assert!(second.is_some());
        drop((first, second));
    }

    /// A launch under way holds a keep of its clone, and a keep waiting
    /// holds every launch that comes after it.
    #[test]
    fn a_launch_holds_a_keep_and_a_waiting_keep_holds_launches() {
        let locks = Locks::new();
        let launch = locks.launch(&address("agentplot/flume"));
        thread::scope(|scope| {
            let (took, taken) = mpsc::channel();
            let locks = &locks;
            scope.spawn(move || {
                let _hold = locks.keep(&address("agentplot/flume"));
                let _ = took.send(());
            });
            assert!(taken.recv_timeout(A_WHILE).is_err(), "held by the launch");
            assert!(
                locks.try_launch(&address("agentplot/flume")).is_none(),
                "a waiting keep holds a later launch"
            );
            drop(launch);
            taken.recv_timeout(Duration::from_secs(5)).unwrap();
        });
        assert!(locks.try_launch(&address("agentplot/flume")).is_some());
    }

    /// Nothing of one address holds another's lock.
    #[test]
    fn nothing_of_one_address_holds_another() {
        let locks = Arc::new(Locks::new());
        let kept = locks.keep(&address("agentplot/flume"));
        let launch = locks.launch(&address("agentplot/timber"));
        assert!(taken_within_a_while(&locks, |locks| locks.keep(&address("agentplot/sluice"))));
        assert!(taken_within_a_while(&locks, |locks| locks.launch(&address("agentplot/sluice"))));
        assert!(locks.try_launch(&address("agentplot/sluice")).is_some());
        drop((kept, launch));
        assert!(locks.held().is_empty(), "no lock is left held");
    }
}
