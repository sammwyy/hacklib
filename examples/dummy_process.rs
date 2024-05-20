use std::{process, thread, time::Duration};

fn main() {
    let pid = process::id();
    let mut text = String::from("Hello World");
    let ptr = text.as_mut_ptr();

    loop {
        println!("PID: {} PTR: {:?}       {}", pid, ptr, text);
        thread::sleep(Duration::from_secs(1));
    }
}
