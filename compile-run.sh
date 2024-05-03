#!/bin/bash

#export LOG=TRACE
#cargo build --release
#rust-objcopy --binary-architecture=riscv64 target/riscv64gc-unknown-none-elf/release/os --strip-all -O binary target/riscv64gc-unknown-none-elf/release/os.bin
#qemu-system-riscv64 -machine virt -nographic -bios ./src/bootloader/rustsbi-qemu.bin -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000

make run LOG=TRACE