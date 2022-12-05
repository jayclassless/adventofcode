# https://adventofcode.com/2022/day/4

from .util import get_lines

Assignment = tuple[int, int]

def get_pair(line: str) -> tuple[Assignment, Assignment]:
    pair = line.split(',')
    first = tuple(map(int, pair[0].split('-')))
    second = tuple(map(int, pair[1].split('-')))
    return (
        first,
        second
    )

def is_contained_in(first: Assignment, second: Assignment) -> bool:
    return (first[0] >= second[0]) and (first[1] <= second[1])

def either_contains_other(first: Assignment, second: Assignment) -> bool:
    return is_contained_in(first, second) or is_contained_in(second, first)

def overlaps(first: Assignment, second: Assignment) -> bool:
    return (
        ((first[0] <= second[0]) and (first[1] >= second[0]))
        or ((first[0] <= second[1]) and (first[1] >= second[1]))
        or ((first[0] >= second[0]) and (first[1] <= second[1]))
    )


data = get_lines('4/input.txt')
pairs = [get_pair(line) for line in data]

num_fully_contained = len([
    pair
    for pair in pairs
    if either_contains_other(pair[0], pair[1])
])
print(f"Fully Contained: {num_fully_contained}")

num_overlaps = len([
    pair
    for pair in pairs
    if overlaps(pair[0], pair[1])
])
print(f"Overlaps: {num_overlaps}")
