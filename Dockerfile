FROM rust:1.92 AS builder
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM debian:trixie-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates
COPY --from=builder /app/target/release/st2-rs /
COPY --from=builder /app/.env /
COPY --from=builder /app/tokencache.json /
COPY --from=builder /app/client_secret.json /
CMD ["/st2-rs"]
