# syntax=docker/dockerfile:experimental
FROM ubuntu:22.04

# Setup cargo and base deps
RUN apt update && apt install -y \
  git \
  curl \
  jq \
  gcc \
  pkg-config \
  libssl-dev \
  libsqlite3-dev \
  libpq-dev \
  python3
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /breez-liquid
COPY . .

# Build the application
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/breez-liquid/target \
    cargo test --no-run

# Setup bitcoin core
RUN curl -lL https://bitcoincore.org/bin/bitcoin-core-26.0/bitcoin-26.0-x86_64-linux-gnu.tar.gz -o /tmp/bitcoin.tar.xz
RUN tar xf /tmp/bitcoin.tar.xz -C /tmp
RUN rm /tmp/bitcoin.tar.xz
RUN mkdir /root/.bitcoin && cp ./cfg/bitcoin.conf /root/.bitcoin
ENV PATH="/tmp/bitcoin-26.0/bin:${PATH}"

# Setup cln
RUN curl -lL https://github.com/ElementsProject/lightning/releases/download/v23.11.2/clightning-v23.11.2-Ubuntu-22.04.tar.xz -o /tmp/cln.tar.xz
RUN tar xf /tmp/cln.tar.xz -C /tmp && mv /tmp/usr /tmp/clightning
RUN rm /tmp/cln.tar.xz
RUN mkdir -p /tmp/.lightning/swapper
RUN mkdir -p /tmp/.lightning/user
ENV PATH="/tmp/clightning/bin:${PATH}"

CMD ["./startup-network.sh"]
