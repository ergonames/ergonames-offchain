FROM rust:1.67.0

WORKDIR /src/
COPY ./ ./

RUN cargo build --release
CMD ["./target/release/ergonames-sync-engine"]