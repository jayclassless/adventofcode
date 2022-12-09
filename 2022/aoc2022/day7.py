# https://adventofcode.com/2022/day/7

from collections import defaultdict

from .util import get_lines


ROOT = ''

def get_dir_sizes(lines: list[str]) -> dict[str, int]:
    sizes = defaultdict(lambda: 0)

    stack: list[str] = []
    for line in lines:
        match line.split():
            case '$', 'cd', '/': stack = [ROOT]
            case '$', 'cd', '..': stack.pop()
            case '$', 'cd', name: stack.append(name)
            case '$', 'ls': pass
            case 'dir', _: pass
            case size, _:
                for i in range(len(stack)):
                    path = '/'.join(stack[0:(i + 1)])
                    sizes[path] += int(size)

    return dict(sizes)

def part1(sizes: dict[str, int]) -> int:
    return sum([
        value
        for value in sizes.values()
        if value <= 100000
    ])

def part2(sizes: dict[str, int]) -> int:
    total = 70000000
    needed = 30000000
    used = sizes[ROOT]

    for value in sorted(sizes.values()):
        if (total - (used - value)) >= needed:
            return value

    return -1


data = get_lines('7/input.txt')
sizes = get_dir_sizes(data)
print(f"Part 1: {part1(sizes)}")
print(f"Part 2: {part2(sizes)}")
