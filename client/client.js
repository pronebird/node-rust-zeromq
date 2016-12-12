var zmq = require('zmq');

// create one-way pull socket
var pull_socket = zmq.socket('pull');

// create request-reply socket
var req_socket = zmq.socket('req');

// setup event handlers
pull_socket.on('message', function (message) {
  console.log('Received: ' + message.toString('utf8'));
});

req_socket.on('message', function (message) {
  console.log('<- ' + message.toString('utf8'));
});

// ping server every second
setInterval(function () {
  req_socket.send("Ping");
  console.log("-> Ping");
}, 1000);

// connect sockets
pull_socket.connect('tcp://127.0.0.1:9998');
req_socket.connect('tcp://127.0.0.1:9999');
