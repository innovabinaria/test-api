FROM rust:alpine3.20 AS builder
RUN apk add musl-dev --no-cache
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.20
WORKDIR /app
COPY --from=builder /src/target/release/hello_world .
ENTRYPOINT [ "./hello_world" ]