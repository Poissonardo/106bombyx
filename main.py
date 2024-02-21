#!/usr/bin/python3
from sys import argv


def step_1(args):
    n = int(args[1])
    k = float(args[2])
    populations = [n]
    for i in range(0, 100):
        if i != 0:
            populations.append(populations[i - 1] * k * (1000 - populations[i - 1]) / 1000)
        print(f"{i + 1} {populations[i]:.2f}")


def step_2(args):
    k = 1.00
    n = int(args[1])
    i0 = int(args[2])
    i1 = int(args[3])
    for i in range(301):
        print("\n")
        for j in range(i1 - i0 + 1):
            populations = [n]
            for i_bis in range(0, i1 + 1):
                if i_bis != 0:
                    populations.append(populations[i_bis - 1] * k * (1000 - populations[i_bis - 1]) / 1000)
            result = populations[i0 + j]
            print(f"{k:.2f} {result:.2f}")
        k += 0.01


def main(args):
    if len(args) == 3:
        step_1(args)
        exit(0)
    elif len(args) == 4:
        step_2(args)
        exit(0)
    else:
        print("USAGE: ./106bombyx n [k | i0 i1]")
        exit(84)


if __name__ == '__main__':
    main(argv)
