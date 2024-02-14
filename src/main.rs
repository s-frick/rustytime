mod cli;
mod rtime;
mod settings;

use clap::Parser;

use crate::cli::Cli;
use settings::Settings;

fn main() {
    let settings = Settings::new().unwrap();
    // Print out our settings (as a HashMap)
    println!("\n{:?} \n\n-----------", settings);

    match Cli::parse().cmd {
        // TODO: make commands separate structs
        cli::Commands::Start { at, tags } => {
            rtime::start(cli::Commands::Start { at, tags }, settings)
        }
        cli::Commands::Stop { at } => rtime::stop(at, settings),
        cli::Commands::Status => rtime::status(settings),
        cli::Commands::Log { format } => rtime::log(format, settings),
    }
}
