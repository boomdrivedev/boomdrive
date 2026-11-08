//! The runner's report file (design Decision 11): the runner's whole
//! report, kept at `<data>/control/report.json` and written whole through
//! a temporary file and a rename after every want that changes it, before
//! the next. A restart reads it, so an act or a command carried out just
//! before a stop is answered from it and not carried out again (core.10).

use crate::config::Address;
use crate::facts;
use boomdrive_control::{Id, Launched, Outcome, Report};
use serde_json::Value;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

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

/// The report `file` holds, none where it is missing, or why it cannot be
/// read.
pub fn read(file: &Path) -> Result<Option<Report>, String> {
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
    let partial = file.with_extension("json.partial");
    let written = || -> io::Result<()> {
        if let Some(dir) = file.parent() {
            fs::create_dir_all(dir)?;
        }
        let text = serde_json::to_string_pretty(report)? + "\n";
        let mut out = File::create(&partial)?;
        out.write_all(text.as_bytes())?;
        out.sync_all()?;
        fs::rename(&partial, file)
    };
    written().map_err(|error| format!("{} cannot be written: {error}", file.display()))
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
/// reads its phase tidied or gone. A place with no status carried has not
/// ended.
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
        .is_some_and(facts::ended)
}

#[cfg(test)]
mod tests {
    use super::*;
    use boomdrive_control::{Answer, At, Carried, Lease};

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
    fn a_file_that_is_no_report_is_named() {
        let dir = tempfile::tempdir().unwrap();
        let file = file(dir.path());
        fs::write(&file, "{\"launches\": 7}").unwrap();
        let why = read(&file).unwrap_err();
        assert!(why.starts_with(&format!("{} does not read", file.display())));
    }
}
