from .error_handling import *

def get_user_int(value: str) -> int:
    try:
        n = int(value)
        if (n < 0):
            raise NegativeNumberException
    except ValueError:
        display_error("Invalid argument \"{}\", please enter a valid integer value".format(value))
        exit(84)
    except NegativeNumberException:
        display_error("Invalid argument \"{}\", please enter a positive value".format(value))
        exit(84)
    return n

def get_user_float(value: str) -> float:
    try:
        k = float(value)
        if (k < 0):
            raise NegativeNumberException
    except ValueError:
        display_error("Invalid argument \"{}\", please enter a valid floating point number".format(value))
        exit(84)
    except NegativeNumberException:
        display_error("Invalid argument \"{}\", please enter a positive value".format(value))
        exit(84)
    return k
