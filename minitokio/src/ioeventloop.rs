use mio::{Events, Poll, Token, Interest, Registry};
use mio::event::Source;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use mio::net::TcpStream;

pub struct IoEventLoop {
    registry: Registry
}

impl IoEventLoop {
    pub fn new() -> Self {
        let poll = Poll::new().unwrap();
        // try_clone allows to creates a new independently owned Registry.
        // Event sources registered with this Registry will be registered 
        // with the original Registry and Poll instance.
        let registry = poll.registry().try_clone().unwrap();

        let io = IoEventLoop {
            registry
        };

        // TODO: feels clumsy
        io.run(poll);

        io
    }

    fn run(&self, mut poll: Poll) {
        thread::spawn(move || {
            loop {
                println!("In Da Loop");

                let mut event_list = Events::with_capacity(2);

                poll.poll(&mut event_list, Some(Duration::from_secs(10))).unwrap();

                event_list.iter().for_each(|event| {
                    println!("{:#?}", event);
                })
            }
        });
    }

    pub fn register(&self, stream: &mut impl Source, interest: Interest) {
        // I'll use a real token later to keep an internal hasmap of wakers and events
        let _ = self.registry.register(stream, Token(3000), interest);
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