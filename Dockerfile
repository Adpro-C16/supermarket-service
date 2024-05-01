# from debian
FROM debian:buster-slim

RUN apt install -y protobuf-compiler libprotobuf-dev

# install rust
RUN apt-get update && apt-get install -y curl
RUN apt-get install -y build-essential libssl-dev pkg-config
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

# copy all codes
COPY . .

# build
RUN cargo build --release

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
EXPOSE 8080

# run
CMD ["./target/release/management"]