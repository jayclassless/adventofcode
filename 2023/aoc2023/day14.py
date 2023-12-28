# https://adventofcode.com/2023/day/14

from copy import deepcopy
from typing import TypeAlias

import numpy

from .util import get_lines


Map: TypeAlias = list[list[str]]


def parse_map(lines: list[str]) -> Map:
    return [list(line) for line in lines]


def rotate(map: Map) -> Map:
    return numpy.rot90(map, 3).tolist()


def roll_north(map: Map) -> Map:
    while True:
        previous = deepcopy(map)

        for y in range(1, len(map)):
            for x in range(len(map[y])):
                if map[y][x] != "O":
                    continue

                if map[y - 1][x] == ".":
                    map[y - 1][x] = "O"
                    map[y][x] = "."

        if map == previous:
            break

    return map


def print_map(map: Map):
    for line in map:
        print("".join(line))
    print(" ")


def find_load(map: Map) -> int:
    total_load = 0
    for idx, line in enumerate(map):
        load = len(map) - idx
        total_load += load * len([x for x in line if x == "O"])

    return total_load


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = parse_map(data)
    map = roll_north(map)

    total_load = find_load(map)
    print(f"Total Load: {total_load}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = parse_map(data)

    for _ in range(1000000000):
        map = roll_north(map)
        map = rotate(map)
        map = roll_north(map)
        map = rotate(map)
        map = roll_north(map)
        map = rotate(map)
        map = roll_north(map)
        map = rotate(map)

    total_load = find_load(map)
    print(f"Total Load: {total_load}")


solve_part1("14/example1.txt")
solve_part1("14/input1.txt")
solve_part2("14/example2.txt")
# solve_part2("14/input2.txt")
