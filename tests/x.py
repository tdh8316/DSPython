from uno import *

seven_five = 3 / 4

a = 12345


def g(x: int) -> int:
    return x


def f(x: int) -> int:
    return g(x)


def setup():
    serial_begin(9600)

    pin_mode(13, 1)

    a = 1
    print(a)

    print("Hello, world!")

    my_own_x = f(69)

    print(my_own_x)

    my_own_x = 13


def loop():
    digital_write(13, 1)

    print(a)
