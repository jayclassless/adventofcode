# https://adventofcode.com/2025/day/5

from aoc2025.util import get_lines


def get_input(input: str) -> tuple[list[tuple[int, int]], list[int]]:
    fresh = []
    available = []

    getting_fresh = True
    for line in get_lines(input):
        if not line:
            getting_fresh = False
            continue

        if getting_fresh:
            fresh.append(tuple(int(part) for part in line.split("-")))
        else:
            available.append(int(line))

    return fresh, available


def solve_part1(input: str) -> None:
    fresh, available = get_input(input)

    count = 0
    for ingredient in available:
        for start, end in fresh:
            if ingredient >= start and ingredient <= end:
                count += 1
                break

    print(count)


def solve_part2(input: str) -> None:
    fresh, _ = get_input(input)
    fresh = sorted(fresh)

    collapsed = [fresh[0]]
    for start, end in fresh[1:]:
        if (start - 1) <= collapsed[-1][1]:
            collapsed[-1] = (min(collapsed[-1][0], start), max(collapsed[-1][1], end))
        else:
            collapsed.append((start, end))

    total = sum([end - start + 1 for start, end in collapsed])

    print(total)


solve_part1("5/example1.txt")
solve_part1("5/input1.txt")
solve_part2("5/example1.txt")
solve_part2("5/input1.txt")
