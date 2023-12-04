# https://adventofcode.com/2023/day/3

import re
from collections import defaultdict
from math import prod

from .util import get_lines


def find_symbols(lines: list[str]) -> dict[tuple[int, int], str]:
    symbols = {}

    for y, line in enumerate(lines):
        for x, char in enumerate(line):
            if not char.isdigit() and char != ".":
                symbols[(x, y)] = char

    return symbols


def solve_part1(input: str) -> None:
    print(input)

    data = get_lines(input)
    symbols = find_symbols(data)
    part_numbers = []

    for y, line in enumerate(data):
        for match in re.finditer(r"\d+", line):
            for symx, symy in symbols.keys():
                if ((match.start() - 1) <= symx <= match.end()) and (
                    (y - 1) <= symy <= (y + 1)
                ):
                    part_numbers.append(int(match.group()))

    print(f"Sum of Part Numbers: {sum(part_numbers)}")


def solve_part2(input: str) -> None:
    print(input)

    data = get_lines(input)
    symbols = find_symbols(data)
    possible_gears = defaultdict(list)

    for y, line in enumerate(data):
        for match in re.finditer(r"\d+", line):
            for (symx, symy), char in symbols.items():
                if ((match.start() - 1) <= symx <= match.end()) and (
                    (y - 1) <= symy <= (y + 1)
                ):
                    if char == "*":
                        possible_gears[(symx, symy)].append(int(match.group()))

    gears = {gear: parts for gear, parts in possible_gears.items() if len(parts) == 2}
    result = sum([prod(parts) for parts in gears.values()])
    print(f"Sum of Gear Ratios: {result}")


solve_part1("3/example1.txt")
solve_part1("3/input1.txt")
solve_part2("3/example2.txt")
solve_part2("3/input2.txt")
