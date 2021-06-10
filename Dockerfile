FROM rust:slim-buster as rust

FROM debian:stable-slim
RUN apt update
RUN apt install -y python3 python3-dev curl gcc pkg-config libssl-dev xvfb libxi6 libnss3 wget
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt install -y nodejs
RUN wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb
RUN apt install -y ./google-chrome-stable_current_amd64.deb

RUN adduser app
USER app
WORKDIR /home/app
COPY --from=rust --chown=app:app /usr/local/cargo /home/app/.cargo
ENV PATH="/home/app/.cargo/bin:$PATH"
RUN rustup install stable
RUN cargo install wasm-pack
RUN mkdir nc2
WORKDIR nc2