# BL602 Rust example

A simple example program using the BL602 Rust HAL, using the serial bootloader so anyone can use Rust on their BL602-based board.  
There's no debugger support in this crate - use https://github.com/sipeed/bl602-rust-guide or a standalone debugger if you need that 

## Try it out!

Grab the toolchain for bl602
```
rustup target add riscv32imac-unknown-none-elf
```

Install cargo-blflash
```
cargo install cargo-blflash
```

Enter bootloader mode on the board by holding the boot button and pressing the en button

run
```
cargo blflash --port /dev/ttyUSB0
```

When you see  
INFO  blflash] Success  
connect to the board with a terminal emulator like putty or minicom
```
minicom -D /dev/ttyUSB0 -b2000000
```

then hit the en button again to run your program!

## License

This project is licensed under either of Mulan PSL v2 or MIT.

```
Copyright (c) 2020 Sipeed Co.,Ltd.
bl602-hal is licensed under Mulan PSL v2.
You can use this software according to the terms and conditions of the Mulan PSL v2.
You may obtain a copy of Mulan PSL v2 at:

    http://license.coscl.org.cn/MulanPSL2

THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
See the Mulan PSL v2 for more details.
```
