FROM rustlang/rust:nightly-bullseye as builder

# RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz
# RUN tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz
# RUN echo $PATH
# RUN cp cargo-binstall /usr/local/cargo/bin 
# RUN cargo binstall cargo-leptos -y
#RUN cargo install --git https://github.com/akesson/cargo-leptos cargo-leptos
RUN cargo install --locked cargo-leptos
RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown
RUN mkdir -p /app
WORKDIR /app
COPY . .
ENV LEPTOS_BIN_TARGET_TRIPLE="x86_64-unknown-linux-gnu"
RUN cargo leptos --manifest-path=./Cargo.toml build --release -vv

FROM rustlang/rust:nightly-bullseye as runner
COPY --from=builder /app/target/server/x86_64-unknown-linux-gnu/release/benwis_leptos /app/
COPY --from=builder /app/target/site /app/site
COPY --from=builder /app/db /app/db
COPY --from=builder /app/Cargo.toml /app/
WORKDIR /app
ENV RUST_LOG="info"
ENV LEPTOS_OUTPUT_NAME="benwis_leptos"
ENV LEPTOS_ENVIRONMENT="prod_no_trace"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 3000
CMD ["/app/benwis_leptos"]