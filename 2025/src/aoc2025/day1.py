# https://adventofcode.com/2025/day/1

from dataclasses import dataclass

from aoc2025.util import get_lines, Rotation


@dataclass
class Step:
    direction: Rotation
    value: int


def get_steps(input: str) -> list[Step]:
    steps = []
    for line in get_lines(input):
        steps.append(
            Step(Rotation.LEFT if line[0] == "L" else Rotation.RIGHT, int(line[1:]))
        )
    return steps


def solve_part1(input: str) -> None:
    positions = [50]
    for step in get_steps(input):
        dir = -1 if step.direction == Rotation.LEFT else 1
        new_pos = (positions[-1] + (dir * step.value)) % 100
        positions.append(new_pos)

    print(len([p for p in positions if p == 0]))


def solve_part2(input: str) -> None:
    saw_zero = 0
    last_position = 50
    for step in get_steps(input):
        dir = -1 if step.direction == Rotation.LEFT else 1

        new_pos = (last_position + (dir * step.value)) % 100
        if step.direction == Rotation.LEFT:
            saw_zero += (((100 - last_position) % 100) + step.value) // 100
        else:
            saw_zero += (last_position + step.value) // 100

        last_position = new_pos

    print(saw_zero)


solve_part1("1/example1.txt")
solve_part1("1/input1.txt")
solve_part2("1/example1.txt")
solve_part2("1/input1.txt")
