# https://adventofcode.com/2023/day/13

from typing import TypeAlias

from .util import get_lines


Block: TypeAlias = list[str]


def parse_blocks(lines: list[str]) -> list[Block]:
    blocks: list[Block] = []

    block: Block = []
    for line in lines:
        if not line:
            blocks.append(block)
            block = []
            continue

        block.append(line)

    if block:
        blocks.append(block)

    return blocks


def differences(a: str, b: str) -> int:
    return sum([1 if aa != bb else 0 for aa, bb in zip(a, b)])


def rotate(block: Block) -> Block:
    return [list(x) for x in zip(*block)]


def reflected_row(block: Block, threshold: int) -> int:
    for idx in range(1, len(block)):
        if (
            sum(differences(a, b) for a, b in zip(reversed(block[:idx]), block[idx:]))
            == threshold
        ):
            return idx

    return 0


def score_block(block: Block, threshold: int) -> int:
    row = reflected_row(block, threshold)
    if row:
        return row * 100

    col = reflected_row(rotate(block), threshold)
    if col:
        return col

    return 0


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    blocks = parse_blocks(data)
    scores = [score_block(b, 0) for b in blocks]

    print(f"Summary: {sum(scores)}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    blocks = parse_blocks(data)
    scores = [score_block(b, 1) for b in blocks]

    print(f"Summary: {sum(scores)}")


solve_part1("13/example1.txt")
solve_part1("13/input1.txt")
solve_part2("13/example2.txt")
solve_part2("13/input2.txt")
