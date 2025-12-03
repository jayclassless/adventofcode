# https://adventofcode.com/2025/day/2

import re

from aoc2025.util import get_lines

Range = tuple[int, int]


def get_ranges(input: str) -> list[Range]:
    return [tuple([int(x) for x in pair.split("-", 1)]) for pair in input.split(",")]


def find_invalids(rng: Range, repeats=False) -> list[int]:
    if repeats:
        pat = re.compile(r"(.+?)\1+$")
    else:
        pat = re.compile(r"(.+?)\1$")

    return [int(n) for n in range(rng[0], rng[1] + 1) if pat.match(str(n))]


def solve_part1(input: str) -> None:
    ranges = get_ranges(get_lines(input)[0])

    invalids = []
    for rng in ranges:
        invalids.extend(find_invalids(rng))

    print(sum(invalids))


def solve_part2(input: str) -> None:
    ranges = get_ranges(get_lines(input)[0])

    invalids = []
    for rng in ranges:
        invalids.extend(find_invalids(rng, repeats=True))

    print(sum(invalids))


solve_part1("2/example1.txt")
solve_part1("2/input1.txt")
solve_part2("2/example1.txt")
solve_part2("2/input1.txt")
