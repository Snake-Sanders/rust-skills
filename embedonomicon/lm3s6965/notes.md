# Embedded Rust

## LM3S6965 intro

This is a Texas Instrument microcontoller, the example used for the tutorial

https://www.ti.com/product/LM3S6965

## Enviroment setup

The setup is as described in [here] (https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html)

## Qemu for emulation

This device can be emulated with QEMU

`brew install qemu`

then list the executables with

```
ls $(brew --prefix qemu)/bin

elf2dmp                  qemu-system-loongarch64  qemu-system-riscv64
qemu-edid                qemu-system-m68k         qemu-system-rx
qemu-img                 qemu-system-microblaze   qemu-system-s390x
qemu-io                  qemu-system-microblazeel qemu-system-sh4
qemu-nbd                 qemu-system-mips         qemu-system-sh4eb
qemu-storage-daemon      qemu-system-mips64       qemu-system-sparc
qemu-system-aarch64      qemu-system-mips64el     qemu-system-sparc64
qemu-system-alpha        qemu-system-mipsel       qemu-system-tricore
qemu-system-arm          qemu-system-or1k         qemu-system-x86_64
qemu-system-avr          qemu-system-ppc          qemu-system-xtensa
qemu-system-hppa         qemu-system-ppc64        qemu-system-xtensaeb
qemu-system-i386         qemu-system-riscv32
```
