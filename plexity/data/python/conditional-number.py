def conditional_number():
    x = lambda a: a + 5
    y = lambda b: b * 2
    z = x(y(4)) + y(x(3))

    if z != 28:
        print(z)