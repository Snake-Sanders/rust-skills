# Embedded Rust

## Debug commands

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


