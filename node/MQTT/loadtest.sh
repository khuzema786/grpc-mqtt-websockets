#!/bin/bash

# Default values
simulations=1
payloadsize=1500
messages=1000

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --simulations) simulations="$2"; shift ;;
        --payloadsize) payloadsize="$2"; shift ;;
        --messages) messages="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo "Running $simulations simulations with payload size $payloadsize and $messages messages each"

# Function to clean up background processes on exit
cleanup() {
    echo "Cleaning up..."
    kill $server_pid
}

# Set trap to call cleanup function on script exit or interrupt
trap cleanup EXIT INT

# Start the client in the background to listen for published messages
node subscriber.js &
server_pid=$!

sumDurations=0

for i in $(seq 1 $simulations); do
    # Run the script and grep for the specific log line
    duration=$(node publisher.js $payloadsize $messages | awk '/Total time for [0-9]+ messages: [0-9]+ ms/ {print $(NF-1)}')
    echo "Run $i: Duration = $duration ms"
    sumDurations=$((sumDurations + duration))
done

# Calculate average
average=$(echo "scale=2; $sumDurations / $simulations" | bc)

echo "Average Duration: $average ms"