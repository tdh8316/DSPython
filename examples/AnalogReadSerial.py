from arduino import *


def setup():
    serial_begin(9600)


def loop():
    sensor_value = analog_read(A0)
    print(sensor_value)
    delay(1)
