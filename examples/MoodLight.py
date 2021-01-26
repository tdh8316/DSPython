"""
DSPython test
"""

from arduino import *

pin_red = 9
pin_green = 10
pin_blue = 11


def setup():
    serial_begin(9600)

    pin_mode(pin_red, OUTPUT)
    pin_mode(pin_green, OUTPUT)
    pin_mode(pin_blue, OUTPUT)


def loop():
    x = 0.
    while x < PI:
        x = x + 0.00003
        r = 255 * abs(sin(x * 180 / PI))
        g = 255 * abs(sin((x + PI / 3) * 180 / PI))
        b = 255 * abs(sin((x + (2 * PI) / 3) * 180 / PI))

        analog_write(pin_red, int(r))
        analog_write(pin_green, int(g))
        analog_write(pin_blue, int(b))

        print(int(r)); print(",")
        print(int(g)); print(",")
        println(int(b))
