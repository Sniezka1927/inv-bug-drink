cargo test -- --show-output

### prequesites:

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly-x86_64
rustup target add wasm32-unknown-unknown

substrate-contracts-node -lerror,runtime::contracts=debug

cargo +nightly test -- --nocapture
