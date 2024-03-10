use clap::{Parser, Subcommand};

mod commands;
mod exec;
mod project;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Run { name: String },
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let project = project::load()?;

    match cli.command {
        Some(Commands::Run { name }) => {
            commands::run::exec(&project, name)
            // read the config file
        }
        None => {
            if let Some(name) = cli.name {
                return commands::run::exec(&project, name);
            }

            return Err("No command specified")?;
        }
    }
}
