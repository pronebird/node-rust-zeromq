var zmq = require('zmq');
var protobuf = require('protobufjs');

// import protocol
var root = protobuf.loadSync('../protocol/rpc.proto');

var RpcRequest = root.lookup('rpc.RpcRequest');
var RpcResponse = root.lookup('rpc.RpcResponse');

var Service = root.lookup('rpc.Service');
var AckRequest = root.lookup('rpc.AckRequest');
var AckResponse = root.lookup('rpc.AckResponse');

// create one-way pull socket
var pullSocket = zmq.socket('pull');

// setup event handlers
pullSocket.on('message', function (message) {
  console.log('Beacon received: ' + message.toString('utf8'));
});

// create request-reply socket
var reqSocket = zmq.socket('req');

// RPC callback queue – used because zeromq bindings are event based..
var rpcQueue = [];

// setup RPC
var rpcService = Service.create(function (method, requestData, callback) {
  var rpcRequest = RpcRequest.create({ method: method.fullName, data: requestData });
  var bytes = RpcRequest.encode(rpcRequest).finish();

  console.log('-> ' + method.fullName);
  
  // send buffer
  reqSocket.send(bytes);

  // add callback to rpc queue
  // it will be called upon 'message' event arrival
  rpcQueue.push(function (err, responseData) {
    callback(err, responseData);
  });
});

reqSocket.on('error', function (err) {
  var callback = rpcQueue.shift();
  if(!callback) { return; }

  callback(err, null);
});

reqSocket.on('message', function (buffer) {
  var callback = rpcQueue.shift();
  if(!callback) { return; }

  var res = RpcResponse.decode(buffer);
  console.log('<- ' + res.method);

  callback(null, res.data);
});

// ping server every second
setInterval(function () {
  rpcService.ack(AckRequest.create({ message: 'challenge' }), function (err, res) {
    if(err) {
      return console.log('<- error: ', err);
    }

    console.log('<- ', res.message);
  });
  
  console.log("-> challenge");
}, 2000);

// connect sockets
pullSocket.connect('tcp://127.0.0.1:9998');
reqSocket.connect('tcp://127.0.0.1:9999');
