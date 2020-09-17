"""
This module provides access to built-in Arduino functions.

This file contains the signature of the built-in functions
so that the IDEs can provide intelligent code completion, and
DSPython compiler will not compile this file itself
while it compiles the others.

The real implementation of these function can be found
in the rest files located in this directory.
Especially, the low-level functions are defined in the wrapper file,
which is located in the `include` directory.
"""

from typing import Union

from arduino.constants import *
from arduino.uno_pins import *


# noinspection PyShadowingBuiltins
def print(_str: object) -> None:
    ...


def pin_mode(_pin: int, _mode: int) -> None:
    ...


def delay(_milliseconds: int) -> None:
    ...


def serial_begin(_baudrate: int) -> None:
    ...


def digital_write(_pin: int, _level: int) -> None:
    ...


def digital_read(_pin: int) -> int:
    ...


def analog_read(_pin: int) -> int:
    ...


def radians(_deg: Union[int, float]) -> float:
    ...


def degrees(_rad: Union[int, float]) -> float:
    ...


def sin(_rad: Union[int, float]) -> float:
    ...


def cos(_rad: Union[int, float]) -> float:
    ...


def tan(_rad: Union[int, float]) -> float:
    ...
