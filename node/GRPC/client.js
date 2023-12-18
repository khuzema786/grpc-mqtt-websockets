const grpc = require("@grpc/grpc-js");
const protoLoader = require("@grpc/proto-loader");
const packageDefinition = protoLoader.loadSync("message.proto");
const { loadtest } = grpc.loadPackageDefinition(packageDefinition);

(async () => {
  const simulation = process.argv[2];

  for (let i = 0; i < simulation; i++) {
    const clients = process.argv[3];

    const timeTaken = await Promise.all(
      Array.from({ length: clients }, async () => {
        return new Promise((res, rej) => {
          const client = new loadtest.LoadTester(
            "localhost:50051",
            grpc.credentials.createInsecure()
          );

          // Create the bidirectional stream
          const stream = client.StreamPayload();

          stream.on("data", (response) => {
            const match = /Total time for [0-9]+ messages: ([0-9]+) ms/.exec(
              response.message
            );
            if (match) {
              res(parseInt(match[1], 10));
            } else {
              // Send acknowledgment back to the server
              stream.write({ payload: "Acknowledged" });
            }
          });

          stream.on("error", (err) => {
            console.error("Stream encountered an error:", err);
            rej(err);
          });

          stream.on("end", () => {
            console.log("Stream ended by server.");
          });
        });
      })
    );
    console.log(
      `Time taken from simulation ${i + 1} for ${clients} clients:
      ${timeTaken}
      Maximum : ${Math.max(...timeTaken)} ms
      Average : ${
        timeTaken.reduce((acc, el) => acc + el, 0) / timeTaken.length
      } ms`
    );
  }
})();
