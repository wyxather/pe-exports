use super::ImageNtHeaders;

#[repr(C)]
#[derive(Debug)]
pub struct ImageDosHeader {
    e_magic: u16,
    e_cblp: u16,
    e_cp: u16,
    e_crlc: u16,
    e_cparhdr: u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss: u16,
    e_sp: u16,
    e_csum: u16,
    e_ip: u16,
    e_cs: u16,
    e_lfarlc: u16,
    e_ovno: u16,
    e_res: [u16; 4],
    e_oemid: u16,
    e_oeminfo: u16,
    e_res2: [u16; 10],
    e_lfanew: i32,
}

impl ImageDosHeader {
    pub fn nt_headers(self: &Self) -> &ImageNtHeaders {
        let this = self as *const ImageDosHeader as *const ImageNtHeaders;
        unsafe { &*this.byte_offset(self.e_lfanew as isize) }
    }
}
