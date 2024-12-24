# Embedded Rust: The embedonomicon

The subfolders contain the exercises of the book `The embedonomicon`.

https://docs.rust-embedded.org/embedonomicon/index.html

## Targets 

to see rust supported targets run:

`rustc --print=target-list | fzf`

to see the installed targets run:

```
rustup target list | grep installed

aarch64-apple-darwin (installed)
thumbv6m-none-eabi (installed)
thumbv7m-none-eabi (installed)
```

## Debug commands

there are two scripts to start QEMU and the debugger.

1. `./start_qemu.sh` 
2. `./start_dbg.sh`

To stop QEMU you need to kill the process.

1. find the process listening on the port 

```
lsof -i :3333

qemu-system  12345 user  ...  TCP *:3333

```

1. kill the process

```
kill 12345
```

## Object dump 

* show all the symbols

`cargo objdump --bin app -- -s`

* shows functions and variables in asm

`cargo objdump --bin app -- -d --no-show-raw-insn`

* shows headers

```
cargo objdump --bin app -- --section-headers

Sections:
Idx Name            Size     VMA      Type
  0                 00000000 00000000
  1 .vector_table   00000008 00000000 DATA
  2 .text           00000046 00000008 TEXT
  3 .rodata         00000014 00000050 DATA
  4 .bss            00000001 20000000 BSS
  5 .data           00000002 20000002 DATA
```
* shows sections

```
cargo objdump --bin app -- -s --section .vector_table

Contents of section .vector_table:
 0000 00000120 47000000
```

```
cargo objdump --bin app -- -s --section .data

Contents of section .data:
 20000002 0100
```

Note: from the headers table `.text` starts at 
address `00000008 TEXT`

```
cargo objdump --bin app -- -s --section .text

Contents of section .text:
 0008 84b040f2 5c00c0f2 00000168 40680091  ..@.\......h@h..
 0018 019040f2 0000c2f2 00000290 40f20200  ..@.........@...
 0028 c2f20000 0390ffe7 fee780b5 6f4682b0  ............oF..
 0038 40f20900 c0f20000 0190fff7 e1ff80b5  @...............
 0048 6f46fff7 f2ff                        oF....
```

`cargo objdump --bin app -- -s --section .bss`

Inspect symbols

`cargo nm` does not find ARM toolchain, but calling directly the tool 
`arm-none-eabi-nm` works

```
arm-none-eabi-nm -jBC target/thumbv7m-none-eabi/debug/deps/app-6d6232af9c6ab892

0000065c r .L__unnamed_1
...
00000004 R RESET_VECTOR
0000004e T ResetHandler
00000426 t compiler_builtins::mem::memcpy
20000000 b app::BSS
20000002 d app::DATA
00000008 t app::main
0000060c r app::RODATA
...
0000056e t __aeabi_memclr
00000422 t __aeabi_memcpy
20000001 B _ebss
20000004 D _edata
20000000 B _sbss
20000002 D _sdata
000008dc A _sidata
00000032 T main
00000046 T rust_begin_unwind
```
