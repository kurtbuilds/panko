use clap::{Parser, Subcommand, ValueEnum, Args};

mod command;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Output json representing the keys that are shared be
    Shared(command::Shared),
    /// Subtract all keys from the second, leaving only the keys in the first one.
    Subtract(command::Subtract),
    /// Create a struct from a json file.
    Struct(command::Struct),
}

pub fn has_stdin() -> bool {
    !atty::is(atty::Stream::Stdin)
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Command::Shared(shared) => shared.run(),
        Command::Subtract(subtract) => subtract.run(),
        Command::Struct(rust) => rust.run(),
    }
}