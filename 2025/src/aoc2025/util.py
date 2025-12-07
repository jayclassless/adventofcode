from dataclasses import dataclass, replace
from enum import IntEnum
from pathlib import Path
from typing import Any


DATA_DIR = Path(__file__).parent / "data"


def get_lines(file: Path | str) -> list[str]:
    with open(DATA_DIR / file) as fp:
        return [line.rstrip("\n") for line in fp.readlines()]


class Rotation(IntEnum):
    LEFT = 0
    RIGHT = 1


class Direction(IntEnum):
    UP = 0
    RIGHT = 1
    DOWN = 2
    LEFT = 3

    @staticmethod
    def rotate(dir: "Direction", rotation: Rotation) -> "Direction":
        change = 1 if rotation == Rotation.RIGHT else -1
        return Direction((dir.value + change) % 4)


@dataclass(order=True)
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))

    def move(self, direction: Direction, distance=1) -> "Point":
        dx, dy = 0, 0

        if direction == Direction.UP:
            dy = -distance
        elif direction == Direction.DOWN:
            dy = distance
        elif direction == Direction.LEFT:
            dx = -distance
        elif direction == Direction.RIGHT:
            dx = distance

        return Point(x=self.x + dx, y=self.y + dy)


@dataclass(order=True)
class Cell(Point):
    value: Any


@dataclass
class Grid:
    values: list[list[Any]]

    @classmethod
    def from_input(cls, input: str) -> "Grid":
        return cls(values=[list(line) for line in get_lines(input)])

    @property
    def width(self):
        return len(self.values[0])

    @property
    def height(self):
        return len(self.values)

    def cell(self, x: int, y: int) -> Cell | None:
        if self.contains_coords(x, y):
            return Cell(x=x, y=y, value=self.values[y][x])
        return None

    def contains_coords(self, x: int, y: int) -> bool:
        return x >= 0 and x < self.width and y >= 0 and y < self.height

    def contains(self, point: Point) -> bool:
        return self.contains_coords(point.x, point.y)

    def neighbors(self, point: Point) -> list[Cell]:
        possible = [
            (point.x - 1, point.y - 1),
            (point.x, point.y - 1),
            (point.x + 1, point.y - 1),
            (point.x - 1, point.y),
            (point.x + 1, point.y),
            (point.x - 1, point.y + 1),
            (point.x, point.y + 1),
            (point.x + 1, point.y + 1),
        ]

        return [
            Cell(x=x, y=y, value=self.values[y][x])
            for x, y in possible
            if self.contains_coords(x, y)
        ]


@dataclass(order=True)
class Position:
    point: Point
    direction: Direction

    def __hash__(self) -> int:
        return hash((self.point, self.direction))

    def turn(self, rotation: Rotation) -> "Position":
        return replace(self, direction=Direction.rotate(self.direction, rotation))

    def move(self, distance=1) -> "Position":
        return replace(self, point=self.point.move(self.direction, distance))

    def in_grid(self, grid: Grid) -> bool:
        return grid.contains(self.point)
