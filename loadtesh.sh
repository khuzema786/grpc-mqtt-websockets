#!/bin/bash

cd rust/GRPC
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1000 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1000 --clients 1000

# cd ../Websockets
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1000 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1000 --clients 1000

# cd ../MQTT
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 1500 --messages 1000 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 10 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 100 --clients 1000
sh loadtest.sh --simulations 10 --payloadsize 4000 --messages 1000 --clients 1000