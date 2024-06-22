use core::ffi::c_void;

use super::ListEntry;

#[repr(C)]
#[derive(Debug)]
pub struct PebLdrData {
    reserved1: [u8; 8],
    reserved2: *const c_void,
    in_load_order_module_list: ListEntry,
    in_memory_order_module_list: ListEntry,
}

impl PebLdrData {
    pub fn list(self: &Self) -> &ListEntry {
        &self.in_load_order_module_list
    }
}
