# https://adventofcode.com/2025/day/8

from functools import partial
from itertools import product

from aoc2025.util import solve, get_lines, Point3D


class CircuitTracker:
    circuits: list[set[Point3D]]

    def __init__(self):
        self.circuits = []

    def connect(self, a: Point3D, b: Point3D) -> None:
        a_circuit = [idx for idx, c in enumerate(self.circuits) if a in c]
        b_circuit = [idx for idx, c in enumerate(self.circuits) if b in c]

        if a_circuit and b_circuit:
            if a_circuit == b_circuit:
                return
            self.circuits[a_circuit[0]].update(self.circuits[b_circuit[0]])
            del self.circuits[b_circuit[0]]
        elif a_circuit:
            self.circuits[a_circuit[0]].add(b)
        elif b_circuit:
            self.circuits[b_circuit[0]].add(a)
        else:
            self.circuits.append(set([a, b]))


def get_points(input: str) -> list[Point3D]:
    return [
        Point3D(*[int(part) for part in line.split(",")]) for line in get_lines(input)
    ]


def get_ordered_distances(
    points: list[Point3D],
) -> dict[tuple[Point3D, Point3D], float]:
    distances = {}
    for a, b in product(points, points):
        if a == b or (a, b) in distances or (b, a) in distances:
            continue
        distances[(a, b)] = a.distance_from(b)
    return dict(sorted(distances.items(), key=lambda x: x[1]))


def part1(input: str, connections: int) -> int:
    points = get_points(input)
    ordered_distances = get_ordered_distances(points)

    circuits = CircuitTracker()
    for a, b in list(ordered_distances)[:connections]:
        circuits.connect(a, b)

    largest_circuits = sorted(circuits.circuits, key=lambda x: len(x), reverse=True)
    return (
        len(largest_circuits[0]) * len(largest_circuits[1]) * len(largest_circuits[2])
    )


def part2(input: str) -> int | None:
    points = get_points(input)
    total_points = len(points)
    ordered_distances = get_ordered_distances(points)

    circuits = CircuitTracker()
    for a, b in ordered_distances:
        circuits.connect(a, b)
        if len(circuits.circuits) == 1 and len(circuits.circuits[0]) == total_points:
            return a.x * b.x

    return None


solve("8/example1.txt", partial(part1, connections=10), 40)
solve("8/input1.txt", partial(part1, connections=1000), 123420)
solve("8/example1.txt", part2, 25272)
solve("8/input1.txt", part2, 673096646)
