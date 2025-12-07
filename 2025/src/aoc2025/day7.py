# https://adventofcode.com/2025/day/7

from collections import defaultdict

from aoc2025.util import Grid


CHAR_START = "S"
CHAR_SPLIT = "^"


def solve_part1(input: str) -> None:
    grid = Grid.from_input(input)

    splits = 0
    beams = {grid.values[0].index(CHAR_START)}

    for y in range(1, grid.height):
        for x in list(beams):
            cell = grid.cell(x, y)
            if not cell:
                continue  # ??

            if cell.value == CHAR_SPLIT:
                splits += 1
                beams.remove(x)
                beams.add(x + 1)
                beams.add(x - 1)

    print(splits)


def solve_part2(input: str) -> None:
    grid = Grid.from_input(input)

    beams = defaultdict(lambda: 0)
    beams[grid.values[0].index(CHAR_START)] = 1

    for y in range(1, grid.height):
        for x, num_paths in list(beams.items()):
            cell = grid.cell(x, y)
            if not cell:
                continue  # ??

            if cell.value == CHAR_SPLIT:
                del beams[x]
                beams[x + 1] += num_paths
                beams[x - 1] += num_paths

    print(sum(beams.values()))


solve_part1("7/example1.txt")
solve_part1("7/input1.txt")
solve_part2("7/example1.txt")
solve_part2("7/input1.txt")
