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

This produces some noise on macos which seems to be cruft stemming from the OS
and/or Rust test suite runtime. Still need to figure out how to plug these leaks
or mark them as ignored.

To see how the leak detection works, comment out the `slaw_free` call from `Slaw::drop()`.

[bindgen]: <https://github.com/rust-lang/rust-bindgen>