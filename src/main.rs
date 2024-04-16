mod register_file_generator;
mod cli_handler;
mod command_dispatch;

// Clap
use clap::Parser;

// My crates
use crate::cli_handler::*;
use crate::command_dispatch::*;

fn main() {
    // Get user input and dispatch
    let cli_input = Cli::parse();
    match cli_input.command {
        Commands::AddRegister(args) => {
            add_register_handler(args);
        },
        Commands::Generate(args) => {
            generate_handler(args);
        },
        Commands::Bootstrap(args) => {
            bootstrap_handler(args);
        }
    }
}

