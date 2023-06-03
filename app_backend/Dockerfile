# Get me a rust ready linux to start our build
FROM rust:latest as builder

# Set the working directory to /app (note, the docker file is still 'running from' where it is)
WORKDIR /app

# Copy everything from . (where the docker file is in filesystem) to . (where the current working directory is within the image)
COPY . .
# everything adjacent to the dockerfile (rocket.toml, cargo.toml, src/) are all there
# cargo install creates a Rust binary inside the linux usr/local/cargo/bin folder, rather than anywhere nearby
# Look at our newly copied-to-working-directory-within-image project, and run an install-style build with it:
RUN cargo install --path .


FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/app_backend /usr/local/bin/app_backend
# Testing this new line:
COPY templates /usr/local/bin/app_backend
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["app_backend"]

# FROM rust:latest
# WORKDIR /usr/src/app_backend
# COPY . .
# RUN cargo build --release
# CMD cargo run