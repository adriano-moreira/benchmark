FROM rust AS build

WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu
WORKDIR /app
COPY --from=build /app/target/release/rust_project /app/bin/

ENTRYPOINT ["./bin/rust_project"]
