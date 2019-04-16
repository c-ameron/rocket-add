FROM debian:stretch-slim
EXPOSE 8000
COPY ./target/release/rocket-add /usr/local/bin
ENTRYPOINT ["/usr/local/bin/rocket-add"]
