# https://adventofcode.com/2023/day/8

import re
from dataclasses import dataclass
from itertools import cycle
from math import lcm

from .util import get_lines


@dataclass
class Node:
    name: str
    left: str
    right: str


class Map:
    pattern: str
    nodes: dict[str, Node]

    def __init__(self):
        self.nodes = {}

    def add_node(self, name: str, left_name: str, right_name: str) -> None:
        self.nodes[name] = Node(name=name, left=left_name, right=right_name)


def parse_map(lines: list[str]) -> Map:
    m = Map()

    m.pattern = lines[0]

    for line in lines[2:]:
        match = re.match(
            r"(?P<name>.{3}) = \((?P<left_name>.{3}), (?P<right_name>.{3})\)", line
        )
        if not match:
            continue
        m.add_node(**match.groupdict())

    return m


def traverse(map: Map, start: str) -> int:
    steps = 0

    current = map.nodes[start]
    for direction in cycle(map.pattern):
        if direction == "L":
            current = map.nodes[current.left]
        else:
            current = map.nodes[current.right]

        steps += 1

        if current.name.endswith("Z"):
            break

    return steps


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = parse_map(data)

    steps = traverse(map, "AAA")

    print(f"Steps: {steps}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = parse_map(data)

    steps = {
        node: traverse(map, node) for node in map.nodes.keys() if node.endswith("A")
    }

    print(f"Steps: {lcm(*steps.values())}")


solve_part1("8/example1.txt")
solve_part1("8/input1.txt")
solve_part2("8/example2.txt")
solve_part2("8/input2.txt")
