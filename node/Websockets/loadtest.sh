#!/bin/bash

# Default values
simulations=1
payloadsize=1500
messages=1000
clients=1
cores=9

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --simulations) simulations="$2"; shift ;;
        --payloadsize) payloadsize="$2"; shift ;;
        --messages) messages="$2"; shift ;;
        --clients) clients="$2"; shift ;;
        --cores) cores="$2"; shift ;;
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

totalDurations=0
totalFailures=0

mkdir -p temp

for i in $(seq 1 $simulations); do
    simulationDurations=0
    simulationClients=$clients

    # Keep track of running clients
    totalClientsStarted=0
    concurrentClients=0

    # Array to keep track of client PIDs
    client_pids=()

    while [ $totalClientsStarted -lt $clients ]; do
        # Start new clients if we haven't reached the max concurrency
        while [ $concurrentClients -lt $cores ] && [ $totalClientsStarted -lt $clients ]; do
            log_file="temp/client_$(($totalClientsStarted + 1))_log.txt"
            node client.js > "$log_file" &
            client_pids+=($!)
            totalClientsStarted=$((totalClientsStarted + 1))
            concurrentClients=$((concurrentClients + 1))
        done

        # Poll for finished clients and start new ones if slots are available
        new_pids=()
        for pid in "${client_pids[@]}"; do
            if kill -0 "$pid" 2>/dev/null; then
                # Client is still running, add to the new array
                new_pids+=("$pid")
            else
                # Client has finished, decrement the counter
                concurrentClients=$((concurrentClients - 1))
            fi
        done
        client_pids=("${new_pids[@]}")

        # sleep 1  # Adjust the sleep duration as needed
    done

    # Wait for any remaining clients to finish
    for pid in "${client_pids[@]}"; do
        wait $pid
    done

    # Process logs and calculate durations
    for j in $(seq 1 $simulationClients); do
        log_file="temp/client_${j}_log.txt"
        if [ -f "$log_file" ]; then
            duration=$(awk '/Total time for [0-9]+ messages: [0-9]+ ms/ {print $(NF-1)}' "$log_file")
            if [[ -z "$duration" ]]; then
                totalFailures=$((totalFailures + 1))
            else
                simulationDurations=$((simulationDurations + duration))
            fi
        fi
    done

    # Calculate average for this simulation for all clients
    simulationAverage=$(echo "scale=2; $simulationDurations / $simulationClients" | bc)
    echo "Average Duration for Simulation $i for $simulationClients clients : $simulationAverage ms"
    totalDurations=$(echo "$totalDurations + $simulationAverage" | bc)
done

rm -rf temp

# Calculate overall average
overallAverage=$(echo "scale=2; $totalDurations / $simulations" | bc)

echo "Overall Average Duration From All Simulations: $overallAverage ms"
echo "Overall Client Connection Failures From All Simulations: $totalFailures ms"
