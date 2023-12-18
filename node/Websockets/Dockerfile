# Use an official Node runtime as the base image
FROM node:14

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy package.json and package-lock.json
COPY package*.json ./

# Install any dependencies
RUN npm install

# Copy the rest of your application's code
COPY . .

# Your application binds to port 8080, so use the EXPOSE instruction to have it mapped by the Docker daemon
EXPOSE 8080

# Define the command to run your app using CMD
CMD ["node", "server.js"]
