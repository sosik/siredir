FROM rustlang/rust:nightly-slim as build

WORKDIR /app

COPY Cargo.toml /app/
COPY src/ /app/src

RUN cargo build --release


FROM rustlang/rust:nightly-slim

WORKDIR /app

COPY --from=build /app/target/release/siredir /app/

ENTRYPOINT ["/app/siredir"]
