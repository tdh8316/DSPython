from arduino import *


def setup():
    pin_mode(LED_BUILTIN, OUTPUT)


def loop():
    digital_write(LED_BUILTIN, HIGH)
    delay(1000)
    digital_write(LED_BUILTIN, LOW)
    delay(1000)
