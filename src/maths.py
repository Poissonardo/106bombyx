from .user_input import *

def get_i0_num(n, k, i0):
    populations = [n]
    first_gen = 0
    for i in range(0, i0):
        if i != 0:
            populations.append(populations[i - 1] * k * (1000 - populations[i - 1]) / 1000)
        first_gen = populations[i]
    return first_gen

def case1(args):
    n = get_user_int(args[1])
    k = get_user_float(args[2])
    populations = [n]
    if n > 1000:
        display_error("Invalid argument \"{}\", please enter a valid integer value".format(n))
        exit(84)
    for i in range(0, 100):
        if i != 0:
            populations.append(populations[i - 1] * k * (1000 - populations[i - 1]) / 1000)
        print(f"{i + 1} {populations[i]:.2f}")

def case2(args):
    k = 1.00
    n = get_user_int(args[1])
    i0 = get_user_int(args[2])
    i1 = get_user_int(args[3])

    if i0 > i1:
        display_error("Invalid argument \"{}\", i1 must be superior to i0".format(i1))
        exit(84)
    while (k <= 4):
        i0_individuals_number = get_i0_num(n, k, i0)
        populations = [i0_individuals_number]
        for i in range(i1 - (i0 - 1)):
            if i != i1:
                populations.append(populations[i - 1] * k * (1000 - populations[i - 1]) / 1000)
            print(f"{k:.2f} {populations[i]:.2f}")
        k += 0.01