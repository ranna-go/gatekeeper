FROM rust:alpine AS build
WORKDIR /build
COPY src/ src/
COPY Cargo.toml .
COPY Cargo.lock .
RUN cargo build --release

FROM alpine:latest AS final
WORKDIR /app
COPY --from=build /build/target/release/ranna-gatekeeper gatekeeper
ENV BINDADDRESS=0.0.0.0:80
EXPOSE "80"
ENTRYPOINT ["/app/gatekeeper"]