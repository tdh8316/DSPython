# Building Damn Small Python
DSP depends on [LLVM](https://llvm.org/) and [Arduino Software](https://www.arduino.cc/en/main/software).
This is because DSP depends on Arduino library and avr-gcc.

## Build dependencies
The build process itself does not need Arduino Software, but it's required because DSP depends on Arduino Standard Library to compile. 
 - Rust
 - Python 3.4 or higher
 - LLVM 10.0
 - Arduino Software (1.6 or higher)

...

Please see [USAGE](./USAGE.md) to complete setup.