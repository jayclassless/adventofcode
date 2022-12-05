# https://adventofcode.com/2022/day/1

from .util import get_lines


def tally_elves(data: list[str]) -> list[int]:
  elves: list[int] = []

  for line in data:
    if not line or len(elves) == 0:
      elves.append(0)
    if not line:
      continue

    elves[-1] += int(line)

  return elves


data = get_lines('1/input.txt')
elves = sorted(tally_elves(data), reverse=True)
top_calories = elves[0]
top_3_calories = sum(elves[0:3])

print(f"Top Calories: {top_calories}")
print(f"Top 3 Calories: {top_3_calories}")
