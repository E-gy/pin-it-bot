# Build with musl
FROM ekidd/rust-musl-builder:latest as builder
USER root

# First build (&cache) deps
WORKDIR /home/rust/app
RUN cargo install cargo-build-deps
COPY ./Cargo.* ./
RUN cargo build-deps --release

# Build the app
COPY ./src ./src
RUN cargo build --release

# Run in Alpine
FROM alpine:latest
# Set up user
ARG APP_USER=rust
RUN addgroup -S $APP_USER && adduser -S -g $APP_USER $APP_USER
# Update stuff
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*
# Copy exe, expose, and run
USER $APP_USER
WORKDIR /usr/app
COPY --from=builder /home/rust/app/target/x86_64-unknown-linux-musl/release/pin-it-bot ./pin-it-bot
CMD ["./pin-it-bot"]
