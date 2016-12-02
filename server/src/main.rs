extern crate zmq;

use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("ZeroMQ server.");

    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUSH).unwrap();

    socket.bind("tcp://127.0.0.1:9998").unwrap();

    loop {
        socket.send_str("Hello world!", 0).unwrap();
        println!("Sent data");
        sleep(Duration::from_secs(1))
    }
}
