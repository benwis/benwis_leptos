FROM rust:bookworm AS builder

RUN apt-get update && apt-get install -y --no-install-recommends mold clang && rm -rf /var/lib/apt/lists/*

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin \
    && cargo binstall cargo-leptos -y \
    && rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/benwis_leptos /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/db /app/db
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="benwis_leptos"
ENV LEPTOS_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/benwis_leptos"]
