use core::ptr::null;

use super::LdrDataTableEntry;

#[repr(C)]
#[derive(Debug)]
pub struct ListEntry {
    flink: *const ListEntry,
    blink: *const ListEntry,
}

pub struct ListEntryIterator {
    current: *const LdrDataTableEntry,
    end: &'static LdrDataTableEntry,
}

impl ListEntry {
    pub fn next(self: &Self) -> &LdrDataTableEntry {
        let next = self.flink as *const LdrDataTableEntry;
        unsafe { &*next }
    }

    pub fn into_iter(self: &'static Self) -> ListEntryIterator {
        ListEntryIterator::new(self)
    }
}

impl ListEntryIterator {
    pub fn new(list: &'static ListEntry) -> Self {
        let begin = list.next();
        Self {
            current: begin,
            end: begin,
        }
    }
}

impl Iterator for ListEntryIterator {
    type Item = &'static LdrDataTableEntry;

    fn next(self: &mut Self) -> Option<Self::Item> {
        if self.current.is_null() {
            return None;
        }
        let current = unsafe { &*self.current };
        if current.dll_base().is_null() {
            return None;
        }
        let next = current.list().next() as *const LdrDataTableEntry;
        let end = self.end as *const LdrDataTableEntry;
        match next != end {
            true => self.current = next,
            false => self.current = null(),
        }
        Some(current)
    }
}
