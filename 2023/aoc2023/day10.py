# https://adventofcode.com/2023/day/10

from typing import TypeAlias

from .util import get_lines


Point: TypeAlias = tuple[int, int]
Grid: TypeAlias = dict[Point, str]


def parse_input(lines: list[str]) -> tuple[Grid, Point]:
    grid: dict[Point, str] = {}
    start = None

    for y, line in enumerate(lines):
        for x, contents in enumerate(line):
            point = (x, y)
            grid[point] = contents

            if contents == "S":
                start = point

    if not start:
        raise ValueError("did not find start")

    return grid, start


def dimensions(grid: Grid) -> tuple[int, int]:
    return (
        max([p[0] for p in grid.keys()]) + 1,
        max([p[1] for p in grid.keys()]) + 1,
    )


def trace_loop(grid: Grid, start: Point) -> set[Point]:
    width, height = dimensions(grid)
    seen = set([start])
    to_check = [start]

    while to_check:
        x, y = to_check.pop(0)
        current = grid[(x, y)]

        up = (x, y - 1)
        if up not in seen and y > 0 and current in "S|LJ" and grid[up] in "|7F":
            seen.add(up)
            to_check.append(up)

        down = (x, y + 1)
        if (
            down not in seen
            and y < (height - 1)
            and current in "S|7F"
            and grid[down] in "|LJ"
        ):
            seen.add(down)
            to_check.append(down)

        left = (x - 1, y)
        if left not in seen and x > 0 and current in "S-7J" and grid[left] in "-LF":
            seen.add(left)
            to_check.append(left)

        right = (x + 1, y)
        if (
            right not in seen
            and x < (width - 1)
            and current in "S-LF"
            and grid[right] in "-J7"
        ):
            seen.add(right)
            to_check.append(right)

    return seen


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    grid, start = parse_input(data)

    loop = trace_loop(grid, start)

    print(f"Furthest distance: {len(loop) // 2}")


def is_inside_loop(point: Point, loop: set[Point], width: int, height: int) -> bool:
    if point in loop:
        return False

    up = (point[0], point[1] - 1)
    while True:
        if up[1] < 0:
            return False
        if up in loop:
            break
        up = (up[0], up[1] - 1)

    down = (point[0], point[1] + 1)
    while True:
        if down[1] >= height:
            return False
        if down in loop:
            break
        down = (down[0], down[1] + 1)

    left = (point[0] - 1, point[1])
    while True:
        if left[0] < 0:
            return False
        if left in loop:
            break
        left = (left[0] - 1, left[1])

    right = (point[0] + 1, point[1])
    while True:
        if right[1] >= width:
            return False
        if right in loop:
            break
        right = (right[0] + 1, right[1])

    return True


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    grid, start = parse_input(data)
    width, height = dimensions(grid)

    loop = trace_loop(grid, start)
    inside = [
        point for point in grid.keys() if is_inside_loop(point, loop, width, height)
    ]

    print(f"Enclosed: {len(inside)}")


solve_part1("10/example1.txt")
solve_part1("10/example2.txt")
solve_part1("10/input1.txt")
solve_part2("10/example3.txt")
solve_part2("10/example4.txt")
solve_part2("10/input2.txt")
