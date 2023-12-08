use std::pin::Pin;
use futures::task::{self, ArcWake};
use std::sync::{mpsc, Arc, Mutex};
use std::future::Future;
use std::task::Context;

// Task are "just futures that can reschedule themselves"
pub struct Task {
    // We use mutex so Task can have the Sync trait
    pub future: Mutex<Pin<Box<dyn Future<Output=()> + Send>>>,
    // send is Sync since a recent Rust version
    // with both member Sync, Task is Sync
    pub sender: mpsc::Sender<Arc<Task>>
}

impl Task {
    // TODO: this casting disturbs me
    fn schedule(self: &Arc<Self>) {
        let _ = self.sender.send(self.clone());
    }

    pub fn poll(self: &Arc<Self>) {
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