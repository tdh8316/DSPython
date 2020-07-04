from uno import *


def setup():
    serial_begin(9600)

    delay(1000)

    if True:
        print("Hello, world!")

    a = 1

    while a == 1:
        print("While loop in setup function!!!")
        a = a + 1
        delay(1000)
    else:
        print("Out of while loop!!!")

    return


def loop():
    return
