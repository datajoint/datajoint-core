FROM ubuntu:latest

RUN apt-get update && \
    apt-get install \
    git \
    ca-certificates \
    curl \
    gcc \
    libc6-dev \
    openssl \
    libssl-dev \
    pkg-config \
    -qqy \
    --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

COPY . .

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo build