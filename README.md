# Rust Voting Smart Contracts on the NEAR blockchain

1. Test the contract 

    `cargo test -- --nocapture`

2. Build the contract

    `RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release`
