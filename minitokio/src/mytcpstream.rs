use std::{net::SocketAddr, borrow::BorrowMut};
use futures::Future;
use mio::Interest;
use mio::net::TcpStream;
use crate::ioeventloop::IoEventLoop;
use std::task::{Context, Poll};
use std::pin::Pin;

pub struct MyTcpStream {
    io: IoEventLoop
}

impl MyTcpStream {
    fn new() -> Self {
        // TODO: how do I get the main eventloop and not spawn a new one ?
        MyTcpStream {
            io: IoEventLoop::new()
        }
    }

    pub fn connect() -> MyTcpStreamConnectFuture {
        let stream = MyTcpStream::new();

        let server_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

        let client_stream = TcpStream::connect(server_addr).unwrap();

        MyTcpStreamConnectFuture {
            io: stream.io,
            ready: false,
            stream: client_stream,
        }
    }
}
//#[pin_project::pin_project]
pub struct MyTcpStreamConnectFuture {
    io: IoEventLoop,
    ready: bool,
    stream: mio::net::TcpStream
}

impl Future for MyTcpStreamConnectFuture {
    // TODO: change it to a stream, now just for debug purpose
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.ready {
            false => {
                //let this = self.project();
                let waker = cx.waker().clone();

                let this = Pin::into_inner(self);

                this.io.register_with_waker(&mut this.stream, Interest::WRITABLE, &waker);

                Poll::Pending

            },
            true => {
                Poll::Ready("Ready")
            }
        }
    }
}



