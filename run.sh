cargo b --release
sudo setcap cap_net_admin=eip target/release/tcp-ip
target/release/tcp-ip &

pid=$!

sudo ip addr add 192.168.0.1/24 dev mytun
sudo ip link set up dev mytun

wait $pid