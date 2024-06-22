use core::{arch::asm, ffi::c_void};

use super::PebLdrData;

#[repr(C)]
#[derive(Debug)]
pub struct Peb {
    reserved1: [u8; 2],
    being_debugged: u8,
    reserved2: [u8; 1],
    reserved3: [*const c_void; 2],
    ldr: *const PebLdrData,
    process_parameters: *const c_void,
    reserved4: [*const c_void; 3],
    atl_thunk_slist_ptr: *const c_void,
    reserved5: *const c_void,
    reserved6: u32,
    reserved7: *const c_void,
    reserved8: u32,
    atl_thunk_slist_ptr32: u32,
    reserved9: [*const c_void; 45],
    reserved10: [u8; 96],
    post_process_init_routine: *const c_void,
    reserved11: [u8; 128],
    reserved12: [*const c_void; 1],
    session_id: u32,
}

impl Peb {
    pub fn current() -> &'static Peb {
        let nt_current_peb = || -> *const Peb { __readgsqword(0x60) as *const Peb };
        unsafe { &*nt_current_peb() }
    }

    pub fn ldr(self: &Self) -> &PebLdrData {
        unsafe { &*self.ldr }
    }
}

fn __readgsqword(offset: u32) -> u64 {
    let out: u64;
    unsafe {
        asm!(
            "mov {}, gs:[{:e}]",
            lateout(reg) out,
            in(reg) offset,
            options(nostack, pure, readonly),
        )
    }
    out
}
