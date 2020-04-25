# Blink the built-in LED of Arduino Uno!
from uno import *


def setup() -> None:
    pin_mode(13, 1)
    return None


def loop() -> None:
    digital_write(13, 1)
    delay(1000)
    digital_write(13, 0)
    delay(1000)
    return None
