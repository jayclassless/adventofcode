from itertools import zip_longest
from pathlib import Path
from typing import Iterable


DATA_DIR = Path(__file__).parent / 'data'


def get_lines(file: Path | str) -> list[str]:
    with open(DATA_DIR / file) as fp:
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
