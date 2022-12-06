# https://adventofcode.com/2022/day/6

from .util import get_lines

def find_first_marker(data: str, num_unique: int) -> int:
  for i in range(num_unique, len(data)):
    window = data[(i - num_unique):i]
    if len(set(window)) == num_unique:
      return i
  return -1

data = get_lines('6/input.txt')
starts = [find_first_marker(line, 4) for line in data]
print(f'First packets after index: {starts}')
starts = [find_first_marker(line, 14) for line in data]
print(f'First messages after index: {starts}')
