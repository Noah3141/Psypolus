# Get me a rust ready linux to start our build
# FROM rust:latest as builder

# # Set the working directory to /app (note, the docker file is still 'running from' where it is)
# WORKDIR /app

# # Copy everything from . (where the docker file is in filesystem) to . (where the current working directory is within the image)
# COPY . .
# # everything adjacent to the dockerfile (rocket.toml, cargo.toml, src/) are all there
# # cargo install creates a Rust binary inside the linux usr/local/cargo/bin folder, rather than anywhere nearby
# # Look at our newly copied-to-working-directory-within-image project, and run an install-style build with it:
# RUN cargo install --path .


# FROM debian:buster-slim as runner
# # Copy this *file* from the builder linux microuniverse we labeled above, to the debian we're talking about now
# COPY --from=builder /usr/local/cargo/bin/app_backend /usr/local/bin/app_backend
# # Testing this new line:
# COPY templates /usr/local/bin/
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 8000
# CMD ["app_backend"]



# Use a base image with the latest version of Rust installed
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the local application code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Specify the command to run when the container starts
CMD ["./target/release/app_backend"]