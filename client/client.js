var zmq = require('zmq');
var socket = zmq.socket('pull');

socket.on('message', function (message) {
  console.log('Received: ' + message.toString('utf8'));
});

socket.connect('tcp://127.0.0.1:9998');
