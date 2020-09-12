"""
This module provides access to built-in Arduino functions
"""

from arduino.uno_pins import *
from arduino.constants import *


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


def radians(_deg) -> float:
    ...


def degrees(_rad) -> float:
    ...
