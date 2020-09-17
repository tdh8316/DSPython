"""
Fibonacci series
"""

from arduino import *


def fib(n: int):
    a = 0
    b = 1
    s = 0
    count = 1

    while count <= n:
        print(s)

        count = count + 1
        a = b
        b = s
        s = a + b


def setup():
    serial_begin(9600)
    fib(10)


def loop():
    return None
