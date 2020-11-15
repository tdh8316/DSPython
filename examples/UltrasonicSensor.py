"""
This is the DSPython implementation of the Arduino example.

Created for DSPython by Donghyeok Tak <tdh8316@naver.com>
"""

from arduino import *

trig = 13
echo = 12


def setup():
    serial_begin(9600)

    pin_mode(trig, OUTPUT)
    pin_mode(echo, INPUT)

    print("DSPython - Measure distance using an ultrasonic sensor")


def loop():
    digital_write(trig, HIGH)
    delay(10)
    digital_write(trig, LOW)

    duration: float = pulse_in(echo, HIGH)
    distance: float = float(340 * duration / 10000) / 2

    print(distance)

    delay(1000)
