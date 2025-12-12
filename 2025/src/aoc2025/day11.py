# https://adventofcode.com/2025/day/11

from functools import cache

from aoc2025.util import solve, get_lines, HashableDict


class Rack(HashableDict[str, list[str]]):
    pass


def get_rack(input: str) -> Rack:
    rack = Rack()

    for line in get_lines(input):
        source, dests = line.split(":", 1)
        rack[source] = dests.strip().split(" ")

    return rack


@cache
def count_paths(rack: Rack, start: str, end: str) -> int:
    if start == end:
        return 1

    return sum([count_paths(rack, out, end) for out in rack.get(start, [])])


def part1(input: str) -> int:
    rack = get_rack(input)
    return count_paths(rack, "you", "out")


def part2(input: str) -> int:
    rack = get_rack(input)

    paths = (
        count_paths(rack, "svr", "dac")
        * count_paths(rack, "dac", "fft")
        * count_paths(rack, "fft", "out")
    )
    paths += (
        count_paths(rack, "svr", "fft")
        * count_paths(rack, "fft", "dac")
        * count_paths(rack, "dac", "out")
    )

    return paths


solve("11/example1.txt", part1, 5)
solve("11/input1.txt", part1, 472)
solve("11/example2.txt", part2, 2)
solve("11/input1.txt", part2, 526811953334940)
