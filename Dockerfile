# Build stage
FROM rust:slim-bullseye AS builder

RUN sed -i "s@http://\(deb\|security\).debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        lsb-release \
        ca-certificates \
        libc6 \
        clang \
        libclang-dev \
        cmake \
        libssl-dev \
        autoconf automake libtool curl make g++ unzip \
        lsb-release \
        git \
        pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /robustmq

COPY . .

ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
ENV CARGO_HTTP_MULTIPLEXING=false

#ENV LIBCLANG_PATH=/usr/lib/llvm-11/lib/

RUN cargo fetch

RUN cargo build --release && \
    chmod -R 777 ./bin/*

# Final stage for placement-center
FROM rust:slim-bullseye AS placement-center

RUN sed -i "s@http://\(deb\|security\).debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        lsb-release \
        libc6 \
        clang \
        libclang-dev \
        cmake \
        libssl-dev \
        autoconf automake libtool curl make g++ unzip \
        git \
        pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /robustmq

COPY --from=builder /robustmq/bin/ /robustmq/bin/
COPY --from=builder /robustmq/target/release/placement-center /robustmq/libs/placement-center
#COPY --from=builder /robustmq/config/ /robustmq/config/

ENTRYPOINT ["/robustmq/libs/placement-center","--conf=config/node.toml"]

# Final stage for broker-mqtt
FROM rust:slim-bullseye AS broker-mqtt

RUN sed -i "s@http://\(deb\|security\).debian.org@http://mirrors.aliyun.com@g" /etc/apt/sources.list && \
    apt-get update && \
    apt-get install -y --no-install-recommends \
        lsb-release \
        ca-certificates \
        libc6 \
        clang \
        libclang-dev \
        cmake \
        libssl-dev \
        autoconf automake libtool curl make g++ unzip \
        git \
        pkg-config && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /robustmq

COPY --from=builder /robustmq/bin/ /robustmq/bin/
COPY --from=builder /robustmq/target/release/mqtt-server /robustmq/libs/mqtt-server
#COPY --from=builder /robustmq/config/ /robustmq/config/

ENTRYPOINT ["/robustmq/libs/mqtt-server","--conf=config/node.toml"]