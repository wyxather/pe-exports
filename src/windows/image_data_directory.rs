#[repr(C)]
#[derive(Debug)]
pub struct ImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

impl ImageDataDirectory {
    pub fn virtual_address(self: &Self) -> &u32 {
        &self.virtual_address
    }

    pub fn is_forwarded_export(self: &Self, export_function_rva: u32) -> bool {
        export_function_rva.ge(self.virtual_address())
            && export_function_rva.le(&(self.virtual_address + self.size))
    }
}
