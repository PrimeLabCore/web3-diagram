set -e
cd ../
cargo build --release
cd ./example
../target/release/cargo-diagram -O --input diagram.md