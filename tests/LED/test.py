from uno import *

foo = 6974


def setup() -> None:
    a = 1 + 1
    b = a + foo

    pin_mode(b - 6963, 1)

    return None


def loop() -> None:
    digital_write(13, 1)

    return None
