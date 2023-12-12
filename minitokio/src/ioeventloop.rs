// Macro from mio
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

// use std::io::{Write, BufReader, BufRead};
// use std::os::fd::AsRawFd;
// use std::time::Duration;
// use std::thread;
// use std::sync::mpsc;

// use crate::task::Task;

use libc::{self, epoll_event};
// use std::os::unix::io::{AsRawFd, RawFd};

// Oneshot: if we get notified, the interest is removed
// TODO: is it relevant in Edge Triggered Mode ?
const READ_FLAGS: i32 = libc::EPOLLET | libc::EPOLLONESHOT | libc::EPOLLIN;
const WRITE_FLAGS: i32 = libc::EPOLLET | libc::EPOLLONESHOT | libc::EPOLLOUT;

pub enum Interest {
    Readable,
    Writeable
}

pub struct IoEventLoop {
    epoll_fd: i32
}

impl IoEventLoop {
    pub fn new() -> Self {
        let epoll_fd = syscall!(epoll_create1(0)).unwrap();

        IoEventLoop {
            epoll_fd
        }
    }

    // TODO: self mutable is not needed but we are changing the epoll_fd ...
    pub fn register_interest(&self, source_fd: i32, interest: Interest) {
        let event_type = match interest {
            Interest::Readable => READ_FLAGS,
            Interest::Writeable => WRITE_FLAGS
        };

        let mut interest_event = epoll_event {
            events: event_type as u32,
            // for now arbitraty key but could be used to
            // map interest with a token, a waker, ...
            u64: 3000 as u64
        };

        // TODO: AFAII epoll_ctl just needs a pointer to read and release
        // and not to mutate it in place ?
        // This is disturbing because we think about lifetime of
        // interest_event, but it is not relevant
        // moreover there is an implicit cast between
        // &mut epoll_event and *mut epoll_event
        let _ = syscall!(epoll_ctl(
            self.epoll_fd, libc::EPOLL_CTL_ADD, source_fd, &mut interest_event)
        ).unwrap();
    }

    pub fn run(&self) {
        let mut event_list: Vec<libc::epoll_event> = Vec::with_capacity(1024);

        event_list.clear();

        let res = syscall!(epoll_wait(
            self.epoll_fd,
            event_list.as_mut_ptr() as *mut libc::epoll_event,
            1024,
            10000
        )).unwrap();

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
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}