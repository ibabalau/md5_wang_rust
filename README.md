MD5 Collision Attack by Wang et al. implementation in Rust

Based on Patrick Stach's 2005 implementation of the attack in C.

Build:
cargo build --release

Running with default IV:
cargo run --release

Run with custom IV (warning: iv values need to be in hex format):
cargo run --release 0x874587a2 0xf09dfbdf 0x17732fb1 0x9299e527

Output:
m0.bin and m1.bin, which both have the same md5 hash
