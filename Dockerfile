FROM ubuntu
MAINTAINER Addison Crump <me@addisoncrump.info>

EXPOSE 8080

ENV SOURCES=/sources

RUN apt update -y
RUN apt install -y file build-essential pkg-config libssl-dev curl
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --disable-sudo
RUN mkdir -p $SOURCES
ADD ./ $SOURCES
WORKDIR $SOURCES
RUN cargo build --release
CMD ROCKET_ENV=production ./target/release/latex-bot
