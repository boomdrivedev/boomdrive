//! The stand-in for Claude Code that the quick start's last step's test
//! runs as `claude` (design Decision 6 of the change
//! boomdrives-quick-start-takes-a-stranger-to-a-working-drive), built only
//! under the feature `stand-in`, as Flume's `flume-stand-in` is built
//! only under Flume's `tools`. Flume's printed command runs it in the
//! place. It runs the hooks Flume's plugin names in
//! `<--plugin-dir>/hooks/hooks.json` as its own children, as Claude Code
//! runs them, since Flume takes a stage's start only from a hook whose
//! parent runs the program the place's pool names: the exec form with no
//! shell, or the line by `sh -c` where the hook names no arguments, the
//! hook's input on standard input, and `CLAUDE_PLUGIN_ROOT`,
//! `CLAUDE_PROJECT_DIR` and `CLAUDE_CODE_SESSION_ID` over its own
//! environment. In place of a model it makes the stage's one change: it
//! runs SessionStart, then UserPromptSubmit with the prompt it was given,
//! writes [`CHANGE`] and commits it on the place's branch, then runs Stop
//! and SessionEnd, and exits 0. It reads no answer of a hook.

use serde_json::{json, Value};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// The file the stand-in writes and commits, and its text.
const CHANGE: (&str, &str) = ("change.txt", "the stand-in's change\n");

/// The session the stand-in runs, as Claude Code names one.
const SESSION: &str = "00000000-0000-4000-8000-000000000000";

fn main() {
    let mut plugin = None;
    let mut prompt = String::new();
    let mut words = std::env::args().skip(1);
    while let Some(word) = words.next() {
        match word.as_str() {
            "--plugin-dir" => plugin = words.next().map(PathBuf::from),
            "--model"
            | "--append-system-prompt"
            | "--append-system-prompt-file"
            | "--permission-mode"
            | "--settings"
            | "--effort"
            | "--resume" => {
                words.next();
            }
            flag if flag.starts_with("--") => {}
            given => prompt = given.to_string(),
        }
    }
    let cwd = std::env::current_dir().expect("the place");
    let session = Session::new(cwd, plugin);
    session.hook("SessionStart", json!({ "source": "startup" }));
    session.hook("UserPromptSubmit", json!({ "prompt": prompt }));
    session.change();
    session.hook(
        "Stop",
        json!({
            "stop_hook_active": false,
            "last_assistant_message": "Done.",
        }),
    );
    session.hook("SessionEnd", json!({ "reason": "prompt_input_exit" }));
}

/// The session in the place `cwd`, under the plugin at `plugin`.
struct Session {
    cwd: PathBuf,
    plugin: Option<PathBuf>,
    hooks: Value,
}

impl Session {
    fn new(cwd: PathBuf, plugin: Option<PathBuf>) -> Session {
        let hooks = plugin
            .as_deref()
            .and_then(|dir| std::fs::read_to_string(dir.join("hooks/hooks.json")).ok())
            .and_then(|text| serde_json::from_str::<Value>(&text).ok())
            .map(|file| file["hooks"].clone())
            .unwrap_or(Value::Null);
        Session { cwd, plugin, hooks }
    }

    /// Every command hook the plugin names for `event`, each run to its
    /// end with `input` and the session's common fields on its standard
    /// input.
    fn hook(&self, event: &str, mut input: Value) {
        let transcript = transcript(&self.cwd);
        let fields = input.as_object_mut().expect("an object");
        fields.insert("session_id".into(), json!(SESSION));
        fields.insert("transcript_path".into(), json!(transcript));
        fields.insert("cwd".into(), json!(self.cwd));
        fields.insert("hook_event_name".into(), json!(event));
        for group in self.hooks[event].as_array().into_iter().flatten() {
            for hook in group["hooks"].as_array().into_iter().flatten() {
                if hook["type"] == "command" {
                    self.command(hook, &input);
                }
            }
        }
    }

    /// One command hook, given `input`, run to its end.
    fn command(&self, hook: &Value, input: &Value) {
        let program = hook["command"].as_str().unwrap_or_default();
        let mut command = match hook["args"].as_array() {
            Some(args) => {
                let mut command = Command::new(program);
                command.args(args.iter().filter_map(Value::as_str));
                command
            }
            None => {
                let mut command = Command::new("sh");
                command.args(["-c", program]);
                command
            }
        };
        if let Some(root) = &self.plugin {
            command.env("CLAUDE_PLUGIN_ROOT", root);
        }
        let spawned = command
            .current_dir(&self.cwd)
            .env("CLAUDE_PROJECT_DIR", &self.cwd)
            .env("CLAUDE_CODE_SESSION_ID", SESSION)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .spawn();
        let mut child = match spawned {
            Ok(child) => child,
            Err(error) => {
                eprintln!("the hook {program} could not run: {error}");
                return;
            }
        };
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(input.to_string().as_bytes());
        }
        let _ = child.wait();
    }

    /// [`CHANGE`] written in the place and committed on its branch.
    fn change(&self) {
        let (file, text) = CHANGE;
        std::fs::write(self.cwd.join(file), text).expect("the change written");
        for args in [
            vec!["add", "--", file],
            vec!["commit", "--quiet", "-m", "fix: the stand-in's change"],
        ] {
            let status = Command::new("git")
                .args(&args)
                .current_dir(&self.cwd)
                .status()
                .expect("git runs");
            assert!(status.success(), "git {}", args.join(" "));
        }
    }
}

/// The session's transcript, where Claude Code keeps one for the place
/// `cwd`: under the configuration home's `projects`, in the folder its
/// path names with each `/` and `.` a `-`.
fn transcript(cwd: &Path) -> PathBuf {
    let home = std::env::var_os("CLAUDE_CONFIG_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".claude")
        });
    let folder: String = cwd
        .to_string_lossy()
        .chars()
        .map(|c| if c == '/' || c == '.' { '-' } else { c })
        .collect();
    home.join("projects")
        .join(folder)
        .join(format!("{SESSION}.jsonl"))
}
