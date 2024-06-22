use core::slice::from_raw_parts;

use super::{ImageFileHeader, ImageOptionalHeader, ImageSectionHeaders};

#[repr(C)]
#[derive(Debug)]
pub struct ImageNtHeaders {
    signature: u32,
    file_header: ImageFileHeader,
    optional_header: ImageOptionalHeader,
}

impl ImageNtHeaders {
    pub fn optional_header(self: &Self) -> &ImageOptionalHeader {
        &self.optional_header
    }

    pub fn section_headers(self: &Self) -> &[ImageSectionHeaders] {
        let optional_header = self.optional_header() as *const ImageOptionalHeader;
        unsafe {
            let section_headers = optional_header
                .byte_offset(self.file_header.size_of_optional_header().clone() as isize)
                as *const ImageSectionHeaders;
            from_raw_parts(
                section_headers,
                self.file_header.number_of_sections().clone().into(),
            )
        }
    }
}
