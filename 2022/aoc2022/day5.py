# https://adventofcode.com/2022/day/5

import re

from .util import get_lines


State = list[list[str]]
Step = tuple[int, int, int]


def get_puzzle(lines: list[str]) -> tuple[State, list[Step]]:
  state_lines: list[str] = []
  steps: list[Step] = []

  is_step = False
  for line in lines:
    if not line:
      is_step = True
      continue

    if is_step:
      parts = line.split(' ')
      steps.append((
        int(parts[1]),
        int(parts[3]),
        int(parts[5])
      ))

    else:
      state_lines.append(line)

  num_stacks = int(re.split(r'\s+', state_lines[-1].strip())[-1])
  state: State = []
  for i in range(0, num_stacks):
    state.append(list())

  crate_positions = list(map(lambda x: (x * 4) + 1, range(0, num_stacks)))
  for line in state_lines[:-1]:
    crates = [
      line[position].strip()
      for position in crate_positions
    ]
    for idx, crate in enumerate(crates):
      if not crate:
        continue
      state[idx].insert(0, crate)

  return state, steps

def execute_step_part1(state: State, step: Step):
  from_pos = step[1] - 1
  to_pos = step[2] - 1

  for _ in range(0, step[0]):
    crate = state[from_pos].pop()
    state[to_pos].append(crate)

def execute_step_part2(state: State, step: Step):
  from_pos = step[1] - 1
  to_pos = step[2] - 1

  starting = state[from_pos]
  crates = starting[len(starting) - step[0]:]
  remaining = starting[:-1 * step[0]]

  state[from_pos] = remaining
  state[to_pos].extend(crates)

def get_tops(state: State) -> str:
  tops = ''

  for stack in state:
    tops += stack[-1]

  return tops


data = get_lines('5/input.txt')

state, steps = get_puzzle(data)
for step in steps:
  execute_step_part1(state, step)
print(f"Part 1 Top Crates: {get_tops(state)}")

state, steps = get_puzzle(data)
for step in steps:
  execute_step_part2(state, step)
print(f"Part 2 Top Crates: {get_tops(state)}")
