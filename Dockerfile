FROM rust:slim as build

WORKDIR /
COPY . .

RUN cargo build --release

EXPOSE 8000

CMD ["cargo", "run", "--release"]
