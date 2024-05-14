# from debian
FROM debian:buster-slim AS builder

# install rust
RUN apt-get update && apt-get install -y curl
RUN apt-get install -y build-essential libssl-dev pkg-config
RUN apt install -y protobuf-compiler libprotobuf-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN rustup default nightly

# create a new directory in the image and set it as the working directory
WORKDIR /app

# copy project files
COPY . .

RUN cargo build --release 

FROM gcr.io/distroless/cc-debian11 AS runner
# # set the working directory in the runner stage
# WORKDIR /app
COPY --from=builder /target/release/management /management

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080

# WORKDIR /app
EXPOSE 8080

CMD ["/management"]