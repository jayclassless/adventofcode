# https://adventofcode.com/2022/day/14


from dataclasses import dataclass
from typing import Union

from .util import get_lines


@dataclass
class Point:
  x: int
  y: int

@dataclass
class Path:
  start: Point
  end: Point

Cave = list[list[str]]


EMPTY = ' '
ROCK = '#'
SAND = 'o'


def get_paths(lines: list[str]) -> list[Path]:
  paths: list[Path] = []

  for line in lines:
    coords: list[Point] = []
    for coord in line.split(' -> '):
      x, y = [int(n) for n in coord.split(',', 2)]
      coords.append(Point(x=x, y=y))
    
    for i in range(len(coords) - 1):
      paths.append(Path(
        start=coords[i],
        end=coords[i + 1]
      ))

  return paths

def make_cave(paths: list[Path], endless: bool = True) -> Cave:
  max_x = max([
    max(path.start.x, path.end.x)
    for path in paths
  ]) + 1
  max_y = max([
    max(path.start.y, path.end.y)
    for path in paths
  ]) + 1

  if not endless:
    max_x *= 2

  cave: Cave = [
    [EMPTY] * max_x
    for _ in range(max_y)
  ]

  if not endless:
    cave.append([EMPTY] * max_x)
    cave.append([ROCK] * max_x)

  for path in paths:
    if path.start.x == path.end.x:
      x = path.start.x
      ys = sorted([path.start.y, path.end.y])
      for y in range(ys[0], ys[1] + 1):
        cave[y][x] = ROCK
    elif path.start.y == path.end.y:
      y = path.start.y
      xs = sorted([path.start.x, path.end.x])
      for x in range(xs[0], xs[1] + 1):
        cave[y][x] = ROCK

  return cave

def drop_sand(cave: Cave, origination: Point) -> Union[Point, None]:
  x, y = origination.x, origination.y

  if cave[y][x] != EMPTY:
    # origination point is clogged
    return None

  while True:
    y += 1

    if y >= len(cave):
      # fell out the bottom
      return None

    if cave[y][x] == EMPTY:
      # can move down
      continue

    if (x - 1) >= 0 and cave[y][x - 1] == EMPTY:
      # can slide left
      x -= 1
      continue

    if (x + 1) < len(cave[0]) and cave[y][x + 1] == EMPTY:
      # can slide right
      x += 1
      continue

    cave[y - 1][x] = SAND
    return Point(x=x, y=y -1)

def print_cave(cave: Cave):
  min_x, max_x = len(cave[0]), 0
  min_y, max_y = len(cave), 0

  for y, row in enumerate(cave):
    for x, value in enumerate(row):
      if value != EMPTY:
        if y > max_y: max_y = y
        if y < min_y: min_y = y
        if x > max_x: max_x = x
        if x < min_x: min_x = x

  for y, row in enumerate(cave):
    if y > max_y or y < min_y: continue
    for x, value in enumerate(row):
      if x > max_x or x < min_x: continue
      print(value, end='')
    print('')

def part1(cave: Cave) -> int:
  num_dropped = 0

  while drop_sand(cave, Point(x=500, y=0)):
    num_dropped += 1

  return num_dropped

def part2(cave: Cave) -> int:
  x = part1(cave)

  # cave.pop()
  # print_cave(cave)

  return x


data = get_lines('14/input.txt')

cave = make_cave(get_paths(data))
print(f"Part 1: {part1(cave)}")

cave = make_cave(get_paths(data), endless=False)
print(f"Part 2: {part2(cave)}")
