const net = require('net');

function createClient() {
  const client = net.createConnection("\\\\.\\pipe\\my_bidirectional_pipe", () => {
    console.log('Connected to server');

    // Read input from user and send it to the server
    client.write("TEste");
    // process.stdin.on('data', (data) => {
    //   client.write(data);
    // });
  });

  client.on('data', (data) => {
    console.log('Received:', data.toString());
  });

  client.on('end', () => {
    console.log('Disconnected from server');
  });

  client.on('error', (err) => {
    console.error('Error:', err);
  });
}

createClient();  // Create the first client
createClient();  // Create another client

// You can continue creating more clients as needed
