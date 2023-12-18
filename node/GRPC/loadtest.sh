#!/bin/bash

# Default values
simulations=1
payloadsize=1500
messages=1000
clients=1

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --simulations) simulations="$2"; shift ;;
        --payloadsize) payloadsize="$2"; shift ;;
        --messages) messages="$2"; shift ;;
        --clients) clients="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo "Running $simulations simulations for each of $clients clients with payload size $payloadsize and $messages messages"

# Function to clean up background processes on exit
cleanup() {
    echo "Cleaning up..."
    CONTAINER_ID=$(docker ps -q --filter "publish=50051")
    # If a container is found, stop and remove it
    if [ ! -z "$CONTAINER_ID" ]; then
        docker stop $CONTAINER_ID
        docker rm $CONTAINER_ID
    fi
    pkill -P $$ # Kill all child processes of this script
}

# Set trap to call cleanup function on script exit or interrupt
trap cleanup EXIT INT

# Start the server in the background
# docker build -t node-grpc-server .
docker run -e "PAYLOAD_SIZE=$payloadsize" -e "TOTAL_MESSAGES=$messages" -d -p 50051:50051 node-grpc-server

sleep 2

node --max-old-space-size=4096 client.js $simulations $clients