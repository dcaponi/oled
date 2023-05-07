# OLED Rust Fun

## Getting Started
I used a ESP32 C3 board. I hooked the power to the 3.3V pin, ground to ground (obviously) and clock and data to pins 6 and 7 respectively.

Then build and `espflash <wherever your esp32 is mounted> target/riscv32imc-unknown-none-elf/debug/oled --monitor`

## Things you might need
Low Level Build & Flash tools

sudo apt-get install -y clang llvm flex bison gperf cmake
ninja-build ccache libffi-devlibssl-dev libusb-1.0-0 libudev-dev
dfu-util package-config
espflash, ldrpoxy cargo install espflash ldproxy

llvm tools preview rustup component add llvm-tools-preview

Add the risc-v target for esp32 devices rustup target add riscv32imc-unknown-none-elf