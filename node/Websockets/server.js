const WebSocket = require("ws");
const wss = new WebSocket.Server({ port: 8080 });

function generatePayload(size) {
  return "x".repeat(size);
}

const payloadSize = parseInt(process.env.PAYLOAD_SIZE || "1500");
const totalMessages = parseInt(process.env.TOTAL_MESSAGES || "1000");

wss.on("connection", function connection(ws) {
  let ackCount = 0;
  let start = new Date();

  // Function to send messages
  function sendMessages() {
    for (let i = 0; i < totalMessages; i++) {
      ws.send(JSON.stringify({ message: generatePayload(payloadSize) }));
    }
  }

  ws.on("message", function incoming() {
    ackCount++;
    if (ackCount === totalMessages) {
      let duration = new Date() - start;
      //   console.log(`Total time for ${totalMessages} messages: ${duration} ms`);
      ws.send(
        JSON.stringify({
          message: `Total time for ${totalMessages} messages: ${duration} ms`,
        })
      );
      ws.close(); // Close the WebSocket connection
    }
  });

  sendMessages();
});

console.log("WebSocket server started on ws://localhost:8080");
