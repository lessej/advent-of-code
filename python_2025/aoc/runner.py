import importlib
import sys
from datetime import datetime


def main():
    day = int(sys.argv[1])
    solution = importlib.import_module(f"solutions.day{day:02d}")
    start = datetime.now()

    part_1 = solution.part_1()
    part_2 = solution.part_2()

    end = datetime.now()
    print(f"Part 1: {part_1} \nPart 2: {part_2}")
    print(f"Day {day:02d} took {int((end - start).total_seconds() * 1000)}ms")


if __name__ == "__main__":
    main()
