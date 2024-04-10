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
    let json_object = json::parse(&json_string).expect("Invalid JSON object!");
    
    // Grab the family name and widths
    let register_family = String::from(json_object["register_family"].as_str().expect("Invalid register family!").to_string());
    let mut register_family_widths: Vec<u8> = Vec::new();
    for width in json_object["register_family_widths"].members() {
        register_family_widths.push(width.as_u8().expect("Invalid register width!"));
    }

    // Grab the registers
    let json_register_array = &json_object["registers"];
    let mut registers: Vec<Register> = Vec::new();
    for json_register in json_register_array.members() {
        registers.push(Register {
            name: json_register["name"].as_str().expect("Invalid register name!").to_string(),
            _size: json_register["size"].as_u8().expect("Invalid register size!"),
            fields: Vec::new() 
        });

        // Grab the fields
        for json_field in json_register["fields"].members() {
            registers.last_mut().expect("Somehow an empty register list?").fields.push(Field {
                name: json_field["name"].as_str().expect("Invalid field name!").to_string(),
                least_significant_bit: json_field["lsb"].as_u8().expect("Invalid lsb!"),
                most_significant_bit: json_field["msb"].as_u8().expect("Invalid msb!"),
                read: json_field["read"].as_bool().expect("Invalid read boolean!"),
                write: json_field["write"].as_bool().expect("Invalid write boolean!"),
                negative: match json_field["negative"].as_bool() {
                    Some(set) => set,
                    None => false,
                },
            });
        }
    }

    create_base_register_files(&register_family_widths, &register_family);
    // Write the files with the data
    for register in registers {
        write_register_to_file(&register, &register_family);
    }
}

