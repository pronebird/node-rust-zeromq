extern crate zmq;

use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    // create new zmq context
    let ctx = zmq::Context::new();

    // create one-way push socket to send messages from server to client
    let push_socket = ctx.socket(zmq::PUSH).unwrap();
    push_socket.bind("tcp://127.0.0.1:9998").unwrap();

    // create reply socket for synchronous request-reply communication
    let rep_socket = ctx.socket(zmq::REP).unwrap();
    rep_socket.bind("tcp://127.0.0.1:9999").unwrap();

    // spin off threads
    let push_thread = thread::spawn(move || {
        loop {
            let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
            let msg = format!("The time is {}", time);

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

    // wait for threads to complete execution
    let _ = push_thread.join();
    let _ = rep_thread.join();
}
