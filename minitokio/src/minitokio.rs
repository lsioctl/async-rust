use std::future::Future;
use std::sync::{mpsc, Arc, Mutex};

use crate::task::Task;

pub struct MiniTokio {
    initial_sender: Option<mpsc::Sender<Arc<Task>>>,
    scheduled: mpsc::Receiver<Arc<Task>>,
}

impl MiniTokio {
    pub fn new() -> Self {
        let (sender, scheduled) = mpsc::channel();
        MiniTokio { initial_sender: Some(sender), scheduled }
    }

    pub fn spawn(&mut self, async_fn: impl Future<Output=()> + Send + 'static) {
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

    pub fn run(&self) {
        // will block forever if a sender still exists
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}