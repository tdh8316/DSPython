def fib_recursive(n: int):
    """
    @return int
    """
    if n == 0:
        return 0
    elif n == 1:
        return 1
    else:
        return fib_recursive(n - 1) + fib_recursive(n - 2)


def main():
    print("10th fibonacci number is:")
    print(fib_recursive(10))
