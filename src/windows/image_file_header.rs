#[repr(C)]
#[derive(Debug)]
pub struct ImageFileHeader {
    machine: ImageFileMachine,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: ImageFileCharacteristics,
}

#[repr(transparent)]
#[derive(Debug)]
struct ImageFileMachine(pub u16);

#[repr(transparent)]
#[derive(Debug)]
struct ImageFileCharacteristics(pub u16);

impl ImageFileHeader {
    pub fn number_of_sections(self: &Self) -> &u16 {
        &self.number_of_sections
    }

    pub fn size_of_optional_header(self: &Self) -> &u16 {
        &self.size_of_optional_header
    }
}
