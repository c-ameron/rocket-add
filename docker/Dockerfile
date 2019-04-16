FROM rustlang/rust:nightly AS builder
WORKDIR /workdir
ENV CARGO_HOME=/workdir/.cargo
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./.cargo ./.cargo
RUN rustup show && \
	  mkdir -p ./src && \
      echo 'fn main() {}' > ./src/main.rs && \
      cargo +nightly build --release
COPY ./src ./src
RUN ls -lAR ./src && cargo +nightly build --release

FROM debian:stretch-slim
EXPOSE 8000
COPY --from=0 /workdir/target/release/rocket-add /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rocket-add"]
