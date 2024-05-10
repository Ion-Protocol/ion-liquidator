FROM rust:1.76 as builder

RUN apt-get update && apt-get install -y llvm clang libclang-dev

RUN USER=root cargo new --bin ion-liquidator
WORKDIR /ion-liquidator

COPY ./Cargo.toml ./Cargo.lock ./

# Create a dummy `src/main.rs` to satisfy Cargo's build process for dependency caching
RUN mkdir -p src && echo "fn main() {println!(\"if you see this, the cache broke\")}" >src/main.rs

# Also, set up a dummy `bindings` directory if `bindings` is a path dependency
RUN mkdir -p bindings/src && echo "// This is a dummy file" >bindings/src/lib.rs && echo "[package]\nname = \"bindings\"\nversion = \"0.1.0\"\nedition = \"2018\"\n\n[lib]\npath = \"src/lib.rs\"" >bindings/Cargo.toml

RUN cargo build --release -j $(nproc)

# Remove the dummy src and bindings to avoid conflicts with actual source
RUN rm -rf bindings/

COPY ./bindings ./bindings

RUN cargo build --release -j $(nproc)

RUN rm -rf src/

COPY ./src ./src

RUN cargo build --release -j $(nproc)

FROM ubuntu:22.04

RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /ion-liquidator/target/release/ion-liquidator /usr/local/bin/ion-liquidator

COPY entrypoint.sh /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]

# CMD ["ion-liquidator"]
