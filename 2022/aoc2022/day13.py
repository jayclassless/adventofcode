# https://adventofcode.com/2022/day/13

from dataclasses import dataclass
from functools import cmp_to_key, reduce
from itertools import zip_longest
from typing import Union

from .util import get_lines


Value = Union[int, list['Value']]
Packet = list[Value]
MaybeBool = bool | None

@dataclass
class Pair:
  left: Packet
  right: Packet


def get_pairs(lines: list[str]) -> list[Pair]:
  pairs: list[Pair] = []

  i = 0
  while i < len(lines):
    left = eval(lines[i])
    i += 1
    right = eval(lines[i])
    i += 2
    pairs.append(Pair(left=left, right=right))

  return pairs

def is_ordered(left: Packet, right: Packet) -> MaybeBool:
  for l, r in zip_longest(left, right, fillvalue=None):
    if isinstance(l, int) and isinstance(r, int):
      if l < r:
        return True
      if r < l:
        return False

    elif isinstance(l, list) and isinstance(r, list):
      check = is_ordered(l, r)
      if check is not None:
        return check

    elif l is None:
      return True
    elif r is None:
      return False

    else:
      if isinstance(l, int):
        l = [l]
      if isinstance(r, int):
        r = [r]
      check = is_ordered(l, r)
      if check is not None:
        return check

  return None

def part1(pairs: list[Pair]) -> int:
  indicies: list[int] = []

  for i, pair in enumerate(pairs):
    if is_ordered(pair.left, pair.right):
      indicies.append(i + 1)

  return sum(indicies)

DIVIDERS: list[Packet] = [
  [[2]],
  [[6]]
]

def part2(pairs: list[Pair]) -> int:
  packets: list[Packet] = list(DIVIDERS)
  for pair in pairs:
    packets.append(pair.left)
    packets.append(pair.right)

  ordered = sorted(
    packets,
    key=cmp_to_key(
      lambda x, y: -1 if is_ordered(x, y) else 1
    )
  )

  return reduce(
    lambda x, y: x * y,
    [
      ordered.index(divider) + 1
      for divider in DIVIDERS
    ]
  )


data = get_lines('13/input.txt')
pairs = get_pairs(data)

print(f"Part 1: {part1(pairs)}")
print(f"Part 2: {part2(pairs)}")
