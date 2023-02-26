FROM rust:1.67.0

WORKDIR /src/
COPY ./ ./

RUN apt-get update && \
    apt-get install -y openjdk-11-jre-headless && \
    apt-get clean;

RUN cargo build --release

RUN wget https://github.com/ergonames/transaction-utils/releases/download/v0.0.1/ergonames-transaction-utils.jar

CMD ["./target/release/ergonames-chain-tracker"]