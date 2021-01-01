use std::ffi::CString;

fn main() {
    let program_name = CString::new("modify-memory.exe").unwrap();
    let mut value = -1;
    let ptr_x = &mut value as *mut i32;
    let module_addr =
        unsafe { winapi::um::libloaderapi::GetModuleHandleA(program_name.as_ptr() as *const i8) };
    println!(
        "My pid: {}, value_ptr_addr: {:p}, base_addr: {:p}",
        std::process::id(),
        ptr_x,
        module_addr
    );
    println!("Initial value: {}", value);

    let mut old_value = value;
    loop {
        if value != old_value {
            old_value = value;
            println!("New value: {}", value);
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}
