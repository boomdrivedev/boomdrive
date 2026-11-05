//! An inbox the control plane names (design Decision 14; c-inbox.a):
//! weir's `inbox/` at the root of a tip of a branch of a repository the
//! runner keeps, read from the clone through git, which fetches nothing,
//! and checked whole by weir-signals. It is carried as references: each
//! record file by its path with its text as the tip holds it, and each
//! item's brief by its digest, since the copy of an item names its record
//! and not its brief. An inbox holds no raw material, and nothing a
//! capture's pointer names is read (core.11).

use crate::git::{At, Git, Words};
use boomdrive_control::InboxFile;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;
use weir_signals::layout::INBOX;
use weir_signals::{InboxAt, Place};

/// An inbox's branch as one control pass reads it in the clone.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Read {
    /// origin holds no branch of its name, as the last fetch left the
    /// clone.
    NoBranch,
    /// Its tip is the one last carried, and nothing of it is read again.
    Unmoved,
    /// The files under `inbox/` at `tip`, which weir takes.
    Taken {
        tip: String,
        files: BTreeMap<String, InboxFile>,
    },
    /// The tip `tip`, whose files weir refuses, with weir's words.
    Refused { tip: String, words: String },
}

/// The inbox on `branch` in the bare clone at `bare`: the tip of
/// `refs/remotes/origin/<branch>`, which the keeper's fetch brings, and,
/// where it is not `carried`, the tip last carried, its files under
/// `inbox/` read by `ls-tree` and `cat-file` and checked whole by weir.
pub fn read(git: &Git, bare: &Path, branch: &str, carried: Option<&str>) -> Result<Read, Words> {
    let at = At::GitDir(bare);
    let named = format!("refs/remotes/origin/{branch}");
    if git.ask(at, ["check-ref-format", &named])?.is_none() {
        return Ok(Read::NoBranch);
    }
    let commit = format!("{named}^{{commit}}");
    let Some(tip) = git.ask(at, ["rev-parse", "--verify", "--quiet", &commit])? else {
        return Ok(Read::NoBranch);
    };
    let tip = tip.trim().to_string();
    if carried == Some(tip.as_str()) {
        return Ok(Read::Unmoved);
    }
    let mut read = Vec::new();
    for (path, object) in blobs(&git.bytes(at, ["ls-tree", "-r", "-z", &tip, INBOX])?) {
        let Ok(path) = String::from_utf8(path) else {
            let words = "a path under `inbox/` is not UTF-8".to_string();
            return Ok(Read::Refused { tip, words });
        };
        let bytes = git.bytes(at, ["cat-file", "blob", &object])?;
        read.push((path, bytes));
    }
    let mut texts = Vec::new();
    for (path, bytes) in &read {
        match std::str::from_utf8(bytes) {
            Ok(text) => texts.push((path.as_str(), text)),
            Err(_) => {
                let words = format!("`{path}` is not UTF-8 text");
                return Ok(Read::Refused { tip, words });
            }
        }
    }
    if let Err(error) = InboxAt::read(texts.iter().copied()) {
        let words = error.to_string();
        return Ok(Read::Refused { tip, words });
    }
    let files = read
        .into_iter()
        .filter_map(|(path, bytes)| {
            let file = match Place::of(&path)? {
                Ok(Place::Brief(_)) => InboxFile::Digest(digest(&bytes)),
                Ok(_) => InboxFile::Text(String::from_utf8(bytes).ok()?),
                Err(_) => return None,
            };
            Some((path, file))
        })
        .collect();
    Ok(Read::Taken { tip, files })
}

/// `sha256:<hex>` of `bytes`.
pub fn digest(bytes: &[u8]) -> String {
    format!("sha256:{:x}", Sha256::digest(bytes))
}

/// The path and object of each blob `ls-tree -r -z` lists in `listed`,
/// each entry `<mode> <type> <object>\t<path>` ending in a NUL. A
/// submodule's commit is no file the tip holds, and is passed over.
fn blobs(listed: &[u8]) -> Vec<(Vec<u8>, String)> {
    listed
        .split(|byte| *byte == 0)
        .filter_map(|entry| {
            let tab = entry.iter().position(|byte| *byte == b'\t')?;
            let (head, path) = (&entry[..tab], &entry[tab + 1..]);
            let head = std::str::from_utf8(head).ok()?;
            let mut fields = head.split(' ');
            let (_mode, kind, object) = (fields.next()?, fields.next()?, fields.next()?);
            (kind == "blob").then(|| (path.to_vec(), object.to_string()))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_blob_listed() {
        let listed = b"100644 blob aaa\tinbox/moves/moves.rec\0\
                       160000 commit bbb\tinbox/vendored\0\
                       100644 blob ccc\tinbox/items/a b/brief.md\0";
        assert_eq!(
            blobs(listed),
            [
                (b"inbox/moves/moves.rec".to_vec(), "aaa".to_string()),
                (b"inbox/items/a b/brief.md".to_vec(), "ccc".to_string()),
            ]
        );
        assert_eq!(blobs(b""), []);
    }

    #[test]
    fn a_digest_of_bytes() {
        assert_eq!(
            digest(b""),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }
}
