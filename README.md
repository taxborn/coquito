# :coconut: Coquito
An experimental programming language written in Rust to teach myself compilers, 
Rust itself, and (maybe) [LLVM](https://github.com/llvm/llvm-project).

# Running
There are two ways to run this compiler

## With [cargo](https://doc.rust-lang.org/cargo/) (recommended for now)
```
cargo run -- --file <file to compile>
```
or for a general help menu:

```
cargo run -- --help
```

## Precompiled Binaries (easier, standalone)
You can get a precompiled binary in the [releases](https://github.com/taxborn/coquito/releases). 
Currently Coquito in a sense supports Windows, MacOS, and Linux, however I only really test it on Linux.
CI tests the compiler in each of these operating systems so it *should* work everywhere, but I don't 
guarantee anything.

Once the binary is downloaded, you can execute it in a command line like so:
```
./coquito --file <file to compile>
```

or for a general help menu:

```
./coquito --help
```
