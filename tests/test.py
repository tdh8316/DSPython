from uno import *


def setup() -> None:
    string = "Hello, world!"
    begin(9600)
    pin_mode(13, 1)
    print(string)
    return None


def loop():
    digital_write(13, 1)
    delay(1000)
    digital_write(13, 0)
    delay(1000)
    return None
