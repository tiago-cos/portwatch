# Build
FROM clux/muslrust:stable AS builder

WORKDIR /volume

COPY . .

RUN cargo build --release

# Run
FROM alpine:latest

RUN apk add --no-cache cronie

COPY target/x86_64-unknown-linux-musl/release/portsmith /usr/local/bin/portsmith

RUN echo "*/5 * * * * /usr/local/bin/portsmith" > /etc/crontabs/root

CMD ["crond", "-f"]