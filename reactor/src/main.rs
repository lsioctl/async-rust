// Macro from mio
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

use libc::{self, epoll_event};
use std::os::unix::io::{AsRawFd, RawFd};
// use std::io;
// use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// Oneshot: if we get notified, the interest is removed
const READ_FLAGS: i32 = libc::EPOLLONESHOT | libc::EPOLLIN;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();
    listener.set_nonblocking(true).unwrap();
    let listener_fd = listener.as_raw_fd();

    let epoll_fd = syscall!(epoll_create1(0)).unwrap();
    println!("{}", epoll_fd);

    let mut read_event = epoll_event {
        events: READ_FLAGS as u32,
        // for now arbitrary key
        u64: 3000 as u64
    };

    // add interest for the event on ou listening socket
    syscall!(epoll_ctl(epoll_fd, libc::EPOLL_CTL_ADD, listener_fd, &mut read_event)).unwrap();

    let mut event_list: Vec<libc::epoll_event> = Vec::with_capacity(1024);

    event_list.clear();

    let res = syscall!(epoll_wait(
        epoll_fd,
        event_list.as_mut_ptr() as *mut libc::epoll_event,
        1024,
        10000
    )).unwrap();

    //let _ = listener.accept();

    println!("res: {}", res);

    // if we do not set the length, it stays 0 as we modified the data
    // pointer only
    // safe  as long as the kernel does nothing wrong - copied from mio
    unsafe { event_list.set_len(res as usize) };

    // reference to packed field is unaligned
    // !("{}", event_list.get(0).unwrap().u64);

    for ev in event_list {
        // copy to a local variable to avoid println macro
        // reference to packed field is unaligned
        let what = ev.u64;
        println!("{}", what);
    }


}
