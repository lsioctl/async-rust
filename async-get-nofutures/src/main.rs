//use std::io::BufReader;
// use mio instead of std::net to have non blocking
use mio::net::{TcpStream, SocketAddr};
use mio::{Events, Poll, Token, Interest};
use std::io::{Read, Write, BufReader, BufRead};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let client_write = Token(0);
    let client_read = Token(1);
    // TODO: here compiler goes better without the type
    // SocketAddr as it messes it up with:
    // mio::sys::unix::uds::socketaddr
    // and std::net::SocketAddr (infered by connect call)
    // so I let the type inference from the connect function
    // for the parse type
    // no ? operator as no From<T> trait implementation
    let server_address = "127.0.0.1:8080".parse().unwrap();

    // non blocking connect thanks to mio
    // not: if we attempt to read or write, it could retourn would_block error
    let mut stream = TcpStream::connect(server_address)?;

    let mut event_list = Events::with_capacity(2);
    // TODO here new() return io::Result<Poll>,
    // but ? operator implicit conversion works
    // => there should be an implementation of the trait
    // From<T>
    // like <std::io::Error as From<NulError>>
    // => choose an error propagation and handling (unwrap, ?, ...)
    // for more consistency
    let mut mio_poll = Poll::new()?;
    mio_poll.registry().register(&mut stream, client_write, Interest::WRITABLE);
    mio_poll.registry().register(&mut stream, client_read, Interest::READABLE);

    // event loop
    loop {
        // poll (blocking): waiting for 10 milliseconds
        mio_poll.poll(&mut event_list, Some(Duration::from_millis(10000)))?;
        println!("Polling");

        event_list.iter().for_each(|event| {
            println!("{:#?}", event);
            match event.token() {
                client_write => {
                    // // Note: it seems that the Write trait
                    // // is included with std::io::prelude::*
                    // TODO: can't use ? operator here
                    stream.write(b"GET / HTTP/1.0\n\n").unwrap();
                    println!("Written");
                },
                client_read => {
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
                    
                }
            }
        });
    }
    
    Ok(())
}