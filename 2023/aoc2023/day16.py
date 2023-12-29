# https://adventofcode.com/2023/day/16

from enum import Enum

from .util import get_lines, Point


class Direction(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4


class Map:
    def __init__(self, lines: list[str]) -> None:
        self.energized: set[Point] = set()
        self.grid = [list(line) for line in lines]

    def start(self, point: Point, direction: Direction) -> int:
        to_check: list[tuple[Point, Direction]] = [(point, direction)]
        checked: list[tuple[Point, Direction]] = []
        self.energized = set()

        def queue(point, direction):
            next = self.next_point(point, direction)
            if next and (next, direction) not in checked:
                to_check.append((next, direction))

        while to_check:
            point, direction = to_check.pop(0)

            self.energized.add(point)
            checked.append((point, direction))

            char = self.grid[point.y][point.x]

            if char == ".":
                queue(point, direction)

            elif char == "-":
                if direction in (Direction.LEFT, Direction.RIGHT):
                    queue(point, direction)

                else:
                    queue(point, Direction.LEFT)
                    queue(point, Direction.RIGHT)

            elif char == "|":
                if direction in (Direction.UP, Direction.DOWN):
                    queue(point, direction)

                else:
                    queue(point, Direction.UP)
                    queue(point, Direction.DOWN)

            elif char == "\\":
                if direction == Direction.UP:
                    queue(point, Direction.LEFT)
                elif direction == Direction.DOWN:
                    queue(point, Direction.RIGHT)
                elif direction == Direction.LEFT:
                    queue(point, Direction.UP)
                elif direction == Direction.RIGHT:
                    queue(point, Direction.DOWN)

            elif char == "/":
                if direction == Direction.UP:
                    queue(point, Direction.RIGHT)
                elif direction == Direction.DOWN:
                    queue(point, Direction.LEFT)
                elif direction == Direction.LEFT:
                    queue(point, Direction.DOWN)
                elif direction == Direction.RIGHT:
                    queue(point, Direction.UP)

        return len(self.energized)

    def next_point(self, point: Point, direction: Direction) -> Point | None:
        if direction == Direction.UP:
            if point.y > 0:
                return Point(x=point.x, y=point.y - 1)
        elif direction == Direction.DOWN:
            if point.y < (len(self.grid) - 1):
                return Point(x=point.x, y=point.y + 1)
        elif direction == Direction.LEFT:
            if point.x > 0:
                return Point(x=point.x - 1, y=point.y)
        elif direction == Direction.RIGHT:
            if point.x < (len(self.grid[0]) - 1):
                return Point(x=point.x + 1, y=point.y)
        return None


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = Map(data)
    energized = map.start(Point(x=0, y=0), Direction.RIGHT)

    print(f"Energized Tiles: {energized}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = Map(data)

    starters = []
    y = 0
    for x in range(len(map.grid[y])):
        starters.append((Point(x=x, y=y), Direction.DOWN))
    y = len(map.grid) - 1
    for x in range(len(map.grid[y])):
        starters.append((Point(x=x, y=y), Direction.UP))
    x = 0
    for y in range(len(map.grid)):
        starters.append((Point(x=x, y=y), Direction.RIGHT))
    x = len(map.grid[0]) - 1
    for y in range(len(map.grid)):
        starters.append((Point(x=x, y=y), Direction.LEFT))

    possibles = []
    for idx, (point, direction) in enumerate(starters):
        possibles.append(map.start(point, direction))

    print(f"Best Configuration: {max(possibles)}")


solve_part1("16/example1.txt")
solve_part1("16/input1.txt")
solve_part2("16/example2.txt")
solve_part2("16/input2.txt")
