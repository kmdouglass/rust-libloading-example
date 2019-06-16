This repository contains an example design pattern for interfacing with C-language plugins through
Rust. It works on UNIX-based systems only.

For an explanation of the code, see the corresponding blog post at
http://kmdouglass.github.io/posts/a-simple-plugin-interface-for-the-rust-ffi/

# Instructions

First, from the root of this repository, build the example C library:

```console
$ cd ffi-test
$ make
```

Next, build the Rust binary and run it:

```console
$ cd ..
$ cargo build
$ cargo run
```

# Commit history

The commit history contains earlier attempts at creating this design. Some of these attempts will
not compile.
