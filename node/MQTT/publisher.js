const mqtt = require("mqtt");
const client = mqtt.connect("mqtt://localhost");

const topic = "loadtest/messages-1";
const ackTopic = "loadtest/acknowledgements-1";

client.on("connect", () => {
  console.log("MQTT client is connected");

  let messageCount = 0;
  const payloadSize = parseInt(process.argv[2]); // 1500 : Size in bytes (1.5KB)
  const totalMessages = parseInt(process.argv[3]); // 1000, 10000, 100000
  const startTime = Date.now();

  // Function to generate a 1.5KB message
  function generatePayload(size) {
    return "x".repeat(size);
  }

  client.subscribe(ackTopic);

  client.on("message", (receivedTopic, message) => {
    // console.log(`MQTT message ${receivedTopic} : ${message}`);
    if (receivedTopic === ackTopic) {
      messageCount++;
      if (messageCount === totalMessages) {
        const endTime = Date.now();
        const duration = endTime - startTime;
        console.log(`Total time for ${totalMessages} messages: ${duration} ms`);
        client.end();
      }
    }
  });

  for (let i = 0; i < totalMessages; i++) {
    const payload = generatePayload(payloadSize);
    client.publish(topic, payload);
  }
});

client.on("error", (error) => {
  console.error("MQTT Client Error:", error);
  client.end();
});

client.on("offline", () => {
  console.log("MQTT client is offline");
});

client.on("reconnect", () => {
  console.log("Attempting to reconnect...");
});
