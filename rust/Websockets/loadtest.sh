#!/bin/bash

# Default values
simulations=1
payloadsize=1500
messages=1000
clients=1
max_clients_per_batch=100

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

rm -rf logs
mkdir -p logs

client_pids=() # Array to store client PIDs

# Function to clean up background processes on exit
cleanup() {
    echo "Cleaning up..."    
    # Kill all clients after simulation
    for pid in "${client_pids[@]}"; do
        kill "$pid" 2>/dev/null
    done
    kill $SERVER_PID
    pkill -P $$ # Kill all child processes of this script
}

# Set trap to call cleanup function on script exit or interrupt
trap cleanup EXIT INT

# Set environment variables and start server in the background
TOTAL_MESSAGES=$messages PAYLOAD_SIZE=$payloadsize cargo run --bin server > logs/server.log 2>&1 &

# Store server's PID to later terminate it
SERVER_PID=$!

# Function to check if the server is up by running the client and checking server logs
server_is_up() {
    # Run the client with 1 client in the background
    TOTAL_CLIENTS=1 cargo run --bin client &

    # Store the client's PID
    CLIENT_PID=$!

    # Allow some time for the client to attempt a connection
    sleep 2

    # Check the server log for a specific success message
    if grep -q "Connection Successful" logs/server.log; then
        # If success message is found, kill the client process and return success
        kill $CLIENT_PID
        return 0
    else
        # If success message is not found, kill the client process and return failure
        kill $CLIENT_PID
        return 1
    fi
}

# Wait for the server to start
echo "Waiting for the server to start..."
until server_is_up; do
    sleep 1
done
echo "Server is up and running."

# Initialize total average time
total_average_time=0
total_failures=0

for simulation in $(seq 1 $simulations); do
    total_time=0
    completed_clients=0

    # One process can only use around 140 file descriptors, so we can batch the clients into a group of 100
    for (( i = 0; i < $clients; i += $max_clients_per_batch )); do
        batch_clients=$((i + max_clients_per_batch))
        if [ $batch_clients -gt $clients ]; then
            batch_clients=$clients
        fi

        # Start the client and redirect output to a log file
        TOTAL_CLIENTS=$((batch_clients - i)) cargo run --bin client >> logs/simulation_$simulation.log 2>&1 &
        
        # Store the client's PID
        CLIENT_PID=$!
        client_pids+=("$CLIENT_PID")

        sleep 2
    done

    # Wait for all clients to report their times
    # while [ $completed_clients -lt $clients ]; do
    #     # Check the log file for new complete messages and update the count and total time
    #     if grep -q "Client [0-9]\+: Total time: [0-9]\+ ms" logs/simulation_$simulation.log; then
    #         total_time=$(grep "Client [0-9]\+: Total time: [0-9]\+ ms" logs/simulation_$simulation.log | awk '{sum += $5} END {print sum}')            
    #         completed_clients=$(grep -c "Client [0-9]\+: Total time: [0-9]\+ ms" logs/simulation_$simulation.log)
    #     fi
    #     sleep 1 # Sleep for a short time before checking again
    # done

    total_time=$(grep "Client [0-9]\+: Total time: [0-9]\+ ms" logs/simulation_$simulation.log | awk '{sum += $5} END {print sum}')            
    completed_clients=$(grep -c "Client [0-9]\+: Total time: [0-9]\+ ms" logs/simulation_$simulation.log)
    failures=$((clients - completed_clients))
    total_failures=$((total_failures + failures))
    echo "Failures during Simulation $simulation for $clients clients : $failures"

    simulationAverage=$(echo "scale=2; $total_time / $completed_clients" | bc)
    total_average_time=$(echo "$total_average_time + $simulationAverage" | bc)
    echo "Average Duration for Simulation $simulation for $completed_clients clients : $simulationAverage ms"

    # Kill all clients after simulation
    for pid in "${client_pids[@]}"; do
        kill "$pid" 2>/dev/null
    done
done

simulationAverage=$(echo "scale=2; $total_average_time / $simulations" | bc)
echo "Average Duration for $clients clients across $simulations simulations: $simulationAverage ms"
echo "Failures across $simulations simulations: $total_failures"