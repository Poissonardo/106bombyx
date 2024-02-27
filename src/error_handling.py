PROJECT_NAME = "106bombyx"

class NegativeNumberException(Exception):
    pass

class KInvalidValue(Exception):
    pass

def display_error(error: str) -> None:
    print(PROJECT_NAME + ": " + error)
