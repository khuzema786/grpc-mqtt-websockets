# Steps to loadtest Rust Code
1. Update file descriptors to 4000 to handle large number of open connections during load test `ulimit -n 4000`
2. For MQTT, install mosquitto broker `brew install mosquitto` and `brew services start mosquitto`
3. Configure the ***loadtest.sh*** to run clients concurrently and transfer messages and then run, `sh loadtest.sh`