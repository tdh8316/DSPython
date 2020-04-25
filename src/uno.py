"""
This module provides access to built-in Arduino functions
"""


class Serial(object):
    @staticmethod
    def begin(baudrate: int) -> None:
        ...

    @staticmethod
    def println(string: str) -> None:
        ...


def pin_mode(pin: int, mode: int) -> None:
    ...


def delay(milliseconds: int) -> None:
    ...


def digital_write(pin: int, level: int) -> None:
    ...
