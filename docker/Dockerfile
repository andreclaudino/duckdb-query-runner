FROM docker.io/rust:1.70-slim-buster as builder

ARG binary_name

RUN apt-get update && \
    apt-get install -y \
        build-essential \
        libssl-dev -y \
        pkg-config \
        ca-certificates && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

WORKDIR /application

ADD Cargo.toml /application/Cargo.toml
ADD src /application/src


RUN cargo test && \
    cargo build --release && \
    mkdir /application/bin && \
    mv /application/target/release/${binary_name} /application/bin/${binary_name}

# Final image
FROM docker.io/debian:buster-slim

ARG binary_name

LABEL "org.opencontainers.image.source"="https://github.com/kognitalab/actionooh_campaign_api-rs"
LABEL "org.opencontainers.image.revision"="${GIT_REFERENCE}"
LABEL "org.opencontainers.image.version"="${VERSION}"

WORKDIR /application

RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        libssl-dev -y \
        libgomp1 && \
    rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/* && \
    update-ca-certificates

COPY --from=builder /application/bin/${binary_name} /usr/bin/
RUN ln /usr/bin/${binary_name} /usr/bin/run