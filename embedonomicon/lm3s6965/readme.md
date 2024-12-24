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

## Debug 

1. Launch qemu emulator on a different terminal

This command will create a virtual microcontroller and run our application.
It will simulate a connection to TCP port 3333.

```
qemu-system-arm \
      -cpu cortex-m3 \
      -machine lm3s6965evb \
      -gdb tcp::3333 \
      -S \
      -nographic \
      -kernel target/thumbv7m-none-eabi/debug/app
```

2. Open a debug session using `arm-none-eabi-dbg`.

To run this tool, ensure the ARM toolkit path is sourced.

`source .env`

This command connects to TCP port 3333, opening a debug session with 
QEMU (the emulator). 

QEMU runs our code, allowing step-by-step debugging through this connection.

`arm-none-eabi-gdb -q target/thumbv7m-none-eabi/debug/app`

## app + rt

the original project was converted from app to rt (runtime library)

now the app project uses the layout defined in rt, and the object 
dump shows the `main ` and the `ResetHandler` function

example:

```
cargo objdump --bin app -- -d --no-show-raw-insn

app:	file format elf32-littlearm

Disassembly of section .text:

00000008 <main>:
       8:      	sub	sp, #0x4
       a:      	movs	r0, #0x2a
       c:      	str	r0, [sp]
       e:      	b	0x10 <main+0x8>         @ imm = #-0x2
      10:      	b	0x10 <main+0x8>         @ imm = #-0x4

00000012 <ResetHandler>:
      12:      	push	{r7, lr}
      14:      	mov	r7, sp
      16:      	bl	0x8 <main>              @ imm = #-0x12
```
