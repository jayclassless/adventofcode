# https://adventofcode.com/2025/day/3

from aoc2025.util import get_lines


def find_biggest(line: str, space_after: int) -> tuple[str, int]:
    s = sorted(
        line[: None if space_after == 0 else -space_after],
        reverse=True,
    )[0]
    return (s, line.find(s))


def find_joltage(line: str, num_digits: int) -> int:
    digits = []
    shift = 0
    for i in range(num_digits, 0, -1):
        d, d_idx = find_biggest(line[shift:], i - 1)
        digits.append(d)
        shift += d_idx + 1
    return int("".join(digits))


def solve_part1(input: str) -> None:
    print(sum([find_joltage(line, 2) for line in get_lines(input)]))


def solve_part2(input: str) -> None:
    print(sum([find_joltage(line, 12) for line in get_lines(input)]))


solve_part1("3/example1.txt")
solve_part1("3/input1.txt")
solve_part2("3/example1.txt")
solve_part2("3/input1.txt")
