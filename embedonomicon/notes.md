# Embedded Rust

## Enviroment setup

The setup is as described in [here] (https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html)
The only difference is that the ARM toolchain is not installed via `brew` but
by downloading the ARM package directly from the ARM's dev web.
See notes in [cpp-skill](https://github.com/Snake-Sanders/cpp-skills/blob/main/rp2040_promicro.md) repo.

The toolchain for ARM are installed under 

`/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin`

I have issues with `cargo nm`. Cargo does not find the ARM toolchain, 
so either add it to the path or call the arm tool directly `arm-none-eabi-nm`:

```
cargo nm -- target/thumbv7m-none-eabi/debug/deps/app-*.o | grep '[0-9]* [^N] '
```

Using the ARM toolchain directly

```
arm-none-eabi-nm target/thumbv7m-none-eabi/debug/deps/basic_main-69eff080a9f4340c.o
```

## Memory Layout

https://docs.rust-embedded.org/embedonomicon/memory-layout.html
