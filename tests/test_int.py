def abs(x: int) -> int:
    """
    @return int
    """
    if x < 0:
        return -x
    return x


def add(a: int, b: int):
    """
    @return int
    """
    return a + b


def main():
    """
    @return int
    """
    negative_one: int = add(7, -8)
    positive_one: int = add(-6, 7)

    if -negative_one == positive_one:
        return 100

    return 90
