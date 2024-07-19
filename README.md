# trophy-cases-ffi


```bash
export LD_LIBRARY_PATH=/root/.rustup/toolchains/nightly-2022-03-05-aarch64-unknown-linux-gnu/lib/rustlib/aarch64-unknown-linux-gnu/lib/


RUSTFLAGS=-Zsanitizer=leak cargo build -Zbuild-std --target aarch64-unknown-linux-gnu
 ```




# cobyla
 
## leak sanitanizer run successfully
```bash
RUSTFLAGS=-Zsanitizer=leak cargo run -Zbuild-std --target aarch64-unknown-linux-gnu
```

cobyla results
```bash
   Normal return from subroutine COBYLA

   NFVALS =   35   F = 1.000000E+01    MAXCV = 0.000000E+00
   X = 0.000000E+00  -9.970964E-05
status = 0
x = [0.0, -9.970963653135306e-5]

=================================================================
==3140==ERROR: LeakSanitizer: detected memory leaks

Direct leak of 16 byte(s) in 1 object(s) allocated from:
    #0 0xaaaacbc1ae88 in malloc /checkout/src/llvm-project/compiler-rt/lib/lsan/lsan_interceptors.cpp:75:3
    #1 0xaaaacbcd5bb0 in std::sys::pal::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::h33b41eab9aa0707a /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/pal/unix/alloc.rs:14:13
    #2 0xffff9c7f74c8 in __libc_start_main csu/../csu/libc-start.c:392:3
    #3 0xaaaacbbf4d6c in _start (/root/trading-programming/trophy-original/cobyla/target/aarch64-unknown-linux-gnu/debug/cobyla+0x14d6c) (BuildId: ece91992ab3e07d60772659ff146001db3c030d5)

SUMMARY: LeakSanitizer: 16 byte(s) leaked in 1 allocation(s).
```


## address sanitanizer
```bash
RUSTFLAGS=-Zsanitizer=address cargo run -Zbuild-std --target aarch64-unknown-linux-gnu
```
cobyla results
```bash
   Normal return from subroutine COBYLA

   NFVALS =   35   F = 1.000000E+01    MAXCV = 0.000000E+00
   X = 0.000000E+00  -9.970964E-05
status = 0
x = [0.0, -9.970963653135306e-5]

=================================================================
==4676==ERROR: LeakSanitizer: detected memory leaks

Direct leak of 16 byte(s) in 1 object(s) allocated from:
    #0 0xaaaaba47807c in malloc /checkout/src/llvm-project/compiler-rt/lib/asan/asan_malloc_linux.cpp:68:3
    #1 0xaaaaba5ca218 in std::sys::pal::unix::alloc::_$LT$impl$u20$core..alloc..global..GlobalAlloc$u20$for$u20$std..alloc..System$GT$::alloc::h33b41eab9aa0707a /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/pal/unix/alloc.rs:14:13
    #2 0xaaaaba5343a4 in __rdl_alloc /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/alloc.rs:402:13
    #3 0xaaaaba4a718c in alloc::alloc::Global::alloc_impl::he6c0f7cbc827c294 /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:183:73
    #4 0xaaaaba4a7624 in _$LT$alloc..alloc..Global$u20$as$u20$core..alloc..Allocator$GT$::allocate::hf78bfcd5ab0462d6 /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/alloc.rs:243:9
    #5 0xaaaaba4a897c in cobyla::main::h14c8e2231834a9b5 /root/trading-programming/trophy-original/cobyla/src/main.rs:15:27
    #6 0xaaaaba4a9b04 in core::ops::function::FnOnce::call_once::h5912c617545ac49d /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
    #7 0xaaaaba4bc8fc in std::panic::catch_unwind::h1531f908992a6d3c /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:350:14
    #8 0xaaaaba4bf758 in std::panicking::try::do_call::h98bb483227c8301e /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:557:40
    #9 0xaaaaba4bc94c in std::panic::catch_unwind::hacaefa4bc3737d52 /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:350:14
    #10 0xaaaaba4a77d8 in std::rt::lang_start::h41e7105440ff52e2 /root/.rustup/toolchains/nightly-aarch64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:162:17
    #11 0xaaaaba4a8d00 in main (/root/trading-programming/trophy-original/cobyla/target/aarch64-unknown-linux-gnu/debug/cobyla+0x1b8d00) (BuildId: 8c5bf3b16d427c68c4d0136168f2f5fd0455a207)
    #12 0xffffb22674c8 in __libc_start_main csu/../csu/libc-start.c:392:3
    #13 0xaaaaba3b7d6c in _start (/root/trading-programming/trophy-original/cobyla/target/aarch64-unknown-linux-gnu/debug/cobyla+0xc7d6c) (BuildId: 8c5bf3b16d427c68c4d0136168f2f5fd0455a207)

SUMMARY: AddressSanitizer: 16 byte(s) leaked in 1 allocation(s).
```

成功检测到Cobyla，查看FFI-Checker的Trophy Case
https://github.com/lizhuohua/rust-ffi-checker/blob/master/trophy-case/README.md

