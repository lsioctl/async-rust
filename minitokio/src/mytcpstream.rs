use std::net::SocketAddr;
use std::task::{Context, Poll};
use std::pin::Pin;

use futures::Future;
use mio::Interest;
use mio::net::TcpStream;

use crate::minitokio::IO_EVENT_LOOP;

pub struct MyTcpStream {
}

impl MyTcpStream {
    pub fn connect() -> MyTcpStreamConnectFuture {
        let server_addr: SocketAddr = "127.0.0.1:8000".parse().unwrap();

        let client_stream = TcpStream::connect(server_addr).unwrap();

        MyTcpStreamConnectFuture {
            ready: false,
            stream: client_stream,
        }
    }
}
//#[pin_project::pin_project]
pub struct MyTcpStreamConnectFuture {
    ready: bool,
    stream: mio::net::TcpStream
}

impl Future for MyTcpStreamConnectFuture {
    // TODO: change it to a stream, now just for debug purpose
    type Output = &'static str;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.ready {
            false => {
                println!("Polled !");
                //let this = self.project();
                let waker = cx.waker().clone();

                let this = Pin::into_inner(self);

                let mut io_mutex_guard = IO_EVENT_LOOP.get().unwrap().lock().unwrap();
                
                io_mutex_guard.register_with_waker(&mut this.stream, Interest::WRITABLE, &waker);

                Poll::Pending

            },
            true => {
                Poll::Ready("Ready")
            }
        }
    }
}



