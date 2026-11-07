//! Each drive's VS Code workspace at the repository root (core.43; record
//! 0017 as revised; design Decision 1): `<repository_root>/<drive>.code-workspace`,
//! whose `folders` are exactly the drive's kept repositories and whose
//! other members are kept as they were. A file the runner cannot read as
//! a JSON object, a link or a folder at its path is left as it is and
//! named, never with the file's text (core.11).

use crate::config::Config;
use serde::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::{json, Serializer, Value};
use std::fs::{self, OpenOptions};
use std::io::{self, Write as _};
use std::path::{Path, PathBuf};

/// A drive's workspace as the configuration wants it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wanted {
    /// The drive, as the file names it.
    pub drive: String,
    /// The workspace's file, `<repository_root>/<drive>.code-workspace`.
    pub file: PathBuf,
    /// The path of each of the drive's kept repositories' view from the
    /// repository root, `<owner>/<name>` as the file writes it, in the
    /// file's order.
    pub folders: Vec<String>,
}

/// What stands at a workspace's path, read without following a link.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stands {
    /// Nothing.
    Missing,
    /// A regular file, holding these bytes.
    File(Vec<u8>),
    /// A symbolic link, wherever it points.
    Link,
    /// A folder, or anything else that is not a regular file.
    Other,
}

/// What becomes of a workspace.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Act {
    /// Nothing is done or said: its folders are the wanted ones.
    Leave,
    /// The file is written to hold this text.
    Write(String),
    /// What stands is left as it is, and named with what is wrong.
    Name(String),
}

/// Each drive of `config`, in the file's order, with its workspace's file
/// at the repository root and its folders: each of its repositories by
/// the path of its view from the root, `<owner>/<name>` as the file
/// writes it, where `config::view_path` puts it. A drive keeping none has
/// no folders.
pub fn wanted(config: &Config) -> Vec<Wanted> {
    config
        .drives()
        .map(|drive| Wanted {
            drive: drive.name.clone(),
            file: file(config, &drive.name),
            folders: drive
                .repositories
                .iter()
                .map(|address| format!("{}/{}", address.owner(), address.name()))
                .collect(),
        })
        .collect()
}

/// The workspace of the drive `drive` at `config`'s repository root,
/// `<repository_root>/<drive>.code-workspace`.
pub fn file(config: &Config, drive: &str) -> PathBuf {
    config.repository_root.join(file_name(drive))
}

/// The workspace file's name for `name`, `<name>.code-workspace`.
pub fn file_name(name: &str) -> String {
    format!("{name}.code-workspace")
}

/// What stands at `path`, read with `symlink_metadata`, so a link is seen
/// as a link.
pub fn stands(path: &Path) -> io::Result<Stands> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Stands::Missing),
        Err(error) => return Err(error),
    };
    let kind = metadata.file_type();
    if kind.is_symlink() {
        Ok(Stands::Link)
    } else if kind.is_file() {
        fs::read(path).map(Stands::File)
    } else {
        Ok(Stands::Other)
    }
}

/// Design Decisions 4 and 5, for what `stands` at a workspace's path and
/// the `folders` wanted there. Nothing there is made, `folders` and an
/// empty `settings`. A JSON object whose `folders` equal the wanted ones,
/// as JSON values, is left; any other has its `folders` set where they
/// stand, or added first where it has none, and every other member kept
/// in its order. Anything else is named with what is wrong.
pub fn decide(stands: &Stands, folders: &[String]) -> Act {
    let wanted = Value::Array(
        folders
            .iter()
            .map(|folder| json!({ "path": folder }))
            .collect(),
    );
    match stands {
        Stands::Missing => Act::Write(text(&json!({ "folders": wanted, "settings": {} }))),
        Stands::Link => Act::Name("it is a link".to_string()),
        Stands::Other => Act::Name("it is not a file".to_string()),
        Stands::File(bytes) => match serde_json::from_slice::<Value>(bytes) {
            Err(error) => Act::Name(format!("it is not JSON the runner reads: {error}")),
            Ok(Value::Object(mut members)) => {
                match members.get_mut("folders") {
                    Some(folders) if *folders == wanted => return Act::Leave,
                    Some(folders) => *folders = wanted,
                    None => {
                        members.shift_insert(0, "folders".to_string(), wanted);
                    }
                }
                Act::Write(text(&Value::Object(members)))
            }
            Ok(_) => Act::Name("it is not a JSON object".to_string()),
        },
    }
}

/// `value` as VS Code writes a workspace: indented with tabs, with a
/// newline at the end.
fn text(value: &Value) -> String {
    let mut text = Vec::new();
    let mut serializer = Serializer::with_formatter(&mut text, PrettyFormatter::with_indent(b"\t"));
    // A JSON value's keys are strings and it is written to memory, so
    // neither its writing nor its UTF-8 can fail.
    value
        .serialize(&mut serializer)
        .expect("a JSON value is written to memory");
    text.push(b'\n');
    String::from_utf8(text).expect("serde_json writes UTF-8")
}

/// The partial file a workspace at `path` is written to before it is
/// renamed over it, `.<file name>.partial` beside it.
pub fn partial(path: &Path) -> PathBuf {
    let name = path
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default();
    path.with_file_name(format!(".{name}.partial"))
}

/// Writes `text` as the workspace at `path`, whole, to its partial file,
/// then renames that over it, so VS Code never reads half a file. The
/// folder is made where it is missing, a partial file an earlier write
/// left is written over, and an old file's permissions are set on the
/// new one. Where it fails, the partial file is removed.
pub fn write(path: &Path, text: &str) -> io::Result<()> {
    let partial = partial(path);
    let written = put(path, &partial, text);
    if written.is_err() {
        let _ = fs::remove_file(&partial);
    }
    written
}

/// `write`'s steps, `partial` its partial file.
fn put(path: &Path, partial: &Path, text: &str) -> io::Result<()> {
    if let Some(folder) = path.parent() {
        fs::create_dir_all(folder)?;
    }
    match fs::remove_file(partial) {
        Err(error) if error.kind() != io::ErrorKind::NotFound => return Err(error),
        _ => {}
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(partial)?;
    file.write_all(text.as_bytes())?;
    drop(file);
    if let Ok(old) = fs::symlink_metadata(path) {
        if old.is_file() {
            fs::set_permissions(partial, old.permissions())?;
        }
    }
    fs::rename(partial, path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    fn config(text: &str) -> Config {
        crate::config::parse(text, Path::new("/c/config.toml"), Path::new("/h")).unwrap()
    }

    fn folders(paths: &[&str]) -> Vec<String> {
        paths.iter().map(|path| path.to_string()).collect()
    }

    /// The file `text` holds, as `decide` reads it.
    fn file(text: &str) -> Stands {
        Stands::File(text.as_bytes().to_vec())
    }

    /// The JSON `decide` wrote, read back.
    fn written(act: Act) -> Value {
        match act {
            Act::Write(text) => serde_json::from_str(&text).unwrap(),
            other => panic!("not written: {other:?}"),
        }
    }

    /// The names of `value`'s members, in its order.
    fn members(value: &Value) -> Vec<&str> {
        value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect()
    }

    /// Two drives sharing the owner `agentplot`: each lists its own
    /// repositories, in the file's order, by their paths from the root.
    #[test]
    fn two_drives_sharing_an_owner() {
        let config = config(
            "repository_root = \"/h/Code\"\n\
             [accounts.own.drives.home]\n\
             repositories = [\"agentplot/flume\", \"afterthought/crew\"]\n\
             [accounts.own.drives.work]\n\
             repositories = [\"agentplot/sluice\"]\n",
        );
        assert_eq!(
            wanted(&config),
            [
                Wanted {
                    drive: "home".to_string(),
                    file: PathBuf::from("/h/Code/home.code-workspace"),
                    folders: folders(&["agentplot/flume", "afterthought/crew"]),
                },
                Wanted {
                    drive: "work".to_string(),
                    file: PathBuf::from("/h/Code/work.code-workspace"),
                    folders: folders(&["agentplot/sluice"]),
                },
            ]
        );
    }

    /// An owner written in two cases: each folder is its view's path as
    /// the file writes its address.
    #[test]
    fn an_owner_written_in_two_cases() {
        let config = config(
            "[accounts.own.drives.home]\n\
             repositories = [\"AgentPlot/flume\", \"agentplot/sluice\"]\n",
        );
        assert_eq!(
            wanted(&config),
            [Wanted {
                drive: "home".to_string(),
                file: PathBuf::from("/h/Code/home.code-workspace"),
                folders: folders(&["AgentPlot/flume", "agentplot/sluice"]),
            }]
        );
    }

    /// A drive keeping none has its workspace, with no folders; a host
    /// with no drive has none.
    #[test]
    fn a_drive_keeping_none() {
        let config = config("[accounts.client.drives.garden]\nrepositories = []\n");
        assert_eq!(
            wanted(&config),
            [Wanted {
                drive: "garden".to_string(),
                file: PathBuf::from("/h/Code/garden.code-workspace"),
                folders: vec![],
            }]
        );
        assert_eq!(wanted(&self::config("accounts = {}\n")), []);
    }

    #[test]
    fn nothing_there_is_made() {
        assert_eq!(
            decide(&Stands::Missing, &folders(&["flume", "sluice"])),
            Act::Write(
                "{\n\
                 \t\"folders\": [\n\
                 \t\t{\n\
                 \t\t\t\"path\": \"flume\"\n\
                 \t\t},\n\
                 \t\t{\n\
                 \t\t\t\"path\": \"sluice\"\n\
                 \t\t}\n\
                 \t],\n\
                 \t\"settings\": {}\n\
                 }\n"
                .to_string()
            )
        );
    }

    #[test]
    fn folders_already_wanted_are_left() {
        let stands = file("{\"folders\": [{\"path\": \"flume\"}], \"settings\": {}}");
        assert_eq!(decide(&stands, &folders(&["flume"])), Act::Leave);
    }

    #[test]
    fn folders_already_wanted_in_another_layout_are_left() {
        let stands = file(
            "{\n  \"settings\": {\"editor.tabSize\": 2},\n  \"folders\": [\n    { \"path\" : \"flume\" }\n  ]\n}\n",
        );
        assert_eq!(decide(&stands, &folders(&["flume"])), Act::Leave);
    }

    #[test]
    fn other_folders_are_set_and_the_rest_kept() {
        let stands = file(
            "{\"folders\": [{\"path\": \"flume\"}], \"settings\": {\"editor.tabSize\": 2}, \
             \"extensions\": {\"recommendations\": [\"a.b\"]}}",
        );
        let value = written(decide(&stands, &folders(&["flume", "sluice"])));
        assert_eq!(members(&value), ["folders", "settings", "extensions"]);
        assert_eq!(
            value,
            json!({
                "folders": [{"path": "flume"}, {"path": "sluice"}],
                "settings": {"editor.tabSize": 2},
                "extensions": {"recommendations": ["a.b"]},
            })
        );
    }

    #[test]
    fn folders_are_set_where_they_stand() {
        let stands = file("{\"settings\": {}, \"folders\": [], \"launch\": {}}");
        let value = written(decide(&stands, &folders(&["flume"])));
        assert_eq!(members(&value), ["settings", "folders", "launch"]);
        assert_eq!(value["folders"], json!([{"path": "flume"}]));
    }

    #[test]
    fn a_file_without_folders_gains_them_first() {
        let stands = file("{\"settings\": {\"editor.tabSize\": 2}}");
        let value = written(decide(&stands, &folders(&["flume"])));
        assert_eq!(members(&value), ["folders", "settings"]);
        assert_eq!(
            value,
            json!({"folders": [{"path": "flume"}], "settings": {"editor.tabSize": 2}})
        );
    }

    #[test]
    fn a_comment_is_named_and_never_quoted() {
        let stands = file("{\n\t// mine\n\t\"folders\": []\n}\n");
        let Act::Name(wrong) = decide(&stands, &folders(&["flume"])) else {
            panic!("a file with a comment is not named");
        };
        assert!(
            wrong.starts_with("it is not JSON the runner reads: "),
            "{wrong}"
        );
        assert!(wrong.contains("line 2 column"), "{wrong}");
        assert!(!wrong.contains("mine"), "{wrong}");
    }

    #[test]
    fn a_trailing_comma_is_named() {
        let stands = file("{\"folders\": [],}");
        let Act::Name(wrong) = decide(&stands, &folders(&["flume"])) else {
            panic!("a file with a trailing comma is not named");
        };
        assert!(
            wrong.starts_with("it is not JSON the runner reads: "),
            "{wrong}"
        );
    }

    #[test]
    fn json_that_is_not_an_object_is_named() {
        for text in ["[{\"path\": \"flume\"}]", "\"flume\"", "null"] {
            assert_eq!(
                decide(&file(text), &folders(&["flume"])),
                Act::Name("it is not a JSON object".to_string()),
                "{text}"
            );
        }
    }

    #[test]
    fn a_link_is_named() {
        assert_eq!(
            decide(&Stands::Link, &folders(&["flume"])),
            Act::Name("it is a link".to_string())
        );
    }

    #[test]
    fn a_folder_is_named() {
        assert_eq!(
            decide(&Stands::Other, &folders(&["flume"])),
            Act::Name("it is not a file".to_string())
        );
    }

    #[test]
    fn what_stands_is_read_without_following_a_link() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.code-workspace");
        assert_eq!(stands(&path).unwrap(), Stands::Missing);
        fs::create_dir(&path).unwrap();
        assert_eq!(stands(&path).unwrap(), Stands::Other);
        fs::remove_dir(&path).unwrap();
        let elsewhere = dir.path().join("elsewhere.json");
        fs::write(&elsewhere, "{}").unwrap();
        std::os::unix::fs::symlink(&elsewhere, &path).unwrap();
        assert_eq!(stands(&path).unwrap(), Stands::Link);
        fs::remove_file(&path).unwrap();
        fs::write(&path, "{}").unwrap();
        assert_eq!(stands(&path).unwrap(), file("{}"));
    }

    #[test]
    fn a_write_keeps_the_old_files_permissions_and_leaves_no_partial_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("o.code-workspace");
        fs::write(&path, "{}").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
        fs::write(partial(&path), "half a fi").unwrap();

        write(&path, "{\"folders\": []}\n").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "{\"folders\": []}\n");
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o640
        );
        assert!(!partial(&path).exists());
        let mut left: Vec<_> = fs::read_dir(dir.path())
            .unwrap()
            .map(|entry| entry.unwrap().file_name())
            .collect();
        left.sort();
        assert_eq!(left, ["o.code-workspace"]);
    }

    #[test]
    fn a_write_makes_the_repository_root() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Code/home.code-workspace");

        write(&path, "{}\n").unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), "{}\n");
        assert_eq!(
            partial(&path),
            dir.path().join("Code/.home.code-workspace.partial")
        );
        assert!(!partial(&path).exists());
    }
}
