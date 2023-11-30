use std::future::Future;
use std::thread;
use std::time::{Duration, Instant};
use std::task::{Context, Poll};
use std::sync::{mpsc, Arc, Mutex};
use std::pin::Pin;
use futures::task::{self, ArcWake};

struct Delay {
    when: Instant
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


// Task are "just futures that can reschedule themselves"
struct Task {
    // We use mutex so Task can have the Sync trait
    future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
    // send is Sync since a recent Rust version
    // with both member Sync, Task is Sync
    sender: mpsc::Sender<Arc<Task>>
}

impl Task {
    // TODO: this casting disturbs me
    fn schedule(self: &Arc<Self>) {
        let _ = self.sender.send(self.clone());
    }

    fn poll(self: &Arc<Self>) {
        // this uses the "self cast" (still TODO to understand)
        // and ArcWake implementation
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.lock().unwrap();

        // as_mut is a generic method to go
        // fomr &mut Pin<Pointer<T>> to Pin<&mut T>
        let _ = future.as_mut().poll(&mut cx);
    }
}

// hook the schedule function with the std::Waker::wake()
// use of futures' crate ArcWake instead of playing with
// the vtables of the low level API RawWakerVtable
impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        arc_self.schedule();
    }
}

struct MiniTokio {
    initial_sender: Option<mpsc::Sender<Arc<Task>>>,
    scheduled: mpsc::Receiver<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        MiniTokio { initial_sender: Some(sender), scheduled }
    }

    fn spawn(&mut self, async_fn: impl Future<Output=()> + Send + 'static) {
        match & mut self.initial_sender {
            Some(sender) => {
                let task = Arc::new(
                    Task {
                        future: Mutex::new(Box::pin(async_fn)),
                        sender: sender.clone()
                    }
                );
        
                let _ = sender.send(task);

                // Drop the original Sender to avoid run blocking forever once Task is done
                self.initial_sender = None;
            },
            None => {
                panic!("Can't spawn twice");
            }
            
        }
    }

    fn run(&self) {
        // will block forever if a sender still exists
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}



fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_secs(3);

        let delay = Delay { when };

        let out = delay.await;

        println!("{}", out);

    });

    mini_tokio.run();

}
