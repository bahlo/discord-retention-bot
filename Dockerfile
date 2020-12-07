FROM rust:alpine AS builder
WORKDIR /app
RUN apk add --no-cache libc-dev
COPY . /app
RUN cargo build --release

FROM alpine
COPY --from=builder /app/target/release/discord-retention-bot /usr/local/bin/discord-retention-bot
CMD ["discord-retention-bot"]