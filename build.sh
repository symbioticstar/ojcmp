set -e
cd /build
cargo build --release
cargo install --path .
cargo clean