FROM rust:latest as builder

ENV LIBDIR /usr/lib/redis/modules

ADD . /RIS
WORKDIR /RIS

# Set up a build environment
RUN apt-get update -y && apt-get install git wget clang cmake -y

# Build the source
RUN set -ex ;\
    cargo build --release ;\
    mv target/release/libintervalsets.so target/release/ris.so

# Package the runner
FROM redis:latest

ENV LIBDIR /usr/lib/redis/modules
WORKDIR /data
RUN mkdir -p "$LIBDIR"
COPY --from=builder /RIS/target/release/ris.so "$LIBDIR"

CMD ["redis-server", "--loadmodule", "/usr/lib/redis/modules/ris.so"]