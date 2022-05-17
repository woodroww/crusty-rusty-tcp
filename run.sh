#!/bin/bash
cargo build --release
ext=$?
if [[ $ext -ne 0 ]]; then
		exit $ext
fi
sudo setcap CAP_NET_ADMIN=eip ./target/release/crusty-rusty-tcp
./target/release/crusty-rusty-tcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
trap "kill $pid" INT TERM
wait $pid

