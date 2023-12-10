# https://adventofcode.com/2023/day/4

import re
from dataclasses import dataclass

from .util import get_lines


@dataclass
class Card:
    id: int
    winning_numbers: list[int]
    drawn_numbers: list[int]
    copies: int = 1

    @classmethod
    def from_line(cls, line: str) -> "Card":
        label, numbers = line.split(":", 1)

        id = int(label.split(" ")[-1])

        winning, drawn = numbers.split("|")
        winning_numbers = [int(i) for i in re.split(r"\s+", winning.strip())]
        drawn_numbers = [int(i) for i in re.split(r"\s+", drawn.strip())]

        return Card(
            id=id,
            winning_numbers=winning_numbers,
            drawn_numbers=drawn_numbers,
        )

    @property
    def matches(self) -> list[int]:
        return list(set(self.winning_numbers) & set(self.drawn_numbers))

    @property
    def points(self) -> int:
        p = 0

        winning = self.matches
        if winning:
            p = 1
            winning.pop()
            while winning:
                p *= 2
                winning.pop()

        return p


def solve_part1(input: str) -> None:
    print(input)

    data = get_lines(input)
    cards = [Card.from_line(line) for line in data]

    total = sum([card.points for card in cards])

    print(f"Total Score: {total}")


def solve_part2(input: str) -> None:
    print(input)

    data = get_lines(input)
    cards = [Card.from_line(line) for line in data]

    for idx in range(len(cards)):
        matches = cards[idx].matches
        if not matches:
            continue
        for i in range(len(matches)):
            cards[idx + i + 1].copies += cards[idx].copies

    total = sum([card.copies for card in cards])

    print(f"Total Cards: {total}")


solve_part1("4/example1.txt")
solve_part1("4/input1.txt")
solve_part2("4/example2.txt")
solve_part2("4/input2.txt")
