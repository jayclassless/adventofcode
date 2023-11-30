from pathlib import Path


DATA_DIR = Path(__file__).parent / "data"


def get_lines(file: Path | str) -> list[str]:
    with open(DATA_DIR / file) as fp:
        return [line.rstrip("\n") for line in fp.readlines()]
