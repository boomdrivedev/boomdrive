//! The runner's report file (design Decision 11): the runner's whole
//! report, kept at `<data>/control/report.json` and written whole through
//! a temporary file and a rename after every want that changes it, before
//! the next. A restart reads it, so an act or a command carried out just
//! before a stop is answered from it and not carried out again (core.10).
//! Each part of the control plane keeps its share of the one file: the
//! entries its control plane owns, saved under the file's own lock, every
//! other part's entries kept as they stand. The file names in `parts` the
//! labels of the parts whose share it holds (record 0026; core.36).

use crate::config::Address;
use crate::{facts, flume};
use boomdrive_control::{Id, Launched, Outcome, Report};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// The runner's control directory in `data`, `<data>/control`: its report
/// file and the folder its briefs pass through. It is outside
/// `repositories/`, so the keeper never takes it for a clone.
pub fn control_dir(data: &Path) -> PathBuf {
    data.join("control")
}

/// The report file in `control_dir`, `report.json`.
pub fn file(control_dir: &Path) -> PathBuf {
    control_dir.join("report.json")
}

/// The report file as it is written: the report, and the labels of the
/// parts whose share it holds. A file with no `parts`, as the release
/// before wrote it, holds every part's share.
#[derive(Debug, Default, Serialize, Deserialize)]
struct Written {
    #[serde(flatten)]
    report: Report,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    parts: Option<BTreeSet<String>>,
}

/// The report `file` holds, none where it is missing, or why it cannot be
/// read.
pub fn read(file: &Path) -> Result<Option<Report>, String> {
    Ok(read_written(file)?.map(|written| written.report))
}

/// The file `file` as it is written, none where it is missing, or why it
/// cannot be read.
fn read_written(file: &Path) -> Result<Option<Written>, String> {
    let text = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("{} cannot be read: {error}", file.display())),
    };
    serde_json::from_str(&text)
        .map(Some)
        .map_err(|error| format!("{} does not read as a report: {error}", file.display()))
}

/// Writes `report` whole to `file`: to `<file>.partial` beside it, synced,
/// then renamed over it, so the file holds one report or the one before.
pub fn save(file: &Path, report: &Report) -> Result<(), String> {
    write(file, report)
}

/// Writes `value` whole to `file`, as [`save`] does.
fn write(file: &Path, value: &impl Serialize) -> Result<(), String> {
    let partial = file.with_extension("json.partial");
    let written = || -> io::Result<()> {
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        let text = serde_json::to_string_pretty(value)? + "\n";
        let mut out = File::create(&partial)?;
        out.write_all(text.as_bytes())?;
        out.sync_all()?;
        fs::rename(&partial, file)
    };
    written().map_err(|error| format!("{} cannot be written: {error}", file.display()))
}

/// The one report file every part of the control plane keeps its share
/// of, and its own lock, which a part's save holds for that save alone,
/// never across a tool's call, a pull or a report (record 0026; design
/// Decision 5).
#[derive(Debug)]
pub struct ReportFile {
    path: PathBuf,
    saving: Mutex<()>,
}

impl ReportFile {
    /// The report file in `control_dir`.
    pub fn new(control_dir: &Path) -> ReportFile {
        ReportFile {
            path: file(control_dir),
            saving: Mutex::new(()),
        }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The share of the part `label`, the entries `owns` holds, where the
    /// file names `label` in `parts` or names no `parts`; none where the
    /// file is missing, or names others in `parts` alone, so the part
    /// takes its share from its pull (core.10). Why the file cannot be
    /// read, where it cannot.
    pub fn share(&self, label: &str, owns: &dyn Fn(&Id) -> bool) -> Result<Option<Report>, String> {
        let Some(written) = read_written(&self.path)? else {
            return Ok(None);
        };
        if written
            .parts
            .as_ref()
            .is_some_and(|parts| !parts.contains(label))
        {
            return Ok(None);
        }
        let mut share = written.report;
        retain(&mut share, |id| owns(id));
        Ok(Some(share))
    }

    /// Saves `share` as the part `label`'s share, under the file's own
    /// lock: the file read, the entries `owns` holds replaced by
    /// `share`'s, every other entry kept as it stands, `label` added to
    /// `parts`, and the whole written by rename. A file with no `parts`
    /// holds every part's share, and goes on holding every part's: its
    /// `parts` stays unnamed, so no part it holds reads it as another's
    /// alone.
    pub fn save_share(
        &self,
        label: &str,
        owns: &dyn Fn(&Id) -> bool,
        share: &Report,
    ) -> Result<(), String> {
        let _saving = self.saving.lock().unwrap_or_else(|e| e.into_inner());
        let mut written = read_written(&self.path)?.unwrap_or_else(|| Written {
            report: Report::default(),
            parts: Some(BTreeSet::new()),
        });
        retain(&mut written.report, |id| !owns(id));
        let report = &mut written.report;
        report.launches.extend(cloned(&share.launches));
        report.acts.extend(cloned(&share.acts));
        report.commands.extend(cloned(&share.commands));
        report.inboxes.extend(cloned(&share.inboxes));
        report.previews.extend(cloned(&share.previews));
        if let Some(parts) = &mut written.parts {
            parts.insert(label.to_string());
        }
        write(&self.path, &written)
    }
}

/// `report` holding only the entries whose id `keep` holds.
fn retain(report: &mut Report, keep: impl Fn(&Id) -> bool) {
    report.launches.retain(|id, _| keep(id));
    report.acts.retain(|id, _| keep(id));
    report.commands.retain(|id, _| keep(id));
    report.inboxes.retain(|id, _| keep(id));
    report.previews.retain(|id, _| keep(id));
}

/// `entries`, each cloned.
fn cloned<T: Clone>(entries: &BTreeMap<Id, T>) -> impl Iterator<Item = (Id, T)> + '_ {
    entries
        .iter()
        .map(|(id, entry)| (id.clone(), entry.clone()))
}

/// Each launch of `report`, with its place, that holds the repository at
/// `address` (record 0021; core.5): one whose recorded repository is that
/// address, as addresses compare, answered `launched` at a place that has
/// not ended. A launch recording no repository, or one not an address,
/// holds none. The keeper, the hello and the control pass read draining
/// by it alike.
pub fn holding(report: &Report, address: &Address) -> Vec<(Id, String)> {
    report
        .launches
        .iter()
        .filter(|(_, launched)| {
            let repository = launched.repository.as_deref().map(Address::parse);
            matches!(repository, Some(Ok(repository)) if repository == *address)
        })
        .filter_map(|(id, launched)| {
            let place = launched.answer.place_dir.as_deref()?;
            let live = launched.answer.outcome == Outcome::Launched && !ended(launched, place);
            live.then(|| (id.clone(), place.to_string()))
        })
        .collect()
}

/// Whether the place at `place` has ended, by the status `launched`
/// carries of it: the status stands not listed, or the place's entry in it
/// reads its phase tidied or gone and its record pending for no end of
/// the runner's, so that a drained clone waits for the runner to collect
/// it (record 0030). A place with no status carried has not ended.
fn ended(launched: &Launched, place: &str) -> bool {
    let Some(carried) = &launched.status else {
        return false;
    };
    if !carried.listed {
        return true;
    }
    serde_json::from_str::<Value>(&carried.text)
        .ok()
        .as_ref()
        .and_then(|answer| facts::entry(answer, place))
        .is_some_and(|entry| facts::ended(entry) && !facts::pending_for(entry, flume::RECIPIENT))
}

#[cfg(test)]
mod tests {
    use super::*;
    use boomdrive_control::{Answer, At, Carried, Lease, Previewed};

    fn a_report() -> Report {
        let mut report = Report::default();
        report.answer(
            &Id::new("home/12"),
            Lease {
                number: 3,
                expires: Some(At::parse("2026-11-01T10:00:40Z").unwrap()),
            },
            Answer {
                outcome: Outcome::Launched,
                place_dir: Some("/places/a".into()),
                words: None,
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
        );
        report
    }

    #[test]
    fn a_report_saved_reads_back() {
        let dir = tempfile::tempdir().unwrap();
        let file = file(&control_dir(dir.path()));
        save(&file, &a_report()).unwrap();
        assert_eq!(read(&file).unwrap(), Some(a_report()));
        let names: Vec<_> = fs::read_dir(file.parent().unwrap())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(names, ["report.json"], "no partial file is left");
    }

    #[test]
    fn a_report_saved_again_replaces_the_last() {
        let dir = tempfile::tempdir().unwrap();
        let file = file(dir.path());
        save(&file, &a_report()).unwrap();
        save(&file, &Report::default()).unwrap();
        assert_eq!(read(&file).unwrap(), Some(Report::default()));
    }

    #[test]
    fn a_missing_file_reads_as_none() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(read(&file(dir.path())).unwrap(), None);
    }

    /// `id` answered launched at its place, under lease 1.
    fn launched(report: &mut Report, id: &str) {
        report.answer(
            &Id::new(id),
            Lease {
                number: 1,
                expires: None,
            },
            Answer {
                outcome: Outcome::Launched,
                place_dir: Some(format!("/places/{id}")),
                words: None,
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
        );
    }

    /// A report holding a launch for each of `ids`.
    fn launches(ids: &[&str]) -> Report {
        let mut report = Report::default();
        for id in ids {
            launched(&mut report, id);
        }
        report
    }

    /// Whether `id` is `label`'s, under `<label>/`.
    fn of(label: &str) -> impl Fn(&Id) -> bool + '_ {
        move |id| id.as_str().starts_with(&format!("{label}/"))
    }

    /// The launches' ids `file` holds, and the parts it names.
    fn held(file: &Path) -> (Vec<String>, Option<Vec<String>>) {
        let written = read_written(file).unwrap().expect("the file");
        let ids = written.report.launches.keys().map(Id::to_string).collect();
        let parts = written.parts.map(|parts| parts.into_iter().collect());
        (ids, parts)
    }

    /// The work-into-flume spec's "A part's save keeps every other part's
    /// share": a file holding `home/1` and `work/2`, and `home` saving its
    /// share holding `home/1` and `home/3`, holds all three and names both.
    #[test]
    fn a_save_keeps_every_other_parts_entries() {
        let dir = tempfile::tempdir().unwrap();
        let shared = ReportFile::new(dir.path());
        shared
            .save_share("work", &of("work"), &launches(&["work/2"]))
            .unwrap();
        shared
            .save_share("home", &of("home"), &launches(&["home/1"]))
            .unwrap();
        shared
            .save_share("home", &of("home"), &launches(&["home/1", "home/3"]))
            .unwrap();
        assert_eq!(
            held(shared.path()),
            (
                vec!["home/1".into(), "home/3".into(), "work/2".into()],
                Some(vec!["home".into(), "work".into()])
            )
        );
        let home = shared.share("home", &of("home")).unwrap();
        assert_eq!(home, Some(launches(&["home/1", "home/3"])));
        let work = shared.share("work", &of("work")).unwrap();
        assert_eq!(work, Some(launches(&["work/2"])));
        let names: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        assert_eq!(names, ["report.json"], "no partial file is left");
    }

    /// A part's previews are of its share, as its launches are: `home`'s
    /// save holds its preview `home/4` and keeps `work`'s `work/5`, and
    /// each part's share reads back holding its own alone.
    #[test]
    fn a_save_keeps_every_parts_previews() {
        let previewed = |id: &str| {
            let mut report = Report::default();
            report.previews.insert(
                Id::new(id),
                Previewed {
                    lease: Lease {
                        number: 1,
                        expires: None,
                    },
                    repository: "example/crew".into(),
                    revision: "R1".into(),
                    done: false,
                    outcome: Outcome::Done,
                    text: Some("{}".into()),
                    words: None,
                    status: None,
                    at: At::parse("2026-11-01T10:00:00Z").unwrap(),
                },
            );
            report
        };
        let dir = tempfile::tempdir().unwrap();
        let shared = ReportFile::new(dir.path());
        shared
            .save_share("work", &of("work"), &previewed("work/5"))
            .unwrap();
        shared
            .save_share("home", &of("home"), &previewed("home/4"))
            .unwrap();
        let home = shared.share("home", &of("home")).unwrap();
        assert_eq!(home, Some(previewed("home/4")));
        let work = shared.share("work", &of("work")).unwrap();
        assert_eq!(work, Some(previewed("work/5")));
        shared
            .save_share("home", &of("home"), &Report::default())
            .unwrap();
        let home = shared.share("home", &of("home")).unwrap();
        assert_eq!(home, Some(Report::default()));
        let work = shared.share("work", &of("work")).unwrap();
        assert_eq!(work, Some(previewed("work/5")));
    }

    /// The work-into-flume spec's "A file from the release before": a file
    /// with no `parts` holds every part's share, and goes on holding every
    /// part's once one of them saves to it. The release before reads the
    /// file this release writes as the report it holds.
    #[test]
    fn a_file_with_no_parts_holds_every_part() {
        let dir = tempfile::tempdir().unwrap();
        let shared = ReportFile::new(dir.path());
        save(shared.path(), &launches(&["home/1", "work/2"])).unwrap();
        assert_eq!(
            shared.share("work", &of("work")).unwrap(),
            Some(launches(&["work/2"]))
        );
        shared
            .save_share("home", &of("home"), &launches(&["home/3"]))
            .unwrap();
        assert_eq!(
            held(shared.path()),
            (vec!["home/3".into(), "work/2".into()], None)
        );
        assert_eq!(
            shared.share("work", &of("work")).unwrap(),
            Some(launches(&["work/2"]))
        );
        assert_eq!(
            read(shared.path()).unwrap(),
            Some(launches(&["home/3", "work/2"]))
        );
    }

    /// A part the file does not name has no share of it, nor has any part
    /// of a file that is missing: each takes its share from its pull.
    #[test]
    fn a_part_the_file_does_not_name_has_no_share() {
        let dir = tempfile::tempdir().unwrap();
        let shared = ReportFile::new(dir.path());
        assert_eq!(shared.share("home", &of("home")).unwrap(), None);
        shared
            .save_share("home", &of("home"), &launches(&["home/1"]))
            .unwrap();
        assert_eq!(shared.share("work", &of("work")).unwrap(), None);
        shared
            .save_share("work", &of("work"), &launches(&["work/2"]))
            .unwrap();
        assert_eq!(
            shared.share("work", &of("work")).unwrap(),
            Some(launches(&["work/2"]))
        );
    }

    /// Two parts saving at once each keep the other's entries: each save
    /// reads and writes the file under its lock.
    #[test]
    fn parts_saving_at_once_keep_each_others_entries() {
        let dir = tempfile::tempdir().unwrap();
        let shared = ReportFile::new(dir.path());
        std::thread::scope(|scope| {
            for label in ["home", "work"] {
                let shared = &shared;
                scope.spawn(move || {
                    for n in 0..20 {
                        let ids: Vec<String> = (0..=n).map(|k| format!("{label}/{k}")).collect();
                        let ids: Vec<&str> = ids.iter().map(String::as_str).collect();
                        shared
                            .save_share(label, &of(label), &launches(&ids))
                            .unwrap();
                    }
                });
            }
        });
        let (ids, parts) = held(shared.path());
        assert_eq!(ids.len(), 40, "{ids:?}");
        assert_eq!(parts, Some(vec!["home".into(), "work".into()]));
    }

    /// A launch at `place` in `repository`, answered `outcome`, its status
    /// carried with the place's `phase`, `listed` or not, where one is.
    fn launch(
        repository: Option<&str>,
        outcome: Outcome,
        place: &str,
        status: Option<(&str, bool)>,
    ) -> Launched {
        Launched {
            lease: Lease {
                number: 1,
                expires: None,
            },
            repository: repository.map(str::to_string),
            answer: Answer {
                outcome,
                place_dir: Some(place.to_string()),
                words: None,
                at: At::parse("2026-11-01T10:00:00Z").unwrap(),
            },
            status: status.map(|(phase, listed)| Carried {
                text: format!(
                    r#"{{"status":{{"places":[{{"place":"{place}","phase":{phase}}}]}}}}"#
                ),
                at: At::parse("2026-11-01T10:01:00Z").unwrap(),
                listed,
            }),
            entry: None,
        }
    }

    #[test]
    fn the_places_that_hold_a_repository() {
        let live = Some((r#""live""#, true));
        let tidied = r#"{"tidied":{"over":{"landed":{}},"at":7,"pending_for":["r"]}}"#;
        let launches = [
            (
                "a",
                launch(Some("example/kit"), Outcome::Launched, "/p/a", live),
            ),
            (
                "b",
                launch(Some("Example/Kit"), Outcome::Launched, "/p/b", None),
            ),
            (
                "c",
                launch(
                    Some("example/kit"),
                    Outcome::Launched,
                    "/p/c",
                    Some((r#"{"over":{"over":"done"}}"#, true)),
                ),
            ),
            (
                "d",
                launch(
                    Some("example/kit"),
                    Outcome::Launched,
                    "/p/d",
                    Some((tidied, true)),
                ),
            ),
            (
                "e",
                launch(
                    Some("example/kit"),
                    Outcome::Launched,
                    "/p/e",
                    Some((r#"{"gone":{"over":"done"}}"#, true)),
                ),
            ),
            (
                "f",
                launch(
                    Some("example/kit"),
                    Outcome::Launched,
                    "/p/f",
                    Some((r#""live""#, false)),
                ),
            ),
            (
                "g",
                launch(Some("example/other"), Outcome::Launched, "/p/g", live),
            ),
            (
                "h",
                launch(Some("example/kit"), Outcome::Held, "/p/h", None),
            ),
            (
                "i",
                launch(Some("example/kit"), Outcome::Refused, "/p/i", None),
            ),
            ("j", launch(None, Outcome::Launched, "/p/j", live)),
            (
                "k",
                launch(Some("not an address"), Outcome::Launched, "/p/k", live),
            ),
        ];
        let mut report = Report::default();
        for (id, launched) in launches {
            report.launches.insert(Id::new(id), launched);
        }
        let mut placeless = launch(Some("example/kit"), Outcome::Launched, "/p/l", live);
        placeless.answer.place_dir = None;
        report.launches.insert(Id::new("l"), placeless);
        let held: Vec<(String, String)> = holding(&report, &Address::parse("example/kit").unwrap())
            .into_iter()
            .map(|(id, place)| (id.to_string(), place))
            .collect();
        assert_eq!(
            held,
            [
                ("a".to_string(), "/p/a".to_string()),
                ("b".to_string(), "/p/b".to_string()),
                ("c".to_string(), "/p/c".to_string()),
            ]
        );
        let other = holding(&report, &Address::parse("example/other").unwrap());
        assert_eq!(other, [(Id::new("g"), "/p/g".to_string())]);
        assert!(holding(&report, &Address::parse("example/none").unwrap()).is_empty());
    }

    #[test]
    fn a_place_pending_for_the_runner_has_not_ended() {
        let tidied = |pending: &str| {
            format!(r#"{{"tidied":{{"over":{{"landed":{{}}}},"at":7,"pending_for":{pending}}}}}"#)
        };
        let launches = [
            ("a", tidied(r#"["boomdrive"]"#)),
            ("b", tidied(r#"["scratch/planner","boomdrive"]"#)),
            ("c", tidied(r#"["scratch/planner"]"#)),
            ("d", tidied("[]")),
        ];
        let mut report = Report::default();
        for (id, phase) in &launches {
            let place = format!("/p/{id}");
            let mut launched = launch(
                Some("example/kit"),
                Outcome::Launched,
                &place,
                Some((phase, true)),
            );
            if let Some(status) = launched.status.as_mut() {
                status.text = status.text.replace(
                    &format!(r#""place":"{place}""#),
                    &format!(r#""place":"{place}@7""#),
                );
            }
            report.launches.insert(Id::new(*id), launched);
        }
        let held: Vec<String> = holding(&report, &Address::parse("example/kit").unwrap())
            .into_iter()
            .map(|(id, _)| id.to_string())
            .collect();
        assert_eq!(held, ["a", "b"]);
    }

    #[test]
    fn a_file_that_is_no_report_is_named() {
        let dir = tempfile::tempdir().unwrap();
        let file = file(dir.path());
        fs::write(&file, "{\"launches\": 7}").unwrap();
        let why = read(&file).unwrap_err();
        assert!(why.starts_with(&format!("{} does not read", file.display())));
    }
}
