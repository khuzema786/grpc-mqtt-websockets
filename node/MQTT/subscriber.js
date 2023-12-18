const mqtt = require("mqtt");
const client = mqtt.connect("mqtt://localhost");

const topic = "loadtest/messages-1";
const ackTopic = "loadtest/acknowledgements-1";

client.on("connect", () => {
  console.log("MQTT client is connected");
  client.subscribe(topic);
});

client.on("message", (receivedTopic, message) => {
  console.log(`MQTT message ${receivedTopic}`);
  if (receivedTopic === topic) {
    // Process the message and send an acknowledgment
    client.publish(ackTopic, "Acknowledged");
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
