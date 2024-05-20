use hack::client::Client;

const PID: u32 = 12892;
const ADDRESS: usize = 0x1a719aa2910;

fn main() {
    let mut client = Client::new(PID);
    if !client.init() {
        println!("{} process isn't running.", PID);
    }

    let result = client.write_str(ADDRESS, String::from("HACKED"));
    if result {
        println!("Success writing memory ({}) PID {}", ADDRESS, PID);
    } else {
        println!("Failed writing memory ({}) PID {}", ADDRESS, PID);
    }
}
