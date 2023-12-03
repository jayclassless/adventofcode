# https://adventofcode.com/2023/day/2

from dataclasses import dataclass
from math import prod

from .util import get_lines


@dataclass
class Game:
    id: int
    sets: list[tuple[int, int, int]]

    def is_possible(self, red, blue, green):
        for s in self.sets:
            if s[0] > red or s[1] > blue or s[2] > green:
                return False
        return True

    def minimum_bag(self) -> tuple[int, int, int]:
        return (
            max([s[0] for s in self.sets]),
            max([s[1] for s in self.sets]),
            max([s[2] for s in self.sets]),
        )


def parse_line(line: str) -> Game:
    label, sets = line.split(":", 1)

    parsed_sets = []
    for s in sets.split(";"):
        red = blue = green = 0
        for cube in s.split(","):
            num, kind = cube.strip().split(" ")
            if kind == "red":
                red = int(num)
            elif kind == "blue":
                blue = int(num)
            elif kind == "green":
                green = int(num)
        parsed_sets.append((red, blue, green))

    return Game(
        id=int(label.split(" ")[-1]),
        sets=parsed_sets,
    )


def solve_part1(input: str) -> None:
    print(input)

    data = get_lines(input)
    games = [parse_line(line) for line in data]

    possible = [game.id for game in games if game.is_possible(12, 14, 13)]

    print(f"Sum of Possible: {sum(possible)}")


def solve_part2(input: str) -> None:
    print(input)

    data = get_lines(input)
    games = [parse_line(line) for line in data]

    powers = [prod(game.minimum_bag()) for game in games]

    print(f"Sum of Powers: {sum(powers)}")


solve_part1("2/example1.txt")
solve_part1("2/input1.txt")
solve_part2("2/example2.txt")
solve_part2("2/input2.txt")
