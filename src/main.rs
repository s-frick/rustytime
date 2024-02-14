use clap::Parser;

use crate::cli::Cli;

mod cli;
mod rtime;

fn main() {
    match Cli::parse().cmd {
        // TODO: make commands separate structs
        cli::Commands::Start { at, tags } => rtime::start(cli::Commands::Start { at, tags }),
        cli::Commands::Stop { at } => rtime::stop(at),
        cli::Commands::Status => rtime::status(),
        cli::Commands::Log { format } => println!("{:?}", format),
    }
}
