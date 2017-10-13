FROM rust:1.19.0

WORKDIR /home/chess-engine

COPY . .

RUN cargo install

RUN cargo build --release
