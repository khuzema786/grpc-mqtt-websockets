const WebSocket = require("ws");
try {
  const ws = new WebSocket("ws://localhost:8080");

  ws.on("open", function open() {
    console.log("Connected to the server");
  });

  ws.on("message", function incoming(data) {
    // Assuming the incoming message is a JSON string
    try {
      const response = JSON.parse(data);
      if (response.message.startsWith("Total")) {
        console.log(response.message);
      } else {
        //   console.log("Message from server:", response.message.length, "bytes");
        // Send acknowledgment back to the server
        ws.send("Ack");
      }
    } catch (error) {
      console.error("Error parsing JSON:", error);
    }
  });

  ws.on("error", function error(err) {
    console.error("Error in WebSocket connection:", err);
  });
} catch (err) {
  console.error("Error in connection:", err);
}
