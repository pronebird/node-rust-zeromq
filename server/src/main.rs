extern crate zmq;

use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    println!("ZeroMQ server.");

    let ctx = zmq::Context::new();
    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind("tcp://127.0.0.1:9998").unwrap();

    let rep_socket = ctx.socket(zmq::REP).unwrap();
    rep_socket.bind("tcp://127.0.0.1:9999").unwrap();

    let push_thread = thread::spawn(move || {
        loop {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let msg = format!("Time is {}", time);

            push_socket.send_str(&msg, 0).unwrap();

            println!("Sent data");

            thread::sleep(Duration::from_secs(3));
        }
    });

    let rep_thread = thread::spawn(move || {
        let mut msg = zmq::Message::new().unwrap();

        loop {
            rep_socket.recv(&mut msg, 0).unwrap();

            println!("Received {}", msg.as_str().unwrap());

            rep_socket.send_str("Pong", 0).unwrap();
        }
    });

    push_thread.join();
    rep_thread.join();
}
