// server.js

const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const packageDefinition = protoLoader.loadSync("message.proto");
const { loadtest } = grpc.loadPackageDefinition(packageDefinition);

const server = new grpc.Server();

server.addService(loadtest.LoadTester.service, {
  StreamPayload: (stream) => {
    const start = Date.now();

    // Function to generate a 1.5KB message payload
    function generatePayload(size) {
      return "x".repeat(size);
    }

    const payloadSize = parseInt(
      process.env.PAYLOAD_SIZE || process.argv[2] || "1500"
    );
    const totalMessages = parseInt(
      process.env.TOTAL_MESSAGES || process.argv[3] || "1000"
    );

    let messageCount = 0;
    let ackCount = 0;

    // Send messages to client
    for (let i = 0; i < totalMessages; i++) {
      const payload = generatePayload(payloadSize);
      stream.write({ message: payload });
      messageCount++;
    }

    console.log(
      `All ${messageCount} messages sent. Awaiting acknowledgements...`
    );

    // Handle client acknowledgments
    stream.on("data", (ack) => {
      ackCount++;
      // console.log("Acknowledgement from client:", ack.payload);
      if (ackCount === totalMessages) {
        console.log("All messages acknowledged by client.");
        const duration = Date.now() - start;
        console.log(`Total time for ${totalMessages} messages: ${duration} ms`);
        stream.write({
          message: `Total time for ${totalMessages} messages: ${duration} ms`,
        });
        // stream.end(); // Close the stream after all messages are acknowledged
      }
    });

    stream.on("error", (err) => {
      console.error("Stream encountered an error:", err);
    });

    stream.on("end", () => {
      console.log("Stream ended by client.");
    });
  },
});

server.bindAsync(
  "0.0.0.0:50051",
  grpc.ServerCredentials.createInsecure(),
  (err, port) => {
    if (err) throw err;
    server.start();
    console.log(`Server running at 0.0.0.0:${port}`);
  }
);
