use std::net::ToSocketAddrs;
use std::io;

fn main() {
    println!("Enter Domain To Resolve To IP");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = format!("{}:80", input.trim());

    match input.to_socket_addrs() {
        Ok(addrs) => {
            for addr in addrs {
                println!("Resolved IP: {}", addr.ip());
            }
        }
        Err(e) => {
            println!("Failed to resolve please try again :0 '{}': {}", input, e);
        }
    }
}
