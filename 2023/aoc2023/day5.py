# https://adventofcode.com/2023/day/5

from functools import cache
from itertools import batched

from .util import get_lines


class Mapping:
    source: str
    destination: str
    maps: dict[range, int]

    def __init__(self):
        self.maps = {}

    @classmethod
    def from_header(cls, line: str) -> "Mapping":
        names = line.split(" ")[0]
        source, destination = names.split("-to-")

        m = Mapping()
        m.source = source
        m.destination = destination

        return m

    def add_mapping(self, line: str) -> None:
        dest_start, source_start, range_len = [int(i) for i in line.split(" ")]
        self.maps[range(source_start, (source_start + range_len))] = (
            source_start - dest_start
        )

    def destination_for(self, source: int) -> int:
        for source_rng, adjustment in self.maps.items():
            if source in source_rng:
                return source - adjustment
        return source


class Almanac:
    maps: dict[str, Mapping]

    def __init__(self):
        self.maps = {}

    def add_map(self, map: Mapping):
        self.maps[map.source] = map

    # @cache
    def location_for(self, source: int, map_type: str) -> int:
        map = self.maps.get(map_type)
        if not map:
            return source
        dest = map.destination_for(source)
        return self.location_for(dest, map.destination)


def parse_almanac(lines: list[str]) -> Almanac:
    almanac = Almanac()

    map = None
    for idx in range(len(lines)):
        if not map:
            map = Mapping.from_header(lines[idx])
            almanac.add_map(map)
            continue

        if not lines[idx]:
            map = None
            continue

        map.add_mapping(lines[idx])

    return almanac


def solve_part1(input: str) -> None:
    print(input)
    data = get_lines(input)

    seeds = [int(i) for i in data[0].split(":", 1)[-1].strip().split(" ")]
    almanac = parse_almanac(data[2:])

    locations = [almanac.location_for(seed, "seed") for seed in seeds]

    print(f"Lowest location: {min(locations)}")


def solve_part2(input: str) -> None:
    print(input)
    data = get_lines(input)

    seeds_ranges = [int(i) for i in data[0].split(":", 1)[-1].strip().split(" ")]
    almanac = parse_almanac(data[2:])

    lowest_location = None
    for start, count in batched(seeds_ranges, 2):
        for seed in range(start, (start + count)):
            location = almanac.location_for(seed, "seed")
            if lowest_location is None or location < lowest_location:
                lowest_location = location

    print(f"Lowest location: {lowest_location}")


solve_part1("5/example1.txt")
solve_part1("5/input1.txt")
solve_part2("5/example2.txt")
# TODO: brute force too slow
# solve_part2("5/input2.txt")
