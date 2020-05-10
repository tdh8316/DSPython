from uno import *


LED_BUILTIN = 13

LED_ON = 0


def setup() -> None:
    begin(9600)

    pin_mode(LED_BUILTIN, 1)

    return None


def loop():
    if LED_ON == 1:
        digital_write(LED_BUILTIN, 1)
    else:
        digital_write(LED_BUILTIN, 0)

    print("Hello, world!")
    delay(1000)

    return None
