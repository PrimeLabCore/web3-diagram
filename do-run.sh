set -e
cargo build --workspace --release 
cd ./example
../target/release/web3d --cname="Test Smart Contract" -O --input diagram.md