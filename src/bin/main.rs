use change_memory_value::input::{read_input, read_memory_addr};
use change_memory_value::process::{open_process, write_to_process};

fn main() {
    let proc_id: u32 = read_input("What process id?").expect("No process id?");
    let addr = read_memory_addr("What memory addr?").expect("No memory addr?");

    let handle = open_process(proc_id).expect("No handle");

    loop {
        let new_value: usize = read_input("New value?").unwrap();

        if !write_to_process(handle, addr, new_value) {
            break;
        }
    }
}
