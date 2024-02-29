from .user_input import *
from math import floor

def get_i0_num(n, k, i0):
    result = n
    first_gen = 0
    for i in range(0, i0):
        if i != 0:
            result = result * k * ((1000 - result) / 1000)
        first_gen = result
    return first_gen

def case1(args):
    n = get_user_int(args[1])
    k = get_user_float(args[2])
    result = n
    if n > 1000:
        display_error("Invalid argument \"{}\", please enter a valid integer value".format(n))
        exit(84)
    for i in range(0, 100):
        if i != 0:
            result = result * k * (1000 - result) / 1000
        if result < 0:
            result = 0
        print(f"{i + 1} {result:.2f}")

def case2(args):
    k = float(1.0)
    n = get_user_int(args[1])
    i0 = get_user_int(args[2])
    i1 = get_user_int(args[3])

    if n > 1000:
        display_error("Invalid argument \"{}\", please enter a valid integer value".format(n))
        exit(84)
    if i0 > i1:
        display_error("Invalid argument \"{}\", i1 must be superior to i0".format(i1))
        exit(84)
    while (k < 4.0):
        i0_individuals_number = get_i0_num(n, k, i0)
        result = i0_individuals_number
        for i in range(i1 - (i0 - 1)):
            if i != i1:
                result = result * k * ((1000 - result) / 1000)
            print(f"{k:.2f} {round(result, 2):.2f}")
        k += 0.01