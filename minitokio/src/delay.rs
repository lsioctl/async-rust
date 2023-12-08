use std::future::Future;
use std::thread;
use std::time::Instant;
use std::task::{Context, Poll};

pub struct Delay {
    pub when: Instant
}
     
impl Future for Delay {
    type Output = &'static str;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() > self.when {
            Poll::Ready("done")
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            // TODO: logic too redundant with other branch
            // maybe it will be solved with a more realistic example
            thread::spawn(move || {
                let now = Instant::now();

                if now < when {
                    thread::sleep(when - now);
                }

                waker.wake();
            });

            Poll::Pending
        }
    }
}