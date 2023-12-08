use std::time::{Duration, Instant};

use crate::minitokio::MiniTokio;
use crate::delay::Delay;

mod task;
mod minitokio;
mod ioeventloop;
mod delay;


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
