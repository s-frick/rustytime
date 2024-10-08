mod cli;
mod rtime;
mod settings;

use clap::Parser;

use crate::cli::Cli;
use settings::Settings;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let settings = Settings::new().unwrap();
    let rtime = rtime::RTime::new(settings);

    match Cli::parse().cmd {
        // TODO: make commands separate structs
        cli::Commands::Start { at, tags } => rtime.start(cli::Commands::Start { at, tags }),
        cli::Commands::Stop { at } => rtime.stop(at),
        cli::Commands::Status => rtime.status(),
        cli::Commands::Log { format, from, to } => rtime.log(format, from, to),
        cli::Commands::Version => println!("RustyTime v{}", VERSION),
    }
}
