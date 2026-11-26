# B27 — BILA-T27 v0.6 RC3
Vietnamese linguistic-aware lossless compression: CVNSS4.0, Base27, ternary Huffman, LZ27, Markov3 and arithmetic coding. Rust native and WASM. Research release candidate; no claim of superiority to LZMA2.

## Native
```sh
cargo test --manifest-path native-rust/Cargo.toml
cargo build --release --manifest-path native-rust/Cargo.toml
native-rust/target/release/b27 c input.txt output.b27 9
native-rust/target/release/b27 d output.b27 restored.txt
native-rust/target/release/b27 cs input.txt output.b27t 6
native-rust/target/release/b27 ds-strict output.b27t restored.txt
```
Single container original input limit 16 MiB; native streaming uses 256 KiB blocks. Strict streaming requires B27T; legacy B27S has no whole-stream digest. No overwrite. SHA-256 is integrity, not authentication.

[Web Studio](https://xulytiengviet.github.io/B27/web-studio.html) · [CI](https://github.com/xulytiengviet/B27/actions) · [Format draft](FORMAT_SPEC.md) · [RC3 acceptance](docs/rc3.md) · [Security](SECURITY.md)

JavaScript reference: node native.cjs c|d input output. Legacy B27F: node b27.cjs. Native supports B27F/B27N reads and B27T streaming.
Benchmarks: python3 bench-native.py corpus.txt (Linux GNU time and xz). The CI smoke corpus is synthetic only; independent corpus and full security review remain mandatory for scientific acceptance.
