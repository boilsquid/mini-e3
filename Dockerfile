FROM rust:1.74-bookworm

WORKDIR /usr/src/mini-e3

COPY . .

RUN cargo install --path .

CMD ["mini-e3"] 