from uno import *


def add(a: float, b: int) -> float:
    return a + float(b)


def setup():
    begin(9600)
    print(add(3.0, 2))
    print(6.9*7.4)
    pin_mode(9, 0)


def loop():
    a = digital_read(9)

    if a == 1:
        print("Pin 9 is HIGH!!")
    elif a == 0:
        print("Pin 9 is LOW!!")
    else:
        print("WTF?")

    delay(1000)
