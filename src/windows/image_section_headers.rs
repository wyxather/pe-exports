use core::{ops::BitAnd, slice::from_raw_parts};

#[repr(C)]
#[derive(Debug)]
pub struct ImageSectionHeaders {
    name: [u8; 8],
    virtual_size: u32,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_linenumbers: u32,
    number_of_relocations: u16,
    number_of_linenumbers: u16,
    characteristics: u32,
}

enum ImageSection {
    MemExecute = 0x20000000,
    MemRead = 0x40000000,
}

impl ImageSectionHeaders {
    #[allow(dead_code)]
    pub fn name(self: &Self) -> &[u8; 8] {
        &self.name
    }

    pub fn virtual_address(self: &Self) -> &u32 {
        &self.virtual_address
    }

    pub fn virtual_size(self: &Self) -> &u32 {
        &self.virtual_size
    }

    pub fn is_executable(self: &Self) -> bool {
        self.characteristics
            .bitand(ImageSection::MemExecute as u32)
            .ne(&(0 as u32))
    }

    pub fn is_readable(self: &Self) -> bool {
        self.characteristics
            .bitand(ImageSection::MemRead as u32)
            .ne(&(0 as u32))
    }

    pub fn is_code_section(self: &Self) -> bool {
        let slice = unsafe { from_raw_parts(&self.name as *const u8, 5) };
        self.is_executable() && slice.eq(b".text")
    }

    pub fn is_vmt_section(self: &Self) -> bool {
        let slice = unsafe { from_raw_parts(&self.name as *const u8, 6) };
        self.is_readable() && slice.eq(b".rdata")
    }

    pub fn is_data_section(self: &Self) -> bool {
        let slice = unsafe { from_raw_parts(&self.name as *const u8, 5) };
        self.is_readable() && slice.eq(b".data")
    }
}
