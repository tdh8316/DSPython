from uno import *

LED_BUILTIN = 13

INPUT = 0
OUTPUT = 1

LOW = 0
HIGH = 1


def setup():
    pin_mode(LED_BUILTIN, OUTPUT)


def loop():
    digital_write(LED_BUILTIN, HIGH)
    delay(1000)
    digital_write(LED_BUILTIN, LOW)
    delay(1000)
