# B27 v0.6-alpha — Rust/WASM codec
Native raw B27N profiles 0/1/4/7, auto 255. Huffman3, LZ27, Markov3 arithmetic coding, SHA-256. Payload API supports all B27N profile numbers for use with the existing JS CVNSS transform/inverse. Standalone Rust CLI rejects linguistic profiles and B27F explicitly; use native.cjs for those. No claim of full native CVNSS compatibility or streaming yet.

    cargo test --manifest-path native-rust/Cargo.toml
    cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- c input.txt output.b27 auto
    cargo run --release --manifest-path native-rust/Cargo.toml --bin b27 -- d output.b27 restored.txt

WASM: build with --lib --target wasm32-unknown-unknown, run test-wasm.cjs with resulting wasm path. ABI: b27_input(length), write memory, b27_run(op,mode,limit), copy output_ptr/output_len. op 0/1 container, 2/3 payload. Return 0 success, -1 failure. Reacquire memory view after calls. Buffers are owned by module. Input cap 16 MiB for container compression. Security audit, fair benchmarks, full native CVNSS ruleset, streaming and fuzzing remain release gates.
