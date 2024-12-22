# Embedded Rust

the toolchain for ARM are installed under 

`/Applications/ArmGNUToolchain/14.2.rel1/arm-none-eabi/bin`

I have issues with `cargo nm`. Cargo does not find the ARM toolchain, 
so either add it to the path or
call the arm tool directly `arm-none-eabi-nm`:

```
cargo nm -- target/thumbv7m-none-eabi/debug/deps/app-*.o | grep '[0-9]* [^N] '
```

```
arm-none-eabi-nm target/thumbv7m-none-eabi/debug/deps/basic_main-69eff080a9f4340c.o
```
