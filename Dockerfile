# Build
FROM clux/muslrust:stable AS builder

WORKDIR /volume

COPY . .

RUN cargo build --release

# Run
FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache cronie

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/portsmith /usr/local/bin/portsmith
COPY --from=builder /volume/entrypoint.sh /usr/local/bin/entrypoint.sh

RUN echo "*/5 * * * * cd /app && /usr/local/bin/portsmith >> /proc/1/fd/1 2>> /proc/1/fd/2" > /etc/crontabs/root

ENTRYPOINT ["entrypoint.sh"]
