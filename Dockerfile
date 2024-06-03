# Use the official Rust Docker image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the project files into the container
COPY . .

# Build the project and run the tests
RUN cargo build --release

# Use the same Rust image to run the tests
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/myapp

# Copy the built project from the builder stage
COPY --from=builder /usr/src/myapp/target /usr/src/myapp/target
COPY --from=builder /usr/src/myapp/Cargo.toml /usr/src/myapp/Cargo.toml
COPY --from=builder /usr/src/myapp/src /usr/src/myapp/src
COPY --from=builder /usr/src/myapp/tests /usr/src/myapp/tests

# Define environment variables
ENV API_BASE_URL=
ENV API_KEY=
ENV API_SECRET=
ENV CARGO_TARGET_DIR=/usr/src/myapp/target

# Run the BDD tests
CMD ["cargo", "test"]