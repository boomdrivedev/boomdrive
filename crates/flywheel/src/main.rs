//! `flywheel`, the runner on each host: its command line over the
//! library (design 10). A usage error prints the usage on standard error
//! and exits 2.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

/// The runner on each host: keeps the clones of the repositories its
/// configuration names.
#[derive(Parser)]
#[command(name = "flywheel", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Keep the configured repositories, a pass at once and then one on
    /// the timer, until stopped.
    Run {
        /// The configuration file, in place of
        /// `$XDG_CONFIG_HOME/flywheel/config.toml`.
        #[arg(long, value_name = "FILE")]
        config: Option<PathBuf>,
        /// Make one pass and exit: 0 where every git call succeeded, 1
        /// where one failed.
        #[arg(long)]
        once: bool,
    },
}

fn main() -> ExitCode {
    match Cli::parse().command {
        Command::Run { config, once } => flywheel::run::main(config.as_deref(), once),
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
