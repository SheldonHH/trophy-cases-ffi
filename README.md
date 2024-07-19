# trophy-cases-ffi


```bash
export LD_LIBRARY_PATH=/root/.rustup/toolchains/nightly-2022-03-05-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/


RUSTFLAGS=-Zsanitizer=leak cargo build -Zbuild-std --target aarch64-unknown-linux-gnu
 ```

run successfully
```bash
RUSTFLAGS=-Zsanitizer=leak cargo run -Zbuild-std --target aarch64-unknown-linux-gnu
```