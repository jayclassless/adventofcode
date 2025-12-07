# https://adventofcode.com/2025/day/4

from aoc2025.util import get_lines, Grid, Cell


CHAR_ROLL = "@"
CHAR_REMOVED = "x"


def make_grid(input: str) -> Grid:
    return Grid(values=[list(line) for line in get_lines(input)])


def find_accessible(grid: Grid) -> list[Cell]:
    accessible = []

    for x in range(grid.width):
        for y in range(grid.height):
            cell = grid.cell(x, y)
            if cell is None or cell.value != CHAR_ROLL:
                continue

            rolls = [n for n in grid.neighbors(cell) if n.value == CHAR_ROLL]

            if len(rolls) < 4:
                accessible.append(cell)

    return accessible


def solve_part1(input: str) -> None:
    grid = make_grid(input)
    print(len(find_accessible(grid)))


def solve_part2(input: str) -> None:
    grid = make_grid(input)

    total = 0
    while len(accessible := find_accessible(grid)) > 0:
        total += len(accessible)

        for cell in accessible:
            grid.values[cell.y][cell.x] = CHAR_REMOVED

    print(total)


solve_part1("4/example1.txt")
solve_part1("4/input1.txt")
solve_part2("4/example1.txt")
solve_part2("4/input1.txt")
