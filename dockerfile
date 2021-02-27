FROM rust:latest as builder

ENV LIBDIR /usr/lib/redis/modules

ADD . /RIS
WORKDIR /RIS

# Set up a build environment
RUN apt-get update && apt-get install build-essential cmake
RUN git clone https://github.com/llvm-mirror/clang.git && cd clang && cmake
#RUN apt-get update -y && apt install libclang1-10=1:10.0.0-4ubuntu1 libllvm10=1:10.0.0-4ubuntu1

# Build the source
RUN set -ex ;\
    cargo build --release ;\
    mv target/release/redisintervalsets.so target/release/ris.so

# Package the runner
FROM redis:latest

ENV LIBDIR /usr/lib/redis/modules
WORKDIR /data
RUN mkdir -p "$LIBDIR"
COPY --from=builder /RIS/target/release/ris.so "$LIBDIR"

CMD ["redis-server", "--loadmodule", "/usr/lib/redis/modules/ris.so"]