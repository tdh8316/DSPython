from uno import *


def setup():
    serial_begin(9600)

    pin_mode(9, 1)


def loop() -> None:
    volt = digital_read(9)

    while volt != 1:
        print("Low...")
    else:
        print("Finally High!!!")

    return None
