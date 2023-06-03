# FROM rust:1 as builder
# WORKDIR /app
# COPY . .
# RUN cargo install --path .


# FROM debian:buster-slim as runner
# #COPY --from=builder /usr/local/cargo/bin/app_backend /usr/local/bin/app_backend
# #COPY --from=builder /usr/local/cargo/src/templates /usr/local/bin/templates
# #COPY --from=builder /usr/local/cargo/ /usr/local/bin/templates
# ENV ROCKET_ADDRESS=0.0.0.0
# EXPOSE 8000
# CMD ["app_backend"]

FROM rust:latest
WORKDIR /usr/src/app_backend
COPY . .
RUN cargo build --release
CMD cargo run