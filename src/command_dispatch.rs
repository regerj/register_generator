use std::fs::OpenOptions;
use std::io;
use std::io::Write;

use crate::cli_handler::*;
use crate::register_file_generator::register::*;
use crate::register_file_generator::file_generator::*;

pub fn add_register_handler(args: AddRegisterArgs) {
    let mut register_family = pull_existing_json(args.path.clone());

    if !register_family.register_family_widths.contains(&args.size) {
        register_family.register_family_widths.push(args.size);
    }

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
        .truncate(true)
        .open(args.path.clone())
        .unwrap();

    match file.write_all(serde_json::to_string_pretty(&register_family).unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => panic!("Couldn't write to {}: {}", args.path, why)
    }
}

pub fn generate_handler(args: GenerateArgs) {
    let register_family = pull_existing_json(args.path);
    // Generate the files
    generate_files(&register_family);
}

pub fn bootstrap_handler(args: BootstrapArgs) {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(args.path.clone())
        .unwrap();

    let reg_family = RegisterFamily {
        register_family: args.name.clone(),
        register_family_widths: Vec::new(),
        registers: Vec::new(),
    };

    match file.write_all(serde_json::to_string_pretty(&reg_family).unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => panic!("Couldn't write to {}: {}", args.path, why)
    }
}
