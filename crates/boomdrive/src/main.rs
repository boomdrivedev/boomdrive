//! `boomdrive`, the runner on each host: its command line over the
//! library (design 10). A usage error prints the usage on standard error
//! and exits 2.

use boomdrive::forge_token::Operation;
use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{CommandFactory, Parser, Subcommand};
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
    /// Give a served repository's forge token in git's credential-helper
    /// protocol, minted from the Boomdrive App's key the configuration's
    /// `[forge]` names: `get` reads git's request on standard input and
    /// answers on standard output alone, and `erase` revokes nothing.
    ForgeToken {
        /// The configuration file, in place of
        /// `$XDG_CONFIG_HOME/boomdrive/config.toml`.
        #[arg(long, value_name = "FILE")]
        config: Option<PathBuf>,
        /// What git asks: `get` or `erase`.
        #[arg(value_enum)]
        operation: Operation,
    },
}

fn main() -> ExitCode {
    let cli = Cli::try_parse().unwrap_or_else(|error| with_usage(error).exit());
    match cli.command {
        Command::Run { config, once } => boomdrive::run::main(config.as_deref(), once),
        Command::Enroll {
            config,
            json,
            label,
        } => boomdrive::enroll::main(config.as_deref(), json, &label),
        Command::ForgeToken { config, operation } => {
            boomdrive::forge_token::main(config.as_deref(), operation)
        }
    }
}

/// clap's `error`, carrying the usage of the command named first where
/// clap gives none, as for a value that is none of an argument's, so every
/// usage error prints the usage: `forge-token store` among them.
fn with_usage(mut error: clap::Error) -> clap::Error {
    if error.kind() != ErrorKind::InvalidValue || error.get(ContextKind::Usage).is_some() {
        return error;
    }
    let mut cli = Cli::command();
    cli.build();
    let usage = match std::env::args_os().nth(1) {
        Some(named) => match cli.find_subcommand_mut(named) {
            Some(command) => command.render_usage(),
            None => cli.render_usage(),
        },
        None => cli.render_usage(),
    };
    error.insert(ContextKind::Usage, ContextValue::StyledStr(usage));
    error
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory as _;

    #[test]
    fn the_command_line_is_well_formed() {
        Cli::command().debug_assert();
    }
}
