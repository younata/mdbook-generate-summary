FROM rust:slim

WORKDIR /code
COPY . /code
RUN cargo build --release && cp target/release/mdbook-generate-summary /usr/local/bin/

