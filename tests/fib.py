"""
Fibonacci series
"""

from arduino import *


def fib_while(n: int) -> int:
    a = 0
    b = 1
    res = 0
    count = 1

    while count < n:
        count = count + 1
        a = b
        b = res
        res = a + b

    return res


def fib_recursion(n: int) -> int:
    if n <= 0:
        print("n must be an integer greater than zero!")
        # If the function ends without a return statement
        # DSPython automatically adds `return 0` that fits the type of this function
    elif n == 1:
        return 0
    elif n == 2:
        return 1
    else:
        return fib_recursion(n - 1) + fib_recursion(n - 2)


def setup():
    serial_begin(9600)

    print("fib(10) Using loop:")
    println(fib_while(10))

    print("fib(10) Using recursion:")
    println(fib_recursion(10))


def loop():
    return None
