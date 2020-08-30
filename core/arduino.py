"""
This module provides access to built-in Arduino functions
"""

# noinspection PyUnresolvedReferences
from core.arduino_pins import *
# noinspection PyUnresolvedReferences
from core.uno import *


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
