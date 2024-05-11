FROM rust:1 as build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/echo-server /
ENTRYPOINT ["/echo-server"]
