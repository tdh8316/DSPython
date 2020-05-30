from uno import *


def setup():
    begin(9600)

    pin_mode(13, 0)

    return None


def loop():

    # a=digital_read(13)

    print(digital_read(13))

    return None
