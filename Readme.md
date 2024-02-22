# Three Rusty Hellos

A simple rust "hello world" but with some cross-compiling to three different targets: Linux, Windows, and ideally Os X too.
This is really a simple "how-to" or maybe "how-I-did-it" in my specific situation.

## Some places to read more
The [rust book has a section on cross-compilation](https://rust-lang.github.io/rustup/cross-compilation.html).
With a short look around it quickly became appearant that the `cross` crate is a good way to simplify cross-compiling. It handles the `linker` and `gcc` stuff for you in the simple cases, and handles it in containers (docker or podman).  Also, I found [rust book - rust-by-example section](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) to be helpful in understanding how to use `cfg` attribute.

### Key terms for cross-compiling
Not necessarily covered here, but maybe useful to know (even if some or all might be obvious) or use to find the answer you seek.
* `target` - The system you are compiling for.
* `host` - The system you are compiling on.
* `toolchain` - The compiler and other tools used to compile the code.
* `triple` - A string that identifies the target system. It is in the form of `arch-vendor-os` or `arch-vendor-os-env`. You can find the triple for your target by figuring out the architecture, vendor, system, and ABI of the target system. Tips for finding these are in the `rust-cross` documentation, and just a querry or two away.
* `linker` - The program that links the compiled code into an executable. Names for the linkers don't always match the `triple` names. Hence, the `cross` crate is helpful in handling this for you.

### rust-cross
The documentation is easy to follow and was helpful for me.
* [rust-cross github](https://github.com/japaric/rust-cross/)
* [rust-cross about Triples](https://github.com/japaric/rust-cross/blob/master/README.md#the-target-triple)
* [rust-cross README.md](https://github.com/japaric/rust-cross/blob/master/README.md)

Below is simply me following the instructions in the above documentation.

## Cross Setup
Since I have rust and docker already set up in my environment, I'm skipping those steps.

Install `cross` via cargo. 
```bash
cargo install cross
```
Output:
```bash
Finished release [optimized] target(s) in 28.67s
    Installing [...]/.cargo/bin/cross
    Installing [...]/.cargo/bin/cross-util
    Installed package `cross v0.2.5` (executables `cross`, `cross-util`)
```

### Quick attempt to use cross
Copy-Pasted right from the `cross` create documentation.
```bash
cross run --target aarch64-unknown-linux-gnu
```
Keep in mind that the first time you run this command it will take a while to download the docker or podman image.

Output:
```bash
Status: Downloaded newer image for ghcr.io/cross-rs/aarch64-unknown-linux-gnu:0.2.5
    Compiling three_rusty_hellos v0.1.0 (/project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.12s
    Running `/linux-runner aarch64 /target/aarch64-unknown-linux-gnu/debug/three_rusty_hellos`
```

## Use cross from linux to target: windows
```bash
cross run --target x86_64-pc-windows-gnu
```
output:
```bash
Status: Downloaded newer image for ghcr.io/cross-rs/x86_64-pc-windows-gnu:0.2.5
    Compiling three_rusty_hellos v0.1.0 (/project)
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
    Running `wine /target/x86_64-pc-windows-gnu/debug/three_rusty_hellos.exe`
```

## Target osx
Well, there is a fair bit more work to do for osx. It seems appoarchable, but I skipped it for now. Maybe you can let me know if everything runs smoothly for you.
Helpful starting point might be [cross-rs' cross-toolchains git repo readme section "apple-targets"](https://github.com/cross-rs/cross-toolchains?tab=readme-ov-file#apple-targets)

## After the basic "Hello World"
First, I added a simple `cfg` attribute to the `main.rs` file to print out the target system. I found the [rust book - rust-by-example section on attribututs/cfg](https://doc.rust-lang.org/rust-by-example/attribute/cfg.html) to be helpful in understanding how to use the `cfg` attribute.
Second, I added the `colored` crate to the `Cargo.toml` file and used it to print out the target system in color. It's in the `cargo.toml` but if you're doing it yourself you can use `cargo add colored`. Otherwise you can check out the  [colored crate documentation](https://docs.rs/colored/2.1.0/colored/index.html) to be helpful in understanding how to use the crate.