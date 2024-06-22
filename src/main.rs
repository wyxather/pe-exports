mod windows;

use windows::PortableExecutable;

fn main() {
    if let Some(pe) = PortableExecutable::find("kernel32") {
        let export_function = pe.export_function("EnterCriticalSection");
        println!("{:#?}", export_function);
    }
    if let Some(pe) = PortableExecutable::find("ntdll") {
        let export_function = pe.export_function("RtlEnterCriticalSection");
        println!("{:#?}", export_function);
    }
}
