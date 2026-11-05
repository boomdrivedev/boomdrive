//! A repository's clone (design 3 to 6): a bare repository in the
//! runner's data directory, in the bare-container layout Flume reads,
//! carrying the runner's record of it. It is made whole or not at all,
//! fetched over https and never by ssh, and its HEAD names origin's
//! default branch. Every branch is read at origin's tracking ref: the
//! runner moves no branch of the clone, whose own ref of a branch sluice
//! keeps is sluice's copy (core.14; sluice's 124).

use crate::config::Address;
use crate::git::{self, At, Git, Words};
use std::ffi::OsStr;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// The section of a clone's own git config that holds the runner's
/// records of it.
const SECTION: &str = "boomdrive";

/// The key in a clone's own git config that records the address it was
/// made for, as written then.
const ADDRESS: &str = "boomdrive.address";

/// Where every clone is kept: `<data>/repositories`.
pub fn repositories(data: &Path) -> PathBuf {
    data.join("repositories")
}

/// The container of `address`'s clone, `<data>/repositories/<owner>/<name>`:
/// the directory `flume launch --repository` names.
pub fn container(data: &Path, address: &Address) -> PathBuf {
    repositories(data)
        .join(address.owner())
        .join(address.name())
}

/// The bare repository in `container`, `<container>/<name>.git`, by the
/// container's own folder name.
pub fn bare(container: &Path) -> PathBuf {
    let mut name = container.file_name().unwrap_or_default().to_os_string();
    name.push(".git");
    container.join(name)
}

/// The folder `address`'s clone is made in before it is renamed into
/// place, `<data>/repositories/<owner>/.<name>.partial`.
fn partial(data: &Path, address: &Address) -> PathBuf {
    repositories(data)
        .join(address.owner())
        .join(format!(".{}.partial", address.name()))
}

/// Whether `name` is a folder the runner makes clones in: `.<name>.partial`.
pub fn is_partial(name: &OsStr) -> bool {
    name.to_str().is_some_and(|name| {
        name.strip_prefix('.')
            .and_then(|name| name.strip_suffix(".partial"))
            .is_some_and(|name| !name.is_empty())
    })
}

/// origin's URL: GitHub's https address, which the host's git may
/// rewrite (design 5).
pub fn url(address: &Address) -> String {
    format!("https://github.com/{address}.git")
}

/// Why a clone was not made or fetched, or its origin not read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Failure {
    /// The host's git would reach origin by ssh, at this URL, so nothing
    /// was fetched or read (design 5).
    Ssh(String),
    /// A git call failed, in git's words.
    Git(Words),
    /// The file system refused, in its words.
    Io(String),
}

impl fmt::Display for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Failure::Ssh(url) => write!(f, "git would fetch it by ssh, from {url}"),
            Failure::Git(words) => write!(f, "{words}"),
            Failure::Io(words) => f.write_str(words),
        }
    }
}

impl From<Words> for Failure {
    fn from(words: Words) -> Failure {
        Failure::Git(words)
    }
}

/// The runner's clones in `data`, each by the address it records, with
/// its container. A folder of `repositories/<owner>/` is the runner's
/// clone only when its bare repository records an address and that
/// address is the folder's own, ignoring case; any other folder is not
/// the runner's, a partial one among them (design 3). A clone made before
/// the rename has its records carried over first.
pub fn find(git: &Git, data: &Path) -> io::Result<Vec<(Address, PathBuf)>> {
    let mut clones = Vec::new();
    for owner in folders(&repositories(data))? {
        for container in folders(&owner)? {
            carry_over(git, &container).map_err(|words| {
                io::Error::other(format!(
                    "{}: its records from before the rename are not carried over: {words}",
                    container.display()
                ))
            })?;
            if let Some(address) = recorded(git, &container) {
                clones.push((address, container));
            }
        }
    }
    Ok(clones)
}

/// Carries over the records of a clone a runner made in `container`
/// before the rename, which name it `flywheel` (record 0016): its view's
/// commit, `refs/flywheel/view`, moves to the view's ref, then its
/// section, the address and the view's path, is renamed `boomdrive`. The
/// clone is then the runner's as it was, its view where it stands
/// (core.14, core.36). A folder whose bare repository records no address
/// under the old name, or one under the new name too, is left as it is.
pub fn carry_over(git: &Git, container: &Path) -> Result<(), Words> {
    let bare = bare(container);
    let at = At::GitDir(&bare);
    let before = crate::BEFORE_RENAME;
    let address = format!("{before}.address");
    if !matches!(
        git.ask(at, ["config", "--local", "--get", &address]),
        Ok(Some(_))
    ) || git
        .ask(at, ["config", "--local", "--get", ADDRESS])?
        .is_some()
    {
        return Ok(());
    }
    let view = format!("refs/{before}/view");
    if let Some(commit) = git.ask(at, ["rev-parse", "--verify", "--quiet", &view])? {
        git.run(at, ["update-ref", crate::view::COMMIT, commit.trim()])?;
        git.run(at, ["update-ref", "-d", &view])?;
    }
    git.run(
        at,
        ["config", "--local", "--rename-section", before, SECTION],
    )?;
    Ok(())
}

/// The folders in `dir`, by name, none where `dir` is missing. A link is
/// not followed.
fn folders(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut folders = Vec::new();
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            folders.push(entry.path());
        }
    }
    folders.sort();
    Ok(folders)
}

/// The address `container` records as the runner's clone, where it is
/// one: its bare repository's `boomdrive.address`, naming the container's
/// own owner and name.
pub fn recorded(git: &Git, container: &Path) -> Option<Address> {
    let said = git
        .ask(
            At::GitDir(&bare(container)),
            ["config", "--local", "--get", ADDRESS],
        )
        .ok()??;
    let address = Address::parse(said.trim()).ok()?;
    let is = |part: Option<&OsStr>, written: &str| {
        part.and_then(OsStr::to_str)
            .is_some_and(|part| part.eq_ignore_ascii_case(written))
    };
    let owner = container.parent().and_then(Path::file_name);
    (is(container.file_name(), address.name()) && is(owner, address.owner())).then_some(address)
}

/// Makes `address`'s clone in `data` and gives its container (design 4).
/// It is made in `.<name>.partial` beside the container, any such folder
/// left by an earlier run removed first, and renamed into place once its
/// first fetch is in. A step that fails removes the partial folder, and
/// the owner's folder where that leaves it empty, so a failed clone
/// leaves nothing in the data directory.
pub fn make(git: &Git, data: &Path, address: &Address) -> Result<PathBuf, Failure> {
    let container = container(data, address);
    let partial = partial(data, address);
    if recorded(git, &partial).is_some() {
        return Err(Failure::Io(format!(
            "{} is another repository's clone",
            partial.display()
        )));
    }
    if partial.symlink_metadata().is_ok() {
        fs::remove_dir_all(&partial).map_err(|error| io_failure(&partial, error))?;
    }
    let made = make_in(git, &partial, &container, address);
    if made.is_err() {
        let _ = fs::remove_dir_all(&partial);
        if let Some(owner) = partial.parent() {
            let _ = fs::remove_dir(owner);
        }
    }
    made.map(|()| container)
}

/// Design 4's steps in `partial`, then the rename to `container`.
fn make_in(git: &Git, partial: &Path, container: &Path, address: &Address) -> Result<(), Failure> {
    fs::create_dir_all(partial).map_err(|error| io_failure(partial, error))?;
    let name = format!("{}.git", address.name());
    git.run(At::Dir(partial), ["init", "--quiet", "--bare", &name])?;
    let bare = partial.join(&name);
    for (key, value) in [
        ("remote.origin.url", url(address)),
        (
            "remote.origin.fetch",
            "+refs/heads/*:refs/remotes/origin/*".to_string(),
        ),
        ("remote.origin.followRemoteHEAD", "always".to_string()),
        (ADDRESS, address.to_string()),
    ] {
        git.run(At::GitDir(&bare), ["config", key, &value])?;
    }
    fetch(git, &bare)?;
    if let Some(default) = default_of(git, &bare)? {
        settle_head(git, &bare, &default)?;
    }
    let gitfile = partial.join(".git");
    fs::write(&gitfile, format!("gitdir: ./{name}\n"))
        .map_err(|error| io_failure(&gitfile, error))?;
    if container.symlink_metadata().is_ok() {
        return Err(Failure::Io(format!(
            "{} appeared while the clone was made",
            container.display()
        )));
    }
    fs::rename(partial, container).map_err(|error| io_failure(container, error))
}

fn io_failure(path: &Path, error: io::Error) -> Failure {
    Failure::Io(format!("{}: {error}", path.display()))
}

/// Fetches `bare` from origin (design 5): `fetch --prune --quiet origin`,
/// which prunes the branches origin deleted and, by
/// `remote.origin.followRemoteHEAD`, keeps `origin/HEAD` at origin's
/// default. No fetch is run where the host's git would reach origin by
/// ssh.
pub fn fetch(git: &Git, bare: &Path) -> Result<(), Failure> {
    not_by_ssh(git, bare)?;
    git.run(At::GitDir(bare), ["fetch", "--prune", "--quiet", "origin"])?;
    Ok(())
}

/// The commit origin's `branch` stands at, as `ls-remote` reads it from
/// origin now, or none where origin holds no such branch (design Decision
/// 15). Like a fetch, it never reaches origin by ssh.
pub fn origin_commit(git: &Git, bare: &Path, branch: &str) -> Result<Option<String>, Failure> {
    not_by_ssh(git, bare)?;
    let name = format!("refs/heads/{branch}");
    let listed = git.run(At::GitDir(bare), ["ls-remote", "origin", &name])?;
    Ok(listed.lines().find_map(|line| {
        let (commit, named) = line.split_once('\t')?;
        (named == name).then(|| commit.to_string())
    }))
}

/// Refused where the host's git would reach `bare`'s origin by ssh.
fn not_by_ssh(git: &Git, bare: &Path) -> Result<(), Failure> {
    let url = git.run(At::GitDir(bare), ["ls-remote", "--get-url", "origin"])?;
    let url = url.trim();
    if git::is_ssh(url) {
        return Err(Failure::Ssh(url.to_string()));
    }
    Ok(())
}

/// origin's default branch, as the last fetch left `origin/HEAD` naming
/// it: none where origin names none, or names a branch the fetch did not
/// bring (design 6).
pub fn default_of(git: &Git, bare: &Path) -> Result<Option<String>, Words> {
    let Some(head) = git.ask(
        At::GitDir(bare),
        ["symbolic-ref", "--quiet", "refs/remotes/origin/HEAD"],
    )?
    else {
        return Ok(None);
    };
    let Some(branch) = head.trim().strip_prefix("refs/remotes/origin/") else {
        return Ok(None);
    };
    let brought = git
        .ask(
            At::GitDir(bare),
            [
                "rev-parse",
                "--verify",
                "--quiet",
                &format!("refs/remotes/origin/{branch}^{{commit}}"),
            ],
        )?
        .is_some();
    Ok(brought.then(|| branch.to_string()))
}

/// Points the clone's HEAD at `default`, origin's default branch, where it
/// names another, and creates that branch at origin's tip where the clone
/// lacks it, so HEAD names a commit (design 6). A branch the clone has is
/// never moved: a creation is the one write sluice's protection passes
/// (core.14). Says whether it did either.
pub fn settle_head(git: &Git, bare: &Path, default: &str) -> Result<bool, Words> {
    let branch = format!("refs/heads/{default}");
    let at = At::GitDir(bare);
    let lacked = git
        .ask(at, ["rev-parse", "--verify", "--quiet", &branch])?
        .is_none();
    if lacked {
        let tip = format!("refs/remotes/origin/{default}");
        git.run(at, ["update-ref", &branch, &tip, ""])?;
    }
    let head = git.ask(at, ["symbolic-ref", "--quiet", "HEAD"])?;
    let pointed = head.as_deref().map(str::trim) != Some(branch.as_str());
    if pointed {
        git.run(at, ["symbolic-ref", "HEAD", &branch])?;
    }
    Ok(lacked || pointed)
}

/// A worktree of a clone, as `worktree list --porcelain` names it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Worktree {
    /// Its folder.
    pub path: PathBuf,
    /// The branch it has checked out, `refs/heads/…`, where it is on one.
    pub branch: Option<String>,
    /// Whether it is the bare repository itself.
    pub bare: bool,
}

/// The worktrees of `bare`, the bare repository itself first.
pub fn worktrees(git: &Git, bare: &Path) -> Result<Vec<Worktree>, Words> {
    let listed = git.run(At::GitDir(bare), ["worktree", "list", "--porcelain", "-z"])?;
    Ok(read_worktrees(&listed))
}

/// The worktrees `worktree list --porcelain -z` lists in `listed`: each
/// field ends in NUL, and each worktree in an empty field.
fn read_worktrees(listed: &str) -> Vec<Worktree> {
    let mut worktrees = Vec::new();
    let mut fields = listed.split('\0').peekable();
    while fields.peek().is_some() {
        let mut worktree: Option<Worktree> = None;
        for field in fields.by_ref() {
            if field.is_empty() {
                break;
            }
            let (key, value) = field.split_once(' ').unwrap_or((field, ""));
            match (key, worktree.as_mut()) {
                ("worktree", _) => {
                    worktree = Some(Worktree {
                        path: PathBuf::from(value),
                        branch: None,
                        bare: false,
                    })
                }
                ("branch", Some(worktree)) => worktree.branch = Some(value.to_string()),
                ("bare", Some(worktree)) => worktree.bare = true,
                _ => {}
            }
        }
        worktrees.extend(worktree);
    }
    worktrees
}

#[cfg(test)]
mod tests {
    use super::*;

    fn address(text: &str) -> Address {
        Address::parse(text).unwrap()
    }

    #[test]
    fn a_clones_paths() {
        let data = Path::new("/d/boomdrive");
        let flume = address("agentplot/flume");
        let container = container(data, &flume);
        assert_eq!(
            container,
            Path::new("/d/boomdrive/repositories/agentplot/flume")
        );
        assert_eq!(
            bare(&container),
            Path::new("/d/boomdrive/repositories/agentplot/flume/flume.git")
        );
        assert_eq!(
            partial(data, &flume),
            Path::new("/d/boomdrive/repositories/agentplot/.flume.partial")
        );
        assert_eq!(url(&flume), "https://github.com/agentplot/flume.git");
    }

    #[test]
    fn partial_folders() {
        assert!(is_partial(OsStr::new(".flume.partial")));
        assert!(is_partial(OsStr::new("..github.partial")));
        for name in ["flume", ".partial", "..partial", ".github", "flume.partial"] {
            assert!(!is_partial(OsStr::new(name)), "{name}");
        }
    }

    #[test]
    fn worktrees_listed() {
        let listed = "worktree /d/r.git\0bare\0\0\
                      worktree /w/one\0HEAD 1a2b\0branch refs/heads/main\0\0\
                      worktree /w/two\0HEAD 3c4d\0detached\0prunable gone\0\0";
        assert_eq!(
            read_worktrees(listed),
            [
                Worktree {
                    path: PathBuf::from("/d/r.git"),
                    branch: None,
                    bare: true,
                },
                Worktree {
                    path: PathBuf::from("/w/one"),
                    branch: Some("refs/heads/main".to_string()),
                    bare: false,
                },
                Worktree {
                    path: PathBuf::from("/w/two"),
                    branch: None,
                    bare: false,
                },
            ]
        );
        assert_eq!(read_worktrees(""), []);
    }
}
