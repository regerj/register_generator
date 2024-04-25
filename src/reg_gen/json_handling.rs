use std::{fs::File, io::Read};

use crate::reg_gen::register::*;

pub fn pull_existing_json(path: &String) -> RegisterFamily {
    let mut file = File::open(path).expect("Couldn't open the input file!");

    // Read in the string as json
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).expect("Failed to read input file as string!");
    return serde_json::from_str(&json_string).expect("Failed to interpret JSON!");
}
