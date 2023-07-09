# Use an official Rust runtime as a parent image
FROM rust:1.70.0

# Set the working directory in the container to /usr/src/myapp
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/myapp
COPY . .

# Build the application
RUN cargo build --release

# Run the application when the container launches
CMD ["cargo", "run", "--bin", "storm-app"]