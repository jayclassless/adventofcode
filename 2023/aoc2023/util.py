from dataclasses import dataclass
from pathlib import Path


DATA_DIR = Path(__file__).parent / "data"


def get_lines(file: Path | str) -> list[str]:
    with open(DATA_DIR / file) as fp:
        return [line.rstrip("\n") for line in fp.readlines()]


@dataclass
class Point:
    x: int
    y: int

    def __hash__(self) -> int:
        return hash((self.x, self.y))
