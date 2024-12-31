# Build
FROM clux/muslrust:stable AS builder

WORKDIR /volume

COPY . .

RUN cargo build --release

# Run
FROM alpine:latest

WORKDIR /app

RUN apk add --no-cache cronie

COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/portwatch /usr/local/bin/portwatch
COPY --from=builder /volume/entrypoint.sh /usr/local/bin/entrypoint.sh

RUN echo "*/5 * * * * cd /app && /usr/local/bin/portwatch >> /proc/1/fd/1 2>> /proc/1/fd/2" > /etc/crontabs/root

ENTRYPOINT ["entrypoint.sh"]
