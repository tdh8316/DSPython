# Warning
This project is in the initial stage now and probably not do what you want.
You can browse the [tests](https://github.com/tdh8316/dsp/tree/master/tests) directory to see upstream working, or [example](https://github.com/tdh8316/dsp/tree/master/examples) directory to learn DSP.

[Language roadmap](https://github.com/tdh8316/dsp/blob/master/ROADMAP.md)
> Disappointed? Please consider contributing!

# DSP - Damn Small Python

DSP is a restricted Python subset compiler intended for use in Arduino.

The [Micropython](https://github.com/micropython/micropython) project aims to put an implementation of Python 3 on microcontrollers, however, not available on Arduino.
This project was started to resolve this issue and aims to use a seemingly Python-like programming language on Arduino.

>**Note that it runs directly on the Arduino board, not through serial communication.**

Here is an example program that blinks the built-in LED.
```python
# Blink the built-in LED of Arduino Uno!
from uno import *

def setup():
    pin_mode(13, 1)

def loop():
    digital_write(13, 1)
    delay(1000)
    digital_write(13, 0)
    delay(1000)
```

Use below command to run [blink example](https://github.com/tdh8316/dsp/tree/master/examples/Blink.py):
```
dsp flash examples/Blink.py --port SERIAL_PORT
```

## Usage
```
$ dsp -h
DSP: Damn Small Python is a Python compiler for Arduino

USAGE:
    dsp <command> <file> [-p PORT] [--emit-llvm] [--no-opt]

FLAGS:
        --emit-llvm    Emits llvm ir
    -h, --help         Prints help information
        --no-opt       Disables optimization
    -V, --version      Prints version information

OPTIONS:
    -p, --port <port>    Serial port to flash

ARGS:
    <command>    Command to execute {build,flash}
    <file>       Source file path

```

## The reason this project exists
I wanted to program Arduino in other languages as well as C++.
But because it is impossible to bring standard Python to Arduino, I decided to make a Python compiler that is available to upload directly to the Arduino.

The distinctive feature of DSP is that it uses LLVM internally instead of emitting [C++](https://arduino.github.io/arduino-cli/sketch-build-process/).

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
 - Support Multi-core
 
# Credit
- The python parser is based on [RustPython](https://github.com/RustPython/RustPython)
- Value handler from [testlang](https://github.com/AcrylicShrimp/testlang-rust/)
- LLVM binding for rust is [Inkwell](https://github.com/TheDan64/inkwell)
