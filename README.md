# rlsof

This package parses the output of `lsof -F0` and produces a Vec<HashMap<&str, String>> of the data.

# Building

pyo3 requires the *nightly* rust toolchain currently.

```
cargo build --release
cp target/release/librlsof.so /path/to/rlsof.so
```
