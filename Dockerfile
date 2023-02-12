FROM rust:alpine as base

COPY . /src

RUN rustup update 1.64 && rustup default 1.64

RUN apk update \
    && apk add \
        gcc \
        g++

RUN cd /src && cargo build --release

FROM alpine as tool

COPY --from=base /src/target/release/oxipng /usr/local/bin

WORKDIR /src
ENTRYPOINT [ "oxipng" ]
CMD [ "--help" ]
