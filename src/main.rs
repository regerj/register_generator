use core::panic;
// Stdlib
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Read;
use std::io::Write;

// Clap
use clap::Args;
use clap::Parser;
use clap::Subcommand;

// My crate
use crate::register_file_generator::register::*;
use crate::register_file_generator::file_generator::*;
mod register_file_generator;

// CLI parsing structs
#[derive(Subcommand)]
enum Commands {
    /// Add a register to an existing JSON configuration file
    AddRegister(AddRegisterArgs),
    /// Generate new header files
    Generate(GenerateArgs),
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct AddRegisterArgs {
    /// Path to the configuration JSON
    #[arg(short, long)]
    path: String,
    /// Name of the new register
    #[arg(short, long)]
    name: String,
    /// Size of the new register (must be supported by register family)
    #[arg(short, long)]
    size: u8,
}

#[derive(Args)]
struct GenerateArgs {
    /// Path to the configuration JSON
    #[arg(short, long)]
    path: String,
}

fn pull_existing_json(path: String) -> RegisterFamily {
    let mut file = File::open(path).expect("Couldn't open the input file!");

    // Read in the string as json
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).expect("Failed to read input file as string!");
    return serde_json::from_str(&json_string).expect("Failed to interpret JSON!");
}

fn add_register_field(register: &mut Register) {
    print!("Name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line!");
    let name = name.trim().to_string();

    print!("LSB: ");
    io::stdout().flush().unwrap();
    let mut lsb = String::new();
    io::stdin().read_line(&mut lsb).expect("Failed to read line!");
    let lsb: u8 = lsb.trim().parse().expect("Invalid input!");

    print!("MSB: ");
    io::stdout().flush().unwrap();
    let mut msb = String::new();
    io::stdin().read_line(&mut msb).expect("Failed to read line!");
    let msb: u8 = msb.trim().parse().expect("Invalid input!");

    print!("Read: ");
    io::stdout().flush().unwrap();
    let mut read = String::new();
    io::stdin().read_line(&mut read).expect("Failed to read line!");
    let read: bool = read.trim().parse().expect("Invalid input!");

    print!("Write: ");
    io::stdout().flush().unwrap();
    let mut write = String::new();
    io::stdin().read_line(&mut write).expect("Failed to read line!");
    let write: bool = write.trim().parse().expect("Invalid input!");

    print!("Negative: ");
    io::stdout().flush().unwrap();
    let mut negative = String::new();
    io::stdin().read_line(&mut negative).expect("Failed to read line!");
    let negative: bool = negative.trim().parse().expect("Invalid input!");

    register.fields.push(Field {name, lsb, msb, read, write, negative: Some(negative)});
}

fn add_register_handler(args: AddRegisterArgs) {
    let mut register_family = pull_existing_json(args.path.clone());

    let mut register = Register {
        name: args.name,
        size: args.size,
        fields: vec![],
    };

    loop {
        print!("Would you like to add a(nother) field to this register (y/n): ");
        io::stdout().flush().unwrap();
        let mut response = String::new();

        io::stdin().read_line(&mut response).expect("Failed to read line!");
        response = response.trim().to_string();

        match response.as_str() {
            "y" => {
                add_register_field(&mut register);
            },
            "n" => {
                break;
            },
            _ => {
                println!("Invalid input!");
                continue;
            }
        }
    }

    register_family.registers.push(register);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(false)
        .open(args.path.clone())
        .unwrap();

    match file.write_all(serde_json::to_string_pretty(&register_family).unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => panic!("Couldn't write to {}: {}", args.path, why)
    }
}

fn generate_handler(args: GenerateArgs) {
    let register_family = pull_existing_json(args.path);
    // Generate the files
    generate_files(&register_family);
}

fn main() {
    // Get user input and dispatch
    let cli_input = Cli::parse();
    match cli_input.command {
        Commands::AddRegister(args) => {
            add_register_handler(args);
        },
        Commands::Generate(args) => {
            generate_handler(args);
        }
    }
}

