# Warning
This project is in the initial stage now and probably not do what you want.
You can browse the [tests](https://github.com/tdh8316/dsp/tree/master/tests) directory to see upstream working, or [example](https://github.com/tdh8316/dsp/tree/master/examples) directory to learn DSP.

> Are you disappointed? Please consider contributing!

# DSPython - Damn Small Python
> Python compiler for small places

DSPython is a restricted Python subset compiler intended for use in Arduino.

The [Micropython](https://github.com/micropython/micropython) project aims to put an implementation of Python 3 on microcontrollers, however, not available on Arduino.

DSPython uses [LLVM](http://llvm.org/) to provide a way to compile programs written in the Python programming language.
It generates LLVM IR, which is intended to be similar to C++'s. Accordingly, the DSPython is internally not a Python at all.

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

To compile and upload this source, you can specific the serial port to upload by providing the `--upload` option.
For example, this compiles and uploads the [blink example](https://github.com/tdh8316/dsp/tree/master/examples/Blink.py) to the Arduino:

```
dspython examples/Blink.py --upload YOUR_PORT
```

## Usage
```
usage: dspython [-u PORT] [-o OPT_LEVEL] FILE

positional arguments:
    FILE             Source file

optional arguments:
    -u PORT, --upload PORT
                     Serial Port to upload hex
    -o OPT_LEVEL
                     LLVM Optimization level
```

# Installation
## Building from source
## Installer packages

# Contributing
Contributions are more than welcome!

I have trouble with continuing this project because this is my first use of rust.
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

