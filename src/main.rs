mod reg_gen;
mod cli_structs;
mod command_handlers;
mod tui_handler;

// Clap
use clap::Parser;

// My crates
use crate::cli_structs::*;
use crate::command_handlers::*;

fn main() -> Result<(), std::io::Error> {
    // Get user input and dispatch
    let cli_input = Cli::parse();
    return match cli_input.command {
        Commands::AddRegister(args) => add_register_handler(args),
        Commands::Generate(args) => generate_handler(args),
        Commands::Bootstrap(args) => bootstrap_handler(args),
        Commands::Tui(args) => tui_handler(args),
    }
}

