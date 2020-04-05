FROM rust:1.42-stretch

RUN apt-get update && \
	apt-get install -y gcc-arm-linux-gnueabihf
RUN rustup target add armv7-unknown-linux-gnueabihf
COPY conf/.cargo/config $HOME/.cargo/config

RUN mkdir /app
WORKDIR /app

ADD Cargo.toml /app/Cargo.toml
ADD src /app/src

CMD ["cargo", "build", "--release", "--target=armv7-unknown-linux-gnueabihf"]
