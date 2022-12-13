# https://adventofcode.com/2022/day/12

from collections import deque
from typing import Deque

from .util import get_lines


HeightMap = list[list[str]]


def make_map(lines: list[str]) -> HeightMap:
  return [
    [cell for cell in line]
    for line in lines
  ]

def get_value(input: str) -> int:
  input = input.replace('S', 'a').replace('E', 'z')
  return ord(input)

def find_shortest_path(heightmap: HeightMap, start_values: list[str]) -> int:
  queue: Deque[tuple[int, int, int, int]] = deque(
    (row, col, 0, get_value(heightmap[row][col]))
    for row in range(len(heightmap))
    for col in range(len(heightmap[row]))
    if heightmap[row][col] in start_values
  )

  seent = set((row, col) for row, col, _, _ in queue)

  def push(row, col, depth, value):
    # out of bounds
    if row < 0 or row >= len(heightmap):
      return
    if col < 0 or col >= len(heightmap[row]):
      return

    # been there
    if (row, col) in seent:
      return

    # too steep
    next_value = get_value(heightmap[row][col])
    if next_value > value + 1:
      return

    seent.add((row, col))
    queue.append((row, col, depth + 1, next_value))

  while queue:
      row, col, depth, value = queue.popleft()

      if heightmap[row][col] == 'E':
        return depth

      push(row - 1, col, depth, value)
      push(row + 1, col, depth, value)
      push(row, col - 1, depth, value)
      push(row, col + 1, depth, value)

  return -1  # shouldn't get here?

def part1(heightmap: HeightMap) -> int:
  return find_shortest_path(heightmap, ['S'])

def part2(heightmap: HeightMap) -> int:
  return find_shortest_path(heightmap, ['S', 'a'])


data = get_lines('12/input.txt')
heightmap = make_map(data)

print(f"Part 1: {part1(heightmap)}")
print(f"Part 2: {part2(heightmap)}")
