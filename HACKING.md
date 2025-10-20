# hacking

This thing is still a big WIP.  For now, to get started, you'll need:

1. a Mac with a functioning C compiler toolchain -- the crate builds an
   abridged, vendored copy of libLoam and libPlasma from source (will add
   support to dynamically link to libPlasma + libLoam later)
2. libyaml installed to /opt/homebrew (`brew install libyaml`)
3. a stable rust toolchain, `rustup add stable`

Then run `cargo test` to build everything and run the test suite.

## adding more unsafe bindings

Unsafe bindings are generated using the [rust-bindgen][] tool from the file
`all-the-headers.h`.  I didn't want to shave the build.rs yak so it's committed
in the repo for right now.

## sanitizers

To run the test suite with asan + leak detection:

```
ASAN_OPTIONS=detect_leaks=1 RUSTFLAGS="-Zsanitizer=address" cargo +nightly test
```

On my Mac, this produces four noisy leaks stemming from libobjc and/or libdyld,
which occur even in a trivial empty test suite.  To suppress these, pass the
provided suppressions file:

```
ASAN_OPTIONS=detect_leaks=1 LSAN_OPTIONS=suppressions=$PWD/etc/suppressions.txt RUSTFLAGS="-Zsanitizer=address" cargo +nightly test
```

To see how the leak detection works when bugs in the Rust bindings cause a leak,
comment out the `slaw_free` call from `Slaw::drop()`.  This causes every slaw
allocation made by the bindings to leak.  Alternately, call
`std::mem::forget()` on various Slaw values in the test suite.

[bindgen]: <https://github.com/rust-lang/rust-bindgen>