var zmq = require('zmq');
var pull_socket = zmq.socket('pull');
var req_socket = zmq.socket('req');

pull_socket.on('message', function (message) {
  console.log('Received: ' + message.toString('utf8'));
});

req_socket.on('message', function (message) {
  console.log('<- ' + message.toString('utf8'));
});

setInterval(function () {
  req_socket.send("Ping");
  console.log("-> Ping");
}, 1000);

pull_socket.connect('tcp://127.0.0.1:9998');
req_socket.connect('tcp://127.0.0.1:9999');
