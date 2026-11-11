//! `boomdrive`, the runner on each host: its command line over the
//! library (design 10). A usage error prints the usage on standard error
//! and exits 2.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

/// The runner on each host: keeps the clones of the repositories its
/// configuration names, and carries its control plane's work into Flume.
#[derive(Parser)]
#[command(name = "boomdrive", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Keep the configured repositories, a pass at once and then one on
    /// the timer, and carry the control plane's work on a timer of its
    /// own, until stopped.
    Run {
        /// The configuration file, in place of
        /// `$XDG_CONFIG_HOME/boomdrive/config.toml`.
        #[arg(long, value_name = "FILE")]
        config: Option<PathBuf>,
        /// Make one keeping pass, then one control pass where the
        /// configuration names a control plane, and exit: 0 where every
        /// git call succeeded and the control pass pulled and reported, 1
        /// otherwise.
        #[arg(long)]
        once: bool,
    },
    /// Enroll a partition of the configuration's control plane once, with
    /// its one-time token read from standard input, keeping the key made
    /// for it in the host's credential store.
    Enroll {
        /// The configuration file, in place of
        /// `$XDG_CONFIG_HOME/boomdrive/config.toml`.
        #[arg(long, value_name = "FILE")]
        config: Option<PathBuf>,
        /// Answer as one JSON object on standard output.
        #[arg(long)]
        json: bool,
        /// The partition's label, as its control-plane table names it.
        label: String,
    },
    /// Set up one drive of one account on this Mac, which Nix does not
    /// manage: its account's crew state repository made where none
    /// exists, the drive named in crew's file, the runner's file written,
    /// the set's herdr plugins linked and the runner loaded as an agent of
    /// the login. Each answer is a flag; a terminal is asked what no flag
    /// and nothing setup left on the host gives. Run again with the same
    /// answers, it changes nothing.
    Setup {
        /// The account the drive is of.
        #[arg(long, value_name = "NAME")]
        account: Option<String>,
        /// The drive's name.
        #[arg(long, value_name = "NAME")]
        drive: Option<String>,
        /// The account's crew state repository, made private where none
        /// exists.
        #[arg(long, value_name = "OWNER/NAME")]
        state_repository: Option<String>,
        /// A repository the drive keeps, once for each: the drive's whole
        /// list.
        #[arg(
            long = "repository",
            value_name = "OWNER/NAME",
            conflicts_with_all = ["add_repositories", "remove_repositories"]
        )]
        repositories: Vec<String>,
        /// A repository added after the drive's own, once for each.
        #[arg(long = "add-repository", value_name = "OWNER/NAME")]
        add_repositories: Vec<String>,
        /// A repository the drive no longer keeps, once for each.
        #[arg(long = "remove-repository", value_name = "OWNER/NAME")]
        remove_repositories: Vec<String>,
        /// The drive's blueprints repository, once for each, the first its
        /// default: one the drive keeps.
        #[arg(long, value_name = "OWNER/NAME")]
        blueprints: Vec<String>,
        /// How a role of crew's runs, `standing` or `on-demand`, once for
        /// each role changed.
        #[arg(long, num_args = 2, value_names = ["ROLE", "HOW"])]
        runs: Vec<String>,
        /// How many of the drive's changes, or how many hours, make a
        /// review's batch due.
        #[arg(
            long,
            num_args = 2,
            value_names = ["AFTER_CHANGES", "AFTER_HOURS"],
            action = clap::ArgAction::Set
        )]
        review: Option<Vec<String>>,
        /// Answer as one JSON object on standard output.
        #[arg(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Run { config, once } => boomdrive::run::main(config.as_deref(), once),
        Command::Enroll {
            config,
            json,
            label,
        } => boomdrive::enroll::main(config.as_deref(), json, &label),
        Command::Setup {
            account,
            drive,
            state_repository,
            repositories,
            add_repositories,
            remove_repositories,
            blueprints,
            runs,
            review,
            json,
        } => {
            let pairs = |words: Vec<String>| -> Vec<(String, String)> {
                words
                    .chunks(2)
                    .map(|pair| (pair[0].clone(), pair[1].clone()))
                    .collect()
            };
            let flags = boomdrive::setup::Flags {
                account,
                drive,
                state_repository,
                repositories,
                add_repositories,
                remove_repositories,
                blueprints,
                runs: pairs(runs),
                review: review.and_then(|words| pairs(words).into_iter().next()),
            };
            boomdrive::setup::main(&flags, json)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn the_command_line_is_well_formed() {
        Cli::command().debug_assert();
    }
}
