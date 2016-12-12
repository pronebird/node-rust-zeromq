var zmq = require('zmq');
var protobuf = require('protobufjs');

// import protocol
var rpc = protobuf.loadSync('../protocol/rpc.proto');
var Request = rpc.lookup('rpc.Request');
var Response = rpc.lookup('rpc.Response');
var RequestType = rpc.lookup('rpc.RequestType');

// create one-way pull socket
var pull_socket = zmq.socket('pull');

// create request-reply socket
var req_socket = zmq.socket('req');

// setup event handlers
pull_socket.on('message', function (message) {
  console.log('Beacon received: ' + message.toString('utf8'));
});

req_socket.on('message', function (buffer) {
  var res = Response.decode(buffer);
  console.log('<- ' + res.message);
});

// ping server every second
setInterval(function () {
  var message = Request.create();
  message.type = RequestType.values.PING;
  message.query = "challenge";

  var bytes = Request.encode(message).finish();
  req_socket.send(bytes);
  
  console.log("-> challenge");
}, 2000);

// connect sockets
pull_socket.connect('tcp://127.0.0.1:9998');
req_socket.connect('tcp://127.0.0.1:9999');
