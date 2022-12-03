# https://adventofcode.com/2022/day/3

import string

from functools import reduce
from itertools import zip_longest
from pathlib import Path
from typing import Iterable


def get_lines(file: Path | str) -> list[str]:
    with open(file) as fp:
        return [
            line.rstrip('\n')
            for line in fp.readlines()
        ]

# https://docs.python.org/3/library/itertools.html#itertools-recipes
def grouper(iterable: Iterable[str], n, *, incomplete='fill', fillvalue=None):
    args = [iter(iterable)] * n
    if incomplete == 'fill':
        return zip_longest(*args, fillvalue=fillvalue)
    if incomplete == 'strict':
        return zip(*args, strict=True)
    if incomplete == 'ignore':
        return zip(*args)
    else:
        raise ValueError('Expected fill, strict, or ignore')

def find_common_item(sacks: Iterable[str]) -> str:
    return ''.join(reduce(
        lambda x, y: set(x).intersection(set(y)),
        sacks
    ))


PRIORITIES = {
    c: i + 1
    for i, c in enumerate(string.ascii_lowercase + string.ascii_uppercase)
}

EXAMPLE = Path(__file__).parent / 'example.txt'
ACTUAL = Path(__file__).parent / 'input.txt'

data = get_lines(ACTUAL)

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
