
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;

fn main() {

    let target = "127.0.0.1";
    let start_port = 1;
    let end_port = 1024;
    let timeout = Duration::from_millis(10);


    println!("Scanning {} on ports {}-{}", target, start_port, end_port);

    // channel to communicate between the threads we open
    let (tx, rx) = mpsc::channel();

    for port in start_port..=end_port {
        let tx = tx.clone();
        let target = target.to_string();
        thread::spawn(move || {
           let address = format!("{}:{}", target, port);
           if let Ok(socket) = address.parse::<SocketAddr>() {
               if TcpStream::connect_timeout(&socket, timeout).is_ok() {
                   tx.send(port).unwrap();
               }
           }

        });

    
    }
    // close all threads when finished. 
    drop(tx);
    
    for port in rx {
        println!("Port {} is open", port);
    }
}
