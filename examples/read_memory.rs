use hack::{client::Client, utils::find_pid_by_name};

const PROCESS_NAME: &str = "Banana.exe";
const ADDRESS: usize = 0x87115C888;

fn main() {
    let pid = find_pid_by_name(PROCESS_NAME).unwrap_or(0);
    let mut client = Client::new(pid);
    if !client.init() {
        println!("{} pid found but isn't running.", PROCESS_NAME);
    }

    let result: i32 = client.read_mem(ADDRESS);
    println!("Read memory value ({}) PID {}", result, pid);
}
