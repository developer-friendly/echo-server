FROM rust:1 AS build

WORKDIR /app
COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot

EXPOSE 3000
COPY --from=build /app/target/release/echo-server /

ENTRYPOINT ["/echo-server"]
