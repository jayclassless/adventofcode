# https://adventofcode.com/2022/day/3

import string

from functools import reduce
from typing import Iterable

from .util import get_lines, grouper


def find_common_item(sacks: Iterable[str]) -> str:
    return ''.join(reduce(
        lambda x, y: set(x).intersection(set(y)),
        sacks
    ))


PRIORITIES = {
    c: i + 1
    for i, c in enumerate(string.ascii_lowercase + string.ascii_uppercase)
}

data = get_lines('3/input.txt')

common_items = [
    find_common_item([sack[:len(sack)//2], sack[len(sack)//2:]])
    for sack in data
]
total_priority = sum([PRIORITIES[item] for item in common_items])
print(f"Part 1 Total Priority: {total_priority}")

common_items = [
    find_common_item(group)
    for group in grouper(data, 3)
]
total_priority = sum([PRIORITIES[item] for item in common_items])
print(f"Part 2 Total Priority: {total_priority}")
