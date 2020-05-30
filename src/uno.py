"""
This module provides access to built-in Arduino functions
"""


def pin_mode(_pin: int, _mode: int) -> None:
    ...


def delay(_milliseconds: int) -> None:
    ...


def begin(_baudrate: int) -> None:
    ...


def digital_write(_pin: int, _level: int) -> None:
    ...


def digital_read(_pin: int) -> int:
    ...
