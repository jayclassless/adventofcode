# https://adventofcode.com/2022/day/8

from .util import get_lines


Grid = list[list[int]]


def make_grid(lines: list[str]) -> Grid:
  grid: Grid = []

  for row, line in enumerate(lines):
    grid.append([])
    for cell in line:
      grid[row].append(int(cell))

  return grid

def is_visible(grid: Grid, row: int, col: int) -> bool:
  if row == 0 or row == (len(grid) - 1):
    return True
  if col == 0 or col == (len(grid[0]) - 1):
    return True

  target = grid[row][col]

  above = [
    grid[r][col]
    for r in range(row)
  ]
  if all([i < target for i in above]):
    return True

  below = [
    grid[r][col]
    for r in range(row + 1, len(grid))
  ]
  if all([i < target for i in below]):
    return True

  left = [
    grid[row][c]
    for c in range(col)
  ]
  if all([i < target for i in left]):
    return True

  right = [
    grid[row][c]
    for c in range(col + 1, len(grid[0]))
  ]
  if all([i < target for i in right]):
    return True

  return False

def part1(grid: Grid) -> int:
  visible: int = 0

  for r in range(len(grid)):
    for c in range(len(grid[0])):
      if is_visible(grid, r, c):
        visible += 1

  return visible

def get_score(grid: Grid, row: int, col: int) -> int:
  target = grid[row][col]

  above = [
    grid[r][col]
    for r in range(row - 1, -1, -1)
  ]

  below = [
    grid[r][col]
    for r in range(row + 1, len(grid))
  ]

  left = [
    grid[row][c]
    for c in range(col - 1, -1, -1)
  ]

  right = [
    grid[row][c]
    for c in range(col + 1, len(grid[0]))
  ]

  def num_vis(trees: list[int]) -> int:
    for i, value in enumerate(trees):
      if value >= target:
        return i + 1
    return len(trees)

  return (
    num_vis(above) \
    * num_vis(below) \
    * num_vis(left) \
    * num_vis(right)
  )

def part2(grid: Grid) -> int:
  best_score: int = 0

  for r in range(len(grid)):
    for c in range(len(grid[0])):
      score = get_score(grid, r, c)
      if score > best_score:
        best_score = score

  return best_score


data = get_lines('8/input.txt')
grid = make_grid(data)

print(f"Part 1: {part1(grid)}")
print(f"Part 2: {part2(grid)}")
