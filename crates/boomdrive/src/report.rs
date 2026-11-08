//! The runner's report file (design Decision 11): the runner's whole
//! report, kept at `<data>/control/report.json` and written whole through
//! a temporary file and a rename after every want that changes it, before
//! the next. A restart reads it, so an act or a command carried out just
//! before a stop is answered from it and not carried out again (core.10).

use boomdrive_control::Report;
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

#[cfg(test)]
mod tests {
    use super::*;
    use boomdrive_control::{Answer, At, Id, Lease, Outcome};

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

    #[test]
    fn a_file_that_is_no_report_is_named() {
        let dir = tempfile::tempdir().unwrap();
        let file = file(dir.path());
        fs::write(&file, "{\"launches\": 7}").unwrap();
        let why = read(&file).unwrap_err();
        assert!(why.starts_with(&format!("{} does not read", file.display())));
    }
}
