use std::net::SocketAddr;
use mio::net::TcpStream;
use std::os::unix::io::AsRawFd;
use minitokio::ioeventloop::{IoEventLoop, Interest};

#[test]
fn epoll_write() {
    // TODO: still issues with casting so I can't use const
    // right now: parse is not a const fn
    let server_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

    let io = IoEventLoop::new();

    let client_stream = TcpStream::connect(server_addr).unwrap();

    io.register_interest(client_stream.as_raw_fd(), Interest::Writeable);

    io.run();
}
