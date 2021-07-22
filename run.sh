#!/usr/bin/env bash

ulimit -n $(ulimit -Hn)

pushd web-toy
target/release/web-toy --socket-addrs \
  127.0.10.1:8080 \
  127.0.10.2:8080 \
  127.0.10.3:8080 \
  127.0.10.4:8080 \
  127.0.10.5:8080 \
  --client-shutdown 0 \
  --client-timeout 0 &
popd

pushd httplatencies
time target/release/httplatencies \
  -l \
  127.0.20.1 \
  127.0.20.2 \
  127.0.20.3 \
  127.0.20.4 \
  127.0.20.5 \
  -p 1 \
  -u \
  http://127.0.10.1:8080/noop \
  http://127.0.10.2:8080/noop \
  http://127.0.10.3:8080/noop \
  http://127.0.10.4:8080/noop \
  http://127.0.10.5:8080/noop \
  -t $1
popd
kill -15 %?web-toy
