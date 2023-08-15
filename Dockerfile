FROM rust:slim as build

WORKDIR /myProjects/home-file-server
COPY . .

RUN cargo build

EXPOSE 8000

CMD ["cargo", "r"]
