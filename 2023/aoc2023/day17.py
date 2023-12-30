# https://adventofcode.com/2023/day/17

from heapq import heappop, heappush
from typing import TypeAlias

from .util import get_lines, Direction, Point, Position, Rotation


def get_grid(lines: list[str]) -> list[list[int]]:
    return [[int(char) for char in line] for line in lines]


State: TypeAlias = tuple[int, Position, int]


def find_path(grid: list[list[int]], min_steps: int, max_steps: int) -> int:
    target = Point(
        x=len(grid[-1]) - 1,
        y=len(grid) - 1,
    )

    to_check: list[State] = [
        (0, Position(Point(0, 0), Direction.RIGHT), 0),
        (0, Position(Point(0, 0), Direction.DOWN), 0),
    ]
    checked: set[tuple[Position, int]] = set()

    while to_check:
        cost, position, steps = heappop(to_check)

        if position.point == target and steps >= min_steps:
            return cost

        if (position, steps) in checked:
            continue
        checked.add((position, steps))

        if steps >= min_steps:
            left = position.turn(Rotation.LEFT).move()
            if left.in_grid(grid):
                heappush(
                    to_check,
                    (
                        cost + grid[left.point.y][left.point.x],
                        left,
                        1,
                    ),
                )

            right = position.turn(Rotation.RIGHT).move()
            if right.in_grid(grid):
                heappush(
                    to_check,
                    (
                        cost + grid[right.point.y][right.point.x],
                        right,
                        1,
                    ),
                )

        if steps < max_steps:
            forward = position.move()
            if forward.in_grid(grid):
                heappush(
                    to_check,
                    (
                        cost + grid[forward.point.y][forward.point.x],
                        forward,
                        steps + 1,
                    ),
                )

    return -1


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    grid = get_grid(data)
    cost = find_path(grid, 0, 3)

    print(f"Heat Loss: {cost}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    grid = get_grid(data)
    cost = find_path(grid, 4, 10)

    print(f"Heat Loss: {cost}")


solve_part1("17/example1.txt")
solve_part1("17/input1.txt")
solve_part2("17/example2.txt")
solve_part2("17/input2.txt")
