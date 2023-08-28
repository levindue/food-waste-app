mod logic;
mod server;

fn main() {
    if let Err(_) = server::start("127.0.0.1:8080") {
        eprintln!("Failed to start the server.");
    }
}
