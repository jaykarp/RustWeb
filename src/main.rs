extern crate ctrlc;
use std::io::prelude::*;
use std::io::stdin;
use std::io::Result;
use std::net::TcpStream;
use std::process::exit;
use std::str;
use std::thread;

fn main() -> Result<()> {
    ctrlc::set_handler(move || {
        println!("GoodByte!");
        exit(127);
    })
    .expect("Ctrlc handler failed.");

    // Create TcpStream and then clone to allow access in multiple threads
    let mut client_stream = TcpStream::connect("localhost:8080")?;
    let mut client_stream_clone = client_stream.try_clone()?;
    // Get STDIN
    let mut localstream = stdin();

    // Spawn a new thread to handle Client
    thread::spawn(move || -> Result<()> {
        client_stream.write("Pipe Welcome!\n".as_bytes())?;
        loop {
            let mut client_buff = [0; 128];
            client_stream.read(&mut client_buff)?;
            match client_buff[0] {
                0 => break,
                _ => (),
            };
            let client: String = str::from_utf8(&mut client_buff)
                .unwrap()
                .chars()
                .map(|c| match c {
                    '\n' => ' ',
                    _ => c,
                })
                .collect();
            println!("{}", client);
        }
        client_stream.write("GoodByte!".as_bytes())?;
        println!("Client Exited");
        exit(127)
    });

    // Continue in Main thread to handle Local
    println!("Pipe Welcome!");
    loop {
        let mut local_buff = [0; 128];
        localstream.read(&mut local_buff)?;
        client_stream_clone.write(&mut local_buff)?;
    }
}
