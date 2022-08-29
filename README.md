

## ch03-coop 问题

```bash
[rustsbi] RustSBI version 0.2.0-alpha.6
.______       __    __      _______.___________.  _______..______   __
|   _  \     |  |  |  |    /       |           | /       ||   _  \ |  |
|  |_)  |    |  |  |  |   |   (----`---|  |----`|   (----`|  |_)  ||  |
|      /     |  |  |  |    \   \       |  |      \   \    |   _  < |  |
|  |\  \----.|  `--'  |.----)   |      |  |  .----)   |   |  |_)  ||  |
| _| `._____| \______/ |_______/       |__|  |_______/    |______/ |__|

[rustsbi] Implementation: RustSBI-QEMU Version 0.0.2
[rustsbi-dtb] Hart count: cluster0 with 1 cores
[rustsbi] misa: RV64ACDFIMSU
[rustsbi] mideleg: ssoft, stimer, sext (0x222)
[rustsbi] medeleg: ima, ia, bkpt, la, sa, uecall, ipage, lpage, spage (0xb1ab)
[rustsbi] pmp0: 0x10000000 ..= 0x10001fff (rwx)
[rustsbi] pmp1: 0x80000000 ..= 0x8fffffff (rwx)
[rustsbi] pmp2: 0x0 ..= 0xffffffffffffff (---)
[rustsbi] enter supervisor 0x80200000
[TRACE][0,-] clear bss finish
[TRACE][0,-] logging init bss finish
[DEBUG][0,-] Hello, World!
[ INFO][0,-] .text [0x80200000, 0x80205000)
[ INFO][0,-] .rodata [0x80205000, 0x80218000)
[ INFO][0,-] .data [0x80218000, 0x8021c000)
[ INFO][0,-] .bss [0x8022c000, 0x8022d000)
[ERROR][0,-] Unsupported trap Exception(InstructionFault), stval = 0x0!
[ERROR][0,-] Panicked at src/trap/mod.rs:64 [kernel] Cannot continue!

```

先分析下，再debug

1. 比较镜像文件
```bash
jojo in nuc01 in rcore-x on  ch03-coop [!?] 
❯ ls -l os/target/riscv64gc-unknown-none-elf/release/os.bin 
-rwxrwxr-x 1 jojo jojo 112368 Apr 17 10:12 os/target/riscv64gc-unknown-none-elf/release/os.bin

jojo in nuc01 in rcore-x on  ch03-coop [!?] 
❯ ls -l ~/code/rcore-os/rCore-Tutorial-v3/os/target/riscv64gc-unknown-none-elf/release/os.bin 
-rwxrwxr-x 1 jojo jojo 108113 Apr 17 10:13 /home/jojo/code/rcore-os/rCore-Tutorial-v3/os/target/riscv64gc-unknown-none-elf/release/os.bin
```

```bash
> make debug LOG=trace
```
