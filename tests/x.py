from uno import *


def setup():
    serial_begin(9600)

    pin_mode(9, 1)


def loop() -> None:
    count = 0

    while count < 10:
        print(count)
        count = count + 1

    return None
