use std::ffi::CString;
use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::tlhelp32::{
    CreateToolhelp32Snapshot, Process32Next, PROCESSENTRY32, TH32CS_SNAPALL,
};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::winuser::{FindWindowA, GetWindowThreadProcessId};

type Handle = winapi::ctypes::c_void;
#[derive(Debug)]
pub enum WindowHandleError {
    NoWindow,
    NoProcess,
    NoHandle,
    Unknown,
}
pub fn open_process(proc_id: u32) -> Result<*mut Handle, WindowHandleError> {
    let handle = unsafe { OpenProcess(PROCESS_ALL_ACCESS, 0, proc_id as DWORD) };

    match handle != std::ptr::null_mut() {
        true => Ok(handle),
        false => Err(WindowHandleError::NoHandle),
    }
}
pub fn find_window(window_name: &str) -> Result<u32, WindowHandleError> {
    let hwnd: HWND = unsafe {
        let window_name = match CString::new(window_name) {
            Ok(value) => value,
            Err(_) => return Err(WindowHandleError::Unknown),
        };
        FindWindowA(std::ptr::null(), window_name.as_ptr() as *const i8)
    };
    if hwnd.is_null() {
        return Err(WindowHandleError::NoWindow);
    }
    let proc_id = unsafe { GetWindowThreadProcessId(hwnd as HWND, std::ptr::null_mut()) };
    match proc_id {
        0 => Err(WindowHandleError::NoProcess),
        id => Ok(id),
    }
}
pub unsafe fn get_process_name(process_entry: &PROCESSENTRY32) -> String {
    std::ffi::CStr::from_ptr(&process_entry.szExeFile[0])
        .to_string_lossy()
        .into_owned()
}
pub fn get_process_list() -> Vec<String> {
    let mut processes = vec![];
    unsafe {
        let handle = CreateToolhelp32Snapshot(TH32CS_SNAPALL, 0);
        let mut pe32 = std::mem::zeroed::<PROCESSENTRY32>();
        pe32.dwSize = std::mem::size_of::<PROCESSENTRY32>() as DWORD;

        while Process32Next(handle, &mut pe32) != 0 {
            if pe32.th32ProcessID == 0 {
                continue;
            }
            let name = get_process_name(&pe32);
            /*                let pid = DWORD::from(pe32.th32ProcessID.clone());
            let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid.clone());*/

            processes.push(name);
        }
    };
    return processes;
}

pub fn write_to_process<T>(handle: *mut Handle, addr: usize, new_value: T) -> bool {
    let mut input = new_value;
    let written = unsafe {
        WriteProcessMemory(
            handle,
            addr as *mut _,
            &mut input as *mut _ as *mut _,
            std::mem::size_of::<T>(),
            std::ptr::null_mut(),
        )
    };

    written != 0
}
