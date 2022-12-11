# https://adventofcode.com/2022/day/9

from enum import Enum

from .util import get_lines


class Direction(Enum):
  UP = 'U'
  DOWN = 'D'
  LEFT = 'L'
  RIGHT = 'R'

Move = tuple[Direction, int]
Position = tuple[int, int]


def get_moves(lines: list[str]) -> list[Move]:
  moves: list[Move] = []

  for line in lines:
    direction, distance = line.split(' ', 2)
    moves.append((Direction(direction), int(distance)))

  return moves

DIR_CHANGES: dict[Direction, tuple[int, int]] = {
  Direction.UP: (0, 1),
  Direction.DOWN: (0, -1),
  Direction.LEFT: (-1, 0),
  Direction.RIGHT: (1, 0)
}

def adjust_position(leader: Position, follower: Position) -> Position:
  dx = leader[0] - follower[0]
  dy = leader[1] - follower[1]
  if dx == 0 or dy == 0:
    if abs(dx) >= 2:
      follower = (
        follower[0] + (dx // abs(dx)),
        follower[1]
      )
    elif abs(dy) >= 2:
      follower = (
        follower[0],
        follower[1] + (dy // abs(dy))
      )
  elif (abs(dx), abs(dy)) != (1, 1):
    follower = (
      follower[0] + (dx // abs(dx)),
      follower[1] + (dy // abs(dy))
    )

  return follower

def simulate(moves: list[Move], num_knots: int) -> int:
  rope: list[Position] = [(0, 0)] * num_knots
  path: list[Position] = [
    (0, 0)
  ]

  for move in moves:
    hdx, hdy = DIR_CHANGES[move[0]]

    for _ in range(move[1]):
      rope[0] = (
        rope[0][0] + hdx,
        rope[0][1] + hdy
      )
      for i in range(num_knots - 1):
        rope[i + 1] = adjust_position(rope[i], rope[i + 1])
        if (i + 2) == num_knots:
          path.append(rope[i + 1])

  return len(set(path))


data = get_lines('9/input.txt')
moves = get_moves(data)

print(f"Part 1: {simulate(moves, 2)}")
print(f"Part 2: {simulate(moves, 10)}")
