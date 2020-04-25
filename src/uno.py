"""
This module provides access to built-in Arduino functions
"""


def pin_mode(pin: int, mode: int) -> None:
    ...


def delay(milliseconds: int) -> None:
    ...


def begin(baudrate: int) -> None:
    ...


def digital_write(pin: int, level: int) -> None:
    ...
