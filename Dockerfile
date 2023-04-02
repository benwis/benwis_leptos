FROM rust:latest as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
RUN echo $PATH
RUN cp cargo-binstall /usr/local/cargo/bin 
RUN cargo binstall cargo-leptos
RUN mkdir -p /app
WORKDIR /app
COPY . .
ENV LEPTOS_BIN_TARGET_TRIPLE = "x86_64-unknown-linux-gnu"
RUN cargo leptos build --release

FROM debian:buster-slim
COPY --from=builder /app/target/server/release/benwis_leptos /app/
COPY --from=builder /app/target/site /app/
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="benwis_leptos"
ENV APP_ENVIRONMENT="production"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
CMD ["benwis_leptos"]