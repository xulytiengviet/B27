# B27 v0.6 release candidate
Native CQN/CVSS rules and all B27N profiles; legacy B27F decode; block streaming B27S. Public JS inspection API not fully ported. See [acceptance gates](../docs/v06-acceptance.md).

```sh
cargo test --manifest-path native-rust/Cargo.toml
cargo build --release --manifest-path native-rust/Cargo.toml
cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- c input.txt output.b27 9
cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- d output.b27 restored.txt
cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- cs input.txt output.b27s 6
cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- ds output.b27s restored.txt
```
WASM ABI unchanged. Browser Studio loads trusted WASM artifact. No stable release until gates pass.
