//! The scratch host every test runs on (design 11): a temporary directory
//! holding the home, a git configuration and a forge standing in for
//! GitHub. Every git call a test makes, and every one the runner makes in
//! it, runs under its environment, so no test reads the user's git
//! configuration, reaches the network or holds a credential (core.4).

#![allow(dead_code)]

use flywheel::git::Git;
use flywheel::pass::Outcome;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// A configuration keeping `addresses`, in order, with every other
/// setting its default.
pub fn keeping(addresses: &[&str]) -> String {
    let entries: Vec<String> = addresses
        .iter()
        .map(|address| format!("{address:?}"))
        .collect();
    format!("repositories = [{}]\n", entries.join(", "))
}

/// A scratch host, removed when dropped.
pub struct Scratch {
    _dir: TempDir,
    /// The temporary directory, as git names it.
    pub root: PathBuf,
    /// `HOME`. The views root of a test is inside it.
    pub home: PathBuf,
    /// The git configuration, `GIT_CONFIG_GLOBAL`.
    pub gitconfig: PathBuf,
    /// The forge, `<owner>/<name>.git` for each repository on it.
    pub forge: PathBuf,
}

impl Scratch {
    pub fn new() -> Scratch {
        let dir = tempfile::Builder::new()
            .prefix("flywheel-")
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
                 \tname = flywheel tests\n\
                 \temail = tests@flywheel.invalid\n\
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
        Scratch {
            _dir: dir,
            root,
            home,
            gitconfig,
            forge,
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

    /// The runner's binary under the scratch environment.
    pub fn flywheel(&self) -> Command {
        self.command(env!("CARGO_BIN_EXE_flywheel"))
    }

    /// git as the runner runs it in-process on the scratch host.
    pub fn runner_git(&self) -> Git {
        Git::new(self.overlay())
    }

    /// The runner's configuration file where it reads it by default,
    /// under `XDG_CONFIG_HOME`.
    pub fn config_file(&self) -> PathBuf {
        self.home.join(".config/flywheel/config.toml")
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

    /// The runner's data directory, under `XDG_DATA_HOME`.
    pub fn data(&self) -> PathBuf {
        self.home.join(".local/share/flywheel")
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

    /// One pass in-process over the configuration `text`, read as the
    /// default file: its outcome and the lines it said.
    pub fn pass(&self, text: &str) -> (Outcome, Vec<String>) {
        let config = flywheel::config::parse(text, &self.config_file(), &self.home)
            .expect("a configuration");
        let mut lines = Vec::new();
        let outcome =
            flywheel::pass::pass(&self.runner_git(), &self.data(), &config, &mut |line| {
                lines.push(line)
            });
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
