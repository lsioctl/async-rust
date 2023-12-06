use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

fn main() -> std::io::Result<()> {
    let server = "127.0.0.1:8080";

    let mut stream = TcpStream::connect(server)?;

    // Note: it seems that the Write trait
    // is included with std::io::prelude::*
    stream.write(b"GET / HTTP/1.0\n\n")?;

    // I will use BufReader instead
    // let mut received = [0; 128];
    // stream.read(&mut received)?;
    // TODO: could not use ? because of mismatch of error types
    // could be interesting to dig ?
    // FromUtf8Error, and 
    // println!("{}", String::from_utf8(received.to_vec()).unwrap());

    let buffer_reader = BufReader::new(&mut stream);

    buffer_reader
        .lines()
        .map(|line| line.unwrap())
        .for_each(|line| { println!("{}", line)});

    Ok(())
}
