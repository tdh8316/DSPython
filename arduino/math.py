from arduino.constants import DEG_TO_RAD, RAD_TO_DEG


def abs__i__(x: int) -> int:
    if x > 0:
        return x
    else:
        return 0-x


def abs__f__(x: float) -> float:
    if x > 0.0:
        return x
    else:
        return 0.0-x


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
