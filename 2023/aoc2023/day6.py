# https://adventofcode.com/2023/day/6

import re
from math import prod

from .util import get_lines


def get_races(lines: list[str]) -> list[tuple[int, int]]:
    times = [int(n) for n in re.findall(r"\d+", lines[0])]
    distances = [int(n) for n in re.findall(r"\d+", lines[1])]
    return list(zip(times, distances))


def winning_plays(time, record) -> list[int]:
    plays = []

    for hold in range(1, time):
        distance = hold * (time - hold)
        if distance > record:
            plays.append(hold)

    return plays


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    times = [int(n) for n in re.findall(r"\d+", data[0])]
    records = [int(n) for n in re.findall(r"\d+", data[1])]
    races = zip(times, records)

    plays = [winning_plays(race[0], race[1]) for race in races]
    margin = prod([len(play) for play in plays])

    print(f"Margin: {margin}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    time = int("".join(re.findall(r"\d+", data[0])))
    record = int("".join(re.findall(r"\d+", data[1])))

    plays = winning_plays(time, record)

    print(f"Winning Plays: {len(plays)}")


solve_part1("6/example1.txt")
solve_part1("6/input1.txt")
solve_part2("6/example2.txt")
solve_part2("6/input2.txt")
