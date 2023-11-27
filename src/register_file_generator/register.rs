pub struct Register {
    pub name: String,
    pub _size: u8,
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: String,
    pub least_significant_bit: u8,
    pub most_significant_bit: u8,
    pub read: bool,
    pub write: bool,
}

impl Field {
    pub fn create_get_method(&self, register_width: u8) -> String {
        format!(
            "\t\tinline uint{3}_t get_{0}() const {{\n\
            \t\t\tuint{3}_t buffer = register_raw >> {1};\n\
            \t\t\treturn buffer & (UINT{3}_MAX >> ({3} - 1 - ({2} - {1})));\n\
            \t\t}}\n",
            self.name,
            self.least_significant_bit,
            self.most_significant_bit,
            register_width
        )
    }
    
    pub fn create_set_method(&self, register_width: u8) -> String {
        format!(
            "\t\tinline bool set_{0}(uint{3}_t value) {{\n\
            \t\t\tif (value >= (1 << ({2} - {1}))) {{\n\
            \t\t\t\treturn false;\n\
            \t\t\t}}\n\
            \t\t\tuint{3}_t mask = static_cast<uint{3}_t>(~((UINT{3}_MAX >> ({3} - 1 - ({2} - {1}))) << {1}));\n\
            \t\t\tregister_raw &= mask;\n\
            \t\t\tvalue = value << {1};\n\
            \t\t\tregister_raw |= value;\n\
            \t\t\treturn true;\n\
            \t\t}}\n",
            self.name,
            self.least_significant_bit,
            self.most_significant_bit,
            register_width
        )
    }
}
