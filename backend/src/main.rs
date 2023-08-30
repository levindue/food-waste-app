mod logic;
mod server;

fn main() {
    let mut manager = match logic::Manager::read_from_file("data.json") {
        Ok(manager) => manager,
        Err(_) => logic::Manager::new(),
    };

    if let Err(_) = server::start("127.0.0.1:8080", &mut manager) {
        eprintln!("Failed to start the server.");
    }
}
