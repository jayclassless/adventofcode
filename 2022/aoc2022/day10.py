# https://adventofcode.com/2022/day/10

from enum import Enum

from .util import get_lines


class Instruction: pass

class NoOp(Instruction): pass

class AddX(Instruction):
  def __init__(self, value: int):
    self.value = value


def get_instructions(lines: list[str]) -> list[Instruction]:
  inst: list[Instruction] = []

  for line in lines:
    if line == 'noop':
      inst.append(NoOp())
    elif line.startswith('addx'):
      _, value = line.split(' ', 2)
      inst.append(AddX(int(value)))

  return inst

def execute(instructions: list[Instruction]) -> list[int]:
  values: list[int] = [1]

  for instruction in instructions:
    if isinstance(instruction, NoOp):
      values.append(values[-1])
      continue

    if isinstance(instruction, AddX):
      values.append(values[-1])
      values.append(values[-1] + instruction.value)

  return values

def part1(values: list[int]) -> int:
  strength = 0

  for i in range(19, len(values), 40):
    strength += ((i + 1) * values[i])

  return strength

def part2(values: list[int]) -> str:
  output: list[str] = []

  i = iter(values)

  for _ in range(6):
    for p in range(40):
      v = next(i)
      if p in (v, v + 1, v - 1):
        output.append('#')
      else:
        output.append('.')
    output.append('\n')

  return ''.join(output)


data = get_lines('10/input.txt')
instructions = get_instructions(data)
values = execute(instructions)

print(f"Part 1: {part1(values)}")
print(f"Part 2:\n{part2(values)}")
