FROM rust:1.61-slim-buster as builder
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated \
    pkg-config python3 libgpgme-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
WORKDIR /src
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir src/ && echo "fn main() {println!(\"failed to build\")}" > src/main.rs
RUN cargo build --release --verbose --bin gpg-tui
RUN rm -f target/release/deps/gpg_tui*
COPY . .
RUN cargo build --locked --release --verbose --bin gpg-tui
RUN mkdir -p build-out && cp target/release/gpg-tui build-out/

FROM debian:buster-slim as runner
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    --allow-unauthenticated \
    libgpgme-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
RUN groupadd -r gpg && \
    useradd -r -g gpg -d /app -s /sbin/nologin gpg-user
WORKDIR /app
COPY --from=builder /src/build-out/gpg-tui .
RUN chown -R gpg-user:gpg /app
USER gpg-user
ENTRYPOINT ["./gpg-tui"]
