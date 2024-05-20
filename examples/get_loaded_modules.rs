use hack::client::Client;

const PID: u32 = 11368;

fn main() {
    let mut client = Client::new(PID);
    if !client.init() {
        println!("{} process isn't running.", PID);
    }

    let modules = client.get_loaded_module_names();
    println!("Loaded {} modules, PID {}", modules.len(), PID);

    for module in modules {
        println!("- {}", module);
    }
}
