//! `flywheel`, the runner on each host: its command line over the
//! library (design 10). A usage error prints the usage on standard error
//! and exits 2.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::ExitCode;

/// The runner on each host: keeps the clones of the repositories its
/// configuration names, and carries its control plane's work into Flume.
#[derive(Parser)]
#[command(name = "flywheel", version)]
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
        /// `$XDG_CONFIG_HOME/flywheel/config.toml`.
        #[arg(long, value_name = "FILE")]
        config: Option<PathBuf>,
        /// Make one keeping pass, then one control pass where the
        /// configuration names a control plane, and exit: 0 where every
        /// git call succeeded and the control pass pulled and reported, 1
        /// otherwise.
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
