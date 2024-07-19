commits: https://gitlab.com/hr567/liboj/-/commits/master

b84a935a16c5a743dc0b87f8410fb86cdc789133


```bash
RUSTFLAGS=-Zsanitizer=address cargo build -Zbuild-std --target aarch64-unknown-linux-gnu

RUSTFLAGS=-Zsanitizer=leak cargo build -Zbuild-std --target aarch64-unknown-linux-gnu

# RUSTFLAGS=-Zsanitizer=thread cargo build -Zbuild-std --target aarch64-unknown-linux-gnu
```
# LibOJ

[![pipeline status](https://gitlab.com/hr567/liboj/badges/master/pipeline.svg)](https://gitlab.com/hr567/liboj/commits/master)

A high performance framework for building online judge system.
