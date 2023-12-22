// use std::time::{Duration, Instant};
// use crate::delay::Delay;

use minitokio::minitokio::MiniTokio;
use minitokio::mytcpstream::MyTcpStream;

// mod delay;


fn main() {
    let mut mini_tokio = MiniTokio::new();

    // mini_tokio.spawn(async {
    //     let when = Instant::now() + Duration::from_secs(3);

    //     let delay = Delay { when };

    //     let out = delay.await;

    //     println!("{}", out);

    // });

    mini_tokio.spawn(async {
        let stream = MyTcpStream::connect().await;

        println!("{}", stream);
    });

    mini_tokio.run();

}
