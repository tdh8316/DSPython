from uno import *


def setup() -> None:
    serial_begin(9600)

    count = 0
    while count < 10:
        print(count)
        count = count + 1

    return None


def loop() -> None:
    return None
