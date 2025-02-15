FROM --platform=$BUILDPLATFORM rust:latest as builder
WORKDIR /usr/src/mindx
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/mindx /usr/bin/mindx
ENV USER root
ENV mindx_RUN_IN_DOCKER=true
RUN mkdir /io
WORKDIR /io
ENTRYPOINT ["/usr/bin/mindx"]

# mindx-0.6.12
