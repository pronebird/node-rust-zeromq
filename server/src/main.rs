extern crate zmq;
extern crate protobuf;

mod rpc;
use rpc::{Request, RequestType, Response};
use protobuf::Message;
use protobuf::core::parse_from_bytes;
use std::{fmt, thread};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

//
// Make RequestType printable
//
impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            RequestType::PING => "PING",
            _ => "UNKNOWN"
        };
        return write!(f, "{}", s);
    }
}

//
// RPC methods
//
fn accept_challenge(req: &Request) -> Response {
    let mut res = Response::new();
    res.set_field_type(req.get_field_type());
    res.set_message(String::from("accepted"));

    return res;
}

fn invalid_request(req: &Request) -> Response {
    let mut res = Response::new();
    res.set_field_type(req.get_field_type());
    res.set_message(String::from("invalid request type"));

    return res;
}

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
            let msg = time.to_string();

            push_socket.send(&msg, 0).unwrap();

            println!("Sent beacon");

            thread::sleep(Duration::from_secs(3));
        }
    });

    let rep_thread = thread::spawn(move || {
        loop {
            // rep_socket.recv(&mut mut_msg, 0).unwrap();
            let bytes = rep_socket.recv_bytes(0).unwrap();
            let request: Request = parse_from_bytes(&bytes).unwrap();
            let request_type = request.get_field_type();

            println!("-> type: {}, query: {}", request_type, request.get_query());
            
            // handle received message
            let response = match request_type {
                RequestType::PING  => accept_challenge(&request),
                _ => invalid_request(&request),
            };

            // send response
            let bytes = response.write_to_bytes().unwrap();
            rep_socket.send(&bytes, 0).unwrap();
        }
    });

    // wait for threads to complete execution
    let _ = push_thread.join();
    let _ = rep_thread.join();
}
