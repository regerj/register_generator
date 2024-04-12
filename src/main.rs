use std::fs::File;
use std::io::Read;

use clap::Parser;

use crate::register_file_generator::register::*;
use crate::register_file_generator::file_generator::*;

mod register_file_generator;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String
}

fn main() {
    // Open the file
    let args = Args::parse();
    let mut file = File::open(args.path).expect("Couldn't open the input file!");

    // Read in the string as json
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).expect("Failed to read input file as string!");
    let register_family: RegisterFamily = serde_json::from_str(&json_string).expect("Failed to interpret JSON!");

    // println!("register_family: {:?}", register_family);

    // Generate the files
    generate_files(&register_family);
}

