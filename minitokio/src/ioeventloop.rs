use mio::{Events, Poll, Token, Interest};
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use mio::net::TcpStream;

pub struct IoEventLoop {
    mio_poll_wrapped: Arc<Mutex<Poll>> 
}

impl IoEventLoop {
    pub fn new() -> Self {
        let mio_poll = Poll::new().unwrap();

        IoEventLoop {
            mio_poll_wrapped: Arc::new(Mutex::new(mio_poll))
        }
    }

    pub fn run(&self) {
        let mio_poll_wrapped = self.mio_poll_wrapped.clone();

        thread::spawn(move || {
            let mut mio_poll_guard = mio_poll_wrapped.lock().unwrap();

            let mut event_list = Events::with_capacity(2);

            mio_poll_guard.poll(&mut event_list, Some(Duration::from_secs(10))).unwrap();

            event_list.iter().for_each(|event| {
                println!("{:#?}", event);
            })  
        });
    }

    pub fn register(&self, stream: &mut TcpStream, interest: Interest) {
        let mio_poll_guard = self.mio_poll_wrapped.lock().unwrap();

        // I'll use a real token later to keep an internal hasmap of wakers and events
        let _ = mio_poll_guard.registry().register(stream, Token(3000), interest);
    }
}

    //

    // TODO: self mutable is not needed but we are changing the epoll_fd ...
    //pub fn register_interest(&self, source_fd: i32, interest: Interest) {
        

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}