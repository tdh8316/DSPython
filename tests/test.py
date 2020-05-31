from uno import *


def add(a: float, b: float) -> float:
    return a + b


def setup():
    begin(9600)
    print(add(3.0, 2.5))
    print(6.9*7.4)
    return


def loop():
    return
