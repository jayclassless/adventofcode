# https://adventofcode.com/2025/day/10

from collections import deque
from dataclasses import dataclass

from aoc2025.util import solve, get_lines


Indicators = tuple[bool, ...]
Button = tuple[int, ...]


@dataclass
class Machine:
    target_indicators: Indicators
    buttons: list[Button]
    joltages: list[int]

    @classmethod
    def from_input(cls, input: str) -> "Machine":
        parts = input.split(" ")

        indicators = tuple(c == "#" for c in parts[0][1:-1])

        buttons = [
            tuple([int(value) for value in part[1:-1].split(",")])
            for part in parts[1:-1]
        ]

        joltages = [int(value) for value in parts[-1][1:-1].split(",")]

        return cls(indicators, buttons, joltages)

    @property
    def initial_indicators(self) -> Indicators:
        return (False,) * len(self.target_indicators)

    def press_button(self, indicators: Indicators, button: Button) -> Indicators:
        return tuple(
            [
                not light if idx in button else light
                for idx, light in enumerate(indicators)
            ]
        )

    def fewest_presses(self) -> int:
        queue = deque([(self.initial_indicators, 0)])
        seen = {self.initial_indicators}

        while len(queue) > 0:
            current, presses = queue.popleft()

            if current == self.target_indicators:
                return presses

            for button in self.buttons:
                next_state = self.press_button(current, button)

                if next_state not in seen:
                    seen.add(next_state)
                    queue.append((next_state, presses + 1))

        return -1


def part1(input: str) -> int:
    machines = [Machine.from_input(line) for line in get_lines(input)]

    return sum([machine.fewest_presses() for machine in machines])


def part2(input: str) -> int:
    pass


solve("10/example1.txt", part1, 7)
solve("10/input1.txt", part1, 498)
# solve("10/example1.txt", part2)
# solve("10/input1.txt", part2)
