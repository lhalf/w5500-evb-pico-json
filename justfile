set shell := ["bash", "-euc"]

build:
    cargo build --release --all-features
    elf2uf2-rs target/thumbv6m-none-eabi/release/w5500-json

check:
    cargo fmt --check --all
    cargo clippy --bins --all-features -- -Dwarnings

test:
    cargo test --lib --target x86_64-unknown-linux-gnu

e2e-test:
    cargo test --test e2e --target x86_64-unknown-linux-gnu