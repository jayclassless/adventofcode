# https://adventofcode.com/2023/day/12

from functools import cache

from .util import get_lines


def parse(lines: list[str]) -> list[tuple[str, tuple[int, ...]]]:
    springs = []

    for line in lines:
        conditions, sizes = line.split(" ", 1)
        springs.append((conditions, tuple([int(i) for i in sizes.split(",")])))

    return springs


@cache
def find_possibilities(condition: str, sizes: tuple[int]) -> int:
    if not condition:
        return 1 if not sizes else 0
    if not sizes:
        return 1 if "#" not in condition else 0

    current = condition[0]
    remainder = condition[1:]

    if current == "#":
        if (
            len(condition) >= sizes[0]
            and all([c != "." for c in condition[: sizes[0]]])
            and (len(condition) == sizes[0] or condition[sizes[0]] != "#")
        ):
            return find_possibilities(condition[sizes[0] + 1 :], sizes[1:])

        return 0

    elif current == ".":
        return find_possibilities(remainder, sizes)

    else:
        return find_possibilities("#" + remainder, sizes) + find_possibilities(
            "." + remainder, sizes
        )


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    springs = parse(data)

    arrangements = [
        find_possibilities(condition, sizes) for condition, sizes in springs
    ]

    print(f"Arrangements: {sum(arrangements)}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    springs = [
        (
            "?".join([condition] * 5),
            sizes * 5,
        )
        for condition, sizes in parse(data)
    ]

    arrangements = [
        find_possibilities(condition, sizes) for condition, sizes in springs
    ]

    print(f"Arrangements: {sum(arrangements)}")


solve_part1("12/example1.txt")
solve_part1("12/input1.txt")
solve_part2("12/example2.txt")
solve_part2("12/input2.txt")
