# Steps to loadtest Rust Code
1. Update file descriptors to 4000 to handle large number of open connections during load test `ulimit -n 4000`
2. For MQTT, install mosquitto broker `brew install mosquitto` and `brew services start mosquitto`
3. Configure the ***loadtest.sh*** to run clients concurrently and transfer messages and then run, `sh loadtest.sh`

# Comparison between different communication protocols

## MQTT
It works on TCP/IP protocol, and is lightweight to work with IoT devices like smart bulbs, sensors etc. It works on the Pub/Sub model and the connection is not persistent. MQTT client will connect and disconnect to read/write messages like ping. MQTT provides Quality of Service (QoS)level, which is an agreement between the sender of a message and the receiver of a message that defines the guarantee of delivery for a specific message. 
There are 3 QoS levels in MQTT:
  - At most once (0) at most once.
  - At least once (1)at least once.
  - Exactly once (2) exactly once.

In MQTT, when a client subscribes to a topic, it typically receives only the messages that are published after the subscription is made. However, MQTT provides a feature called "retained messages" which allows a client to receive the last message that was published to a topic, even if it was published before the client subscribed. If a new retained message is published to the topic, it replaces the previous one. When a new client subscribes to a topic that has a retained message, the MQTT broker immediately sends this retained message to the client. This helps the client to get the last known good value.

AWS MQTT Broker Price ~ 16,757 Rs

## WebSocket
It is based on HTTP 1.1 protocol and came to support bi-directional communication which was not supported with plain HTTP 1.1 protocol. Websockets maintain persistent connection. Message delivery order will be maintained and there are many libraries and frameworks to support websocket and pub/sub over websockets.

## GRPC
This would look similar to websockets but underlying difference is it works on HTTP2 protocol and the data format for request response would be bound to Protobuf, cannot use JSON or XML. But protobuf is more compact and light weight than the latter. The connection would be persistent and the client can invoke the methods in the remote server through the connection as needed. It offers 4 types of method call, traditional request/response model, server-side streaming, client side streaming and bi-directional streaming.
UBER uses GRPC for sending reliable and high performance push notifications to its clients.

<img width="684" alt="Screenshot 2023-12-20 at 10 03 33 PM" src="https://github.com/khuzema786/grpc-mqtt-websockets/assets/38260510/6bdae4a4-bfcb-4834-b4a7-8384f036e62a">

# Load Testing
I conducted load testing for GRPC, Websockets, and MQTT using Rust, transitioning from Node.js due to its limitations in handling concurrent client connections being single-threaded in nature.
The following are the summarized results of load test performed for MQTT/GRPC/Websockets, which measures the average round trip duration for (1, 10, 100, 1000) messages of size (1.5, 4 KB) sent from the server and the acknowledgements received from clients for each message along with the serialization and deserialization of the message payloads as JSON/Protobuf. This test was conducted with clients (1, 100) concurrently connected to the server/broker, measuring duration of the complete cycle of message transmission and response. I have taken an average of durations across 10 samples for message round trips across a client.

<img width="970" alt="Screenshot 2023-12-20 at 10 04 39 PM" src="https://github.com/khuzema786/grpc-mqtt-websockets/assets/38260510/0802ff6d-01c4-4e6c-a5cd-19448de87d84">
<img width="969" alt="Screenshot 2023-12-20 at 10 04 54 PM" src="https://github.com/khuzema786/grpc-mqtt-websockets/assets/38260510/2c2c5dbe-3e8f-436d-88a4-2bec1159b713">

## Observations
1. In Websockets & GRPC performance looks almost comparable for a smaller payload size.
2. In MQTT the duration increases a lot when 100 connections are open concurrently and exchange that many messages concurrently. This increase could be because of the eclipse-mosquitto broker that I am using locally but in order to have a scalable and reliable broker If we use AWS managed service, it may increase infra cost which I calculated to be around 21,000 Rs per Month but not sure if I estimated it correctly.
3. In MQTT if a client disconnects from the broker and the notification is published to the topic on the broker by the server then it gets dropped. Inorder to keep track of messages and handle retries we may need to have same redis stream queue based implementation even in case of MQTT as well similar to GRPC or Websockets.
4. In case of GRPC/Websockets, upon client connection the server can lookup in redis stream for pending messages to be sent but in case of MQTT publisher wouldn't get to know if the client is connected to the topic so handling message retries from publisher could be a little tricky.

## Estimations
1. In GRPC, 1 open connection requires 60 KB. For 50,000 online drivers. Memory required for handling 50,000 open connections is 3 GB.
2. In Websockets, 1 open connection requires 30 KB. For 50,000 online drivers. Memory required for handling 50,000 open connections is 1.5 GB.



