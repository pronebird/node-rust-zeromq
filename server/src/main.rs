extern crate zmq;
extern crate protobuf;

mod rpc;
use rpc::{RpcRequest, RpcResponse, AckRequest, AckResponse};
use protobuf::Message;
use protobuf::core::parse_from_bytes;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

trait RpcConsumer {
    fn consume(&self, req: &RpcRequest) -> RpcResponse;
}

trait RpcController {
    fn ack(&self, req: &AckRequest) -> AckResponse;
}

struct RpcService {}

impl RpcController for RpcService {
     fn ack(&self, _: &AckRequest) -> AckResponse {
        let mut res = AckResponse::new();
        res.set_message(String::from("accepted"));
        return res;
     }
}

impl RpcConsumer for RpcService {
    fn consume(&self, req: &RpcRequest) -> RpcResponse {
        let method_name = req.get_method();
        let data = req.get_data();

        let mut rpc_response = RpcResponse::new();
        rpc_response.set_method(String::from(method_name));

        let result = match method_name {
            ".rpc.Service.ack" => {
                let ack_request: AckRequest = parse_from_bytes(&data).unwrap();
                let ack_response = self.ack(&ack_request);
                println!("<- {}: {}", method_name, ack_request.get_message());
                Ok(ack_response)
            },
            _ => Err("Unknown method"),
        };

        match result {
            Ok(ref x) => {
                let bytes = x.write_to_bytes().unwrap();
                rpc_response.set_data(bytes);
            },
            Err(e) => {
                rpc_response.set_error(String::from(e));
            },
        };

        rpc_response.set_status(result.is_ok());

        return rpc_response;
    }
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
        let service = RpcService {};

        loop {
            let bytes = rep_socket.recv_bytes(0).unwrap();
            let request: RpcRequest = parse_from_bytes(&bytes).unwrap();

            println!("-> {}", request.get_method());
            
            // handle request
            let response = service.consume(&request);

            // send response
            let bytes = response.write_to_bytes().unwrap();
            rep_socket.send(&bytes, 0).unwrap();
        }
    });

    // wait for threads to complete execution
    let _ = push_thread.join();
    let _ = rep_thread.join();
}
