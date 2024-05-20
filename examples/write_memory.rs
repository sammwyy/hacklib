use hack::{client::Client, utils::find_pid_by_name};

const PROCESS_NAME: &str = "Banana.exe";
const ADDRESS: usize = 0x87115C888;
const VALUE: i32 = 1337;

fn main() {
    let pid = find_pid_by_name(PROCESS_NAME).unwrap_or(0);
    let mut client = Client::new(pid);
    if !client.init() {
        println!("{} pid found but isn't running.", PROCESS_NAME);
    }

    let result = client.write_mem(ADDRESS, VALUE);
    if result {
        println!("Success writing memory ({}) PID {}", ADDRESS, pid);
    } else {
        println!("Failed writing memory ({}) PID {}", ADDRESS, pid);
    }
}
