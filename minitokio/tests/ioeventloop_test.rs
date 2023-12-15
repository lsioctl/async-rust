use std::net::SocketAddr;
use mio::Interest;
use mio::net::TcpStream;
use minitokio::ioeventloop::IoEventLoop;

use std::thread;
use std::time::Duration;

#[test]
fn epoll_write() {
    // TODO: still issues with casting so I can't use const
    // right now: parse is not a const fn
    let server_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let io = IoEventLoop::new();

    let mut client_stream = TcpStream::connect(server_addr).unwrap();

    thread::sleep(Duration::from_secs(2));

    io.register(&mut client_stream, Interest::WRITABLE);

    thread::sleep(Duration::from_secs(2));
}
