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

    def in_grid(self, grid: list[list[Any]]) -> bool:
        return (
            self.x >= 0
            and self.x <= (len(grid[0]) - 1)
            and self.y >= 0
            and self.y <= (len(grid) - 1)
        )


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

    def in_grid(self, grid: list[list[Any]]) -> bool:
        return self.point.in_grid(grid)
