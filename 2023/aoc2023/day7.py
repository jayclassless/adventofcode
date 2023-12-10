# https://adventofcode.com/2023/day/7

import functools
from collections import Counter
from typing import TypeAlias, Callable

from .util import get_lines


Round: TypeAlias = tuple[str, int]


def parse_rounds(lines: list[str]) -> list[tuple[str, int]]:
    rounds = []
    for line in lines:
        hand, bid = line.split(" ", 1)
        rounds.append((hand, int(bid)))
    return rounds


def score_hand(hand: str) -> int:
    most_common = Counter(hand).most_common(2)

    if most_common[0][1] == 5:
        return 7
    if most_common[0][1] == 4:
        return 6
    if most_common[0][1] == 3 and most_common[1][1] == 2:
        return 5
    if most_common[0][1] == 3:
        return 4
    if most_common[0][1] == 2 and most_common[1][1] == 2:
        return 3
    if most_common[0][1] == 2:
        return 2

    return 1


def score_wild_hand(hand: str) -> int:
    counts = Counter(hand)
    common = counts.most_common(2)

    if "J" in counts and len(common) > 1:
        if common[0][0] == "J":
            most_common = common[1][0]
        else:
            most_common = common[0][0]
        hand = hand.replace("J", most_common)

    return score_hand(hand)


def compare_high_card(ranking: str, a: str, b: str) -> int:
    for i in range(5):
        value_a = ranking.find(a[i])
        value_b = ranking.find(b[i])
        if value_a == value_b:
            continue
        if value_a > value_b:
            return -1
        elif value_b > value_a:
            return 1
        else:
            return 0
    return 0


def compare_rounds(
    scorer: Callable[[str], int], ranking: str, a: Round, b: Round
) -> int:
    score_a = scorer(a[0])
    score_b = scorer(b[0])

    if score_a == score_b:
        return compare_high_card(ranking, a[0], b[0])
    elif score_a > score_b:
        return 1
    else:
        return -1


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    rounds = sorted(
        parse_rounds(data),
        key=functools.cmp_to_key(
            functools.partial(compare_rounds, score_hand, "AKQJT98765432")
        ),
    )

    winnings = sum([round[1] * idx for idx, round in enumerate(rounds, start=1)])

    print(f"Winnings: {winnings}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    rounds = sorted(
        parse_rounds(data),
        key=functools.cmp_to_key(
            functools.partial(compare_rounds, score_wild_hand, "AKQT98765432J")
        ),
    )

    winnings = sum([round[1] * idx for idx, round in enumerate(rounds, start=1)])

    print(f"Winnings: {winnings}")


solve_part1("7/example1.txt")
solve_part1("7/input1.txt")
solve_part2("7/example2.txt")
solve_part2("7/input2.txt")
