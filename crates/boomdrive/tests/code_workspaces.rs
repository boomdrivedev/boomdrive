//! Each owner's VS Code workspace beside its views, where the
//! configuration asks: its folders the owner's kept repositories, set at
//! every pass, the rest of the file kept, and a file the runner cannot
//! read left as it is and named (the views spec; core.43; design
//! Decisions 1, 3 to 6).

mod common;

use boomdrive::pass::Outcome;
use common::{keeping, Scratch};
use serde_json::{json, Value};
use std::ffi::OsStr;
use std::fs;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;

/// A configuration keeping `addresses`, in order, that asks for the
/// workspaces.
fn asking(addresses: &[&str]) -> String {
    format!("code_workspaces = true\n{}", keeping(addresses))
}

/// `owner`'s workspace under the default repository root.
fn workspace(scratch: &Scratch, owner: &str) -> PathBuf {
    scratch
        .home
        .join("Code")
        .join(owner)
        .join(format!("{owner}.code-workspace"))
}

/// The JSON the file at `path` holds.
fn json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).expect("the workspace")).expect("JSON")
}

/// The names of `value`'s members, in its order.
fn members(value: &Value) -> Vec<String> {
    value
        .as_object()
        .expect("an object")
        .keys()
        .cloned()
        .collect()
}

/// The line a pass says when it writes the workspace `file` with
/// `folders`.
fn written(file: &Path, folders: &str) -> String {
    format!(
        "boomdrive: workspace written at {}: {folders}",
        file.display()
    )
}

/// The beginning of the line a pass says when it leaves `file`.
fn left(file: &Path) -> String {
    format!("boomdrive: workspace left at {}: ", file.display())
}

/// The lines among `lines` that name a workspace.
fn of_workspaces(lines: &[String]) -> Vec<&String> {
    lines
        .iter()
        .filter(|line| line.starts_with("boomdrive: workspace "))
        .collect()
}

/// Every file under `dir` whose name ends in `.code-workspace`.
fn workspaces_under(dir: &Path) -> Vec<PathBuf> {
    let mut found = Vec::new();
    let Ok(read) = fs::read_dir(dir) else {
        return found;
    };
    for entry in read {
        let path = entry.expect("an entry").path();
        let kind = fs::symlink_metadata(&path).expect("the entry").file_type();
        if kind.is_dir() {
            found.extend(workspaces_under(&path));
        } else if path.extension() == Some(OsStr::new("code-workspace")) {
            found.push(path);
        }
    }
    found
}

/// A folder's mode, set for a test and set back to 0755 when dropped, so
/// the scratch folder can be removed.
struct Mode(PathBuf);

impl Mode {
    fn set(folder: &Path, mode: u32) -> Mode {
        fs::set_permissions(folder, fs::Permissions::from_mode(mode)).expect("the folder's mode");
        Mode(folder.to_path_buf())
    }
}

impl Drop for Mode {
    fn drop(&mut self) {
        let _ = fs::set_permissions(&self.0, fs::Permissions::from_mode(0o755));
    }
}

#[test]
fn the_first_workspaces() {
    let scratch = Scratch::new();
    let kept = ["agentplot/flume", "afterthought/crew", "agentplot/sluice"];
    for address in kept {
        scratch.origin(address);
    }

    let (outcome, lines) = scratch.pass(&asking(&kept));

    let agentplot = workspace(&scratch, "agentplot");
    let afterthought = workspace(&scratch, "afterthought");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        json(&agentplot),
        json!({"folders": [{"path": "flume"}, {"path": "sluice"}], "settings": {}})
    );
    assert_eq!(members(&json(&agentplot)), ["folders", "settings"]);
    assert_eq!(
        json(&afterthought),
        json!({"folders": [{"path": "crew"}], "settings": {}})
    );
    assert_eq!(lines.len(), 8, "{lines:#?}");
    assert_eq!(
        lines[6..],
        [
            written(&agentplot, "flume, sluice"),
            written(&afterthought, "crew"),
        ]
    );
}

#[test]
fn no_workspace_asked_for() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");

    let (outcome, lines) = scratch.pass(&keeping(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert!(scratch.home.join("Code/agentplot/flume").exists());
    assert_eq!(
        workspaces_under(&scratch.home.join("Code")),
        [] as [PathBuf; 0]
    );
    assert_eq!(of_workspaces(&lines), [] as [&String; 0]);
}

#[test]
fn a_kept_repository_with_no_view() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.empty_origin("agentplot/empty");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/flume", "agentplot/empty"]));

    let file = workspace(&scratch, "agentplot");
    assert_eq!(outcome, Outcome::Kept);
    assert!(!scratch.home.join("Code/agentplot/empty").exists());
    assert_eq!(
        json(&file)["folders"],
        json!([{"path": "flume"}, {"path": "empty"}])
    );
    assert_eq!(of_workspaces(&lines), [&written(&file, "flume, empty")]);
}

#[test]
fn the_users_settings_kept() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&asking(&["agentplot/flume"]));
    let file = workspace(&scratch, "agentplot");
    fs::write(
        &file,
        "{\n\
         \t\"folders\": [\n\
         \t\t{\n\
         \t\t\t\"path\": \"flume\"\n\
         \t\t}\n\
         \t],\n\
         \t\"settings\": {\n\
         \t\t\"editor.tabSize\": 2\n\
         \t},\n\
         \t\"extensions\": {\n\
         \t\t\"recommendations\": [\"rust-lang.rust-analyzer\"]\n\
         \t}\n\
         }\n",
    )
    .expect("the user's settings");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/flume", "agentplot/sluice"]));

    let value = json(&file);
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(members(&value), ["folders", "settings", "extensions"]);
    assert_eq!(
        value,
        json!({
            "folders": [{"path": "flume"}, {"path": "sluice"}],
            "settings": {"editor.tabSize": 2},
            "extensions": {"recommendations": ["rust-lang.rust-analyzer"]},
        })
    );
    assert_eq!(of_workspaces(&lines), [&written(&file, "flume, sluice")]);
}

#[test]
fn folders_edited_by_hand() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    let config = asking(&["agentplot/flume", "agentplot/sluice"]);
    scratch.pass(&config);
    let file = workspace(&scratch, "agentplot");
    let mut edited = json(&file);
    edited["folders"] = json!([{"path": "sluice"}]);
    fs::write(
        &file,
        serde_json::to_string_pretty(&edited).expect("the edit"),
    )
    .expect("the folders edited");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [written(&file, "flume, sluice")]);
    assert_eq!(
        json(&file),
        json!({"folders": [{"path": "flume"}, {"path": "sluice"}], "settings": {}})
    );
}

#[test]
fn a_dropped_repository_leaves_its_workspace() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&asking(&["agentplot/flume", "agentplot/sluice"]));
    let sluice = scratch.home.join("Code/agentplot/sluice");
    let put = scratch.git(&sluice, &["rev-parse", "HEAD"]);
    fs::write(sluice.join("README"), "mine\n").expect("a change in the view");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/flume"]));

    let file = workspace(&scratch, "agentplot");
    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        lines,
        [
            "agentplot/sluice: left: held by the view's changes in 1 file".to_string(),
            written(&file, "flume"),
        ]
    );
    assert_eq!(json(&file)["folders"], json!([{"path": "flume"}]));
    assert_eq!(
        fs::read_to_string(sluice.join("README")).expect("the change"),
        "mine\n"
    );
    assert_eq!(scratch.git(&sluice, &["rev-parse", "HEAD"]), put);
}

#[test]
fn a_quiet_pass_leaves_the_workspaces() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("afterthought/crew");
    let config = asking(&["agentplot/flume", "afterthought/crew"]);
    scratch.pass(&config);
    let files = [
        workspace(&scratch, "agentplot"),
        workspace(&scratch, "afterthought"),
    ];
    let before: Vec<(Vec<u8>, u64)> = files
        .iter()
        .map(|file| {
            let ino = fs::metadata(file).expect("the workspace").ino();
            (fs::read(file).expect("the workspace"), ino)
        })
        .collect();

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [] as [String; 0]);
    for (file, (bytes, ino)) in files.iter().zip(before) {
        assert_eq!(fs::read(file).expect("the workspace"), bytes);
        assert_eq!(
            fs::metadata(file).expect("the workspace").ino(),
            ino,
            "{} was written",
            file.display()
        );
    }
}

#[test]
fn a_comment_in_the_workspace() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    scratch.origin("agentplot/sluice");
    scratch.pass(&asking(&["agentplot/flume"]));
    let file = workspace(&scratch, "agentplot");
    let mine = "{\n\
                \t// mine\n\
                \t\"folders\": [\n\
                \t\t{\n\
                \t\t\t\"path\": \"flume\"\n\
                \t\t}\n\
                \t]\n\
                }\n";
    fs::write(&file, mine).expect("a comment of the user's");
    let config = asking(&["agentplot/flume", "agentplot/sluice"]);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
    let named = of_workspaces(&lines);
    assert_eq!(named.len(), 1, "{lines:#?}");
    let prefix = format!("{}it is not JSON the runner reads: ", left(&file));
    assert!(named[0].starts_with(&prefix), "{lines:#?}");
    assert!(!named[0].contains("mine"), "{lines:#?}");

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(lines, [named[0].clone()], "named at every pass");
    assert_eq!(fs::read_to_string(&file).expect("the workspace"), mine);
}

#[test]
fn a_link_at_the_workspaces_path() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let elsewhere = scratch.root.join("elsewhere/mine.code-workspace");
    let theirs = "{\"folders\": [{\"path\": \"..\"}]}\n";
    fs::create_dir_all(elsewhere.parent().expect("its folder")).expect("a folder elsewhere");
    fs::write(&elsewhere, theirs).expect("a workspace elsewhere");
    let file = workspace(&scratch, "agentplot");
    fs::create_dir_all(file.parent().expect("the owner's folder")).expect("the owner's folder");
    std::os::unix::fs::symlink(&elsewhere, &file).expect("a link at the workspace's path");

    let (outcome, lines) = scratch.pass(&asking(&["agentplot/flume"]));

    assert_eq!(outcome, Outcome::Kept);
    assert_eq!(
        of_workspaces(&lines),
        [&format!("{}it is a link", left(&file))]
    );
    assert!(fs::symlink_metadata(&file)
        .expect("the link")
        .file_type()
        .is_symlink());
    assert_eq!(fs::read_link(&file).expect("the link"), elsewhere);
    assert_eq!(fs::read_to_string(&elsewhere).expect("its target"), theirs);
}

#[test]
fn a_workspace_that_cannot_be_written() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = asking(&["agentplot/flume"]);
    scratch.pass(&config);
    let file = workspace(&scratch, "agentplot");
    let edited = "{\"folders\": [], \"settings\": {}}\n";
    fs::write(&file, edited).expect("the folders edited");
    let owner = file.parent().expect("the owner's folder");
    let _mode = Mode::set(owner, 0o555);

    let (outcome, lines) = scratch.pass(&config);

    assert_eq!(outcome, Outcome::Failed);
    assert_eq!(lines.len(), 1, "{lines:#?}");
    let prefix = format!("boomdrive: workspace not written at {}: ", file.display());
    assert!(lines[0].starts_with(&prefix), "{lines:#?}");
    assert_eq!(fs::read_to_string(&file).expect("the workspace"), edited);
    let mut held: Vec<String> = fs::read_dir(owner)
        .expect("the owner's folder")
        .map(|entry| {
            entry
                .expect("an entry")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    held.sort();
    assert_eq!(held, ["agentplot.code-workspace", "flume"]);
}

#[test]
fn a_stopped_pass_writes_no_workspace() {
    let scratch = Scratch::new();
    scratch.origin("agentplot/flume");
    let config = boomdrive::config::parse(
        &asking(&["agentplot/flume"]),
        &scratch.config_file(),
        &scratch.home,
    )
    .expect("a configuration");
    let mut lines = Vec::new();

    boomdrive::pass::pass(
        &scratch.runner_git(),
        &scratch.data(),
        &config,
        &AtomicBool::new(true),
        &mut |line| lines.push(line),
    );

    assert_eq!(lines, [] as [String; 0]);
    assert_eq!(workspaces_under(&scratch.home), [] as [PathBuf; 0]);
}
