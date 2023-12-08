use mio::{Events, Poll, Token, Interest};
use std::io::{Write, BufReader, BufRead};
use std::time::Duration;

pub struct IoEventLoop {
    mio_poll: mio::Poll,
    event_capacity: usize,
    event_list: Events
}

impl IoEventLoop {
    fn new() -> Self {
        let event_capacity = 5;

        IoEventLoop {
            mio_poll: mio::Poll::new().unwrap(),
            event_capacity,
            event_list: Events::with_capacity(event_capacity)
        }
    }

    fn turn(&mut self) {
        // poll at maximum event_capacity events
        let _ = self.mio_poll.poll(&mut self.event_list, Some(Duration::from_secs(10)));
    }
}