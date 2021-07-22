#!/usr/bin/env bash
echo "Building httplatencies..."
pushd httplatencies
cargo build --release
popd

echo "Building web-toy..."
pushd web-toy
cargo build --release
popd

echo "Adding some IPs to loopback device ..."
for i in $(seq 1 5)
do
  sudo ip addr add 127.0.10.${i} dev lo
  sudo ip addr add 127.0.20.${i} dev lo
done
