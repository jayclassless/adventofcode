# https://adventofcode.com/2025/day/6

import operator
from functools import reduce

from aoc2025.util import get_lines


OPER_ADD = "+"
OPER_MULT = "*"


def solve_part1(input: str) -> None:
    lines = get_lines(input)

    operators = lines[-1].split()

    values = []
    for line in lines[:-1]:
        values.append([int(v) for v in line.split()])

    total = 0
    for i in range(len(operators)):
        if operators[i] == OPER_ADD:
            total += sum([v[i] for v in values])
        elif operators[i] == OPER_MULT:
            total += reduce(operator.mul, [v[i] for v in values])

    print(total)


def solve_part2(input: str) -> None:
    lines = get_lines(input)

    operators = [
        (oper, idx)
        for idx, oper in enumerate(lines[-1])
        if oper in (OPER_ADD, OPER_MULT)
    ]

    values = []
    end = len(lines[-1]) - 1
    for oper in reversed(operators):
        start = oper[1]

        column = []
        for i in range(start, end + 1):
            column.append(int("".join([line[i] for line in lines[:-1]])))
        values.append(column)

        end = start - 2

    total = 0
    opers = [o[0] for o in reversed(operators)]
    for i in range(len(operators)):
        if opers[i] == OPER_ADD:
            total += sum(values[i])
        elif opers[i] == OPER_MULT:
            total += reduce(operator.mul, values[i])

    print(total)


solve_part1("6/example1.txt")
solve_part1("6/input1.txt")
solve_part2("6/example1.txt")
solve_part2("6/input1.txt")
