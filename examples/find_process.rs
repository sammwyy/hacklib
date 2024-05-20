use hack::{client::Client, utils::find_pid_by_name};

const PROCESS_NAME: &str = "Banana.exe";

fn main() {
    let pid = find_pid_by_name(PROCESS_NAME);
    if pid.is_none() {
        println!("{} isn't running.", PROCESS_NAME);
    }

    let pid = pid.unwrap();
    let mut client = Client::new(pid);
    if !client.init() {
        println!("{} pid found but isn't running.", PROCESS_NAME);
    }

    println!("Found process {} with PID {}", PROCESS_NAME, client.pid());
}
