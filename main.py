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
    n = int(args[1])
    k1 = float(args[2])
    k2 = float(args[3])
    for i in range():
        pass


def main(args):
    if len(args) == 3:
        step_1(args)
    elif len(args) == 4:
        step_2(args)
    else:
        print("USAGE: ./106bombyx n [k | i0 i1]")


if __name__ == '__main__':
    main(argv)
