use clap::Parser;

fn main() {
    let args = orcho::Cli::parse();

    orcho::run(args);
}
