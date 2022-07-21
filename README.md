# DSPython - Damn Small Python

:snake: Python compiler for small places

## Overview

DSPython is a python compiler intended to generate small binaries for use in small places, such as [Arduino](https://www.arduino.cc/).

> Note that the DSPython is not intended for full compatibility with the Python language.

It uses [RustPython](https://github.com/RustPython/RustPython) to parse the python code alongside [LLVM](https://llvm.org/) to generate bytecode intended to be similar to C++.
