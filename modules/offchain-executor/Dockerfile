FROM rust:1.67.0

WORKDIR /src/
COPY ./ ./

RUN apt-get update && \
    apt-get install -y openjdk-11-jre-headless && \
    apt-get clean;

RUN cargo build --release

RUN wget https://github.com/ergonames/transaction-utils/releases/download/str/ergonames-transaction-utils.jar

CMD ["./target/release/ergonames-offchain-executor"]