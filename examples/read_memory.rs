use hack::client::Client;

const PID: u32 = 12892;
const ADDRESS: usize = 0x1a719aa2910;

fn main() {
    let mut client = Client::new(PID);
    if !client.init() {
        println!("{} process isn't running.", PID);
    }

    let result: String = client.read_str(ADDRESS, 10);
    println!("Read memory value ({}) PID {}", result, PID);
}
