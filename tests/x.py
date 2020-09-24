"""
DSPython test
"""

from arduino import *


def setup():
    serial_begin(9600)

    print(6 / 3.2)
    print(7.4 - 6.9)
    print(6974 / 69.74)
    print(0.1 * 10)


def loop():
    return None
