use std::{io::{self, Write}, collections::HashSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Field {
    pub name: String,
    pub lsb: u8,
    pub msb: u8,
    pub read: bool,
    pub write: bool,
    pub negative: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Register {
    pub name: String,
    pub size: u8,
    pub fields: Vec<Field>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterFamily {
    pub register_family: String,
    pub register_family_widths: Vec<u8>,
    pub registers: Vec<Register>
}

impl Register {
    // Simple function to check if a register is valid
    pub fn is_valid(&self) -> Result<(), String> {
        if self.name.len() == 0 {
            return Err(String::from("No name provided"));
        }

        let supported_register_widths: HashSet<u8> = HashSet::from([8, 16, 32, 64]);
        if !supported_register_widths.contains(&self.size) {
            return Err(String::from("No or invalid register width"));
        }

        Ok(())
    }

    pub fn add_register_field(&mut self) {
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

        self.fields.push(Field {name, lsb, msb, read, write, negative: Some(negative)});
    }
}

impl Field {
    pub fn create_get_method(&self, register_width: u8) -> String {
        return match self.negative {
            Some(true) => format!(
                "\tinline int{3}_t get_{0}() const {{\n\
                \t\tuint{3}_t buffer = register_raw >> {1};\n\
                \t\tuint{3}_t field_raw = buffer & (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
                \t\tif (field_raw & (1 << ({2} - {1}))) {{\n\
                \t\t\tfield_raw |= (UINT{3}_MAX << ({2} - {1} + 1));\n\
                \t\t}}\n\
                \t\treturn field_raw;\n\
                \t}}\n",
                self.name,
                self.lsb,
                self.msb,
                register_width
            ),
            _ => format!(
                "\tinline uint{3}_t get_{0}() const {{\n\
                \t\tuint{3}_t buffer = register_raw >> {1};\n\
                \t\treturn buffer & (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
                \t}}\n",
                self.name,
                self.lsb,
                self.msb,
                register_width
            )
        }
    }

    pub fn create_set_method(&self, register_width: u8) -> String {
        // Negative numbers need to be bounds checked differently
        return match self.negative {
            Some(true) => format!(
                "\tinline bool set_{0}(int{3}_t value) {{\n\
                \t\tif (value < 0) {{\n\
                \t\t\tif (-value > ((int{3}_t)1 << ({2} - {1}))) {{\n\
                \t\t\t\treturn false;\n\
                \t\t\t}}\n\
                \t\t}} else {{\n\
                \t\t\tif (value >= ((int{3}_t)1 << ({2} - {1}))) {{\n\
                \t\t\t\treturn false;\n\
                \t\t\t}}\n\
                \t\t}}\n\
                \t\tuint{3}_t mask = static_cast<uint{3}_t>(~((UINT{3}_MAX >> ({3} - 1 - ({2} - {1}))) << {1}));\n\
                \t\tregister_raw &= mask;\n\
                \t\tvalue &= (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
                \t\tvalue = value << {1};\n\
                \t\tregister_raw |= value;\n\
                \t\treturn true;\n\
                \t}}\n",
                self.name,
                self.lsb,
                self.msb,
                register_width
            ),
            _ => format!(
                "\tinline bool set_{0}(uint{3}_t value) {{\n\
                \t\tif (value >= ((uint{3}_t)1 << ({2} - ({1} - 1)))) {{\n\
                \t\t\treturn false;\n\
                \t\t}}\n\
                \t\tuint{3}_t mask = static_cast<uint{3}_t>(~((UINT{3}_MAX >> ({3} - 1 - ({2} - {1}))) << {1}));\n\
                \t\tregister_raw &= mask;\n\
                \t\tvalue = value << {1};\n\
                \t\tregister_raw |= value;\n\
                \t\treturn true;\n\
                \t}}\n",
                self.name,
                self.lsb,
                self.msb,
                register_width
            )
        }
    }
}
