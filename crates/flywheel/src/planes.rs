//! The control planes this binary carries (design Decisions 1 and 4;
//! core.2): each implementation's word, as `[control_plane]`'s `type`
//! names it, against the function that opens it. The list is data, and
//! the one place outside an implementation that names one: every other
//! module reaches the control plane through `ControlPlane` alone.

use flywheel_control::ControlPlane;
use std::path::Path;

/// A control plane as the runner holds it, on its loop's one thread.
pub type Plane = Box<dyn ControlPlane + Send>;

/// Where an implementation says what it did and could not do, one line
/// each.
pub type Say = Box<dyn FnMut(String) + Send>;

/// Opens an implementation from its own table, `[control_plane.<type>]`,
/// of the configuration file `file`, keeping what it keeps under the
/// runner's data directory.
pub type Opener =
    fn(table: &toml::Table, file: &Path, data_dir: &Path, say: Say) -> Result<Plane, String>;

/// Each implementation this binary carries, by its word.
pub const PLANES: [(&str, Opener); 1] = [("crew", crew)];

fn crew(table: &toml::Table, file: &Path, data_dir: &Path, say: Say) -> Result<Plane, String> {
    Ok(Box::new(flywheel_crew::open(table, file, data_dir, say)?))
}

/// Opens the implementation whose word is `word` from its own table,
/// `table`, of the configuration file `file`, keeping what it keeps under
/// `data_dir`; or why it could not be opened. A word this binary does not
/// carry is refused, naming those it does.
pub fn open(
    word: &str,
    table: &toml::Table,
    file: &Path,
    data_dir: &Path,
    say: Say,
) -> Result<Plane, String> {
    let Some((_, opener)) = PLANES.iter().find(|(carried, _)| *carried == word) else {
        let carried: Vec<String> = PLANES.iter().map(|(word, _)| format!("`{word}`")).collect();
        return Err(format!(
            "{}: `type` is {word:?}, a control plane this binary does not carry: it carries {}",
            file.display(),
            carried.join(", ")
        ));
    };
    opener(table, file, data_dir, say)
}
