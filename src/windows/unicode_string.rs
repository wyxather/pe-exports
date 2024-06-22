use core::slice::from_raw_parts;

#[repr(C)]
#[derive(Debug)]
pub struct UnicodeString {
    length: u16,
    maximum_length: u16,
    buffer: *const u16,
}

impl UnicodeString {
    pub fn utf16(self: &Self) -> &[u16] {
        unsafe { from_raw_parts(self.buffer, (self.length / 2) as usize) }
    }
}
