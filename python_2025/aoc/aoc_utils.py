from pathlib import Path


def load_input(day, as_lines=False):
    with open(f"{Path(__file__).parent.parent}/inputs/day{day:02d}.txt") as f:
        if as_lines:
            return f.readlines()
        return f.read()
