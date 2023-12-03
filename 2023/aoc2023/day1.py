# https://adventofcode.com/2023/day/1

import re
from typing import Callable

from .util import get_lines


def find_digits_part1(value: str) -> str:
    return re.sub(r"\D", "", value)


DIGITS = {
    "1": 1,
    "2": 2,
    "3": 3,
    "4": 4,
    "5": 5,
    "6": 6,
    "7": 7,
    "8": 8,
    "9": 9,
    "0": 0,
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
    # "zero": 0,
}


def find_digits_part2(value: str) -> str:
    found = {}
    for s, i in DIGITS.items():
        idx = value.find(s)
        if idx > -1:
            found[idx] = i
        idx = value.rfind(s)
        if idx > -1:
            found[idx] = i

    return f"{found[min(found.keys())]}{found[max(found.keys())]}"


def solve(input: str, finder: Callable[[str], str]) -> None:
    print(input)

    data = get_lines(input)

    total = 0
    for line in data:
        digits = finder(line)
        calibration = int(digits[0] + digits[-1])
        total += calibration

    print(f"Calibration Sum: {total}")


for input in ("1/example1.txt", "1/input1.txt"):
    solve(input, find_digits_part1)
for input in ("1/example2.txt", "1/input2.txt"):
    solve(input, find_digits_part2)
