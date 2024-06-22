use core::{
    ffi::{c_char, c_void, CStr},
    str::Utf8Error,
};

#[repr(C)]
#[derive(Debug)]
pub struct ImageExportDirectory {
    characteristics: u32,
    time_date_stamp: u32,
    major_version: u16,
    minor_version: u16,
    name: u32,
    base: u32,
    number_of_functions: u32,
    number_of_names: u32,
    address_of_functions: u32,
    address_of_names: u32,
    address_of_name_ordinals: u32,
}

impl ImageExportDirectory {
    pub fn number_of_names(self: &Self) -> &u32 {
        &self.number_of_names
    }

    pub fn address_of_functions(self: &Self, dll_base: *const c_void) -> *const u32 {
        unsafe { dll_base.byte_offset(self.address_of_functions as isize) as *const u32 }
    }

    pub fn address_of_names(self: &Self, dll_base: *const c_void) -> *const u32 {
        unsafe { dll_base.byte_offset(self.address_of_names as isize) as *const u32 }
    }

    pub fn address_of_name_ordinals(self: &Self, dll_base: *const c_void) -> *const u16 {
        unsafe { dll_base.byte_offset(self.address_of_name_ordinals as isize) as *const u16 }
    }

    pub fn name_rva(self: &Self, dll_base: *const c_void, index: &u32) -> u32 {
        let address_of_names = self.address_of_names(dll_base);
        unsafe { *address_of_names.offset(index.clone() as isize) }
    }

    pub fn name(self: &Self, dll_base: *const c_void, index: &u32) -> Result<&str, Utf8Error> {
        let name_rva = self.name_rva(dll_base, index);
        unsafe { CStr::from_ptr(dll_base.byte_offset(name_rva as isize) as *const c_char) }.to_str()
    }

    pub fn name_ordinals(self: &Self, dll_base: *const c_void, index: &u32) -> u16 {
        let address_of_name_ordinals = self.address_of_name_ordinals(dll_base);
        unsafe { *address_of_name_ordinals.offset(index.clone() as isize) }
    }

    pub fn function_rva(self: &Self, dll_base: *const c_void, index: &u32) -> u32 {
        let name_ordinals = self.name_ordinals(dll_base, index);
        let address_of_functions = self.address_of_functions(dll_base);
        unsafe { *address_of_functions.offset(name_ordinals as isize) }
    }
}
