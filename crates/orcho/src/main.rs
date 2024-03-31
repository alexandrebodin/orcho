use clap::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let cli = orcho::Cli::parse();

    match orcho::run(cli) {
        Ok(_) => Ok(()),
        Err(err) => {
            eprintln!("Error: {:#?}", err);
            std::process::exit(1);
        }
    }
}
