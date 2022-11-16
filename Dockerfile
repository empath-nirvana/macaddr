FROM rust:1.65.0 as builder
WORKDIR /usr/src/macaddr
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/macaddr /usr/local/bin/macaddr
ENTRYPOINT ["macaddr"]