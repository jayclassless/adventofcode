# https://adventofcode.com/2025/day/9

from itertools import product

from aoc2025.util import solve, get_lines, Point


def get_points(input: str) -> list[Point]:
    return [Point(*[int(p) for p in line.split(",")]) for line in get_lines(input)]


def area(a: Point, b: Point) -> int:
    return (abs(b.x - a.x) + 1) * (abs(b.y - a.y) + 1)


def part1(input: str) -> int:
    points = get_points(input)

    areas = [area(a, b) for a, b in product(points, points) if a != b]

    return max(areas)


def part2(input: str) -> int:
    pass


solve("9/example1.txt", part1, 50)
solve("9/input1.txt", part1, 4735222687)
# solve("9/example1.txt", part2)
# solve("9/input1.txt", part2)
