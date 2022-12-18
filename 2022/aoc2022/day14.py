# https://adventofcode.com/2022/day/14


from dataclasses import dataclass

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

def make_cave(paths: list[Path]) -> Cave:
  max_x = max([
    max(path.start.x, path.end.x)
    for path in paths
  ]) + 1
  max_y = max([
    max(path.start.y, path.end.y)
    for path in paths
  ]) + 1
  cave: Cave = [
    [EMPTY] * max_x
    for _ in range(max_y)
  ]

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

def drop_sand(cave: Cave, origination: Point) -> bool:
  x, y = origination.x, origination.y

  while True:
    y += 1

    if y >= len(cave):
      # fell out the bottom
      return False

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
    return True

def part1(cave: Cave) -> int:
  num_dropped = 0

  while drop_sand(cave, Point(x=500, y=0)):
    num_dropped += 1

  return num_dropped

def part2(cave: Cave) -> int:
  cave.append([EMPTY] * len(cave[0]))
  cave.append([ROCK] * len(cave[0]))

  return - 1


data = get_lines('14/example.txt')

cave = make_cave(get_paths(data))
print(f"Part 1: {part1(cave)}")

cave = make_cave(get_paths(data))
print(f"Part 2: {part2(cave)}")
