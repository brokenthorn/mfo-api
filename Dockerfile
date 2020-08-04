# Example Dockerfile for mfo-api
# It creates 3 intermediate container images in order to speed up final image rebuild,
#   1. a base image that contains the source code and that also runs cargo test
#   2. a builder image that runs cargo build --release
#   3. the final container image that contains just the mfo-api binary

# 1. Create a cached source code base image which also runs unit tests:
FROM rust:alpine AS base
ENV USER=root

WORKDIR /code

RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY src /code/src
CMD [ "cargo", "test", "--offline" ]

# 2. Create a builder image from the base image and build the app:
FROM base AS builder
RUN apk add musl-dev
RUN cargo build --release --offline

# 3. Create the final container image:
FROM rust:alpine
COPY --from=builder /code/target/release/mfo-api /usr/bin/mfo-api
EXPOSE 8080
ENTRYPOINT [ "/usr/bin/mfo-api" ]