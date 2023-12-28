# https://adventofcode.com/2023/day/11

from .util import get_lines, Point


def find_galaxies(map: list[str], expansion=1) -> list[Point]:
    galaxies = [
        Point(x=x, y=y)
        for x in range(len(map[0]))
        for y in range(len(map))
        if map[y][x] == "#"
    ]

    y = 0
    while y < max([g.y for g in galaxies]):
        if not [g for g in galaxies if g.y == y]:
            for galaxy in galaxies:
                if galaxy.y >= y:
                    galaxy.y += expansion - 1
            y += expansion - 1
        y += 1

    x = 0
    while x < max([g.x for g in galaxies]):
        if not [g for g in galaxies if g.x == x]:
            for galaxy in galaxies:
                if galaxy.x >= x:
                    galaxy.x += expansion - 1
            x += expansion - 1
        x += 1

    return galaxies


def find_lengths(galaxies: list[Point]) -> list[int]:
    lengths: list[int] = []

    for i, galaxy in enumerate(galaxies):
        for next_galaxy in galaxies[i + 1 :]:
            lengths.append(
                abs(galaxy.x - next_galaxy.x) + abs(galaxy.y - next_galaxy.y)
            )

    return lengths


def solve(input: str, expansion) -> None:
    print(f"{input} @ {expansion}")
    data = get_lines(input)

    galaxies = find_galaxies(data, expansion)

    lengths = find_lengths(galaxies)

    print(f"Sum of Lengths: {sum(lengths)}")


solve("11/example1.txt", 1)
solve("11/input1.txt", 1)
solve("11/example2.txt", 10)
solve("11/example2.txt", 100)
solve("11/input2.txt", 1000000)
