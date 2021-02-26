FROM rust:latest as builder

ENV LIBDIR /usr/lib/redis/modules

ADD . /RedisIntervalSets
WORKDIR /RedisIntervalSets

# Build the source
RUN apt-get update && apt-get install libclang-dev
RUN set -ex ;\
    cargo build --release ;\
    mv target/release/redisintervalsets.so target/release/ris.so

#---------------------------------------------------------------------------------------------- 
# Package the runner
FROM redis:latest

ENV LIBDIR /usr/lib/redis/modules
WORKDIR /data
RUN mkdir -p "$LIBDIR"
COPY --from=builder /RedisIntervalSets/target/release/ris.so "$LIBDIR"

CMD ["redis-server", "--loadmodule", "/usr/lib/redis/modules/ris.so"]