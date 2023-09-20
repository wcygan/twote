FROM rust:1.72-alpine3.18

WORKDIR /usr/src/service

COPY . .

RUN cargo install --path .

CMD ["twote-api"]
