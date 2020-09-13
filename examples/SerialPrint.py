"""
This is the DSPython implementation of the Arduino example.

Created for DSPython by Donghyeok Tak <tdh8316@naver.com>
"""

from arduino import *


def setup():
    serial_begin(9600)
    print("Hello, Arduino!")
    print("...from Damn Small Python")


def loop():
    return
