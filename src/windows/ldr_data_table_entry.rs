use core::{
    ffi::{c_char, c_void, CStr},
    ptr::null,
    slice::from_raw_parts,
};

use super::{
    list_entry::ListEntryIterator, ImageDataDirectory, ImageDirectoryEntry, ImageDosHeader,
    ImageExportDirectory, ImageNtHeaders, ImageOptionalHeader, ImageSectionHeaders, ListEntry, Peb,
    UnicodeString,
};

#[repr(C)]
#[derive(Debug)]
pub struct LdrDataTableEntry {
    in_load_order_links: ListEntry,
    in_memory_order_links: ListEntry,
    in_initialization_order_links: ListEntry,
    dll_base: *const c_void,
    reserved3: [*const c_void; 2],
    full_dll_name: UnicodeString,
    base_dll_name: UnicodeString,
    reserved5: [*const c_void; 2],
    check_sum: u32,
    time_date_stamp: u32,
}

impl LdrDataTableEntry {
    pub fn list(self: &Self) -> &ListEntry {
        &self.in_load_order_links
    }

    #[allow(dead_code)]
    pub fn full_dll_name(self: &Self) -> &[u16] {
        self.full_dll_name.utf16()
    }

    pub fn base_dll_name(self: &Self) -> &[u16] {
        self.base_dll_name.utf16()
    }

    pub fn dll_base(self: &Self) -> *const c_void {
        self.dll_base
    }

    pub fn dos_header(self: &Self) -> &ImageDosHeader {
        let dos_header = self.dll_base as *const ImageDosHeader;
        unsafe { &*dos_header }
    }

    pub fn nt_headers(self: &Self) -> &ImageNtHeaders {
        self.dos_header().nt_headers()
    }

    #[allow(dead_code)]
    pub fn optional_header(self: &Self) -> &ImageOptionalHeader {
        self.nt_headers().optional_header()
    }

    pub fn section_headers(self: &Self) -> &[ImageSectionHeaders] {
        self.nt_headers().section_headers()
    }

    #[allow(dead_code)]
    pub fn code_section(self: &Self) -> Option<&[u8]> {
        for section in self.section_headers() {
            if !section.is_code_section() {
                continue;
            }
            return Some(unsafe {
                from_raw_parts(
                    self.dll_base
                        .byte_offset(section.virtual_address().clone() as isize)
                        as *const u8,
                    section.virtual_size().clone() as usize,
                )
            });
        }
        None
    }

    #[allow(dead_code)]
    pub fn vmt_section(self: &Self) -> Option<&[u8]> {
        for section in self.section_headers() {
            if !section.is_vmt_section() {
                continue;
            }
            return Some(unsafe {
                from_raw_parts(
                    self.dll_base
                        .byte_offset(section.virtual_address().clone() as isize)
                        as *const u8,
                    section.virtual_size().clone() as usize,
                )
            });
        }
        None
    }

    #[allow(dead_code)]
    pub fn data_section(self: &Self) -> Option<&[u8]> {
        for section in self.section_headers() {
            if !section.is_data_section() {
                continue;
            }
            return Some(unsafe {
                from_raw_parts(
                    self.dll_base
                        .byte_offset(section.virtual_address().clone() as isize)
                        as *const u8,
                    section.virtual_size().clone() as usize,
                )
            });
        }
        None
    }

    pub fn data_directory(self: &Self, directory: ImageDirectoryEntry) -> &ImageDataDirectory {
        self.optional_header().data_directory(directory)
    }

    #[allow(dead_code)]
    pub fn export_directory(self: &Self) -> &ImageExportDirectory {
        let virtual_address = self
            .data_directory(ImageDirectoryEntry::Export)
            .virtual_address()
            .clone();
        unsafe {
            &*(self.dll_base.byte_offset(virtual_address as isize) as *const ImageExportDirectory)
        }
    }

    pub fn export_directory_from(
        self: &Self,
        data_directory: &ImageDataDirectory,
    ) -> &ImageExportDirectory {
        let virtual_address = data_directory.virtual_address().clone();
        unsafe {
            &*(self.dll_base.byte_offset(virtual_address as isize) as *const ImageExportDirectory)
        }
    }

    pub fn has_same_base_dll_name(self: &Self, name: &str) -> bool {
        let base_dll_name = self.base_dll_name();
        let mut base_dll_name_iter = base_dll_name.iter();
        let mut name_iter = name.as_bytes().iter();
        let mut a = base_dll_name_iter.next();
        let mut b = name_iter.next();
        loop {
            if a.is_none() || b.is_none() {
                break;
            }
            let x = (unsafe { a.unwrap_unchecked() }.clone() as u8).to_ascii_lowercase();
            let y = unsafe { b.unwrap_unchecked() }.to_ascii_lowercase();
            if x.ne(&y) {
                return false;
            }
            a = base_dll_name_iter.next();
            b = name_iter.next();
        }
        true
    }

    pub fn iter() -> ListEntryIterator {
        Peb::current().ldr().list().into_iter()
    }

    #[allow(dead_code)]
    pub fn find(name: &str) -> Option<&LdrDataTableEntry> {
        let mut iter = LdrDataTableEntry::iter();
        loop {
            let entry = iter.next();
            match entry {
                Some(entry) => {
                    if entry.has_same_base_dll_name(name) {
                        return Some(entry);
                    }
                }
                None => break,
            };
        }
        None
    }

    pub fn export_function(self: &Self, name: &str) -> *const c_void {
        let data_directory = self.data_directory(ImageDirectoryEntry::Export);
        let export_directory = self.export_directory_from(data_directory);

        for export_name_index in 0..export_directory.number_of_names().clone() {
            if let Ok(export_name) = export_directory.name(self.dll_base, &export_name_index) {
                if export_name.ne(name) {
                    continue;
                }

                let export_function_rva =
                    export_directory.function_rva(self.dll_base, &export_name_index);
                let export_function =
                    unsafe { self.dll_base.byte_offset(export_function_rva as isize) };

                if data_directory.is_forwarded_export(export_function_rva) {
                    if let Ok(forwarded_export) =
                        unsafe { CStr::from_ptr(export_function as *const c_char) }.to_str()
                    {
                        return LdrDataTableEntry::resolve_forwarded_export(forwarded_export);
                    }
                }

                return export_function;
            }
        }

        null()
    }

    fn resolve_forwarded_export(forwarded_export: &str) -> *const c_void {
        let dot_pos = forwarded_export
            .rfind('.')
            .unwrap_or(forwarded_export.len());
        let entry_name = &forwarded_export[..dot_pos];
        let export_name = &forwarded_export[dot_pos + 1..];
        match LdrDataTableEntry::find(entry_name) {
            Some(entry) => entry.export_function(export_name),
            None => {
                for entry in LdrDataTableEntry::iter() {
                    let non_forwarded_export = entry.non_forwarded_export_function(export_name);
                    if !non_forwarded_export.is_null() {
                        return non_forwarded_export;
                    }
                }
                null()
            }
        }
    }

    fn non_forwarded_export_function(self: &Self, name: &str) -> *const c_void {
        let data_directory = self.data_directory(ImageDirectoryEntry::Export);
        let export_directory = self.export_directory_from(data_directory);

        for export_name_index in 0..export_directory.number_of_names().clone() {
            if let Ok(export_name) = export_directory.name(self.dll_base, &export_name_index) {
                let export_function_rva =
                    export_directory.function_rva(self.dll_base, &export_name_index);

                if data_directory.is_forwarded_export(export_function_rva) {
                    continue;
                }
                if export_name.ne(name) {
                    continue;
                }

                return unsafe { self.dll_base.byte_offset(export_function_rva as isize) };
            }
        }

        null()
    }
}
