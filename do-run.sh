set -e
cargo build --workspace --release 
cd ./example
../target/release/web3d -O --input diagram.md