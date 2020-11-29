# Warning
DSPython is in the very initial stage of a development phase and should not be used in a production environment.

You can browse the [examples](https://github.com/tdh8316/dspython/tree/master/examples) directory to learn how DSPython interacts with Arduino.

> Are you disappointed? Please consider contributing!

# DSPython - Damn Small Python
> 🐍 Python compiler intended to use in Arduino.

The [Micropython](https://github.com/micropython/micropython) project aims to put an implementation of Python 3 on microcontrollers, but it is not for Arduino.

DSPython uses [LLVM](http://llvm.org/) to provide a way to compile programs written in the [Python programming language](https://www.python.org/).
The generated LLVM bytecode is intended to be similar to C++'s.

Accordingly, **DSPython is internally not a Python** at all.

Here is an example program that blinks the built-in LED of Arduino Uno:
```python
# Blink the built-in LED of Arduino Uno!
from arduino import *

def setup():
    pin_mode(13, 1)

def loop():
    digital_write(13, 1)
    delay(1000)
    digital_write(13, 0)
    delay(1000)
```

To compile and upload this source, you can specific the serial port to upload by providing the `--upload-to` option.
For example, this compiles and uploads the [blink example](https://github.com/tdh8316/dsp/tree/master/examples/Blink.py) to the Arduino:

```
dspython examples/Blink.py --upload-to YOUR_PORT
```

## Supported boards
Currently, All examples have been tested only on Arduino Uno.

- [Arduino Uno](https://store.arduino.cc/usa/arduino-uno-rev3)

## Usage

# Installation
## Requirements
- LLVM 10 (include llvm-config)
On Windows, the official LLVM releases do not contain many important components.
You have to use [pre-built LLVM binary](https://ziglang.org/deps/llvm%2bclang%2blld-10.0.0-x86_64-windows-msvc-release-mt.tar.xz) built for [Ziglang](https://github.com/ziglang/zig/wiki/Building-Zig-on-Windows)

- Arduino IDE
You have to set the environment variable named `ARDUINO_DIR` to your arduino IDE location.
This is because DSPython requires Arduino standard headers, avr-gcc compiler, and avrdude.

## Building from source
## Installer packages

# Contributing
Contributions are more than welcome!

This is my first project using LLVM and Rust language.
Please share your opinions. Any ideas would be highly appreciated!

### Project goals
 - Damn small binary size
 - Support Arduino or microcontrollers
 - Programming Arduino with seemingly Python-like language
### Neutral
These are not impossible, but currently not our goals.
 - Compile to other platforms
 - Garbage collector
 - Class and inheritance
### Never
 - Complete Python implementation
 - Compile all python standard libraries
 - Support threading or asynchronous functions

## The reason this project exists
I wanted to program Arduino in other languages as well as C++ and thought the Python language would be a good choice.
But because it is impossible to bring standard Python to Arduino, I decided to make a Python compiler that is available to upload directly to the Arduino.

The distinctive feature of DSP is that it uses LLVM internally instead of emitting [C++](https://arduino.github.io/arduino-cli/sketch-build-process/).

# License
Licensed under the MIT License

Copyright 2020 `Donghyeok Tak`

# Credit
- The python parser is based on [RustPython](https://github.com/RustPython/RustPython)
- Value handler from [testlang](https://github.com/AcrylicShrimp/testlang-rust/)
- LLVM binding for rust is [Inkwell](https://github.com/TheDan64/inkwell)

