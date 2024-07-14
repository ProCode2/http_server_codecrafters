mod http;
use http::Server;

fn main() -> Result<(), std::io::Error> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let mut server = Server::new("127.0.0.1:4221")?;
    server.run();
    Ok(())
}
