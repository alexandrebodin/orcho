use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run(RunArgs),
}

#[derive(Args, Debug)]
pub struct RunArgs {
    name: String,
}

pub fn run(cli: Cli) {
    match cli.command {
        Commands::Run(args) => {
            println!("Running with args: {:?}", args);
        }
    }
}
