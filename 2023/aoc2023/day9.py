# https://adventofcode.com/2023/day/9

from .util import get_lines


def parse_input(lines: list[str]) -> list[list[int]]:
    return [[int(n) for n in line.split(" ")] for line in lines]


def extropolate(seq: list[int]) -> int:
    diffs: list[list[int]] = [seq]

    while True:
        diffs.append([seq[i] - seq[i - 1] for i in range(1, len(seq))])
        all_zeros = not any(diffs[-1])
        if all_zeros:
            break
        seq = diffs[-1]

    for i in range(len(diffs) - 2, -1, -1):
        prev_diff = diffs[i + 1][-1]
        diffs[i].append(diffs[i][-1] + prev_diff)

    return diffs[0][-1]


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    seqs = parse_input(data)

    next_values = [extropolate(seq) for seq in seqs]

    print(f"Sum of Extropolations: {sum(next_values)}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    seqs = [list(reversed(seq)) for seq in parse_input(data)]

    next_values = [extropolate(seq) for seq in seqs]

    print(f"Sum of Extropolations: {sum(next_values)}")


solve_part1("9/example1.txt")
solve_part1("9/input1.txt")
solve_part2("9/example2.txt")
solve_part2("9/input2.txt")
