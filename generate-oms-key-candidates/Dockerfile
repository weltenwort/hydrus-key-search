FROM rust:1.58.1-buster AS builder
ARG SRC_DIR=/usr/src/generate-oms-key-candidates
WORKDIR ${SRC_DIR}
COPY . ${SRC_DIR}/
RUN cargo build --release

FROM builder AS debugger
RUN apt-get install -y --no-install-recommends linux-perf linux-base \
  && rm -rf /var/lib/apt/lists/*

FROM debian:buster-slim
ARG SRC_DIR=/usr/src/generate-oms-key-candidates
COPY --from=builder ${SRC_DIR}/target/release/generate-oms-key-candidates /usr/local/bin/
