from uno import *

pi = 3.14


def setup():
    begin(9600)

    pin_mode(13, 1)

    return None


def loop():

    pi_int = int(pi)

    if pi_int == 3:
        digital_write(10 + pi_int, 1)
    else:
        print("int(pi) != 3")

    return None
