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

