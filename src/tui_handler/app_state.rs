use std::{fs::OpenOptions, io::Write};

use crate::reg_gen::{register::RegisterFamily, json_handling::pull_existing_json};

use super::{tui_root_handler::BasePage, home_page::HomePageState};

pub struct App {
    pub original_path: String,
    pub register_family: RegisterFamily,
    pub register_index: usize,
    pub register_info_index: usize,
    pub field_index: usize,
    pub field_info_index: usize,
    pub state: BasePage,
    pub input: String,
}

impl App {
    pub fn new(path: String) -> App {
        App {
            original_path: path.clone(),
            register_family: pull_existing_json(&path),
            register_index: 0,
            register_info_index: 0,
            field_index: 0,
            field_info_index: 0,
            state: BasePage::Home(HomePageState::SelectRegisterAndField),
            input: String::new(),
        }
    }

    pub fn next_register(&mut self) {
        // Reset field index
        self.field_index = 0;
        self.field_info_index = 0;

        self.register_index = (self.register_index + 1) % self.register_family.registers.len();
    }

    pub fn previous_register(&mut self) {
        // Reset field index
        self.field_index = 0;
        self.field_info_index = 0;

        if self.register_index > 0 {
            self.register_index -= 1;
        } else {
            self.register_index = self.register_family.registers.len() - 1;
        }
    }

    pub fn next_field(&mut self) {
        self.field_info_index = 0;

        self.field_index = (self.field_index + 1) % self.register_family.registers[self.register_index].fields.len();
    }

    pub fn previous_field(&mut self) {
        self.field_info_index = 0;

        if self.field_index > 0 {
            self.field_index -= 1;
        } else {
            self.field_index = self.register_family.registers[self.register_index].fields.len() - 1;
        }
    }

    pub fn next_field_info(&mut self) {
        // Mod 5 because there are 5 field info elements
        self.field_info_index = (self.field_info_index + 1) % 5;
    }

    pub fn previous_field_info(&mut self) {
        if self.field_info_index > 0 {
            self.field_info_index -= 1;
        } else {
            // 4 because there are 5 field info elements
            self.field_info_index = 4;
        }
    }

    pub fn next_register_info(&mut self) {
        // Mod 2 because there are 2 info elements for a register
        self.register_info_index = (self.register_info_index + 1) % 2;
    }

    pub fn previous_register_info(&mut self) {
        if self.register_info_index > 0 {
            self.register_info_index -= 1;
        } else {
            self.register_info_index = 1;
        }
    }

    pub fn set_field_info(&mut self) {
        let field = &mut self.register_family.registers[self.register_index].fields[self.field_index];
        match self.field_info_index {
            0 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.lsb = x;
                }
            },
            1 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.msb = x;
                }
            },
            2 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.read = x;
                }
            },
            3 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.write = x;
                }
            },
            4 => {
                if let Ok(x) = self.input.trim().parse() {
                    field.negative = Some(x);
                }
            },
            _ => (),
        }
    }

    pub fn write_to_file(&mut self) -> Result<(), std::io::Error> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .truncate(true)
            .open(self.original_path.clone())
            .expect("Could not open JSON file!");

        return match file.write_all(serde_json::to_string_pretty(&self.register_family).unwrap().as_bytes()) {
            Ok(_) => Ok(()),
            Err(why) => Err(std::io::Error::new(why.kind(), format!("Couldn't write to {}: {}", self.original_path, why))),
        };
    }

    pub fn get_selected_field_as_string(&self) -> (String, String) {
        let field = &self.register_family.registers[self.register_index].fields[self.field_index];
        match self.field_info_index {
            0 => (String::from("LSB"), field.lsb.to_string()),
            1 => (String::from("MSB"), field.msb.to_string()),
            2 => (String::from("Read"), field.read.to_string()),
            3 => (String::from("Write"), field.write.to_string()),
            4 => (String::from("Negative"), match field.negative { Some(x) => { x.to_string() }, None => { false.to_string() } }),
            _ => (String::from("ERROR"), String::from("ERROR")),
        }
    }
}
