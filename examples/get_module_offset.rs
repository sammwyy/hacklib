use hack::client::Client;

const PID: u32 = 11368;

fn main() {
    let mut client = Client::new(PID);
    if !client.init() {
        println!("{} process isn't running.", PID);
    }

    let offset: usize = client.get_module_offset("kernel32.dll").unwrap();
    println!("Get module offset ({}) PID {}", offset, PID);
}
