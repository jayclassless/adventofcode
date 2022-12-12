# https://adventofcode.com/2022/day/11

from collections import defaultdict
from dataclasses import dataclass
from functools import reduce

from .util import get_lines


@dataclass
class Monkey:
  name: int
  items: list[int]
  operation: str
  test_value: int
  test_pass: int
  test_fail: int

  @staticmethod
  def from_input(input: list[str]) -> 'Monkey':
    return Monkey(
      name=int(input[0][7:-1]),
      items=[int(i) for i in input[1][18:].split(',')],
      operation=input[2][23:],
      test_value=int(input[3][21:]),
      test_pass=int(input[4][29:]),
      test_fail=int(input[5][30:])
    )

  def inspect(self, item: int) -> int:
    new_value = item
    match self.operation.split(' '):
      case '*', 'old': new_value = item * item
      case '*', val: new_value = item * int(val)
      case '+', val: new_value = item + int(val)
      case huh: raise Exception(f"wtf is {huh}")
    return new_value

  def decide_target(self, item: int):
    if item % self.test_value == 0:
      return self.test_pass
    return self.test_fail


def get_monkeys(lines: list[str]) -> list[Monkey]:
  monkeys: list[Monkey] = []

  for i in range(0, len(lines), 7):
    monkeys.append(Monkey.from_input(lines[i:(i + 6)]))

  return monkeys

def do_round(monkeys: list[Monkey], num_rounds: int = 1, worry_reducer = lambda x: x) -> dict[int, int]:
  inspections = defaultdict(lambda: 0)

  for _ in range(num_rounds):
    for monkey in monkeys:
      while monkey.items:
        inspections[monkey.name] += 1
        item = monkey.items.pop(0)
        worry = worry_reducer(monkey.inspect(item))
        target = monkey.decide_target(worry)
        monkeys[target].items.append(worry)

  return inspections

def part1(monkeys: list[Monkey]) -> int:
  inspections = do_round(monkeys, 20, worry_reducer=lambda x: x // 3)
  highest = sorted(inspections.values(), reverse=True)[:2]
  return reduce(lambda x, y: x * y, highest)

def part2(monkeys: list[Monkey]) -> int:
  mod = reduce(
    lambda x, y: x * y,
    [monkey.test_value for monkey in monkeys],
    1
  )
  inspections = do_round(monkeys, 10000, worry_reducer=lambda x: x % mod)
  highest = sorted(inspections.values(), reverse=True)[:2]
  return reduce(lambda x, y: x * y, highest)


data = get_lines('11/input.txt')

monkeys = get_monkeys(data)
print(f"Part 1: {part1(monkeys)}")

monkeys = get_monkeys(data)
print(f"Part 2: {part2(monkeys)}")
