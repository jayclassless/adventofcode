# https://adventofcode.com/2023/day/15

from collections import defaultdict

from .util import get_lines


class Hash:
    def __init__(self):
        self.value = 0

    def update(self, input: str) -> int:
        current = self.value

        for char in input:
            current += ord(char)
            current *= 17
            current %= 256

        self.value = current
        return self.value


class HashMap:
    def __init__(self) -> None:
        self.boxes: dict[int, list[tuple[str, int]]] = defaultdict(list)

    def update(self, input: str):
        if input.endswith("-"):
            label = input[:-1]
            box = Hash().update(label)
            self.boxes[box] = [lens for lens in self.boxes[box] if lens[0] != label]

        else:
            label, focal = input.split("=")
            box = Hash().update(label)

            for i in range(len(self.boxes[box])):
                if self.boxes[box][i][0] == label:
                    self.boxes[box][i] = (label, int(focal))
                    break
            else:
                self.boxes[box].append((label, int(focal)))

    def total_focusing_power(self) -> int:
        powers = []

        for box in range(256):
            for idx, lens in enumerate(self.boxes[box]):
                powers.append((1 + box) * (1 + idx) * lens[1])

        return sum(powers)


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    results = [Hash().update(step) for step in data[0].split(",")]

    print(f"Sum of Results: {sum(results)}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    map = HashMap()
    for step in data[0].split(","):
        map.update(step)

    print(f"Total Focusing Power: {map.total_focusing_power()}")


solve_part1("15/example1.txt")
solve_part1("15/input1.txt")
solve_part2("15/example2.txt")
solve_part2("15/input2.txt")
