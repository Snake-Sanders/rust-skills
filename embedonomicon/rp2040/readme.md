# Embedded Rust

## Rp2040 intro

https://www.youtube.com/watch?v=Duel_Oaases&list=PL_tws4AXg7auiZHZsL-qfrXoMiUONBB0U&index=3

## Enviroment setup

The setup is as described in [here] (https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html)

The only difference is that the ARM toolchain is not installed via 
`brew` but by downloading the ARM package directly from the ARM's dev web.

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

## Linker script

GNU ld linker manual: https://sourceware.org/binutils/docs/ld/

Linker Script Cheat Sheet:

|Syntax|Meaning|
|-|-|
|MEMORY|	Defines memory regions (e.g., FLASH, RAM).|
|ORIGIN(name)|	Gets the start address of the specified memory region.|
|LENGTH(name)|	Gets the length (size) of the specified memory region.|
|SECTION_NAME : { }|	Defines a section in the output binary.|
|KEEP()|	Ensures the section is kept in the final binary.|
|*()|	A wildcard used to include sections from object files.|
|LONG(value)|	Specifies a 32-bit word (long) in the section.|
|> MEMORY_REGION|	Places the section in a specific memory region.|

inspecting the output

`cargo objdump --bin basic_main -- -d --no-show-raw-insn`

result:

```
basic_main:	file format elf32-littlearm

Disassembly of section .text:

10000008 <ResetHandler>:
10000008:      	sub	sp, #0x4
1000000a:      	movs	r0, #0x2a
1000000c:      	str	r0, [sp]
1000000e:      	b	0x10000010 <ResetHandler+0x8> @ imm = #-0x2
10000010:      	b	0x10000010 <ResetHandler+0x8> @ imm = #-0x4
```

vector table

`cargo objdump --bin basic_main -- -s --section .vector_table`

result:

```
basic_main:	file format elf32-littlearm
Contents of section .vector_table:
 10000000 00200420 09000010
```

the little endian encoded address `00 20 04 20` is `20 04 20 00` =
`0x20042000`

the stack address is at the end of the RAM: 
`0x2000_0000 + 0x0042_0000 (264K)`

## Wokwi: HW Emulator

https://wokwi.com/pi-pico

