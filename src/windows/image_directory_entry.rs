#[allow(dead_code)]
#[derive(Debug)]
pub enum ImageDirectoryEntry {
    Architecture = 7,
    BaseReloc = 5,
    BoundImport = 11,
    ComDescriptor = 14,
    Debug = 6,
    DelayImport = 13,
    Exception = 3,
    Export = 0,
    GlobalPtr = 8,
    ImportAddressTable = 12,
    Import = 1,
    LoadConfig = 10,
    Resource = 2,
    Security = 4,
    ThreadLocalStorage = 9,
}
