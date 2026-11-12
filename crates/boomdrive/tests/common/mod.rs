//! The scratch host every test runs on (design 11): a temporary directory
//! holding the home, a git configuration, a forge standing in for GitHub,
//! and a stub `flume` that answers the runner's settle. Every git call a
//! test makes, and every one the runner makes in it, runs under its
//! environment, so no test reads the user's git configuration, reaches
//! the network or holds a credential (core.4), and every run of the
//! runner's binary finds the stub first on its `PATH`, so none settles a
//! Flume of the host's (record 0029).

#![allow(dead_code)]

use boomdrive::forge::Forge;
use boomdrive::git::Git;
use boomdrive::locks::Locks;
use boomdrive::pass::Outcome;
use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tempfile::TempDir;

/// A configuration keeping `addresses`, in order, in the account `own`'s
/// drive `home`, with every other setting its default. Its keys are
/// dotted, so a key of the top level may be written before it and a table
/// after it.
pub fn keeping(addresses: &[&str]) -> String {
    drives(&[("home", addresses)])
}

/// A configuration keeping each of `drives` of the account `own`, in
/// order, each with its addresses in order, and every other setting its
/// default; with none, the account keeps no drive. Its keys are dotted,
/// as `keeping`'s are.
pub fn drives(drives: &[(&str, &[&str])]) -> String {
    if drives.is_empty() {
        return "accounts.own.drives = {}\n".to_string();
    }
    drives
        .iter()
        .map(|(drive, addresses)| {
            let entries: Vec<String> = addresses
                .iter()
                .map(|address| format!("{address:?}"))
                .collect();
            format!(
                "accounts.own.drives.{drive}.repositories = [{}]\n",
                entries.join(", ")
            )
        })
        .collect()
}

/// The scratch host's stub `flume`: it answers `--json settle` with Flume
/// 0.17.1's answer on a scratch host holding no place, `{fixture}`, and
/// exit 0, and anything else with nothing and exit 1.
const FLUME: &str = r#"#!/bin/sh
if [ "$1" = --json ] && [ "$2" = settle ]; then cat '{fixture}'; exit 0; fi
exit 1
"#;

/// A scratch host, removed when dropped.
pub struct Scratch {
    _dir: TempDir,
    /// The temporary directory, as git names it.
    pub root: PathBuf,
    /// `HOME`. The repository root of a test is inside it.
    pub home: PathBuf,
    /// The git configuration, `GIT_CONFIG_GLOBAL`.
    pub gitconfig: PathBuf,
    /// The forge, `<owner>/<name>.git` for each repository on it.
    pub forge: PathBuf,
    /// The folder of the stub `flume`, first on the binary's `PATH`.
    pub bin: PathBuf,
}

impl Scratch {
    pub fn new() -> Scratch {
        let dir = tempfile::Builder::new()
            .prefix("boomdrive-")
            .tempdir()
            .expect("a scratch directory");
        let root = dir
            .path()
            .canonicalize()
            .expect("the scratch directory's path");
        let home = root.join("home");
        let forge = root.join("forge");
        let gitconfig = root.join("gitconfig");
        std::fs::create_dir_all(&home).expect("the scratch home");
        std::fs::create_dir_all(&forge).expect("the scratch forge");
        // GitHub's https addresses are the forge's folders, and no other
        // transport is allowed, so nothing reaches the network.
        std::fs::write(
            &gitconfig,
            format!(
                "[user]\n\
                 \tname = boomdrive tests\n\
                 \temail = tests@boomdrive.invalid\n\
                 [url \"file://{}/\"]\n\
                 \tinsteadOf = https://github.com/\n\
                 [protocol]\n\
                 \tallow = never\n\
                 [protocol \"file\"]\n\
                 \tallow = always\n",
                forge.display()
            ),
        )
        .expect("the scratch git configuration");
        let bin = root.join("bin");
        std::fs::create_dir_all(&bin).expect("the stub's folder");
        let settled =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/flume-0.17.1/settled.json");
        let flume = bin.join("flume");
        std::fs::write(
            &flume,
            FLUME.replace("{fixture}", &settled.to_string_lossy()),
        )
        .expect("the stub flume");
        let mut permissions = std::fs::metadata(&flume)
            .expect("the stub's metadata")
            .permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        std::fs::set_permissions(&flume, permissions).expect("the stub made runnable");
        Scratch {
            _dir: dir,
            root,
            home,
            gitconfig,
            forge,
            bin,
        }
    }

    /// The scratch environment: what `git::Git` is given in-process, and
    /// what every command a test runs is given. git looks for a
    /// repository no higher than the scratch directory: the temporary
    /// directory can be inside one, sluice's inside this repository's.
    pub fn overlay(&self) -> Vec<(OsString, OsString)> {
        vec![
            ("GIT_CEILING_DIRECTORIES".into(), self.root.clone().into()),
            ("HOME".into(), self.home.clone().into()),
            ("XDG_CONFIG_HOME".into(), self.home.join(".config").into()),
            (
                "XDG_DATA_HOME".into(),
                self.home.join(".local/share").into(),
            ),
            ("GIT_CONFIG_GLOBAL".into(), self.gitconfig.clone().into()),
            ("GIT_CONFIG_NOSYSTEM".into(), "1".into()),
        ]
    }

    /// `program` under the scratch environment, run in the home: every
    /// `GIT_` and `XDG_` variable of the test's own removed, then the
    /// overlay.
    pub fn command(&self, program: impl AsRef<OsStr>) -> Command {
        let mut command = Command::new(program);
        for (name, _) in std::env::vars_os() {
            let name_text = name.to_string_lossy();
            if name_text.starts_with("GIT_") || name_text.starts_with("XDG_") {
                command.env_remove(&name);
            }
        }
        command.envs(self.overlay()).current_dir(&self.home);
        command
    }

    /// The runner's binary under the scratch environment, the stub
    /// `flume` first on its `PATH`, so its settle runs no Flume of the
    /// host's. A test that sets `PATH` itself puts its own stubs first.
    pub fn boomdrive(&self) -> Command {
        let mut path = OsString::from(&self.bin);
        if let Some(own) = std::env::var_os("PATH") {
            path.push(":");
            path.push(own);
        }
        let mut command = self.command(env!("CARGO_BIN_EXE_boomdrive"));
        command.env("PATH", path);
        command
    }

    /// git as the runner runs it in-process on the scratch host.
    pub fn runner_git(&self) -> Git {
        Git::new(self.overlay())
    }

    /// The runner's configuration file where it reads it by default,
    /// under `XDG_CONFIG_HOME`.
    pub fn config_file(&self) -> PathBuf {
        self.home.join(".config/boomdrive/config.toml")
    }

    /// Writes `text` as the runner's configuration file and gives its
    /// path.
    pub fn configure(&self, text: &str) -> PathBuf {
        let file = self.config_file();
        std::fs::create_dir_all(file.parent().expect("the file's folder"))
            .expect("the configuration's folder");
        std::fs::write(&file, text).expect("the configuration written");
        file
    }

    /// The runner's local layer, `config.local.toml` beside the default
    /// file, where it reads it whatever `--config` names.
    pub fn local_file(&self) -> PathBuf {
        self.home.join(".config/boomdrive/config.local.toml")
    }

    /// Writes `text` as the runner's local layer and gives its path.
    pub fn configure_local(&self, text: &str) -> PathBuf {
        let file = self.local_file();
        std::fs::create_dir_all(file.parent().expect("the file's folder"))
            .expect("the configuration's folder");
        std::fs::write(&file, text).expect("the local layer written");
        file
    }

    /// The runner's data directory, under `XDG_DATA_HOME`.
    pub fn data(&self) -> PathBuf {
        self.home.join(".local/share/boomdrive")
    }

    /// The container of `address`'s clone, `<data>/repositories/<owner>/<name>`.
    pub fn container(&self, address: &str) -> PathBuf {
        self.data().join("repositories").join(address)
    }

    /// The bare clone of `address`, `<container>/<name>.git`.
    pub fn clone_of(&self, address: &str) -> PathBuf {
        let name = address.rsplit('/').next().expect("a name");
        self.container(address).join(format!("{name}.git"))
    }

    /// Guards `branch` in the bare clone `bare` as sluice's protection
    /// guards a branch it keeps, by a stand-in `reference-transaction`
    /// hook (sluice's 124): it refuses every write that moves or deletes
    /// the branch where it stands, and, where `creations`, every write
    /// that makes it too. Each write it refuses is a line of the file it
    /// gives, `<old> <new> <ref>`, which it makes empty.
    pub fn guard(&self, bare: &Path, branch: &str, creations: bool) -> PathBuf {
        let refused = self.root.join(format!("refused-{branch}"));
        std::fs::write(&refused, "").expect("the guard's file");
        let stands = if creations {
            ""
        } else {
            "  git rev-parse --verify --quiet \"$name\" >/dev/null || continue\n"
        };
        let hook = bare.join("hooks/reference-transaction");
        std::fs::create_dir_all(hook.parent().expect("the hooks folder"))
            .expect("the hooks folder");
        std::fs::write(
            &hook,
            format!(
                "#!/bin/sh\n\
                 [ \"$1\" = prepared ] || exit 0\n\
                 refused=0\n\
                 while read -r old new name; do\n\
                 \x20 [ \"$name\" = refs/heads/{branch} ] || continue\n\
                 {stands}\
                 \x20 echo \"$old $new $name\" >> '{}'\n\
                 \x20 refused=1\n\
                 done\n\
                 [ $refused = 0 ] || echo 'refs/heads/{branch} is guarded' >&2\n\
                 exit $refused\n",
                refused.display()
            ),
        )
        .expect("the hook written");
        let mut permissions = std::fs::metadata(&hook)
            .expect("the hook's metadata")
            .permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut permissions, 0o755);
        std::fs::set_permissions(&hook, permissions).expect("the hook made runnable");
        refused
    }

    /// Makes `address`'s origin silent (record 0042's design Decision 6):
    /// a listener on the loopback address of the test's own, which takes
    /// every connection on a thread of its own and holds it open without a
    /// byte, and GitHub's https address of `address` rewritten to it over
    /// http, which git prefers to the forge's shorter rewrite. Nothing
    /// reaches the network (core.4).
    pub fn silent(&self, address: &str) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a silent listener");
        let port = listener.local_addr().expect("its address").port();
        std::thread::spawn(move || {
            let mut held = Vec::new();
            for stream in listener.incoming().flatten() {
                held.push(stream);
            }
        });
        let mut gitconfig = std::fs::OpenOptions::new()
            .append(true)
            .open(&self.gitconfig)
            .expect("the scratch git configuration");
        write!(
            gitconfig,
            "[url \"http://127.0.0.1:{port}/{address}\"]\n\
             \tinsteadOf = https://github.com/{address}\n\
             [protocol \"http\"]\n\
             \tallow = always\n"
        )
        .expect("the silent origin's rewrite");
    }

    /// One pass in-process over the configuration `text`, read as the
    /// default file: its outcome and the lines it said.
    pub fn pass(&self, text: &str) -> (Outcome, Vec<String>) {
        self.pass_with(&self.runner_git(), text)
    }

    /// One pass as [`Scratch::pass`] makes it, whose git stops a call to
    /// a remote at `limit` (record 0042's design Decision 6).
    pub fn pass_trying_for(&self, text: &str, limit: Duration) -> (Outcome, Vec<String>) {
        self.pass_with(&self.runner_git().trying_for(limit), text)
    }

    /// One pass as [`Scratch::pass`] makes it, each fetch given what
    /// `forge` gives under the configuration's `[forge]` (design Decision
    /// 9).
    pub fn pass_with_forge(&self, text: &str, forge: &Forge) -> (Outcome, Vec<String>) {
        self.pass_by(&self.runner_git(), forge, text)
    }

    /// One pass in-process by `git` over the configuration `text`, with a
    /// forge that calls GitHub's API, which a configuration naming no
    /// `[forge]` never asks.
    fn pass_with(&self, git: &Git, text: &str) -> (Outcome, Vec<String>) {
        self.pass_by(git, &Forge::new(), text)
    }

    /// One pass in-process by `git` over the configuration `text`, each
    /// fetch given what `forge` gives.
    fn pass_by(&self, git: &Git, forge: &Forge, text: &str) -> (Outcome, Vec<String>) {
        let config = boomdrive::config::parse(text, &self.config_file(), &self.home)
            .expect("a configuration");
        let mut lines = Vec::new();
        let outcome = boomdrive::pass::pass(
            git,
            forge,
            &self.data(),
            &config,
            &Locks::new(),
            &AtomicBool::new(false),
            &mut || false,
            &mut |line| lines.push(line),
        );
        (outcome, lines)
    }

    /// git's output from `args` in `dir`, trimmed; panics with git's words
    /// where it fails.
    pub fn git(&self, dir: &Path, args: &[&str]) -> String {
        let output = self
            .command("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .expect("git runs");
        assert!(
            output.status.success(),
            "git {} in {}: {}",
            args.join(" "),
            dir.display(),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    /// Whether git answers yes to `args` in `dir`.
    pub fn holds(&self, dir: &Path, args: &[&str]) -> bool {
        self.command("git")
            .arg("-C")
            .arg(dir)
            .args(args)
            .output()
            .expect("git runs")
            .status
            .success()
    }

    /// A repository on the forge at `address`, `owner/name`, with a first
    /// commit on `main`.
    pub fn origin(&self, address: &str) -> Origin<'_> {
        let origin = self.empty_origin(address);
        origin.commit("main", "README", "a scratch repository\n");
        origin
    }

    /// A repository on the forge at `address` with no commit, its HEAD
    /// naming `main`.
    pub fn empty_origin(&self, address: &str) -> Origin<'_> {
        let bare = self.forge.join(format!("{address}.git"));
        let work = self.root.join("work").join(address);
        for dir in [&bare, &work] {
            std::fs::create_dir_all(dir.parent().expect("an owner's folder"))
                .expect("the owner's folder");
        }
        self.git(
            &self.root,
            &[
                "init",
                "--quiet",
                "--bare",
                "--initial-branch=main",
                &bare.to_string_lossy(),
            ],
        );
        self.git(
            &self.root,
            &[
                "clone",
                "--quiet",
                &bare.to_string_lossy(),
                &work.to_string_lossy(),
            ],
        );
        Origin {
            scratch: self,
            address: address.to_string(),
            bare,
            work,
        }
    }
}

/// A repository on the scratch forge.
pub struct Origin<'a> {
    scratch: &'a Scratch,
    /// Its address, `owner/name`.
    pub address: String,
    /// The bare repository, `<forge>/<owner>/<name>.git`.
    pub bare: PathBuf,
    /// A clone outside the forge that the helpers commit in and push
    /// from, as someone pushing to GitHub would.
    work: PathBuf,
}

impl Origin<'_> {
    /// Its GitHub URL, which the scratch git configuration sends to the
    /// forge.
    pub fn url(&self) -> String {
        format!("https://github.com/{}.git", self.address)
    }

    /// Writes `text` at `file` in a commit on `branch`, from the branch's
    /// tip on the forge, else from `main`'s, else from nothing, pushes it,
    /// and gives the commit.
    pub fn commit(&self, branch: &str, file: &str, text: &str) -> String {
        let scratch = self.scratch;
        scratch.git(&self.work, &["fetch", "--quiet", "--prune", "origin"]);
        let from = [branch, "main"]
            .into_iter()
            .map(|from| format!("refs/remotes/origin/{from}"))
            .find(|from| scratch.holds(&self.work, &["rev-parse", "--verify", "--quiet", from]));
        match from {
            Some(from) => scratch.git(
                &self.work,
                &["switch", "--quiet", "--force-create", branch, &from],
            ),
            None => scratch.git(&self.work, &["switch", "--quiet", "--orphan", branch]),
        };
        let path = self.work.join(file);
        std::fs::create_dir_all(path.parent().expect("the file's folder"))
            .expect("the file's folder");
        std::fs::write(&path, text).expect("the file written");
        scratch.git(&self.work, &["add", "--all"]);
        scratch.git(
            &self.work,
            &["commit", "--quiet", "-m", &format!("{file} on {branch}")],
        );
        scratch.git(&self.work, &["push", "--quiet", "origin", branch]);
        scratch.git(&self.work, &["rev-parse", "HEAD"])
    }

    /// The commit `branch` names on the forge.
    pub fn tip(&self, branch: &str) -> String {
        self.scratch
            .git(&self.bare, &["rev-parse", &format!("refs/heads/{branch}")])
    }

    /// Deletes `branch` on the forge.
    pub fn delete(&self, branch: &str) {
        self.scratch.git(
            &self.bare,
            &["update-ref", "-d", &format!("refs/heads/{branch}")],
        );
    }

    /// Points the forge's HEAD, its default branch, at `branch`.
    pub fn point_head(&self, branch: &str) {
        self.scratch.git(
            &self.bare,
            &["symbolic-ref", "HEAD", &format!("refs/heads/{branch}")],
        );
    }
}
