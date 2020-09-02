HIGH = 0x1
LOW = 0x0

INPUT = 0x0
OUTPUT = 0x1
INPUT_PULLUP = 0x2

PI = 3.1415926535897932384626433832795
HALF_PI = 1.5707963267948966192313216916398
TWO_PI = 6.283185307179586476925286766559
DEG_TO_RAD = 0.017453292519943295769236907684886
RAD_TO_DEG = 57.295779513082320876798154814105
EULER = 2.718281828459045235360287471352

SERIAL = 0x0
DISPLAY = 0x1

LSBFIRST = 0
MSBFIRST = 1

CHANGE = 1
FALLING = 2
RISING = 3


def min__i__(a: int, b: int) -> int:
    if a < b:
        return a
    else:
        return b


def min__f__(a: float, b: float) -> float:
    if a < b:
        return a
    else:
        return b


def max__i__(a: int, b: int) -> int:
    if a > b:
        return a
    else:
        return b


def max__f__(a: float, b: float) -> float:
    if a > b:
        return a
    else:
        return b


def radians__i__(deg: int) -> float:
    return deg * DEG_TO_RAD


def radians__f__(deg: float) -> float:
    return deg * DEG_TO_RAD


def degrees__i__(rad: int) -> float:
    return rad * RAD_TO_DEG


def degrees__f__(rad: float) -> float:
    return rad * RAD_TO_DEG
